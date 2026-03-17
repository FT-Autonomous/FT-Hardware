# MYOSA Vibration Logger (ESP32-WROOM-32E)

This folder contains **firmware + a local web page** to measure and log:

- **Vibration / shaking intensity** (RMS + peak) every **0.5 seconds**
- **Temperature in °C** (ambient via Si7021 if present, otherwise MPU6050 internal temp)

Output goes to:

- **Serial monitor** (human-readable by default, CSV optional)
- **Bluetooth (BLE)** notifications (for a browser-based dashboard via **Web Bluetooth**)

No MYOSA libraries are required — the sensor drivers are implemented directly over I2C.

---

## What “vibration” means here

Every 0.5 seconds the ESP32 computes:

- **RMS linear acceleration magnitude (g)** over the last 0.5s
- **Peak linear acceleration magnitude (g)** over the last 0.5s

“Linear” means we subtract an estimated gravity vector (low‑pass filtered), so slow tilting doesn’t dominate the measurement.

You also get a simple human‑friendly **severity label**:

- `Still`, `Light`, `Caution`, `Partial`, `Severe`

And a **shake score**:

- `0..1000` (scaled from RMS vibration relative to the configured accelerometer range)

The **numbers (RMS/Peak + score)** are the accurate part; the label is a simple bucket for readability.

---

## Hardware

Works with the MYOSA kit boards:

- **Accel/Gyro board**: MPU6050 (GY‑521) at I2C address **0x69** (MYOSA docs) or **0x68** (auto‑detected)
- **Temp/Humidity board (optional)**: Si7021 at I2C address **0x40** (auto‑detected)

### Wiring (ESP32 defaults)

ESP32 I2C defaults:

- **SDA = GPIO 21**
- **SCL = GPIO 22**

Power the sensors from **3.3V** (recommended) and GND.

If your ESP32 uses different I2C pins, edit these constants in `MYOSA_VibrationLogger.ino`:

```cpp
static constexpr int8_t I2C_SDA_PIN = -1;
static constexpr int8_t I2C_SCL_PIN = -1;
```

---

## Firmware: upload to ESP32

Open:

`firmware/MYOSA_VibrationLogger/MYOSA_VibrationLogger.ino`

### Arduino IDE steps

1. Install Arduino IDE.
2. Install the **ESP32** boards package (Boards Manager).
3. Select board: **ESP32 Dev Module** (works for ESP32‑WROOM‑32E).
4. Select your COM/serial port.
5. Click **Upload**.

### Serial output

Open Serial Monitor at **115200 baud**.

By default you’ll see a readable line like:

```
uptime_ms=464916 | rms_g=0.0063 | peak_g=0.0147 | temp_c=20.80 | shake_score=2/1000 | level=0 (Still)
```

Once the web page syncs the clock, `uptime_ms` becomes `epoch_ms`.

If you prefer CSV, set `SERIAL_FORMAT` to `0` near the top of `MYOSA_VibrationLogger.ino`.

---

## Web dashboard: “localhost HTML”

The dashboard uses **Web Bluetooth** to connect directly from the browser.

### Requirements

- Desktop **Chrome** or **Edge** (Firefox does not support Web Bluetooth)
- Opened from **http://localhost** (or HTTPS)

### Run locally

1. Open a terminal in `web_client/`
2. Start a local server:

**macOS / Linux**

```bash
./serve.sh
# or
python3 -m http.server 8000
```

**Windows**

```bat
serve.bat
```

3. Open in your browser:

- `http://localhost:8000`

4. Click **Connect via Bluetooth** and pick `MYOSA-VibeLogger`.

Note: The dashboard expects the firmware packet format used in this folder. If you update the firmware, update the `web_client/` folder too.

The page will show:

- Current vibration (RMS g + Peak g)
- Temperature (°C)
- A chart (last ~2 minutes)
- A table of **shake events** (grouped)
- Buttons to download **CSV logs**

---

## Tweaking

In `MYOSA_VibrationLogger.ino` you can adjust:

- `SAMPLE_HZ` (higher = better vibration fidelity, more I2C traffic)
- `ACCEL_RANGE` (higher = less clipping on strong shakes)
- Thresholds for the severity buckets (shake score thresholds):
  - `THRESH_CAUTION_SCORE`
  - `THRESH_PARTIAL_SCORE`
  - `THRESH_SEVERE_SCORE`

---

## Troubleshooting

- **No device found in browser**: make sure the ESP32 is powered, flashed, and advertising. Use Chrome/Edge.
- **Web Bluetooth error on file://**: serve the page from `http://localhost` as described above.
- **MPU6050 not found**: check SDA/SCL pins, power (3.3V), and common ground.
- **Temperature seems “off”**: if Si7021 isn’t connected it will fall back to MPU6050 internal temperature (chip temp).

