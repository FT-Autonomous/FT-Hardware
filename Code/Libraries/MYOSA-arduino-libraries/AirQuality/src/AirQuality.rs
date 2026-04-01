/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Synopsis of Air Quality
  MYOSA Platform consists of an environmental Air Quality Board. It is equiped with CCS811 IC.
  It is a digital gas sesnor that senses wide range of TVOCs and eCO2. It is is intended for indoor air quality monitoring purposes.
  I2C Address of the board = 0x5B.
  Detailed Information about Air Quality board Library and usage is provided in the link below.
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

use core::fmt;

const CCS811_HW_ID: u8                 = 0x81;
const CCS811_I2C_ADDRESS1: u8          = 0x5B;

/* Defining constant used in the calculation. Depends on hardware. Don't change. */
#[allow(dead_code)]
const REF_RESISTANCE: f32 = 10000.0;

const CCS811_STATUS_REG: u8            = 0x00;   /**< Status register */
const CCS811_MEAS_MODE_REG: u8         = 0x01;   /**< Measurement mode and conditions register */
const CCS811_ALG_RESULT_DATA_REG: u8   = 0x02;   /**< Algorithm result */
#[allow(dead_code)]
const CCS811_RAW_DATA_REG: u8          = 0x03;   /**< Raw ADC data values for resistance and current source used */
const CCS811_ENV_DATA_REG: u8          = 0x05;   /**< Temperature and Humidity data can be written to enable compensation */
const CCS811_NTC_REG: u8               = 0x06;   /**< Provides the voltage across the reference resistor and the voltage across the NTC resistor – from which the ambient temperature can be determined*/
#[allow(dead_code)]
const CCS811_THRESHOLDS_REG: u8        = 0x10;   /**< Thresholds for operation when interrupts are only generated when eCO2 ppm crosses a threshold */
const CCS811_BASELINE_REG: u8          = 0x11;   /**< The encoded current baseline value can be read. A previously saved encoded baseline can be written */
const CCS811_HW_ID_REG: u8             = 0x20;   /**< Hardware ID register. The value is 0x81 */
const CCS811_HW_VER_REG: u8            = 0x21;   /**< Hardware Version register */
const CCS811_FW_BOOT_VER_REG: u8       = 0x23;   /**< Firmware Boot Version. The first 2 bytes contain the firmware version number for the boot code */
const CCS811_FW_APP_VER_REG: u8        = 0x24;   /**< Firmware Application Version. The first 2 bytes contain the firmware version number for the application code */
const CCS811_ERROR_ID_REG: u8          = 0xE0;   /**< Error ID. When the status register reports an error its source is located in this register */
const CCS811_SOFT_RESET_REG: u8        = 0xFF;   /**< If f the correct 4 bytes (0x11 0xE5 0x72 0x8A) are written to this register in a single sequence the device will reset and return to BOOT mode.*/
const CCS811_APP_START_REG: u8         = 0xF4;   /**< */

const CCS811_DRIVE_MODE_MSK: u8        = 0x70;
const CCS811_INT_DATARDY_MSK: u8       = 0x08;
const CCS811_INT_THRESH_MSK: u8        = 0x04;
const CCS811_DRIVE_MODE_POS: u8        = 0x04;
#[allow(dead_code)]
const CCS811_INT_DATARDY_POS: u8       = 0x03;
#[allow(dead_code)]
const CCS811_INT_THRESH_POS: u8        = 0x02;

#[allow(dead_code)]
const CCS811_DRIVE_MODE0: u8           = 0x00;
const CCS811_DRIVE_MODE1: u8           = 0x01;
#[allow(dead_code)]
const CCS811_DRIVE_MODE2: u8           = 0x02;
#[allow(dead_code)]
const CCS811_DRIVE_MODE3: u8           = 0x03;
#[allow(dead_code)]
const CCS811_DRIVE_MODE4: u8           = 0x04;

/*!
 *  list of status codes to indicate sensor operation
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CCS811_STATUS_t {
    SENSOR_SUCCESS,
    SENSOR_ID_ERROR,
    SENSOR_I2C_ERROR,
    SENSOR_INTERNAL_ERROR,
    SENSOR_GENERIC_ERROR,
}

impl fmt::Display for CCS811_STATUS_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CCS811_STATUS_t::SENSOR_SUCCESS        => write!(f, "SENSOR_SUCCESS"),
            CCS811_STATUS_t::SENSOR_ID_ERROR       => write!(f, "SENSOR_ID_ERROR"),
            CCS811_STATUS_t::SENSOR_I2C_ERROR      => write!(f, "SENSOR_I2C_ERROR"),
            CCS811_STATUS_t::SENSOR_INTERNAL_ERROR => write!(f, "SENSOR_INTERNAL_ERROR"),
            CCS811_STATUS_t::SENSOR_GENERIC_ERROR  => write!(f, "SENSOR_GENERIC_ERROR"),
        }
    }
}

/*!
 * Status register bit fields
 */
#[derive(Debug, Clone, Copy, Default)]
pub struct CCS811_STATUS_REG_t {
    pub error: bool,        // bit 0
    // Reserved: bits 1-2
    pub data_ready: bool,   // bit 3
    pub app_valid: bool,    // bit 4
    // Reserved1: bits 5-6
    pub fw_mode: bool,      // bit 7
}

impl CCS811_STATUS_REG_t {
    fn from_byte(val: u8) -> Self {
        CCS811_STATUS_REG_t {
            error:      (val & (1 << 0)) != 0,
            data_ready: (val & (1 << 3)) != 0,
            app_valid:  (val & (1 << 4)) != 0,
            fw_mode:    (val & (1 << 7)) != 0,
        }
    }
}

/*!
 * Measurement Mode register bit fields
 */
#[derive(Debug, Clone, Copy, Default)]
pub struct CCS811_MEAS_MODE_REG_t {
    // Reserved: bits 0-1
    pub int_thresh: bool,   // bit 2
    pub int_datardy: bool,  // bit 3
    pub drive_mode: u8,     // bits 4-6
    // Reserved1: bit 7
}

impl CCS811_MEAS_MODE_REG_t {
    fn from_byte(val: u8) -> Self {
        CCS811_MEAS_MODE_REG_t {
            int_thresh:  (val & (1 << 2)) != 0,
            int_datardy: (val & (1 << 3)) != 0,
            drive_mode:  (val >> 4) & 0x07,
        }
    }
}

/*!
 * Error ID register bit fields
 */
#[derive(Debug, Clone, Copy, Default)]
pub struct CCS811_ERROR_ID_REG_t {
    pub write_reg_invalid: bool,  // bit 0
    pub read_reg_invalid: bool,   // bit 1
    pub measmode_invalid: bool,   // bit 2
    pub max_resistance: bool,     // bit 3
    pub heater_fault: bool,       // bit 4
    pub heater_supply: bool,      // bit 5
    // Reserved: bits 6-7
}

impl CCS811_ERROR_ID_REG_t {
    fn from_byte(val: u8) -> Self {
        CCS811_ERROR_ID_REG_t {
            write_reg_invalid: (val & (1 << 0)) != 0,
            read_reg_invalid:  (val & (1 << 1)) != 0,
            measmode_invalid:  (val & (1 << 2)) != 0,
            max_resistance:    (val & (1 << 3)) != 0,
            heater_fault:      (val & (1 << 4)) != 0,
            heater_supply:     (val & (1 << 5)) != 0,
        }
    }
}

/// Hardware abstraction trait for I2C bus operations.
/// Implement this trait for your target platform.
pub trait I2CBus {
    fn begin_transmission(&mut self, address: u8);
    fn write_byte(&mut self, data: u8);
    fn end_transmission(&mut self) -> u8;
    fn request_from(&mut self, address: u8, quantity: u8, stop: u8);
    fn read_byte(&mut self) -> u8;
    fn available(&self) -> u8;
}

/// Delay abstraction trait.
/// Implement this trait for your target platform.
pub trait DelayMs {
    fn delay_ms(&mut self, ms: u16);
}

/// Printing abstraction trait (mirrors Arduino Serial.print functionality).
/// Implement this trait for your target platform, or use the no-op default.
pub trait Printer {
    fn print_str(&mut self, _s: &str) {}
    fn print_u16(&mut self, _val: u16) {}
    fn println_str(&mut self, _s: &str) {}
}

pub struct AirQuality<I2C: I2CBus, D: DelayMs, P: Printer> {
    _t_voc: u16,
    _co2: u16,
    _v_ref: u16,
    _v_ntc: u16,
    _ref_resistance: f32,
    _resistance: f32,
    _temperature: f32,
    error_id_reg: CCS811_ERROR_ID_REG_t,
    meas_mode_reg: CCS811_MEAS_MODE_REG_t,
    status_reg: CCS811_STATUS_REG_t,
    _i2c_slave_address: u8,
    _is_connected: bool,
    i2c: I2C,
    delay: D,
    printer: P,
}

use CCS811_STATUS_t::*;

impl<I2C: I2CBus, D: DelayMs, P: Printer> AirQuality<I2C, D, P> {

    /**
     *
     */
    pub fn new(i2c: I2C, delay: D, printer: P, i2c_add: Option<u8>, ref_res: Option<f32>) -> Self {
        let _ = i2c_add.unwrap_or(CCS811_I2C_ADDRESS1);
        AirQuality {
            _t_voc: 0,
            _co2: 0,
            _v_ref: 0,
            _v_ntc: 0,
            _ref_resistance: ref_res.unwrap_or(10000.0),
            _resistance: 0.0,
            _temperature: 0.0,
            error_id_reg: CCS811_ERROR_ID_REG_t::default(),
            meas_mode_reg: CCS811_MEAS_MODE_REG_t::default(),
            status_reg: CCS811_STATUS_REG_t::default(),
            _i2c_slave_address: 0x5A,
            _is_connected: false,
            i2c,
            delay,
            printer,
        }
    }

    /**
     *
     */
    pub fn begin(&mut self) -> CCS811_STATUS_t {
        /* make soft-reset the chip */
        let result = self.reset();
        if result != SENSOR_SUCCESS {
            return result;
        }

        /* wait until soft-reset completion */
        self.delay.delay_ms(100);
        if self.get_hw_id() != CCS811_HW_ID {
            return SENSOR_ID_ERROR;
        }

        /* get the status */
        if self.get_status_reg() != SENSOR_SUCCESS {
            return SENSOR_I2C_ERROR;
        }

        /* Check for Application validity */
        if !self.status_reg.app_valid || self.status_reg.error {
            return SENSOR_INTERNAL_ERROR;
        }

        /* Change from bootmode to application running */
        if self.write_byte_reg(CCS811_APP_START_REG) != SENSOR_SUCCESS {
            return SENSOR_I2C_ERROR;
        }

        self.delay.delay_ms(100);
        /* get the status */
        if self.get_status_reg() != SENSOR_SUCCESS {
            return SENSOR_I2C_ERROR;
        }

        /* Check the firmware mode */
        if !self.status_reg.fw_mode {
            return SENSOR_INTERNAL_ERROR;
        }

        if self.set_drive_mode(CCS811_DRIVE_MODE1) != SENSOR_SUCCESS {
            return SENSOR_I2C_ERROR;
        }
        self._is_connected = true;
        SENSOR_SUCCESS
    }

    /**
     *
     */
    pub fn reset(&mut self) -> CCS811_STATUS_t {
        let seq: [u8; 4] = [0x11, 0xE5, 0x72, 0x8A];
        self.write_multi_bytes(CCS811_SOFT_RESET_REG, &seq)
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
    pub fn read_algorithm_results(&mut self) -> CCS811_STATUS_t {
        let mut data = [0u8; 4];
        let result = self.read_multi_bytes(CCS811_ALG_RESULT_DATA_REG, &mut data);
        if result != SENSOR_SUCCESS {
            return result;
        }
        self._co2  = ((data[0] as u16) << 8) | data[1] as u16;
        self._t_voc = ((data[2] as u16) << 8) | data[3] as u16;
        result
    }

    /*
     *
     */
    pub fn set_ref_resistance(&mut self, ref_res: f32) {
        self._ref_resistance = ref_res;
    }

    /*
     *
     */
    pub fn set_base_line(&mut self, base_line: u16) -> CCS811_STATUS_t {
        let data: [u8; 2] = [
            (base_line >> 8) as u8,
            (base_line & 0xFF) as u8,
        ];
        self.write_multi_bytes(CCS811_BASELINE_REG, &data)
    }

    /*
     *
     */
    pub fn get_base_line(&mut self) -> u16 {
        let mut data = [0u8; 2];
        let result = self.read_multi_bytes(CCS811_BASELINE_REG, &mut data);
        if result != SENSOR_SUCCESS {
            return 0;
        }
        ((data[1] as u16) << 8) | data[0] as u16
    }

    /*
     *
     */
    pub fn set_environmental_data(&mut self, relative_humidity: f32, ambient_temperature: f32) -> CCS811_STATUS_t {
        let rh = (relative_humidity * 512.0) as u16;
        let temp = ((ambient_temperature + 25.0) * 512.0) as u16;
        let data: [u8; 4] = [
            /* Calculate humidity higher & lower bytes */
            (rh >> 8) as u8,
            (rh & 0xFF) as u8,
            /* Calculate temperature higher & lower bytes */
            (temp >> 8) as u8,
            (temp & 0xFF) as u8,
        ];
        /* Write enviromental data */
        self.write_multi_bytes(CCS811_ENV_DATA_REG, &data)
    }

    /*
     *
     */
    pub fn read_ntc(&mut self) -> CCS811_STATUS_t {
        let mut data = [0u8; 4];
        let result = self.read_multi_bytes(CCS811_NTC_REG, &mut data);
        if result != SENSOR_SUCCESS {
            return result;
        }
        self._v_ref = ((data[0] as u16) << 8) | data[1] as u16;
        self._v_ntc = ((data[2] as u16) << 8) | data[3] as u16;
        /* Calculate the thermistor resitance */
        let res = (self._v_ntc as f32) * self._ref_resistance / (self._v_ref as f32);
        /* Calculate the temperature */
        let ln_r = libm::logf(res);
        /* T = 1 / {A + B[ln(R)] + C[ln(R)]^3} A = 0.001129148 B = 0.000234125 C = 8.76741E-08 */
        let mut temp = 1.0 / (0.001129148 + (0.000234125 * ln_r) + (0.0000000876741 * ln_r * ln_r * ln_r));
        temp -= 273.15;
        /* update the variables */
        self._resistance = res;
        self._temperature = temp;
        result
    }

    /*
     *
     */
    pub fn enable_data_interrupt(&mut self) -> CCS811_STATUS_t {
        let mut data = 0u8;
        let result = self.read_byte(CCS811_MEAS_MODE_REG, &mut data);
        if result != SENSOR_SUCCESS {
            return result;
        }
        /* set the data interrupt flag */
        data |= CCS811_INT_DATARDY_MSK;
        self.write_byte_reg_val(CCS811_MEAS_MODE_REG, data)
    }

    /*
     *
     */
    pub fn disable_data_interrupt(&mut self) -> CCS811_STATUS_t {
        let mut data = 0u8;
        let result = self.read_byte(CCS811_MEAS_MODE_REG, &mut data);
        if result != SENSOR_SUCCESS {
            return result;
        }
        /* set the data interrupt flag */
        data &= !CCS811_INT_DATARDY_MSK;
        self.write_byte_reg_val(CCS811_MEAS_MODE_REG, data)
    }

    /*
     *
     */
    pub fn enable_thresh_interrupt(&mut self) -> CCS811_STATUS_t {
        let mut data = 0u8;
        let result = self.read_byte(CCS811_MEAS_MODE_REG, &mut data);
        if result != SENSOR_SUCCESS {
            return result;
        }
        /* set the data interrupt flag */
        data |= CCS811_INT_THRESH_MSK;
        self.write_byte_reg_val(CCS811_MEAS_MODE_REG, data)
    }

    /*
     *
     */
    pub fn disable_thresh_interrupt(&mut self) -> CCS811_STATUS_t {
        let mut data = 0u8;
        let result = self.read_byte(CCS811_MEAS_MODE_REG, &mut data);
        if result != SENSOR_SUCCESS {
            return result;
        }
        /* set the data interrupt flag */
        data &= !CCS811_INT_THRESH_MSK;
        self.write_byte_reg_val(CCS811_MEAS_MODE_REG, data)
    }

    /*
     *
     */
    pub fn set_drive_mode(&mut self, mode: u8) -> CCS811_STATUS_t {
        let mut data = 0u8;
        let result = self.read_byte(CCS811_MEAS_MODE_REG, &mut data);
        if result != SENSOR_SUCCESS {
            return result;
        }
        /* set the data interrupt flag */
        data &= !CCS811_DRIVE_MODE_MSK;
        data |= mode << CCS811_DRIVE_MODE_POS;
        self.write_byte_reg_val(CCS811_MEAS_MODE_REG, data)
    }

    /*
     *
     */
    pub fn get_status_reg(&mut self) -> CCS811_STATUS_t {
        let mut val = 0u8;
        let result = self.read_byte(CCS811_STATUS_REG, &mut val);
        if result == SENSOR_SUCCESS {
            self.status_reg = CCS811_STATUS_REG_t::from_byte(val);
        }
        result
    }

    /*
     *
     */
    pub fn get_status_reg_out(&mut self, status: &mut CCS811_STATUS_REG_t) -> CCS811_STATUS_t {
        let mut val = 0u8;
        let result = self.read_byte(CCS811_STATUS_REG, &mut val);
        if result == SENSOR_SUCCESS {
            *status = CCS811_STATUS_REG_t::from_byte(val);
        }
        result
    }

    /*
     *
     */
    pub fn get_meas_mode_reg(&mut self, meas_mode: &mut CCS811_MEAS_MODE_REG_t) -> CCS811_STATUS_t {
        let mut val = 0u8;
        let result = self.read_byte(CCS811_MEAS_MODE_REG, &mut val);
        if result == SENSOR_SUCCESS {
            *meas_mode = CCS811_MEAS_MODE_REG_t::from_byte(val);
        }
        result
    }

    /*
     *
     */
    pub fn get_error_id_reg(&mut self, error_id: &mut CCS811_ERROR_ID_REG_t) -> CCS811_STATUS_t {
        let mut val = 0u8;
        let result = self.read_byte(CCS811_ERROR_ID_REG, &mut val);
        if result == SENSOR_SUCCESS {
            *error_id = CCS811_ERROR_ID_REG_t::from_byte(val);
        }
        result
    }

    /*
     *
     */
    pub fn get_tvoc(&mut self, print: bool) -> u16 {
        if print {
            self.printer.print_str("TVOC: ");
            self.printer.print_u16(self._t_voc);
            self.printer.println_str("ppb");
        }
        self._t_voc
    }

    /*
     *
     */
    pub fn get_co2(&mut self, print: bool) -> u16 {
        if print {
            self.printer.print_str("eCO2: ");
            self.printer.print_u16(self._co2);
            self.printer.println_str("ppm");
        }
        self._co2
    }

    /*
     *
     */
    pub fn get_resistance(&self) -> f32 {
        self._resistance
    }

    /*
     *
     */
    pub fn get_temperature(&self) -> f32 {
        self._temperature
    }

    /*
     *
     */
    pub fn get_hw_id(&mut self) -> u8 {
        let mut hw_id = 0u8;
        let result = self.read_byte(CCS811_HW_ID_REG, &mut hw_id);
        if result != SENSOR_SUCCESS {
            return 0;
        }
        hw_id
    }

    /*
     *
     */
    pub fn is_data_available(&mut self) -> bool {
        if self.get_status_reg() != SENSOR_SUCCESS {
            return false;
        }
        self.status_reg.data_ready
    }

    /*
     *
     */
    pub fn get_hw_version(&mut self) -> Option<[u8; 10]> {
        let mut data = 0u8;
        let mut version_info = [0u8; 10];
        if self.read_byte(CCS811_HW_VER_REG, &mut data) != SENSOR_SUCCESS {
            return None;
        }
        let major = data >> 4;
        let minor = data & 0x0F;
        let s = fmt_version_2(&mut version_info, major, minor);
        if s { Some(version_info) } else { None }
    }

    /*
     *
     */
    pub fn get_fw_boot_version(&mut self) -> Option<[u8; 10]> {
        let mut data = [0u8; 2];
        let mut version_info = [0u8; 10];
        if self.read_multi_bytes(CCS811_FW_BOOT_VER_REG, &mut data) != SENSOR_SUCCESS {
            return None;
        }
        let major = data[0] >> 4;
        let minor = data[0] & 0x0F;
        let patch = data[1];
        let s = fmt_version_3(&mut version_info, major, minor, patch);
        if s { Some(version_info) } else { None }
    }

    /*
     *
     */
    pub fn get_fw_app_version(&mut self) -> Option<[u8; 10]> {
        let mut data = [0u8; 2];
        let mut version_info = [0u8; 10];
        if self.read_multi_bytes(CCS811_FW_APP_VER_REG, &mut data) != SENSOR_SUCCESS {
            return None;
        }
        let major = data[0] >> 4;
        let minor = data[0] & 0x0F;
        let patch = data[1];
        let s = fmt_version_3(&mut version_info, major, minor, patch);
        if s { Some(version_info) } else { None }
    }

    /***********************************************************************************************
     * Platform dependent routines. Change these functions implementation based on microcontroller *
     ***********************************************************************************************/
    /**
     *
     */
    fn read_byte(&mut self, reg: u8, val: &mut u8) -> CCS811_STATUS_t {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg);
        if self.i2c.end_transmission() != 0 {
            return SENSOR_I2C_ERROR;
        }
        self.i2c.request_from(self._i2c_slave_address, 1, 1);
        if self.i2c.available() != 1 {
            return SENSOR_I2C_ERROR;
        }
        *val = self.i2c.read_byte();
        SENSOR_SUCCESS
    }

    /**
     *
     */
    fn read_multi_bytes(&mut self, reg: u8, buf: &mut [u8]) -> CCS811_STATUS_t {
        let length = buf.len() as u8;
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg);
        if self.i2c.end_transmission() != 0 {
            return SENSOR_I2C_ERROR;
        }
        self.i2c.request_from(self._i2c_slave_address, length, 1);
        if self.i2c.available() != length {
            return SENSOR_I2C_ERROR;
        }
        for n_data in 0..length as usize {
            buf[n_data] = self.i2c.read_byte();
        }
        SENSOR_SUCCESS
    }

    /**
     *
     */
    fn write_byte_reg(&mut self, reg: u8) -> CCS811_STATUS_t {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg);
        if self.i2c.end_transmission() == 0 {
            return SENSOR_SUCCESS;
        }
        SENSOR_I2C_ERROR
    }

    /**
     *
     */
    fn write_byte_reg_val(&mut self, reg: u8, val: u8) -> CCS811_STATUS_t {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg);
        self.i2c.write_byte(val);
        if self.i2c.end_transmission() == 0 {
            return SENSOR_SUCCESS;
        }
        SENSOR_I2C_ERROR
    }

    /**
     *
     */
    fn write_multi_bytes(&mut self, reg: u8, out: &[u8]) -> CCS811_STATUS_t {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg);
        for &byte in out {
            self.i2c.write_byte(byte);
        }
        if self.i2c.end_transmission() == 0 {
            return SENSOR_SUCCESS;
        }
        SENSOR_I2C_ERROR
    }

    /**
     *
     */
    fn write_address(&mut self) -> bool {
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.end_transmission() == 0
    }
}

/// Format a two-part version string (e.g. "1.0") into a fixed buffer.
/// Returns true on success.
fn fmt_version_2(buf: &mut [u8; 10], major: u8, minor: u8) -> bool {
    let mut pos = 0usize;
    pos += write_u8_to_buf(&mut buf[pos..], major);
    buf[pos] = b'.'; pos += 1;
    pos += write_u8_to_buf(&mut buf[pos..], minor);
    buf[pos] = 0; // null terminator
    true
}

/// Format a three-part version string (e.g. "1.0.2") into a fixed buffer.
/// Returns true on success.
fn fmt_version_3(buf: &mut [u8; 10], major: u8, minor: u8, patch: u8) -> bool {
    let mut pos = 0usize;
    pos += write_u8_to_buf(&mut buf[pos..], major);
    buf[pos] = b'.'; pos += 1;
    pos += write_u8_to_buf(&mut buf[pos..], minor);
    buf[pos] = b'.'; pos += 1;
    pos += write_u8_to_buf(&mut buf[pos..], patch);
    buf[pos] = 0; // null terminator
    true
}

/// Write a u8 decimal representation into a byte buffer. Returns number of bytes written.
fn write_u8_to_buf(buf: &mut [u8], val: u8) -> usize {
    if val >= 100 {
        buf[0] = b'0' + (val / 100);
        buf[1] = b'0' + ((val / 10) % 10);
        buf[2] = b'0' + (val % 10);
        3
    } else if val >= 10 {
        buf[0] = b'0' + (val / 10);
        buf[1] = b'0' + (val % 10);
        2
    } else {
        buf[0] = b'0' + val;
        1
    }
}
