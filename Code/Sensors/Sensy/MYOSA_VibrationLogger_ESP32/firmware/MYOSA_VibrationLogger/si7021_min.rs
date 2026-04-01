// Minimal Si7021 temperature driver (no external dependencies)
// (MYOSA Temp & Humidity board uses Si7021 at I2C 0x40)

use super::mpu6050_min::I2CBus;

const CMD_RESET: u8 = 0xFE;
const CMD_READ_USER_REG: u8 = 0xE7;
const CMD_MEASURE_TEMP_NOHOLD: u8 = 0xF3;

pub struct SI7021Minimal {
    addr: u8,
    connected: bool,
}

impl SI7021Minimal {
    pub fn new() -> Self {
        SI7021Minimal {
            addr: 0x40,
            connected: false,
        }
    }

    pub fn begin(&mut self, wire: &mut impl I2CBus, addr: u8) -> bool {
        self.addr = addr;
        self.connected = false;

        // Soft reset
        if !self.write_cmd(wire, CMD_RESET) {
            return false;
        }
        delay(20);

        // Try reading user register (simple presence check)
        if !self.write_cmd(wire, CMD_READ_USER_REG) {
            return false;
        }
        let mut reg = [0u8; 1];
        if !self.read_bytes(wire, &mut reg) {
            return false;
        }

        self.connected = true;
        true
    }

    pub fn is_connected(&self) -> bool { self.connected }

    // Reads ambient temperature in C.
    pub fn read_temperature_c(&self, wire: &mut impl I2CBus) -> Option<f32> {
        if !self.connected {
            return None;
        }

        // Trigger measurement (no-hold)
        if !self.write_cmd(wire, CMD_MEASURE_TEMP_NOHOLD) {
            return None;
        }

        // Typical max conversion time ~11ms (depends on resolution). Give it a little margin.
        delay(15);

        let mut buf = [0u8; 3];
        if !self.read_bytes(wire, &mut buf) {
            return None;
        }

        let crc = crc8(&buf[..2]);
        if crc != buf[2] {
            return None;
        }

        let raw = ((buf[0] as u16) << 8) | buf[1] as u16;

        // Datasheet: T = (175.72 * raw / 65536) - 46.85
        Some(175.72 * raw as f32 / 65536.0 - 46.85)
    }

    fn write_cmd(&self, wire: &mut impl I2CBus, cmd: u8) -> bool {
        wire.begin_transmission(self.addr);
        wire.write_byte(cmd);
        let err = wire.end_transmission(true);
        err == 0
    }

    fn read_bytes(&self, wire: &mut impl I2CBus, buf: &mut [u8]) -> bool {
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

// Si70xx CRC8: polynomial 0x31 (x^8 + x^5 + x^4 + 1), init 0x00.
fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0x00;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

fn delay(_ms: u32) {}
