/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Synopsis of Barometric Pressure Board
  MYOSA Platform consists of a Barometric Pressure Board. It is equiped with BMP180 IC which has a pressure sensing range
  of 300-1100 hPa (9000m to -500m above sea level), with a precision up to 0.03hPa/0.25m resolution.
  It also have temperature sensing element with -40 to +85°C operational range, ±2°C temperature accuracy.
  I2C Address of the board = 0x77u.
  Detailed Information about Barometric Pressure board Library and usage is provided in the link below.
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

pub const BMP180_SOFT_REST_VALUE: u8    = 0xB6;   /**< write this value in SOFT_RESET_REG to soft reset the BMP180 */
pub const BMP180_GET_TEMPERATURE: u8    = 0x2E;   /**< write this value in CONTROL_REG to get the temperature */
pub const BMP180_GET_PRESSURE_OSS0: u8  = 0x34;   /**< write this value in CONTROL_REG to get the pressure with oversampling 0 */
pub const BMP180_GET_PRESSURE_OSS1: u8  = 0x74;   /**< write this value in CONTROL_REG to get the pressure with oversampling 1 */
pub const BMP180_GET_PRESSURE_OSS2: u8  = 0xB4;   /**< write this value in CONTROL_REG to get the pressure with oversampling 2 */
pub const BMP180_GET_PRESSURE_OSS3: u8  = 0xF4;   /**< write this value in CONTROL_REG to get the pressure with oversampling 3 */
pub const BMP180_I2C_ADDRESS: u8        = 0x77;   /**< I2C slave address */
pub const BMP180_CHIP_ID: u8            = 0x55;   /**< BMP180 Chip ID */
pub const BMP180_ERROR: u8              = 255;
pub const BMP180_MAX_COEFF_REGS: u8     = 11;     /* number of coefficient registers in BMP180 */

pub const SEA_LEVEL_AVG_PRESSURE: f32   = 1013.25; /* Average sea-level pressure is 1013.25 mbar */

// ============================================================================
// Enums
// ============================================================================

/*!
 * List of registers to control and configure the BMP180 sensor
 */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Bmp180Reg {
    AC1_REG           = 0xAA,
    AC2_REG           = 0xAC,
    AC3_REG           = 0xAE,
    AC4_REG           = 0xB0,
    AC5_REG           = 0xB2,
    AC6_REG           = 0xB4,
    B1_REG            = 0xB6,
    B2_REG            = 0xB8,
    MB_REG            = 0xBA,
    MC_REG            = 0xBC,
    MD_REG            = 0xBE,
    ADC_OUT_XLSB_REG  = 0xF8,  /**< raw data XLSB */
    ADC_OUT_LSB_REG   = 0xF7,  /**< raw data LSB */
    ADC_OUT_MSB_REG   = 0xF6,  /**< raw data MSB */
    CONTROL_REG       = 0xF4,  /**< Controls the measurement, conversion & oversampling */
    SOFT_RESET_REG    = 0xE0,  /**< Resets the BMP180 */
    CHIP_ID_REG       = 0xD0,  /**< BMP180 Chip ID */
}

/*!
 * different sampling accuracy modes in BMP180
 */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Bmp180AccuracyMode {
    ULTRA_LOW_POWER       = 0x00,  /**< OSS0, 4.5ms conversion time */
    STANDARD              = 0x01,  /**< OSS1, 7.5ms conversion time */
    HIGH_RESOLUTION       = 0x02,  /**< OSS2, 13.5ms conversion time */
    ULTRA_HIGH_RESOLUTION = 0x03,  /**< OSS3, 25.5ms conversion time */
}

// ============================================================================
// Structs
// ============================================================================

/*!
 * to store the calibration coefficients
 */
#[derive(Clone, Copy, Debug, Default)]
pub struct Bmp180CalibCoeff {
    pub _AC1: i16,
    pub _AC2: i16,
    pub _AC3: i16,
    pub _AC4: u16,
    pub _AC5: u16,
    pub _AC6: u16,
    pub _B1: i16,
    pub _B2: i16,
    pub _MB: i16,
    pub _MC: i16,
    pub _MD: i16,
}

// ============================================================================
// BarometricPressure
// ============================================================================

pub struct BarometricPressure<I2C: I2CBus, D: DelayMs> {
    _i2c_slave_address: u8,
    _is_connected: bool,
    _accuracy: u8,
    _calib_coeff: Bmp180CalibCoeff,
    i2c: I2C,
    delay: D,
}

impl<I2C: I2CBus, D: DelayMs> BarometricPressure<I2C, D> {
    /**
     *   @brief constructor to initialise the BMP180 sampling accuracy mode
     */
    pub fn new(i2c: I2C, delay: D, accr: Bmp180AccuracyMode) -> Self {
        BarometricPressure {
            _accuracy: accr as u8,
            _i2c_slave_address: BMP180_I2C_ADDRESS,
            _is_connected: false,
            _calib_coeff: Bmp180CalibCoeff::default(),
            i2c,
            delay,
        }
    }

    /**
     *
     */
    pub fn set_accuracy_mode(&mut self, mode: Bmp180AccuracyMode) {
        self._accuracy = mode as u8;
    }

    /**
     *
     */
    pub fn begin(&mut self) -> bool {
        /* Check device ID to verify the communication establishment */
        if self.get_device_id() != BMP180_CHIP_ID {
            return false;
        }
        self._is_connected = true;
        /* Get the sensor coefficeints */
        self.read_calibration_coefficients()
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
    fn get_temperature(&mut self) -> f32 {
        /* get the raw temperature */
        let ut = self.read_raw_temperature();
        /* Compute the temperature */
        if ut == BMP180_ERROR as u16 {
            return BMP180_ERROR as f32;
        }
        let temperature = ((self.compute_b5(ut as i32) + 8) >> 4) as f32 / 10.0;
        temperature
    }

    /**
     *
     */
    pub fn get_temp_c(&mut self) -> f32 {
        let mut temperature = self.get_temperature();
        if temperature == BMP180_ERROR as f32 {
            temperature = 0.0;
        }
        temperature
    }

    /**
     *
     */
    pub fn get_temp_f(&mut self) -> f32 {
        let mut temperature = self.get_temperature() * (9.0 / 5.0) + 32.0;
        if temperature == BMP180_ERROR as f32 {
            temperature = 0.0;
        }
        temperature
    }

    /**
     *
     */
    pub fn get_pressure(&mut self) -> i32 {
        let ut: i32;
        let up: i32;
        let b3: i32;
        let b5: i32;
        let b6: i32;
        let mut x1: i32;
        let x2: i32;
        let x3: i32;
        let mut pressure: i32;
        let b4: u32;
        let b7: u32;

        ut = self.read_raw_temperature() as i32;           //read uncompensated temperature, 16-bit
        if ut == BMP180_ERROR as i32 { return BMP180_ERROR as i32; }  //error handler, collision on i2c bus

        up = self.read_raw_pressure() as i32;              //read uncompensated pressure, 19-bit
        if up == BMP180_ERROR as i32 { return BMP180_ERROR as i32; }  //error handler, collision on i2c bus

        b5 = self.compute_b5(ut);

        /* pressure calculation */
        b6 = b5 - 4000;
        x1 = (((self._calib_coeff._B2 as i32) * ((b6 * b6) >> 12)) >> 11) as i32;
        x2 = ((self._calib_coeff._AC2 as i32) * b6) >> 11;
        x3 = x1 + x2;
        b3 = ((((self._calib_coeff._AC1 as i32) * 4 + x3) << self._accuracy) + 2) / 4;

        x1 = ((self._calib_coeff._AC3 as i32) * b6) >> 13;
        x2 = ((self._calib_coeff._B1 as i32) * ((b6 * b6) >> 12)) >> 16;
        x3 = ((x1 + x2) + 2) >> 2;
        b4 = ((self._calib_coeff._AC4 as u32) * ((x3 + 32768) as u32)) >> 15;
        b7 = ((up - b3) as u32) * (50000u32 >> self._accuracy);

        if b4 == 0 { return BMP180_ERROR as i32; }                     //safety check, avoiding division by zero

        if b7 < 0x80000000 { pressure = ((b7 * 2) / b4) as i32; }
        else               { pressure = ((b7 / b4) * 2) as i32; }

        x1 = ((pressure >> 8) as f64).powi(2) as i32;
        x1 = (x1 * 3038) >> 16;
        x2 = (-7357 * pressure) >> 16;

        pressure = pressure + ((x1 + x2 + 3791) >> 4);
        pressure
    }

    /**
     *
     */
    pub fn get_pressure_pascal(&mut self) -> f32 {
        let mut pressure_pascal = self.get_pressure() as f32;
        if pressure_pascal == BMP180_ERROR as f32 {
            pressure_pascal = 0.0;
        }
        pressure_pascal / 1000.0
    }

    /**
     *
     */
    pub fn get_pressure_hg(&mut self) -> f32 {
        let mut pressure_pascal = self.get_pressure() as f32;
        if pressure_pascal == BMP180_ERROR as f32 {
            pressure_pascal = 0.0;
        }
        pressure_pascal / 133.0
    }

    /**
     *
     */
    pub fn get_pressure_bar(&mut self) -> f32 {
        let mut pressure_pascal = self.get_pressure() as f32;
        if pressure_pascal == BMP180_ERROR as f32 {
            pressure_pascal = 0.0;
        }
        pressure_pascal / 100.0
    }

    /**
     *
     */
    pub fn get_sea_level_pressure(&mut self, altitude: f32) -> f32 {
        let pressure = self.get_pressure_bar();
        let slp = pressure / (1.0 - altitude / 44330.0_f32).powf(5.255);
        slp
    }

    /**
     *
     */
    pub fn get_altitude(&mut self, p0: f32) -> f32 {
        let pressure = self.get_pressure_bar();
        let altitude = 44330.0 * (1.0 - (pressure / p0).powf(1.0 / 5.255));
        altitude
    }

    /**
     *
     */
    fn compute_b5(&self, ut: i32) -> i32 {
        let x1 = ((ut - self._calib_coeff._AC6 as i32) * self._calib_coeff._AC5 as i32) >> 15;
        let x2 = ((self._calib_coeff._MC as i32) << 11) / (x1 + self._calib_coeff._MD as i32);
        x1 + x2
    }

    /**
     *
     */
    pub fn reset(&mut self) {
        self.write8bit(Bmp180Reg::SOFT_RESET_REG, BMP180_SOFT_REST_VALUE);
    }

    /**
     *
     */
    pub fn get_device_id(&mut self) -> u8 {
        if self.read8bit(Bmp180Reg::CHIP_ID_REG) == BMP180_CHIP_ID {
            BMP180_CHIP_ID
        } else {
            BMP180_ERROR
        }
    }

    /**
     *
     */
    fn read_calibration_coefficients(&mut self) -> bool {
        /* get the sensor calibration coefficients */
        self._calib_coeff._AC1 = self.read16bit(Bmp180Reg::AC1_REG) as i16;
        self._calib_coeff._AC3 = self.read16bit(Bmp180Reg::AC3_REG) as i16;
        self._calib_coeff._AC2 = self.read16bit(Bmp180Reg::AC2_REG) as i16;
        self._calib_coeff._AC5 = self.read16bit(Bmp180Reg::AC5_REG);
        self._calib_coeff._AC4 = self.read16bit(Bmp180Reg::AC4_REG);
        self._calib_coeff._AC6 = self.read16bit(Bmp180Reg::AC6_REG);
        self._calib_coeff._B1 = self.read16bit(Bmp180Reg::B1_REG) as i16;
        self._calib_coeff._B2 = self.read16bit(Bmp180Reg::B2_REG) as i16;
        self._calib_coeff._MB = self.read16bit(Bmp180Reg::MB_REG) as i16;
        self._calib_coeff._MC = self.read16bit(Bmp180Reg::MC_REG) as i16;
        self._calib_coeff._MD = self.read16bit(Bmp180Reg::MD_REG) as i16;
        true
    }

    /**
     *
     */
    fn read_raw_temperature(&mut self) -> u16 {
        /* Send the temperature measure command */
        if !self.write8bit(Bmp180Reg::CONTROL_REG, BMP180_GET_TEMPERATURE) {
            return BMP180_ERROR as u16;
        }
        /* wait until measurement completion */
        self.delay.delay_ms(5);
        /* read the raw temperature value */
        self.read16bit(Bmp180Reg::ADC_OUT_MSB_REG)
    }

    /**
     *
     */
    fn read_raw_pressure(&mut self) -> u32 {
        let reg_val: u8;
        let mut raw_pressure: u32;
        let delay_ms: u8;

        match self._accuracy {
            0 => { // ULTRA_LOW_POWER
                reg_val = BMP180_GET_PRESSURE_OSS0;
                delay_ms = 5;
            }
            1 => { // STANDARD
                reg_val = BMP180_GET_PRESSURE_OSS1;
                delay_ms = 8;
            }
            2 => { // HIGH_RESOLUTION
                reg_val = BMP180_GET_PRESSURE_OSS2;
                delay_ms = 14;
            }
            3 => { // ULTRA_HIGH_RESOLUTION
                reg_val = BMP180_GET_PRESSURE_OSS3;
                delay_ms = 26;
            }
            _ => {
                reg_val = 0;
                delay_ms = 0;
            }
        }

        /* Send the pressure measure command */
        if !self.write8bit(Bmp180Reg::CONTROL_REG, reg_val) {
            return BMP180_ERROR as u32;
        }
        /* wait until measurement completion */
        self.delay.delay_ms(delay_ms as u16);
        /* read pressure msb + lsb */
        let val = self.read16bit(Bmp180Reg::ADC_OUT_MSB_REG);
        if val == BMP180_ERROR as u16 {
            return BMP180_ERROR as u32;
        }
        raw_pressure = val as u32;
        /* shift out left 8 times to store the xlsb*/
        raw_pressure <<= 8;
        /* read pressure xlsb */
        raw_pressure |= self.read8bit(Bmp180Reg::ADC_OUT_XLSB_REG) as u32;
        raw_pressure >>= 8 - self._accuracy;
        /* return the raw pressure value */
        raw_pressure
    }

    // *********************************************************************************************
    // Platform dependent routines. Change these functions implementation based on microcontroller *
    // *********************************************************************************************

    /**
     *
     */
    fn read8bit(&mut self, reg: Bmp180Reg) -> u8 {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg as u8);
        if self.i2c.end_transmission() != 0 {
            return BMP180_ERROR;
        }
        self.i2c.request_from(self._i2c_slave_address, 1);
        if self.i2c.available() != 1 {
            return BMP180_ERROR;
        }
        self.i2c.read_byte()
    }

    /**
     *
     */
    fn read16bit(&mut self, reg: Bmp180Reg) -> u16 {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg as u8);
        if self.i2c.end_transmission() != 0 {
            return BMP180_ERROR as u16;
        }
        self.i2c.request_from(self._i2c_slave_address, 2);
        if self.i2c.available() != 2 {
            return BMP180_ERROR as u16;
        }
        let mut value: u16 = (self.i2c.read_byte() as u16) << 8;
        value |= self.i2c.read_byte() as u16;
        value
    }

    /**
     *
     */
    fn write8bit(&mut self, reg: Bmp180Reg, val: u8) -> bool {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg as u8);
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
