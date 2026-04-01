// Minimal MPU6050 (GY-521) driver (no external dependencies)
// - Reads accelerometer (g)
// - Reads internal temperature (C) (chip temperature, not ambient)
//
// Address can be 0x68 or 0x69 depending on AD0 pin.

pub trait I2CBus {
    fn begin_transmission(&mut self, addr: u8);
    fn write_byte(&mut self, val: u8);
    fn end_transmission(&mut self, send_stop: bool) -> u8;
    fn request_from(&mut self, addr: u8, len: usize) -> usize;
    fn read_byte(&mut self) -> u8;
    fn available(&self) -> usize;
}

#[derive(Clone, Copy, PartialEq)]
pub enum AccelRange {
    Range2G  = 0,  // +/-2g
    Range4G  = 1,  // +/-4g
    Range8G  = 2,  // +/-8g
    Range16G = 3,  // +/-16g
}

// MPU6050 registers
const REG_WHO_AM_I: u8     = 0x75;
const REG_PWR_MGMT_1: u8   = 0x6B;
const REG_SMPLRT_DIV: u8   = 0x19;
const REG_CONFIG: u8        = 0x1A;
const REG_GYRO_CONFIG: u8   = 0x1B;
const REG_ACCEL_CONFIG: u8  = 0x1C;
const REG_ACCEL_XOUT_H: u8  = 0x3B;
const REG_TEMP_OUT_H: u8    = 0x41;

pub struct MPU6050Minimal {
    addr: u8,
    connected: bool,
    accel_lsb_per_g: f32,
}

impl MPU6050Minimal {
    pub fn new() -> Self {
        MPU6050Minimal {
            addr: 0,
            connected: false,
            accel_lsb_per_g: 16384.0,
        }
    }

    // Auto-detect 0x68/0x69 unless an address is provided.
    // sample_rate_hz: requested output rate (typical 100..500). Internally uses 1kHz base.
    // dlpf_cfg: 1..6 for typical use (1=184Hz, 2=94Hz, 3=44Hz, ...). Use 1 for vibration.
    pub fn begin(
        &mut self,
        wire: &mut impl I2CBus,
        forced_address: i8,
        sample_rate_hz: u16,
        accel_range: AccelRange,
        dlpf_cfg: u8,
    ) -> bool {
        self.connected = false;

        if forced_address >= 0 {
            if !self.detect(wire, forced_address as u8) {
                return false;
            }
        } else {
            // Try common addresses: 0x69 (MYOSA docs) then 0x68.
            if !self.detect(wire, 0x69) && !self.detect(wire, 0x68) {
                return false;
            }
        }

        if !self.configure(wire, sample_rate_hz, accel_range, dlpf_cfg) {
            self.connected = false;
            return false;
        }

        self.connected = true;
        true
    }

    pub fn is_connected(&self) -> bool { self.connected }
    pub fn address(&self) -> u8 { self.addr }

    // Read accelerometer in g (gravity units). Returns false on I2C failure.
    pub fn read_accel_g(&self, wire: &mut impl I2CBus) -> Option<(f32, f32, f32)> {
        let mut buf = [0u8; 6];
        if !self.read_regs(wire, REG_ACCEL_XOUT_H, &mut buf) {
            return None;
        }

        let ax_raw = ((buf[0] as u16) << 8 | buf[1] as u16) as i16;
        let ay_raw = ((buf[2] as u16) << 8 | buf[3] as u16) as i16;
        let az_raw = ((buf[4] as u16) << 8 | buf[5] as u16) as i16;

        let k = 1.0 / self.accel_lsb_per_g;
        Some((ax_raw as f32 * k, ay_raw as f32 * k, az_raw as f32 * k))
    }

    // Read internal temperature in Celsius. Returns false on I2C failure.
    pub fn read_temperature_c(&self, wire: &mut impl I2CBus) -> Option<f32> {
        let mut buf = [0u8; 2];
        if !self.read_regs(wire, REG_TEMP_OUT_H, &mut buf) {
            return None;
        }
        let raw = ((buf[0] as u16) << 8 | buf[1] as u16) as i16;

        // Datasheet: Temp in C = (raw / 340) + 36.53
        Some(raw as f32 / 340.0 + 36.53)
    }

    fn detect(&mut self, wire: &mut impl I2CBus, addr: u8) -> bool {
        self.addr = addr;
        let mut who = [0u8; 1];
        if !self.read_regs(wire, REG_WHO_AM_I, &mut who) {
            return false;
        }
        // WHO_AM_I is typically 0x68.
        who[0] == 0x68
    }

    fn configure(
        &mut self,
        wire: &mut impl I2CBus,
        mut sample_rate_hz: u16,
        accel_range: AccelRange,
        mut dlpf_cfg: u8,
    ) -> bool {
        // Wake up + select PLL with X gyro as clock source (more stable than internal oscillator)
        // CLKSEL=1, SLEEP=0
        if !self.write_reg(wire, REG_PWR_MGMT_1, 0x01) {
            return false;
        }
        delay(10);

        // DLPF config (1..6 recommended). 1=184Hz, 2=94Hz, 3=44Hz...
        dlpf_cfg &= 0x07;
        if dlpf_cfg == 0 || dlpf_cfg == 7 {
            // Avoid 0/7 here because that switches gyro output rate to 8kHz,
            // which complicates sample-rate math. Force to 1.
            dlpf_cfg = 1;
        }
        if !self.write_reg(wire, REG_CONFIG, dlpf_cfg) {
            return false;
        }

        // Gyro full scale: +/-250 deg/s (0)
        if !self.write_reg(wire, REG_GYRO_CONFIG, 0x00) {
            return false;
        }

        // Accel full scale range
        let accel_cfg = ((accel_range as u8) & 0x03) << 3;
        if !self.write_reg(wire, REG_ACCEL_CONFIG, accel_cfg) {
            return false;
        }

        // Accel sensitivity (LSB per g)
        self.accel_lsb_per_g = match accel_range {
            AccelRange::Range2G  => 16384.0,
            AccelRange::Range4G  => 8192.0,
            AccelRange::Range8G  => 4096.0,
            AccelRange::Range16G => 2048.0,
        };

        // Sample-rate divider math (with DLPF enabled, internal rate is 1kHz)
        if sample_rate_hz < 10 { sample_rate_hz = 10; }
        if sample_rate_hz > 1000 { sample_rate_hz = 1000; }

        // div = (1000 / sample_rate_hz) - 1
        let mut div: u16 = if sample_rate_hz >= 1000 {
            0
        } else {
            let d = 1000 / sample_rate_hz;
            let d = if d == 0 { 1 } else { d };
            d - 1
        };
        if div > 255 { div = 255; }

        if !self.write_reg(wire, REG_SMPLRT_DIV, div as u8) {
            return false;
        }

        true
    }

    fn write_reg(&self, wire: &mut impl I2CBus, reg: u8, val: u8) -> bool {
        wire.begin_transmission(self.addr);
        wire.write_byte(reg);
        wire.write_byte(val);
        let err = wire.end_transmission(true);
        err == 0
    }

    fn read_regs(&self, wire: &mut impl I2CBus, reg: u8, buf: &mut [u8]) -> bool {
        wire.begin_transmission(self.addr);
        wire.write_byte(reg);
        let err = wire.end_transmission(false);  // repeated start
        if err != 0 {
            return false;
        }
        let got = wire.request_from(self.addr, buf.len());
        if got != buf.len() {
            while wire.available() > 0 {
                let _ = wire.read_byte();
            }
            return false;
        }
        for i in 0..buf.len() {
            buf[i] = wire.read_byte();
        }
        true
    }
}

fn delay(_ms: u32) {}
