# FireBeetle ESP32 multi-device serial hub.
#
# Auto-discovers every CH340-based FireBeetle on the host, opens a serial port
# per device on its own reader thread, and routes inbound lines to a per-device
# queue + log file. Send to one device by id, or broadcast to all.
#
# Usage:
#   python3 multi_serial.py # auto-discover, interactive
#   python3 multi_serial.py --ports /dev/ttyUSB0 /dev/ttyUSB1
#   python3 multi_serial.py --logdir ./logs
#
# Interactive commands (stdin):
#   <id> <msg>      send <msg> to device <id>     e.g.  0 hello
#   all <msg>       broadcast <msg> to every device
#   list            show connected devices
#   quit            exit
#
# Requires: pip install pyserial

import argparse
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
CH340_VID_PID = (0x1A86, 0x7523)  # DFRobot FireBeetle uses CH340 USB-serial
BOOT_DELAY_S = 2.0                # ESP32 resets on serial connect


@dataclass
class Device:
    dev_id: int
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
            self.serial.close()


def discover_firebeetle_ports() -> list[str]:
    ports = []
    for p in list_ports.comports():
        if p.vid is not None and (p.vid, p.pid) == CH340_VID_PID:
            ports.append(p.device)
    if not ports:
        # Fallback: any USB-serial-looking device on Linux/mac.
        for p in list_ports.comports():
            if "USB" in p.device or "usbserial" in p.device:
                ports.append(p.device)
    return sorted(ports)


class FireBeetleHub:
    def __init__(self, logdir: Optional[Path] = None,
                 on_line: Optional[Callable[[Device, str], None]] = None):
        self.devices: list[Device] = []
        self.logdir = logdir
        self.on_line = on_line
        if logdir:
            logdir.mkdir(parents=True, exist_ok=True)

    def connect(self, ports: list[str]) -> None:
        for i, port in enumerate(ports):
            try:
                s = serial.Serial(port, BAUD_RATE, timeout=0.1)
            except serial.SerialException as e:
                print(f"[hub] could not open {port}: {e}", file=sys.stderr)
                continue
            log_path = self.logdir / f"device{i}_{Path(port).name}.log" if self.logdir else None
            dev = Device(dev_id=i, port=port, serial=s, log_path=log_path)
            self.devices.append(dev)
            print(f"[hub] device {i} -> {port}"
                  + (f"  (log: {log_path})" if log_path else ""))

        if not self.devices:
            raise RuntimeError("no FireBeetle devices opened")

        time.sleep(BOOT_DELAY_S)

        for dev in self.devices:
            dev._thread = threading.Thread(
                target=self._reader, args=(dev,), daemon=True,
                name=f"firebeetle-rx-{dev.dev_id}")
            dev._thread.start()

    def _reader(self, dev: Device) -> None:
        log_fp = open(dev.log_path, "a", buffering=1) if dev.log_path else None
        try:
            while not dev._stop.is_set():
                try:
                    raw = dev.serial.readline()
                except serial.SerialException as e:
                    print(f"[dev {dev.dev_id}] serial error: {e}", file=sys.stderr)
                    break
                if not raw:
                    continue
                line = raw.decode("utf-8", errors="replace").rstrip("\r\n")
                if not line:
                    continue
                ts = datetime.now().isoformat(timespec="milliseconds")
                dev.rx_queue.put(line)
                if log_fp:
                    log_fp.write(f"{ts}\t{line}\n")
                if self.on_line:
                    try:
                        self.on_line(dev, line)
                    except Exception as e:  # never let a bad callback kill the reader
                        print(f"[dev {dev.dev_id}] callback error: {e}", file=sys.stderr)
                else:
                    print(f"[dev {dev.dev_id}] {line}")
        finally:
            if log_fp:
                log_fp.close()

    def send(self, dev_id: int, msg: str) -> None:
        self.devices[dev_id].send(msg)

    def broadcast(self, msg: str) -> None:
        for dev in self.devices:
            dev.send(msg)

    def shutdown(self) -> None:
        for dev in self.devices:
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
            for dev in hub.devices:
                print(f"  {dev.dev_id}: {dev.port}")
            continue

        head, _, msg = line.partition(" ")
        if head == "all":
            hub.broadcast(msg)
            continue
        if head.isdigit():
            idx = int(head)
            if 0 <= idx < len(hub.devices):
                hub.send(idx, msg)
                continue
        print("usage: '<id> <msg>' | 'all <msg>' | 'list' | 'quit'")


def main() -> int:
    ap = argparse.ArgumentParser(description="FireBeetle ESP32 multi-device serial hub")
    ap.add_argument("--ports", nargs="*", help="explicit port list (skips auto-discovery)")
    ap.add_argument("--logdir", type=Path, default=None,
                    help="write per-device line logs to this directory")
    args = ap.parse_args()

    ports = args.ports or discover_firebeetle_ports()
    if not ports:
        print("no FireBeetle ports found. plug one in or pass --ports.", file=sys.stderr)
        return 1

    hub = FireBeetleHub(logdir=args.logdir)
    try:
        hub.connect(ports)
        repl(hub)
    finally:
        hub.shutdown()
    return 0


if __name__ == "__main__":
    sys.exit(main())
