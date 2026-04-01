/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Synopsis of Temperature And Humidity Board
  MYOSA Platform consists of an Temperature And Humidity Board. It is equiped with Si7021 IC.
  It has ± 3% relative humidity measurements with a range of 0–80% RH, and ±0.4 °C temperature accuracy at a range of -10 to +85 °C.
  I2C Address of the board = 0x40.
  Detailed Information about Temperature and Humidity board Library and usage is provided in the link below.
  Detailed Guide: https://drive.google.com/file/d/1On6kzIq3ejcu9aMGr2ZB690NnFrXG2yO/view

  NOTE
  All information, including URL references, is subject to change without prior notice.
  Please always use the latest versions of software-release for best performance.
  Unless required by applicable law or agreed to in writing, this software is distributed on an
  "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied

  Modifications
  1 December, 2021 by Pegasus Automation
  (as a part of MYOSA Initiative)

  Contact Team MakeSense EduTech for any kind of feedback/issues pertaining to performance or any update request.
  Email: dev.myosa@gmail.com
*/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

/// I2C bus hardware abstraction trait.
/// Implement this for your target platform to provide I2C communication.
pub trait I2CBus {
    fn begin_transmission(&mut self, address: u8);
    fn write_byte(&mut self, data: u8);
    fn write_bytes(&mut self, data: &[u8]);
    fn end_transmission(&mut self) -> u8;
    fn request_from(&mut self, address: u8, quantity: u8) -> u8;
    fn read_byte(&mut self) -> u8;
    fn available(&self) -> u8;
}

/// Platform delay function type.
/// Implement this for your target platform.
pub trait DelayMs {
    fn delay_ms(&mut self, ms: u16);
}

// ============================================================================
// Constants
// ============================================================================

pub const SI7021_I2C_ADDRESS: u8              = 0x40;
pub const SI7021_SOFT_RESET_DELAY: u16        = 0x0F;
pub const SI7021_MEAS_RH_HOLD_MODE: u8       = 0xE5;   /**< Measure Relative Humidity, Hold Master Mode */
pub const SI7021_MEAS_RH_NOHOLD_MODE: u8     = 0xF5;   /**< Measure Relative Humidity, No Hold Master Mode */
pub const SI7021_MEAS_TEMP_HOLD_MODE: u8     = 0xE3;   /**< Measure Temperature, Hold Master Mode */
pub const SI7021_MEAS_TEMP_NOHOLD_MODE: u8   = 0xF3;   /**< Measure Temperature, No Hold Master Mode */
pub const SI7021_READ_TEMP_PREV_RH_MEAS: u8  = 0xE0;   /**< Read Temperature Value from Previous RH Measurement */
pub const SI7021_RESET: u8                    = 0xFE;
pub const SI7021_WRITE_USER_REG: u8          = 0xE6;   /**< Write RH/T User Register 1 */
pub const SI7021_READ_USER_REG: u8           = 0xE7;   /**< Read RH/T User Register 1 */
pub const SI7021_WRITE_HEATER_CNTRL_REG: u8  = 0x51;   /**< Write Heater Control Register */
pub const SI7021_READ_HEATER_CNTRL_REG: u8   = 0x11;   /**< Read Heater Control Register */
pub const SI7021_ID1_CMD0: u8                = 0xFA;   /**< Read Electronic ID 1st Byte */
pub const SI7021_ID1_CMD1: u8                = 0x0F;   /**< Read Electronic ID 1st Byte */
pub const SI7021_ID2_CMD0: u8                = 0xFC;   /**< Read Electronic ID 2nd Byte */
pub const SI7021_ID2_CMD1: u8                = 0xC9;   /**< Read Electronic ID 2nd Byte */
pub const SI7021_FIMWARE_REV_CMD0: u8        = 0x84;   /**< Read Firmware Revision */
pub const SI7021_FIMWARE_REV_CMD1: u8        = 0xB8;   /**< Read Firmware Revision */

// ============================================================================
// Structs
// ============================================================================

/*!
 * list of bitfields to configure the User Register 1
 */
#[derive(Clone, Copy, Debug, Default)]
pub struct UserReg {
    pub res0: bool,     // bit 0
    pub rsvd0: bool,    // bit 1
    pub htre: bool,     // bit 2
    pub rsvd1: u8,      // bits 3..5 (3 bits)
    pub vdds: bool,     // bit 6
    pub res1: bool,     // bit 7
}

impl UserReg {
    /// Pack the bitfields into a single u8 register value
    pub fn to_u8(&self) -> u8 {
        (self.res0 as u8)
            | ((self.rsvd0 as u8) << 1)
            | ((self.htre as u8) << 2)
            | ((self.rsvd1 & 0x07) << 3)
            | ((self.vdds as u8) << 6)
            | ((self.res1 as u8) << 7)
    }

    /// Unpack a u8 register value into the bitfield struct
    pub fn from_u8(val: u8) -> Self {
        UserReg {
            res0:  (val & 0x01) != 0,
            rsvd0: (val & 0x02) != 0,
            htre:  (val & 0x04) != 0,
            rsvd1: (val >> 3) & 0x07,
            vdds:  (val & 0x40) != 0,
            res1:  (val & 0x80) != 0,
        }
    }
}

// ============================================================================
// TempAndHumidity
// ============================================================================

pub struct TempAndHumidity<I2C: I2CBus, D: DelayMs> {
    _i2c_slave_address: u8,
    _is_connected: bool,
    i2c: I2C,
    delay: D,
}

impl<I2C: I2CBus, D: DelayMs> TempAndHumidity<I2C, D> {
    /**
     *
     */
    pub fn new(i2c: I2C, delay: D) -> Self {
        TempAndHumidity {
            _i2c_slave_address: SI7021_I2C_ADDRESS,
            _is_connected: false,
            i2c,
            delay,
        }
    }

    /**
     *
     */
    pub fn begin(&mut self) -> bool {
        if self.reset() {
            self.delay.delay_ms(SI7021_SOFT_RESET_DELAY);
            self._is_connected = true;
            true
        } else {
            false
        }
    }

    /**
     *
     */
    pub fn reset(&mut self) -> bool {
        self.write_byte_cmd(SI7021_RESET)
    }

    /**
     *
     */
    pub fn ping(&mut self) -> bool {
        let get_connect_sts = self.write_address();
        if !self._is_connected && get_connect_sts {
            self.begin();
        }
        self._is_connected = get_connect_sts;
        get_connect_sts
    }

    /**
     *
     */
    pub fn get_relative_humidity(&mut self) -> f32 {
        let mut data = [0u8; 3];
        let rh: f32;

        /*
        if self.read_multi_bytes_reg(SI7021_MEAS_RH_HOLD_MODE, 3, &mut data) == false {
            return 0.0;
        }
        */
        // do { ... } while(0) equivalent -- runs once
        loop {
            if !self.write_byte_cmd(SI7021_MEAS_RH_NOHOLD_MODE) {
                rh = 0.0;
                break;
            }
            self.delay.delay_ms(25);
            if !self.read_multi_bytes(3, &mut data) {
                rh = 0.0;
                break;
            }
            let rh_code = ((data[0] as u16) << 8) | data[1] as u16;
            rh = (125.0 * rh_code as f32 / 65536.0) - 6.0;
            break;
        }
        rh
    }

    /**
     *
     */
    pub fn get_temp_c(&mut self) -> f32 {
        let mut data = [0u8; 3];
        let temperature: f32;

        /*
        if self.read_multi_bytes_reg(SI7021_MEAS_TEMP_HOLD_MODE, 3, &mut data) == false {
            return 0.0;
        }
        */
        loop {
            if !self.write_byte_cmd(SI7021_MEAS_TEMP_NOHOLD_MODE) {
                temperature = 0.0;
                break;
            }
            self.delay.delay_ms(25);
            if !self.read_multi_bytes(3, &mut data) {
                temperature = 0.0;
                break;
            }
            let temp_code = ((data[0] as u16) << 8) | data[1] as u16;
            temperature = (175.72 * temp_code as f32 / 65536.0) - 46.85;
            break;
        }
        temperature
    }

    /**
     *
     */
    pub fn get_temp_f(&mut self) -> f32 {
        let temperature = (self.get_temp_c() * (9.0 / 5.0)) + 32.0;
        temperature
    }

    /**
     *
     */
    pub fn get_heat_index_c(&mut self) -> f32 {
        let t = self.get_temp_c();
        let rh = self.get_relative_humidity();

        let c1: f32 = -8.78469475556;
        let c2: f32 = 1.61139411;
        let c3: f32 = 2.33854883889;
        let c4: f32 = -0.14611605;
        let c5: f32 = -0.012308094;
        let c6: f32 = -0.0164248277778;
        let c7: f32 = 0.002211732;
        let c8: f32 = 0.00072546;
        let c9: f32 = -0.000003582;

        let hi = c1 + (c2 * t) + (c3 * rh) + (c4 * t * rh) + (c5 * t * t)
            + (c6 * rh * rh) + (c7 * t * t * rh) + (c8 * t * rh * rh)
            + (c9 * t * t * rh * rh);
        hi
    }

    /**
     *
     */
    pub fn get_heat_index_f(&mut self) -> f32 {
        let t = self.get_temp_f();
        let rh = self.get_relative_humidity();

        let hi = -42.379 + (2.04901523 * t) + (10.14333127 * rh) - (0.22475541 * t * rh)
            - (0.00683783 * t * t) - (0.05481717 * rh * rh) + (0.00122874 * t * t * rh)
            + (0.00085282 * t * rh * rh) - (0.00000199 * t * t * rh * rh);
        hi
    }

    /**
     *
     */
    pub fn get_serial_number(&mut self) -> u64 {
        let mut data = [0u8; 8];
        let mut serial_number: u64 = 0;

        if self.write_byte_val(SI7021_ID1_CMD0, SI7021_ID1_CMD1) {
            if self.read_multi_bytes(8, &mut data) {
                serial_number = ((data[0] as u64) << 56)
                    | ((data[2] as u64) << 48)
                    | ((data[4] as u64) << 40)
                    | ((data[6] as u64) << 32);
            }
        }

        if self.write_byte_val(SI7021_ID2_CMD0, SI7021_ID2_CMD1) {
            if self.read_multi_bytes(8, &mut data) {
                serial_number |= ((data[0] as u64) << 24)
                    | ((data[2] as u64) << 16)
                    | ((data[4] as u64) << 8)
                    | (data[6] as u64);
            }
        }
        serial_number
    }

    /**
     *
     */
    pub fn get_firmware_version(&mut self) -> &'static str {
        let mut data: u8 = 0;
        let mut version: &str = "Unknown";

        if self.write_byte_val(SI7021_FIMWARE_REV_CMD0, SI7021_FIMWARE_REV_CMD1) {
            let mut buf = [0u8; 1];
            if self.read_multi_bytes(1, &mut buf) {
                data = buf[0];
                if data == 0xFF {
                    version = "1.0";
                }
                if data == 0x20 {
                    version = "2.0";
                }
            }
        }
        version
    }

    // *********************************************************************************************
    // Platform dependent routines. Change these functions implementation based on microcontroller *
    // *********************************************************************************************

    /**
     *
     */
    fn read_byte_reg(&mut self, reg: u8, result: &mut u8) -> bool {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg);
        if self.i2c.end_transmission() != 0 {
            return false;
        }
        self.i2c.request_from(self._i2c_slave_address, 1);
        if self.i2c.available() != 1 {
            return false;
        }
        *result = self.i2c.read_byte();
        true
    }

    /**
     *
     */
    fn read_multi_bytes_reg(&mut self, reg: u8, length: u8, result: &mut [u8]) -> bool {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg);
        if self.i2c.end_transmission() != 0 {
            return false;
        }
        self.i2c.request_from(self._i2c_slave_address, length);
        if self.i2c.available() != length {
            return false;
        }
        for i in 0..length as usize {
            result[i] = self.i2c.read_byte();
        }
        true
    }

    /**
     *
     */
    fn read_multi_bytes(&mut self, length: u8, result: &mut [u8]) -> bool {
        self.i2c.request_from(self._i2c_slave_address, length);
        if self.i2c.available() != length {
            return false;
        }
        for i in 0..length as usize {
            result[i] = self.i2c.read_byte();
        }
        true
    }

    /**
     *
     */
    fn write_byte_cmd(&mut self, reg: u8) -> bool {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg);
        self.i2c.end_transmission() == 0
    }

    /**
     *
     */
    fn write_byte_val(&mut self, reg: u8, val: u8) -> bool {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg);
        self.i2c.write_byte(val);
        self.i2c.end_transmission() == 0
    }

    /**
     *
     */
    fn write_address(&mut self) -> bool {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.end_transmission() == 0
    }
}
