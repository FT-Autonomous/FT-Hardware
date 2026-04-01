/*
  MYOSA Vibration + Temperature Logger (ESP32)
  ------------------------------------------------
  - No external sensor libraries required
  - Uses I2C MPU6050 (Accel/Gyro board in MYOSA kit)
  - Optionally uses I2C Si7021 (Temp+Humidity board in MYOSA kit) for ambient temperature
  - Streams data every 0.5s over:
      * Serial (human-readable by default, CSV optional)
      * BLE notifications (binary packet for Web Bluetooth)

  Hardware:
    - ESP32-WROOM-32E
    - MPU6050 at I2C address 0x69 (MYOSA docs) or 0x68
    - Si7021 at I2C address 0x40 (optional)

  Notes:
    - "Vibration" here is computed as RMS and peak of *linear* acceleration magnitude
      over a 0.5 second window.
    - Linear acceleration is approximated by subtracting a low-pass gravity estimate
      (good for vibration/shaking; not a full IMU fusion).
*/

mod mpu6050_min;
mod si7021_min;

use mpu6050_min::{MPU6050Minimal, AccelRange, I2CBus};
use si7021_min::SI7021Minimal;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

// ------------------------- User-tunable settings -------------------------

// Serial output format:
//   0 = CSV (good for spreadsheets)
//   1 = Human-readable key/value line (easy to read in Serial Monitor)
const SERIAL_FORMAT: u8 = 1;

// Tracking output period (ms). Requirement: 0.5 seconds.
const WINDOW_MS: u32 = 500;

// Sensor sampling rate (Hz). Higher gives better vibration accuracy.
const SAMPLE_HZ: u16 = 500;

// Accelerometer range: more headroom = less clipping on strong shakes.
const ACCEL_RANGE: AccelRange = AccelRange::Range4G;

// MPU6050 DLPF config:
// 1=184Hz, 2=94Hz, 3=44Hz...
const MPU_DLPF_CFG: u8 = 1;

// I2C pins (ESP32 defaults: SDA=21, SCL=22). Leave as -1 to use defaults.
const I2C_SDA_PIN: i8 = -1;
const I2C_SCL_PIN: i8 = -1;

// I2C clock (Hz). 400k helps higher sample rates.
const I2C_CLOCK_HZ: u32 = 400000;

// BLE device name (what users will see when scanning)
const BLE_DEVICE_NAME: &str = "MYOSA-VibeLogger";

// -------------------------------------------------------------------------
// Shake score + severity thresholds (easy to understand)
// -------------------------------------------------------------------------
// We compute a SHAKE SCORE in the range 0..1000 from RMS vibration.
//  - 0..5   => Still (deadband to avoid hypersensitivity)
//  - 6..(CAUTION-1) => Light
//  - CAUTION..(PARTIAL-1) => Caution
//  - PARTIAL..(SEVERE-1) => Partial
//  - SEVERE..1000 => Severe
//
// Set these three thresholds to tune when warnings kick in.
// Tip: Start low, then raise until "Light" covers normal background vibration.
const THRESH_CAUTION_SCORE: u16 = 25;
const THRESH_PARTIAL_SCORE: u16 = 80;
const THRESH_SEVERE_SCORE: u16  = 150;

// Still is always 0..5.
const STILL_SCORE_MAX: u16 = 5;

// ------------------------- BLE UUIDs (must match web client) -------------------------

const SERVICE_UUID: &str   = "7b4e3a0d-6f1b-4d1a-9c62-4c6b8a2e7a10";
const DATA_CHAR_UUID: &str = "7b4e3a0d-6f1b-4d1a-9c62-4c6b8a2e7a11";
const TIME_CHAR_UUID: &str = "7b4e3a0d-6f1b-4d1a-9c62-4c6b8a2e7a12";

// ------------------------- Globals -------------------------

static mut HAS_MPU: bool = false;
static mut HAS_SI7021: bool = false;

static DEVICE_CONNECTED: AtomicBool = AtomicBool::new(false);

// Time sync from browser
static mut EPOCH_BASE_MS: u64 = 0;   // epoch ms at the moment millis_base was captured
static mut MILLIS_BASE: u32 = 0;     // millis() when epoch_base_ms was set
static mut TIME_SYNCED: bool = false;

fn get_epoch_ms_or_zero() -> u64 {
    unsafe {
        if !TIME_SYNCED {
            return 0;
        }
        let base_ms = EPOCH_BASE_MS;
        let base_millis = MILLIS_BASE;
        let now_millis = millis();
        let delta = now_millis.wrapping_sub(base_millis);
        base_ms + delta as u64
    }
}

fn set_epoch_ms(epoch_ms: u64) {
    unsafe {
        EPOCH_BASE_MS = epoch_ms;
        MILLIS_BASE = millis();
        TIME_SYNCED = true;
    }
}

// Binary packet sent over BLE notifications (little-endian)
#[repr(C, packed)]
struct VibePacket {
    t_s: u32,         // Unix time seconds (0 if not synced)
    t_ms: u16,        // milliseconds part (0-999)
    rms_mg: u16,      // RMS linear acceleration magnitude (milli-g)
    peak_mg: u16,     // Peak linear acceleration magnitude (milli-g)
    shake_score: u16, // 0..1000 (derived from RMS)
    temp_c_x100: i16, // temperature (C * 100)
    level: u8,        // 0..4 intensity bucket
}

// Convert configured accelerometer range to its full-scale g value.
// Used to map RMS vibration into a 0..1000 "shake score".
const fn accel_range_full_scale_g(r: AccelRange) -> f32 {
    match r {
        AccelRange::Range2G  => 2.0,
        AccelRange::Range4G  => 4.0,
        AccelRange::Range8G  => 8.0,
        AccelRange::Range16G => 16.0,
    }
}

const SCORE_FULL_SCALE_G: f32 = accel_range_full_scale_g(ACCEL_RANGE);

fn shake_score_from_rms_g(rms_g: f32) -> u16 {
    if !rms_g.is_finite() || rms_g <= 0.0 {
        return 0;
    }
    // Scale RMS (g) relative to the sensor full-scale (g) into 0..1000.
    // Example: With RANGE_4G, rms_g = 0.10g => score = 25.
    let s = (rms_g / SCORE_FULL_SCALE_G) * 1000.0;
    let score = s.round() as i32;
    score.clamp(0, 1000) as u16
}

fn level_from_score(score: u16) -> u8 {
    if score <= STILL_SCORE_MAX {
        return 0; // Still
    }
    if score < THRESH_CAUTION_SCORE {
        return 1; // Light
    }
    if score < THRESH_PARTIAL_SCORE {
        return 2; // Caution
    }
    if score < THRESH_SEVERE_SCORE {
        return 3; // Partial
    }
    4   // Severe
}

fn level_label(level: u8) -> &'static str {
    match level {
        0 => "Still",
        1 => "Light",
        2 => "Caution",
        3 => "Partial",
        _ => "Severe",
    }
}

// Vibration accumulator (for each 0.5 second window)
struct VibeAccum {
    sum_sq: f32,
    peak: f32,
    n: u32,

    // Gravity estimate (low-pass filtered accel)
    gx: f32,
    gy: f32,
    gz: f32,

    gravity_init: bool,
}

impl VibeAccum {
    fn new() -> Self {
        VibeAccum {
            sum_sq: 0.0, peak: 0.0, n: 0,
            gx: 0.0, gy: 0.0, gz: 0.0,
            gravity_init: false,
        }
    }
}

static mut G_ACC: Option<VibeAccum> = None;

fn reset_window() {
    unsafe {
        if let Some(ref mut acc) = G_ACC {
            acc.sum_sq = 0.0;
            acc.peak = 0.0;
            acc.n = 0;
        }
    }
}

fn add_sample(ax_g: f32, ay_g: f32, az_g: f32) {
    // Gravity estimation via simple IIR low-pass filter.
    // Choose a time constant that tracks orientation changes but ignores vibration.
    // alpha = dt / (tau + dt)
    const TAU: f32 = 0.5;                          // seconds
    const DT: f32 = 1.0 / SAMPLE_HZ as f32;        // seconds
    const ALPHA: f32 = DT / (TAU + DT);

    unsafe {
        let acc = G_ACC.as_mut().unwrap();

        if !acc.gravity_init {
            acc.gx = ax_g;
            acc.gy = ay_g;
            acc.gz = az_g;
            acc.gravity_init = true;
        } else {
            acc.gx += ALPHA * (ax_g - acc.gx);
            acc.gy += ALPHA * (ay_g - acc.gy);
            acc.gz += ALPHA * (az_g - acc.gz);
        }

        let lx = ax_g - acc.gx;
        let ly = ay_g - acc.gy;
        let lz = az_g - acc.gz;
        let lin_mag = (lx * lx + ly * ly + lz * lz).sqrt();

        acc.sum_sq += lin_mag * lin_mag;
        if lin_mag > acc.peak {
            acc.peak = lin_mag;
        }
        acc.n += 1;
    }
}

fn read_temperature_c(
    mpu: &MPU6050Minimal,
    si: &SI7021Minimal,
    wire: &mut impl I2CBus,
) -> Option<f32> {
    unsafe {
        if HAS_SI7021 {
            if let Some(t) = si.read_temperature_c(wire) {
                return Some(t);
            }
        }
        if HAS_MPU {
            return mpu.read_temperature_c(wire);
        }
    }
    None
}

fn publish_window(
    mpu: &MPU6050Minimal,
    si: &SI7021Minimal,
    wire: &mut impl I2CBus,
) {
    // Even if the sensor isn't ready, still publish a packet every WINDOW_MS so
    // the BLE dashboard can connect and show a clear "sensor missing" state.

    unsafe {
        let acc = G_ACC.as_ref().unwrap();
        let valid_vibe = HAS_MPU && acc.n > 0;

        let mut rms_g: f32 = f32::NAN;
        let mut peak_g: f32 = f32::NAN;
        let mut shake_score: u16 = 0;
        let mut level: u8 = 0;

        if valid_vibe {
            rms_g = (acc.sum_sq / acc.n as f32).sqrt();
            peak_g = acc.peak;
            shake_score = shake_score_from_rms_g(rms_g);
            level = level_from_score(shake_score);
        }

        let temp_c = read_temperature_c(mpu, si, wire);
        let valid_temp = temp_c.is_some();
        let temp_val = temp_c.unwrap_or(f32::NAN);

        // Timestamp
        let epoch_ms = get_epoch_ms_or_zero();
        let mut t_s: u32 = 0;
        let mut t_ms: u16 = 0;
        if epoch_ms != 0 {
            t_s = (epoch_ms / 1000) as u32;
            t_ms = (epoch_ms % 1000) as u16;
        }

        // Serial output
        let serial_time_ms: u64 = if epoch_ms != 0 { epoch_ms } else { millis() as u64 };

        if valid_vibe {
            if SERIAL_FORMAT == 0 {
                // CSV
                println!("{},{:.4},{:.4},{:.2},{},{},{}", serial_time_ms, rms_g, peak_g, temp_val, shake_score, level, level_label(level));
            } else {
                // Human-readable
                let time_key = if epoch_ms != 0 { "epoch_ms" } else { "uptime_ms" };
                println!("{}={} | rms_g={:.4} | peak_g={:.4} | temp_c={:.2} | shake_score={}/1000 | level={} ({})",
                    time_key, serial_time_ms, rms_g, peak_g, temp_val, shake_score, level, level_label(level));
            }
        } else {
            if SERIAL_FORMAT == 0 {
                // CSV: Use NaN to make it obvious that the sensor isn't available.
                println!("{},nan,nan,{:.2},0,0,NoSensor", serial_time_ms, temp_val);
            } else {
                let time_key = if epoch_ms != 0 { "epoch_ms" } else { "uptime_ms" };
                println!("{}={} | sensor=missing (MPU6050 not detected) | temp_c={:.2}", time_key, serial_time_ms, temp_val);
            }
        }

        // BLE notification
        if DEVICE_CONNECTED.load(Ordering::SeqCst) {
            let mut pkt = VibePacket {
                t_s,
                t_ms,
                rms_mg: 0,
                peak_mg: 0,
                shake_score: 0,
                temp_c_x100: 0,
                level: 0,
            };

            if valid_vibe {
                // Clamp mg values to uint16
                let rms_mg = (rms_g * 1000.0).round() as i32;
                let peak_mg_val = (peak_g * 1000.0).round() as i32;
                pkt.rms_mg = rms_mg.clamp(0, 65534) as u16;
                pkt.peak_mg = peak_mg_val.clamp(0, 65534) as u16;
                pkt.shake_score = shake_score;
                pkt.level = level;
            } else {
                // Sentinel "invalid" values so the web UI can show "sensor missing".
                pkt.rms_mg = 65535;
                pkt.peak_mg = 65535;
                pkt.shake_score = 65535;
                pkt.level = 0;
            }

            if valid_temp {
                let temp_x100 = (temp_val * 100.0).round() as i32;
                pkt.temp_c_x100 = temp_x100.clamp(-32767, 32767) as i16;
            } else {
                pkt.temp_c_x100 = -32768_i16;
            }

            ble_notify(&pkt);
        }
    }

    reset_window();
}

fn main() {
    delay(200);

    unsafe { G_ACC = Some(VibeAccum::new()); }

    let mut wire = DummyI2C;

    // Sensor init
    let mut mpu = MPU6050Minimal::new();
    let mut si7021 = SI7021Minimal::new();

    println!("[Init] Searching for MPU6050...");
    unsafe {
        HAS_MPU = mpu.begin(&mut wire, -1, SAMPLE_HZ, ACCEL_RANGE, MPU_DLPF_CFG);
        if HAS_MPU {
            println!("[Init] MPU6050 found at 0x{:02X}", mpu.address());
        } else {
            println!("[Init] MPU6050 not found. BLE will still start so you can connect.");
            println!("       Fix wiring (SDA=21, SCL=22, 3.3V, GND) and reset to retry.");
        }
    }

    println!("[Init] Searching for Si7021 (optional)...");
    unsafe {
        if si7021.begin(&mut wire, 0x40) {
            HAS_SI7021 = true;
            println!("[Init] Si7021 found (using ambient temperature)");
        } else {
            HAS_SI7021 = false;
            println!("[Init] Si7021 not found (using MPU6050 internal temperature)");
        }
    }

    // BLE init
    println!("[Init] Starting BLE...");
    setup_ble();
    println!("[Init] BLE advertising started");

    // Serial output legend
    println!("# Output every 0.5s");
    println!("#  epoch_ms / uptime_ms : epoch milliseconds if time-synced, otherwise uptime milliseconds");
    println!("#  rms_g                : RMS linear acceleration magnitude over last 0.5s (g)");
    println!("#  peak_g               : Peak linear acceleration magnitude over last 0.5s (g)");
    println!("#  temp_c               : Temperature in Celsius");
    println!("#  shake_score          : 0..1000 (scaled from rms_g relative to accel range)");
    println!("#  level/label          : 0..4 => Still, Light, Caution, Partial, Severe");

    if SERIAL_FORMAT == 0 {
        println!("time_ms,rms_g,peak_g,temp_c,shake_score,level,label");
    } else {
        println!("# Format: key=value | key=value | ...");
    }

    reset_window();

    let mut last_sample_us: u32 = micros();
    let mut last_window_ms: u32 = millis();
    let mut last_probe_ms: u32 = 0;
    let sample_period_us: u32 = 1_000_000 / SAMPLE_HZ as u32;

    loop {
        // Take as many samples as needed to catch up (prevents drift if loop jitters).
        let mut now_us = micros();
        while (now_us.wrapping_sub(last_sample_us) as i32) >= sample_period_us as i32 {
            last_sample_us = last_sample_us.wrapping_add(sample_period_us);

            unsafe {
                if HAS_MPU {
                    if let Some((ax, ay, az)) = mpu.read_accel_g(&mut wire) {
                        add_sample(ax, ay, az);
                    }
                }
            }

            now_us = micros();
        }

        // Publish every WINDOW_MS
        let now_ms = millis();

        // If MPU is missing, probe occasionally so users can fix wiring without re-flashing.
        unsafe {
            if !HAS_MPU && now_ms.wrapping_sub(last_probe_ms) >= 2000 {
                last_probe_ms = now_ms;
                println!("[Init] Re-trying MPU6050...");
                HAS_MPU = mpu.begin(&mut wire, -1, SAMPLE_HZ, ACCEL_RANGE, MPU_DLPF_CFG);
                if HAS_MPU {
                    println!("[Init] MPU6050 found at 0x{:02X}", mpu.address());
                    // If we didn't have Si7021, try it again too.
                    if !HAS_SI7021 {
                        if si7021.begin(&mut wire, 0x40) {
                            HAS_SI7021 = true;
                            println!("[Init] Si7021 found (using ambient temperature)");
                        }
                    }
                }
            }
        }

        if (now_ms.wrapping_sub(last_window_ms) as i32) >= WINDOW_MS as i32 {
            last_window_ms = last_window_ms.wrapping_add(WINDOW_MS);
            publish_window(&mpu, &si7021, &mut wire);
        }
    }
}

// Platform stubs
struct DummyI2C;
impl I2CBus for DummyI2C {
    fn begin_transmission(&mut self, _addr: u8) {}
    fn write_byte(&mut self, _val: u8) {}
    fn end_transmission(&mut self, _send_stop: bool) -> u8 { 0 }
    fn request_from(&mut self, _addr: u8, _len: usize) -> usize { 0 }
    fn read_byte(&mut self) -> u8 { 0 }
    fn available(&self) -> usize { 0 }
}

fn millis() -> u32 { 0 }
fn micros() -> u32 { 0 }
fn delay(_ms: u32) {}
fn setup_ble() {}
fn ble_notify(_pkt: &VibePacket) {}
