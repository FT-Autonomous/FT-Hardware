# FireBeetle ESP32 multi-device serial hub.
#
# Auto-discovers every CH340-based FireBeetle on the host, opens a serial port
# per device on its own reader thread, and routes inbound lines to a per-device
# queue + log file. Send to one device by id, or broadcast to all.
#
# Each FireBeetle is identified by its WiFi MAC (asked via "WHOAMI"), so a board
# gets the same dev_id regardless of which USB port it lands on. The mapping is
# persisted to a JSON file. New boards auto-claim the next free id.
#
# Discovery runs in a background thread, so it doesn't matter whether the boards
# boot before or after the script, or get plugged in mid-run.
#
# Usage:
#   python3 multi_serial.py
#   python3 multi_serial.py --id-map ./firebeetle_ids.json --logdir ./logs
#
# Interactive commands (stdin):
#   <id> <msg>      send <msg> to device <id>     e.g.  0 hello
#   all <msg>       broadcast <msg> to every device
#   list            show connected devices
#   quit            exit
#
# Requires: pip install pyserial

from __future__ import annotations

import argparse
import json
import queue
import sys
import threading
import time
from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path
from typing import Callable, Optional

import serial
from serial.tools import list_ports

BAUD_RATE = 115200
CH340_VID_PID = (0x1A86, 0x7523)
DISCOVERY_INTERVAL_S = 1.0
WHOAMI_TIMEOUT_S = 2.5
ID_MAP_DEFAULT = Path("firebeetle_ids.json")


@dataclass
class Device:
    dev_id: int
    mac: str
    port: str
    serial: serial.Serial
    rx_queue: "queue.Queue[str]" = field(default_factory=queue.Queue)
    log_path: Optional[Path] = None
    _stop: threading.Event = field(default_factory=threading.Event)
    _thread: Optional[threading.Thread] = None

    def send(self, msg: str) -> None:
        if not msg.endswith("\n"):
            msg += "\n"
        self.serial.write(msg.encode("utf-8", errors="replace"))
        self.serial.flush()

    def stop(self) -> None:
        self._stop.set()
        if self._thread:
            self._thread.join(timeout=1.0)
        if self.serial.is_open:
            try:
                self.serial.close()
            except serial.SerialException:
                pass


def discover_firebeetle_ports() -> list[str]:
    ports = []
    for p in list_ports.comports():
        if p.vid is not None and (p.vid, p.pid) == CH340_VID_PID:
            ports.append(p.device)
    if not ports:
        for p in list_ports.comports():
            if "USB" in p.device or "usbserial" in p.device:
                ports.append(p.device)
    return sorted(ports)


def open_serial_no_reset(port: str) -> serial.Serial:
    # Holding DTR/RTS low across open avoids toggling the ESP32 reset line, so
    # a board that's already running keeps running. Best-effort on Linux.
    s = serial.Serial()
    s.port = port
    s.baudrate = BAUD_RATE
    s.timeout = 0.1
    s.dtr = False
    s.rts = False
    s.open()
    return s


def probe_mac(s: serial.Serial, timeout_s: float = WHOAMI_TIMEOUT_S) -> Optional[str]:
    s.reset_input_buffer()
    s.write(b"WHOAMI\n")
    s.flush()
    deadline = time.monotonic() + timeout_s
    while time.monotonic() < deadline:
        raw = s.readline()
        if not raw:
            continue
        line = raw.decode("utf-8", errors="replace").strip()
        if line.startswith("ID:"):
            return line[3:].strip().upper()
    return None


def load_id_map(path: Path) -> dict[str, int]:
    if not path.exists():
        return {}
    return {k.upper(): int(v) for k, v in json.loads(path.read_text()).items()}


def save_id_map(path: Path, mapping: dict[str, int]) -> None:
    path.write_text(json.dumps(mapping, indent=2, sort_keys=True) + "\n")


class FireBeetleHub:
    def __init__(self, id_map_path: Path, logdir: Optional[Path] = None,
                 on_line: Optional[Callable[[Device, str], None]] = None):
        self.id_map_path = id_map_path
        self.mac_to_id: dict[str, int] = load_id_map(id_map_path)
        self.logdir = logdir
        self.on_line = on_line
        self.devices: dict[int, Device] = {}
        self.port_to_id: dict[str, int] = {}
        self._lock = threading.Lock()
        self._stop = threading.Event()
        self._discovery_thread: Optional[threading.Thread] = None
        self._failed_ports: set[str] = set()
        if logdir:
            logdir.mkdir(parents=True, exist_ok=True)

    def start(self) -> None:
        self._discovery_thread = threading.Thread(
            target=self._discovery_loop, daemon=True, name="firebeetle-discovery")
        self._discovery_thread.start()

    def _discovery_loop(self) -> None:
        while not self._stop.is_set():
            current = set(discover_firebeetle_ports())
            with self._lock:
                known_ports = set(self.port_to_id)
            for port in sorted(current - known_ports):
                self._try_add(port)
            for port in known_ports - current:
                self._drop(port, reason="unplugged")
            self._failed_ports &= current
            self._stop.wait(DISCOVERY_INTERVAL_S)

    def _try_add(self, port: str) -> None:
        try:
            s = open_serial_no_reset(port)
        except serial.SerialException as e:
            if port not in self._failed_ports:
                print(f"[hub] could not open {port}: {e}", file=sys.stderr)
                self._failed_ports.add(port)
            return

        mac = probe_mac(s)
        if mac is None:
            if port not in self._failed_ports:
                print(f"[hub] {port}: no WHOAMI reply (will retry)", file=sys.stderr)
                self._failed_ports.add(port)
            s.close()
            return
        self._failed_ports.discard(port)

        with self._lock:
            if mac not in self.mac_to_id:
                used = set(self.mac_to_id.values()) | set(self.devices)
                dev_id = next(i for i in range(256) if i not in used)
                self.mac_to_id[mac] = dev_id
                save_id_map(self.id_map_path, self.mac_to_id)
                print(f"[hub] new board {mac} assigned id {dev_id} "
                      f"(persisted to {self.id_map_path})")
            dev_id = self.mac_to_id[mac]
            if dev_id in self.devices:
                existing = self.devices[dev_id]
                print(f"[hub] id {dev_id} ({mac}) already connected on "
                      f"{existing.port}; ignoring duplicate on {port}",
                      file=sys.stderr)
                s.close()
                return
            log_path = None
            if self.logdir:
                safe_mac = mac.replace(":", "")
                log_path = self.logdir / f"device{dev_id}_{safe_mac}.log"
            dev = Device(dev_id=dev_id, mac=mac, port=port, serial=s, log_path=log_path)
            self.devices[dev_id] = dev
            self.port_to_id[port] = dev_id

        dev._thread = threading.Thread(
            target=self._reader, args=(dev,), daemon=True,
            name=f"firebeetle-rx-{dev_id}")
        dev._thread.start()
        print(f"[hub] device {dev_id} ({mac}) connected on {port}"
              + (f"  (log: {log_path})" if log_path else ""))

    def _drop(self, port: str, reason: str = "closed") -> None:
        with self._lock:
            dev_id = self.port_to_id.pop(port, None)
            dev = self.devices.pop(dev_id, None) if dev_id is not None else None
        if dev:
            print(f"[hub] device {dev.dev_id} ({dev.mac}) {reason} ({port})")
            dev.stop()

    def _reader(self, dev: Device) -> None:
        log_fp = open(dev.log_path, "a", buffering=1) if dev.log_path else None
        try:
            while not dev._stop.is_set():
                try:
                    raw = dev.serial.readline()
                except (serial.SerialException, OSError) as e:
                    print(f"[dev {dev.dev_id}] serial error: {e}", file=sys.stderr)
                    threading.Thread(
                        target=self._drop, args=(dev.port, "errored"),
                        daemon=True).start()
                    break
                if not raw:
                    continue
                line = raw.decode("utf-8", errors="replace").rstrip("\r\n")
                if not line:
                    continue
                if line.startswith("ID:"):
                    continue  # late WHOAMI reply, ignore
                ts = datetime.now().isoformat(timespec="milliseconds")
                dev.rx_queue.put(line)
                if log_fp:
                    log_fp.write(f"{ts}\t{line}\n")
                if self.on_line:
                    try:
                        self.on_line(dev, line)
                    except Exception as e:
                        print(f"[dev {dev.dev_id}] callback error: {e}", file=sys.stderr)
                else:
                    print(f"[dev {dev.dev_id}] {line}")
        finally:
            if log_fp:
                log_fp.close()

    def send(self, dev_id: int, msg: str) -> None:
        with self._lock:
            dev = self.devices.get(dev_id)
        if dev is None:
            print(f"[hub] no device with id {dev_id}", file=sys.stderr)
            return
        try:
            dev.send(msg)
        except (serial.SerialException, OSError) as e:
            print(f"[dev {dev_id}] send failed: {e}", file=sys.stderr)

    def broadcast(self, msg: str) -> None:
        with self._lock:
            devs = list(self.devices.values())
        for dev in devs:
            try:
                dev.send(msg)
            except (serial.SerialException, OSError) as e:
                print(f"[dev {dev.dev_id}] send failed: {e}", file=sys.stderr)

    def shutdown(self) -> None:
        self._stop.set()
        if self._discovery_thread:
            self._discovery_thread.join(timeout=2.0)
        with self._lock:
            devs = list(self.devices.values())
            self.devices.clear()
            self.port_to_id.clear()
        for dev in devs:
            dev.stop()


def repl(hub: FireBeetleHub) -> None:
    print("\nready. commands: '<id> <msg>', 'all <msg>', 'list', 'quit'\n")
    while True:
        try:
            line = input().strip()
        except (EOFError, KeyboardInterrupt):
            print()
            return
        if not line:
            continue
        if line == "quit":
            return
        if line == "list":
            with hub._lock:
                devs = sorted(hub.devices.values(), key=lambda d: d.dev_id)
            if not devs:
                print("  (no devices connected)")
            for dev in devs:
                print(f"  {dev.dev_id}: {dev.mac}  {dev.port}")
            continue

        head, _, msg = line.partition(" ")
        if head == "all":
            hub.broadcast(msg)
            continue
        if head.isdigit():
            hub.send(int(head), msg)
            continue
        print("usage: '<id> <msg>' | 'all <msg>' | 'list' | 'quit'")


def main() -> int:
    ap = argparse.ArgumentParser(description="FireBeetle ESP32 multi-device serial hub")
    ap.add_argument("--id-map", type=Path, default=ID_MAP_DEFAULT,
                    help="JSON file storing MAC->id mapping (auto-created)")
    ap.add_argument("--logdir", type=Path, default=None,
                    help="write per-device line logs to this directory")
    args = ap.parse_args()

    hub = FireBeetleHub(id_map_path=args.id_map, logdir=args.logdir)
    try:
        hub.start()
        repl(hub)
    finally:
        hub.shutdown()
    return 0


if __name__ == "__main__":
    sys.exit(main())
