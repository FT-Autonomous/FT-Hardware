/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Synopsis of Accelerometer and Gyroscope
  MYOSA Platform consists of an Accelerometer and Gyroscope Board. It is equiped with GY521/MPU6050 IC.
  MPU6050 provides a general X/Y/Z direction (3-axis) accelerometer and gyroscope.
  I2C Address of the board = 0x69.
  Detailed Information about Accelerometer And Gyroscope board Library and usage is provided in the link below.
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

#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::f32::consts::PI;

// ---------------------------------------------------------------------------
// Register addresses
// ---------------------------------------------------------------------------

const MPU6050_ADDRESS_AD0_LOW: u8             = 0x68;
const MPU6050_ADDRESS_AD0_HIGH: u8            = 0x69;

const MPU6050_XA_OFFS_USRH_REG: u8           = 0x06;
const MPU6050_XA_OFFS_USRL_REG: u8           = 0x07;
const MPU6050_YA_OFFS_USRH_REG: u8           = 0x08;
const MPU6050_YA_OFFS_USRL_REG: u8           = 0x09;
const MPU6050_ZA_OFFS_USRH_REG: u8           = 0x0A;
const MPU6050_ZA_OFFS_USRL_REG: u8           = 0x0B;
const MPU6050_SELF_TEST_X_REG: u8            = 0x0D;
const MPU6050_SELF_TEST_Y_REG: u8            = 0x0E;
const MPU6050_SELF_TEST_Z_REG: u8            = 0x0F;
const MPU6050_SELF_TEST_A_REG: u8            = 0x10;
const MPU6050_XG_OFFS_USRH_REG: u8           = 0x13;
const MPU6050_XG_OFFS_USRL_REG: u8           = 0x14;
const MPU6050_YG_OFFS_USRH_REG: u8           = 0x15;
const MPU6050_YG_OFFS_USRL_REG: u8           = 0x16;
const MPU6050_ZG_OFFS_USRH_REG: u8           = 0x17;
const MPU6050_ZG_OFFS_USRL_REG: u8           = 0x18;
const MPU6050_SMPLRT_DIV_REG: u8             = 0x19;
const MPU6050_CONFIG_REG: u8                 = 0x1A;
const MPU6050_GYRO_CONFIG_REG: u8            = 0x1B;
const MPU6050_ACCEL_CONFIG_REG: u8           = 0x1C;
const MPU6050_MOTION_THR: u8                 = 0x1F;
const MPU6050_MOTION_DUR: u8                 = 0x20;
const MPU6050_ZERO_MOTION_THR: u8            = 0x21;
const MPU6050_ZERO_MOTION_DUR: u8            = 0x22;
const MPU6050_INT_ENABLE: u8                 = 0x38;
const MPU6050_INT_STATUS: u8                 = 0x3A;
const MPU6050_ACCEL_XOUT_H_REG: u8           = 0x3B;
const MPU6050_ACCEL_XOUT_L_REG: u8           = 0x3C;
const MPU6050_ACCEL_YOUT_H_REG: u8           = 0x3D;
const MPU6050_ACCEL_YOUT_L_REG: u8           = 0x3E;
const MPU6050_ACCEL_ZOUT_H_REG: u8           = 0x3F;
const MPU6050_ACCEL_ZOUT_L_REG: u8           = 0x40;
const MPU6050_TEMP_OUT_H_REG: u8             = 0x41;
const MPU6050_TEMP_OUT_L_REG: u8             = 0x42;
const MPU6050_GYRO_XOUT_H_REG: u8            = 0x43;
const MPU6050_GYRO_XOUT_L_REG: u8            = 0x44;
const MPU6050_GYRO_YOUT_H_REG: u8            = 0x45;
const MPU6050_GYRO_YOUT_L_REG: u8            = 0x46;
const MPU6050_GYRO_ZOUT_H_REG: u8            = 0x47;
const MPU6050_GYRO_ZOUT_L_REG: u8            = 0x48;
const MPU6050_SIGNAL_PATH_RESET_REG: u8      = 0x68;
const MPU6050_USER_CTRL_REG: u8              = 0x6A;
const MPU6050_PWR_MGMT_1_REG: u8             = 0x6B;
const MPU6050_PWR_MGMT_2_REG: u8             = 0x6C;
const MPU6050_FIFO_COUNTH_REG: u8            = 0x72;
const MPU6050_FIFO_COUNTL_REG: u8            = 0x73;
const MPU6050_FIFO_R_W_REG: u8              = 0x74;
const MPU6050_WHO_AM_I_REG: u8               = 0x75;

// ---------------------------------------------------------------------------
// Config register bit masks and positions
// ---------------------------------------------------------------------------

const MPU_CONFIG_DLPF_CFG_MSK: u8            = 0x07;
const MPU_CONFIG_DLPF_CFG_POS: u8            = 0x00;
const MPU_CONFIG_EXT_SYNC_SET_MSK: u8        = 0x38;
const MPU_CONFIG_EXT_SYNC_SET_POS: u8        = 0x03;

// ---------------------------------------------------------------------------
// Gyro config register bit masks and positions
// ---------------------------------------------------------------------------

const MPU_GYRO_CONFIG_XG_ST_MASK: u8         = 0x80;
const MPU_GYRO_CONFIG_XG_ST_POS: u8          = 0x07;
const MPU_GYRO_CONFIG_YG_ST_MASK: u8         = 0x40;
const MPU_GYRO_CONFIG_YG_ST_POS: u8          = 0x06;
const MPU_GYRO_CONFIG_ZG_ST_MASK: u8         = 0x20;
const MPU_GYRO_CONFIG_ZG_ST_POS: u8          = 0x05;
const MPU_GYRO_CONFIG_FS_SEL_MASK: u8        = 0x18;
const MPU_GYRO_CONFIG_FS_SEL_POS: u8         = 0x03;

const MPU_GYRO_CONFIG_FS_SEL_250: u8         = 0x00;
const MPU_GYRO_CONFIG_FS_SEL_500: u8         = 0x01;
const MPU_GYRO_CONFIG_FS_SEL_1000: u8        = 0x02;
const MPU_GYRO_CONFIG_FS_SEL_2000: u8        = 0x03;

// ---------------------------------------------------------------------------
// Accel config register bit masks and positions
// ---------------------------------------------------------------------------

const MPU_ACCEL_CONFIG_XG_ST_MASK: u8        = 0x80;
const MPU_ACCEL_CONFIG_XG_ST_POS: u8         = 0x07;
const MPU_ACCEL_CONFIG_YG_ST_MASK: u8        = 0x40;
const MPU_ACCEL_CONFIG_YG_ST_POS: u8         = 0x06;
const MPU_ACCEL_CONFIG_ZG_ST_MASK: u8        = 0x20;
const MPU_ACCEL_CONFIG_ZG_ST_POS: u8         = 0x05;
const MPU_ACCEL_CONFIG_FS_SEL_MASK: u8       = 0x18;
const MPU_ACCEL_CONFIG_FS_SEL_POS: u8        = 0x03;

const MPU_ACCEL_CONFIG_FS_SEL_2g: u8         = 0x00;
const MPU_ACCEL_CONFIG_FS_SEL_4g: u8         = 0x01;
const MPU_ACCEL_CONFIG_FS_SEL_8g: u8         = 0x02;
const MPU_ACCEL_CONFIG_FS_SEL_16g: u8        = 0x03;

// ---------------------------------------------------------------------------
// Signal path reset register bit masks and positions
// ---------------------------------------------------------------------------

const MPU_SIGNAL_PATH_GYRO_RESET_MSK: u8     = 0x04;
const MPU_SIGNAL_PATH_GYRO_RESET_POS: u8     = 0x02;
const MPU_SIGNAL_PATH_ACCEL_RESET_MSK: u8    = 0x02;
const MPU_SIGNAL_PATH_ACCEL_RESET_POS: u8    = 0x01;
const MPU_SIGNAL_PATH_TEMP_RESET_MSK: u8     = 0x01;
const MPU_SIGNAL_PATH_TEMP_RESET_POS: u8     = 0x00;

// ---------------------------------------------------------------------------
// Power management 1 register bit masks and positions
// ---------------------------------------------------------------------------

const MPU_PWR_MGMT_1_DEVICE_RESET_MSK: u8    = 0x80;
const MPU_PWR_MGMT_1_DEVICE_RESET_POS: u8    = 0x07;
const MPU_PWR_MGMT_1_SLEEP_MSK: u8           = 0x40;
const MPU_PWR_MGMT_1_SLEEP_POS: u8           = 0x06;
const MPU_PWR_MGMT_1_CYCLE_MSK: u8           = 0x20;
const MPU_PWR_MGMT_1_CYCLE_POS: u8           = 0x05;
const MPU_PWR_MGMT_1_TEMP_DIS_MSK: u8        = 0x08;
const MPU_PWR_MGMT_1_TEMP_DIS_POS: u8        = 0x03;
const MPU_PWR_MGMT_1_CLKSEL_MSK: u8          = 0x07;
const MPU_PWR_MGMT_1_CLKSEL_POS: u8          = 0x00;

const MPU6050_CLOCK_INTERNAL: u8             = 0x00;
const MPU6050_CLOCK_PLL_XGYRO: u8            = 0x01;
const MPU6050_CLOCK_PLL_YGYRO: u8            = 0x02;
const MPU6050_CLOCK_PLL_ZGYRO: u8            = 0x03;
const MPU6050_CLOCK_PLL_EXT32K: u8           = 0x04;
const MPU6050_CLOCK_PLL_EXT19M: u8           = 0x05;
const MPU6050_CLOCK_KEEP_RESET: u8           = 0x07;

// ---------------------------------------------------------------------------
// Power management 2 register bit masks and positions
// ---------------------------------------------------------------------------

const MPU_PWR_MGMT_2_LP_WAKE_CTRL_MSK: u8    = 0xC0;
const MPU_PWR_MGMT_2_LP_WAKE_CTRL_POS: u8    = 0x06;
const MPU_PWR_MGMT_2_LP_STBY_XA_MSK: u8      = 0x20;
const MPU_PWR_MGMT_2_LP_STBY_XA_POS: u8      = 0x05;
const MPU_PWR_MGMT_2_LP_STBY_YA_MSK: u8      = 0x10;
const MPU_PWR_MGMT_2_LP_STBY_YA_POS: u8      = 0x04;
const MPU_PWR_MGMT_2_LP_STBY_ZA_MSK: u8      = 0x08;
const MPU_PWR_MGMT_2_LP_STBY_ZA_POS: u8      = 0x03;
const MPU_PWR_MGMT_2_LP_STBY_XG_MSK: u8      = 0x04;
const MPU_PWR_MGMT_2_LP_STBY_XG_POS: u8      = 0x02;
const MPU_PWR_MGMT_2_LP_STBY_YG_MSK: u8      = 0x02;
const MPU_PWR_MGMT_2_LP_STBY_YG_POS: u8      = 0x01;
const MPU_PWR_MGMT_2_LP_STBY_ZG_MSK: u8      = 0x01;
const MPU_PWR_MGMT_2_LP_STBY_ZG_POS: u8      = 0x00;

const MPU_PWR_MGMT_2_LP_WAKE_1P25Hz: u8      = 0x00;
const MPU_PWR_MGMT_2_LP_WAKE_5Hz: u8         = 0x01;
const MPU_PWR_MGMT_2_LP_WAKE_20Hz: u8        = 0x02;
const MPU_PWR_MGMT_2_LP_WAKE_40Hz: u8        = 0x03;

// ---------------------------------------------------------------------------
// Interrupt bit masks and positions
// ---------------------------------------------------------------------------

const MPU_INT_MOTION_DETECT_MSK: u8          = 0x40;
const MPU_INT_MOTION_DETECT_POS: u8          = 0x06;
const MPU_INT_ZMOTION_DETECT_MSK: u8         = 0x20;
const MPU_INT_ZMOTION_DETECT_POS: u8         = 0x05;

// ---------------------------------------------------------------------------
// Miscellaneous
// ---------------------------------------------------------------------------

const MPU_WHO_AM_I_MSK: u8                   = 0x7E;
const CALIBRATION_READINGS: u8               = 50;

// ---------------------------------------------------------------------------
// I2C hardware abstraction trait
// ---------------------------------------------------------------------------

/// Trait abstracting the I2C bus operations (mirrors Arduino Wire semantics).
pub trait I2CBus {
    /// Start an I2C transmission to the given address.
    fn begin_transmission(&mut self, address: u8);
    /// Write a single byte on the bus.
    fn write_byte(&mut self, data: u8);
    /// End the transmission. Returns 0 on success, non-zero on error.
    fn end_transmission(&mut self) -> u8;
    /// Request `count` bytes from the device at `address`.
    fn request_from(&mut self, address: u8, count: u8);
    /// Read a single byte from the bus.
    fn read_byte(&mut self) -> u8;
    /// Return the number of bytes available for reading.
    fn available(&self) -> u8;
}

/// Callback type for printing/logging sensor readings.
/// Implementations supply their own print function (or use the provided no-op).
pub type PrintFn = fn(&str);

fn _no_print(_msg: &str) {}

// ---------------------------------------------------------------------------
// Delay abstraction
// ---------------------------------------------------------------------------

/// Trait for providing a millisecond delay (platform-dependent).
pub trait DelayMs {
    fn delay_ms(&mut self, ms: u16);
}

// ---------------------------------------------------------------------------
// AccelAndGyro struct
// ---------------------------------------------------------------------------

pub struct AccelAndGyro<I2C: I2CBus, D: DelayMs> {
    accel_scale: [f32; 4],
    gyro_scale: [f32; 4],
    i2c_slave_address: u8,
    is_connected: bool,
    i2c: I2C,
    delay: D,
    print_fn: PrintFn,
}

impl<I2C: I2CBus, D: DelayMs> AccelAndGyro<I2C, D> {
    // -----------------------------------------------------------------------
    // Constructor
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn new(i2c: I2C, delay: D, i2c_add: Option<u8>, print_fn: Option<PrintFn>) -> Self {
        let address = i2c_add.unwrap_or(MPU6050_ADDRESS_AD0_HIGH);
        let pf = print_fn.unwrap_or(_no_print);

        /* 1g = 9.80665 m/s^2 */
        /* Update the Accelerometer and Gyrometer scale factors */
        let mut gyro_scale = [0.0f32; 4];
        let mut accel_scale = [0.0f32; 4];
        for fsr_sel in 0u32..4u32 {
            gyro_scale[fsr_sel as usize] =
                (250.0 * (1u32 << (fsr_sel + 1)) as f32) / 32768.0;
            accel_scale[fsr_sel as usize] =
                (2.0 * 9.80665 * (1u32 << (fsr_sel + 1)) as f32) / 32768.0;
        }

        AccelAndGyro {
            accel_scale,
            gyro_scale,
            i2c_slave_address: address,
            is_connected: false,
            i2c,
            delay,
            print_fn: pf,
        }
    }

    // -----------------------------------------------------------------------
    // begin
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn begin(&mut self, calibrate: bool) -> bool {
        if !self.set_clk_source(MPU6050_CLOCK_PLL_XGYRO) {
            return false;
        }
        if !self.set_full_scale_gyro_range(MPU_GYRO_CONFIG_FS_SEL_250) {
            return false;
        }
        if !self.set_full_scale_accel_range(MPU_ACCEL_CONFIG_FS_SEL_2g) {
            return false;
        }
        if !self.set_sleep(false) {
            return false;
        }
        if !self.set_int_zero_motion_enabled(false) {
            return false;
        }
        if !self.set_int_motion_enabled(true) {
            return false;
        }
        if !self.set_motion_detection_threshold(2) {
            return false;
        }
        if !self.set_zero_motion_detection_threshold(2) {
            return false;
        }
        if !self.set_motion_detection_duration(40) {
            return false;
        }
        if !self.set_zero_motion_detection_duration(1) {
            return false;
        }
        /* Calibrate the sensor if required */
        if calibrate {
            if !self.accel_gyro_calibrate() {
                return false;
            }
        }
        self.is_connected = true;
        true
    }

    // -----------------------------------------------------------------------
    // Calibration
    // -----------------------------------------------------------------------

    pub fn accel_gyro_calibrate(&mut self) -> bool {
        let mut sum_ax: f32 = 0.0;
        let mut sum_ay: f32 = 0.0;
        let mut sum_az: f32 = 0.0;
        let mut sum_gx: f32 = 0.0;
        let mut sum_gy: f32 = 0.0;
        let mut sum_gz: f32 = 0.0;
        self.get_accel_x(false);
        self.get_accel_y(false);
        self.get_accel_z(false);
        self.get_gyro_x(false);
        self.get_gyro_y(false);
        self.get_gyro_z(false);
        for _n_reading in 0..CALIBRATION_READINGS {
            sum_ax += self.get_accel_x(false);
            sum_ay += self.get_accel_y(false);
            sum_az += self.get_accel_z(false);
            sum_gx += self.get_gyro_x(false);
            sum_gy += self.get_gyro_y(false);
            sum_gz += self.get_gyro_z(false);
            self.delay.delay_ms(20);
        }
        let cal = CALIBRATION_READINGS as f32;
        let mean_ax = sum_ax / cal;
        let mean_ay = sum_ay / cal;
        let mean_az = sum_az / cal;
        let mean_gx = sum_gx / cal;
        let mean_gy = sum_gy / cal;
        let mean_gz = sum_gz / cal;

        // NOTE: The original C++ code has a copy-paste quirk in the label strings.
        //       We preserve the same output text for 1:1 compatibility.
        let pf = self.print_fn;
        pf("Calibrate values\n");
        pf(&format!("meanAx:{:.2}\n", mean_ax));
        pf(&format!("meanAy:{:.2}\n", mean_ay));
        pf(&format!("meanAx:{:.2}\n", mean_az));  // original label says "meanAx" for Z
        pf(&format!("meanAz:{:.2}\n", mean_gx));  // original label says "meanAz" for Gx
        pf(&format!("meanGx:{:.2}\n", mean_gy));  // original label says "meanGx" for Gy
        pf(&format!("meanGz:{:.2}\n", mean_gz));
        true
    }

    // -----------------------------------------------------------------------
    // ping
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn ping(&mut self) -> bool {
        let get_connect_sts = self.write_address();
        if !self.is_connected && get_connect_sts {
            self.begin(false);
        }
        self.is_connected = get_connect_sts;
        get_connect_sts
    }

    // -----------------------------------------------------------------------
    // getDeviceId
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn get_device_id(&mut self) -> u8 {
        let mut device_id: u8 = 0;
        if !self.read_byte(MPU6050_WHO_AM_I_REG, &mut device_id) {
            device_id = 0;
        }
        device_id & MPU_WHO_AM_I_MSK
    }

    // -----------------------------------------------------------------------
    // reset
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn reset(&mut self) -> bool {
        let mut pwr_mgmt1_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_1_REG, &mut pwr_mgmt1_val) {
            return false;
        }
        pwr_mgmt1_val |= MPU_PWR_MGMT_1_DEVICE_RESET_MSK;
        self.write_byte_val(MPU6050_PWR_MGMT_1_REG, pwr_mgmt1_val)
    }

    // -----------------------------------------------------------------------
    // resetGyroPath
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn reset_gyro_path(&mut self) -> bool {
        let mut signal_path: u8 = 0;
        if !self.read_byte(MPU6050_SIGNAL_PATH_RESET_REG, &mut signal_path) {
            return false;
        }
        signal_path |= MPU_SIGNAL_PATH_GYRO_RESET_MSK;
        self.write_byte_val(MPU6050_PWR_MGMT_1_REG, signal_path)
    }

    // -----------------------------------------------------------------------
    // resetAccelPath
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn reset_accel_path(&mut self) -> bool {
        let mut signal_path: u8 = 0;
        if !self.read_byte(MPU6050_SIGNAL_PATH_RESET_REG, &mut signal_path) {
            return false;
        }
        signal_path |= MPU_SIGNAL_PATH_ACCEL_RESET_MSK;
        self.write_byte_val(MPU6050_PWR_MGMT_1_REG, signal_path)
    }

    // -----------------------------------------------------------------------
    // resetTempPath
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn reset_temp_path(&mut self) -> bool {
        let mut signal_path: u8 = 0;
        if !self.read_byte(MPU6050_SIGNAL_PATH_RESET_REG, &mut signal_path) {
            return false;
        }
        signal_path |= MPU_SIGNAL_PATH_TEMP_RESET_MSK;
        self.write_byte_val(MPU6050_PWR_MGMT_1_REG, signal_path)
    }

    // -----------------------------------------------------------------------
    // setFullScaleGyroRange / getFullScaleGyroRange
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn set_full_scale_gyro_range(&mut self, range: u8) -> bool {
        let mut gyro_config: u8 = 0;
        if !self.read_byte(MPU6050_GYRO_CONFIG_REG, &mut gyro_config) {
            return false;
        }
        gyro_config &= !MPU_GYRO_CONFIG_FS_SEL_MASK;
        gyro_config |= range << MPU_GYRO_CONFIG_FS_SEL_POS;
        self.write_byte_val(MPU6050_GYRO_CONFIG_REG, gyro_config)
    }

    /**
     *
     */
    pub fn get_full_scale_gyro_range(&mut self) -> u8 {
        let mut gyro_config: u8 = 0;
        if !self.read_byte(MPU6050_GYRO_CONFIG_REG, &mut gyro_config) {
            return 0x0F;
        }
        let range = (gyro_config & MPU_GYRO_CONFIG_FS_SEL_MASK) >> MPU_GYRO_CONFIG_FS_SEL_POS;
        range & 0x0F
    }

    // -----------------------------------------------------------------------
    // setFullScaleAccelRange / getFullScaleAccelRange
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn set_full_scale_accel_range(&mut self, range: u8) -> bool {
        let mut accel_config: u8 = 0;
        if !self.read_byte(MPU6050_ACCEL_CONFIG_REG, &mut accel_config) {
            return false;
        }
        accel_config &= !MPU_ACCEL_CONFIG_FS_SEL_MASK;
        accel_config |= range << MPU_ACCEL_CONFIG_FS_SEL_POS;
        self.write_byte_val(MPU6050_ACCEL_CONFIG_REG, accel_config)
    }

    /**
     *
     */
    pub fn get_full_scale_accel_range(&mut self) -> u8 {
        let mut accel_config: u8 = 0;
        if !self.read_byte(MPU6050_ACCEL_CONFIG_REG, &mut accel_config) {
            return 0x0F;
        }
        let range = (accel_config & MPU_ACCEL_CONFIG_FS_SEL_MASK) >> MPU_ACCEL_CONFIG_FS_SEL_POS;
        range & 0x0F
    }

    // -----------------------------------------------------------------------
    // setSleep / getSleepSts
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn set_sleep(&mut self, enable: bool) -> bool {
        let mut pwr_mgmt1_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_1_REG, &mut pwr_mgmt1_val) {
            return false;
        }
        pwr_mgmt1_val &= !MPU_PWR_MGMT_1_SLEEP_MSK;
        if enable {
            pwr_mgmt1_val |= MPU_PWR_MGMT_1_SLEEP_MSK;
        }
        self.write_byte_val(MPU6050_PWR_MGMT_1_REG, pwr_mgmt1_val)
    }

    /**
     *
     */
    pub fn get_sleep_sts(&mut self) -> bool {
        let mut pwr_mgmt1_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_1_REG, &mut pwr_mgmt1_val) {
            return false;
        }
        let sleep_state =
            (pwr_mgmt1_val & MPU_PWR_MGMT_1_SLEEP_MSK) >> MPU_PWR_MGMT_1_SLEEP_POS;
        sleep_state != 0
    }

    // -----------------------------------------------------------------------
    // setCycleMode / getCycleMode
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn set_cycle_mode(&mut self, enable: bool) -> bool {
        let mut pwr_mgmt1_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_1_REG, &mut pwr_mgmt1_val) {
            return false;
        }
        pwr_mgmt1_val &= !MPU_PWR_MGMT_1_CYCLE_MSK;
        if enable {
            pwr_mgmt1_val |= MPU_PWR_MGMT_1_CYCLE_MSK;
        }
        self.write_byte_val(MPU6050_PWR_MGMT_1_REG, pwr_mgmt1_val)
    }

    /**
     *
     */
    pub fn get_cycle_mode(&mut self) -> bool {
        let mut pwr_mgmt1_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_1_REG, &mut pwr_mgmt1_val) {
            return false;
        }
        let cycle_mode =
            (pwr_mgmt1_val & MPU_PWR_MGMT_1_CYCLE_MSK) >> MPU_PWR_MGMT_1_CYCLE_POS;
        cycle_mode != 0
    }

    // -----------------------------------------------------------------------
    // setTempSensorDisable / getTempSensorDisableSts
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn set_temp_sensor_disable(&mut self, enable: bool) -> bool {
        let mut pwr_mgmt1_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_1_REG, &mut pwr_mgmt1_val) {
            return false;
        }
        pwr_mgmt1_val &= !MPU_PWR_MGMT_1_TEMP_DIS_MSK;
        if enable {
            pwr_mgmt1_val |= MPU_PWR_MGMT_1_TEMP_DIS_MSK;
        }
        self.write_byte_val(MPU6050_PWR_MGMT_1_REG, pwr_mgmt1_val)
    }

    /**
     *
     */
    pub fn get_temp_sensor_disable_sts(&mut self) -> bool {
        let mut pwr_mgmt1_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_1_REG, &mut pwr_mgmt1_val) {
            return false;
        }
        let val =
            (pwr_mgmt1_val & MPU_PWR_MGMT_1_TEMP_DIS_MSK) >> MPU_PWR_MGMT_1_TEMP_DIS_POS;
        val != 0
    }

    // -----------------------------------------------------------------------
    // setClkSource / getClkSource
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn set_clk_source(&mut self, src: u8) -> bool {
        let mut pwr_mgmt1_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_1_REG, &mut pwr_mgmt1_val) {
            return false;
        }
        pwr_mgmt1_val &= !MPU_PWR_MGMT_1_CLKSEL_MSK;
        pwr_mgmt1_val |= src << MPU_PWR_MGMT_1_CLKSEL_POS;
        self.write_byte_val(MPU6050_PWR_MGMT_1_REG, pwr_mgmt1_val)
    }

    /**
     *
     */
    pub fn get_clk_source(&mut self) -> u8 {
        let mut pwr_mgmt1_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_1_REG, &mut pwr_mgmt1_val) {
            return 0;
        }
        (pwr_mgmt1_val & MPU_PWR_MGMT_1_CLKSEL_MSK) >> MPU_PWR_MGMT_1_CLKSEL_POS
    }

    // -----------------------------------------------------------------------
    // setWakeFrequency / getWakeFrequency
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn set_wake_frequency(&mut self, frequency: u8) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        pwr_mgmt2_val &= !MPU_PWR_MGMT_2_LP_WAKE_CTRL_MSK;
        pwr_mgmt2_val |= frequency << MPU_PWR_MGMT_2_LP_WAKE_CTRL_POS;
        self.write_byte_val(MPU6050_PWR_MGMT_2_REG, pwr_mgmt2_val)
    }

    /**
     *
     */
    pub fn get_wake_frequency(&mut self) -> u8 {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return 0;
        }
        (pwr_mgmt2_val & MPU_PWR_MGMT_2_LP_WAKE_CTRL_MSK) >> MPU_PWR_MGMT_2_LP_WAKE_CTRL_POS
    }

    // -----------------------------------------------------------------------
    // Standby X/Y/Z Accel
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn set_standby_x_accel(&mut self, enable: bool) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        pwr_mgmt2_val &= !MPU_PWR_MGMT_2_LP_STBY_XA_MSK;
        if enable {
            pwr_mgmt2_val |= MPU_PWR_MGMT_2_LP_STBY_XA_MSK;
        }
        self.write_byte_val(MPU6050_PWR_MGMT_2_REG, pwr_mgmt2_val)
    }

    /**
     *
     */
    pub fn get_standby_x_accel_sts(&mut self) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        let sts =
            (pwr_mgmt2_val & MPU_PWR_MGMT_2_LP_STBY_XA_MSK) >> MPU_PWR_MGMT_2_LP_STBY_XA_POS;
        sts != 0
    }

    /**
     *
     */
    pub fn set_standby_y_accel(&mut self, enable: bool) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        pwr_mgmt2_val &= !MPU_PWR_MGMT_2_LP_STBY_YA_MSK;
        if enable {
            pwr_mgmt2_val |= MPU_PWR_MGMT_2_LP_STBY_YA_MSK;
        }
        self.write_byte_val(MPU6050_PWR_MGMT_2_REG, pwr_mgmt2_val)
    }

    /**
     *
     */
    pub fn get_standby_y_accel_sts(&mut self) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        let sts =
            (pwr_mgmt2_val & MPU_PWR_MGMT_2_LP_STBY_YA_MSK) >> MPU_PWR_MGMT_2_LP_STBY_YA_POS;
        sts != 0
    }

    /**
     *
     */
    pub fn set_standby_z_accel(&mut self, enable: bool) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        pwr_mgmt2_val &= !MPU_PWR_MGMT_2_LP_STBY_ZA_MSK;
        if enable {
            pwr_mgmt2_val |= MPU_PWR_MGMT_2_LP_STBY_ZA_MSK;
        }
        self.write_byte_val(MPU6050_PWR_MGMT_2_REG, pwr_mgmt2_val)
    }

    /**
     *
     */
    pub fn get_standby_z_accel_sts(&mut self) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        let sts =
            (pwr_mgmt2_val & MPU_PWR_MGMT_2_LP_STBY_ZA_MSK) >> MPU_PWR_MGMT_2_LP_STBY_ZA_POS;
        sts != 0
    }

    // -----------------------------------------------------------------------
    // Standby X/Y/Z Gyro
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn set_standby_x_gyro(&mut self, enable: bool) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        pwr_mgmt2_val &= !MPU_PWR_MGMT_2_LP_STBY_XG_MSK;
        if enable {
            pwr_mgmt2_val |= MPU_PWR_MGMT_2_LP_STBY_XG_MSK;
        }
        self.write_byte_val(MPU6050_PWR_MGMT_2_REG, pwr_mgmt2_val)
    }

    /**
     *
     */
    pub fn get_standby_x_gyro_sts(&mut self) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        let sts =
            (pwr_mgmt2_val & MPU_PWR_MGMT_2_LP_STBY_XG_MSK) >> MPU_PWR_MGMT_2_LP_STBY_XG_POS;
        sts != 0
    }

    /**
     *
     */
    pub fn set_standby_y_gyro(&mut self, enable: bool) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        pwr_mgmt2_val &= !MPU_PWR_MGMT_2_LP_STBY_YG_MSK;
        if enable {
            pwr_mgmt2_val |= MPU_PWR_MGMT_2_LP_STBY_YG_MSK;
        }
        self.write_byte_val(MPU6050_PWR_MGMT_2_REG, pwr_mgmt2_val)
    }

    /**
     *
     */
    pub fn get_standby_y_gyro_sts(&mut self) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        let sts =
            (pwr_mgmt2_val & MPU_PWR_MGMT_2_LP_STBY_YG_MSK) >> MPU_PWR_MGMT_2_LP_STBY_YG_POS;
        sts != 0
    }

    /**
     *
     */
    pub fn set_standby_z_gyro(&mut self, enable: bool) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        pwr_mgmt2_val &= !MPU_PWR_MGMT_2_LP_STBY_ZG_MSK;
        if enable {
            pwr_mgmt2_val |= MPU_PWR_MGMT_2_LP_STBY_ZG_MSK;
        }
        self.write_byte_val(MPU6050_PWR_MGMT_2_REG, pwr_mgmt2_val)
    }

    /**
     *
     */
    pub fn get_standby_z_gyro_sts(&mut self) -> bool {
        let mut pwr_mgmt2_val: u8 = 0;
        if !self.read_byte(MPU6050_PWR_MGMT_2_REG, &mut pwr_mgmt2_val) {
            return false;
        }
        let sts =
            (pwr_mgmt2_val & MPU_PWR_MGMT_2_LP_STBY_ZG_MSK) >> MPU_PWR_MGMT_2_LP_STBY_ZG_POS;
        sts != 0
    }

    // -----------------------------------------------------------------------
    // Motion detection threshold / duration
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn get_motion_detection_threshold(&mut self) -> u8 {
        let mut threshold: u8 = 0;
        if self.read_byte(MPU6050_MOTION_THR, &mut threshold) {
            return threshold;
        }
        0
    }

    /**
     *
     */
    pub fn set_motion_detection_threshold(&mut self, threshold: u8) -> bool {
        self.write_byte_val(MPU6050_MOTION_THR, threshold)
    }

    /**
     *
     */
    pub fn get_motion_detection_duration(&mut self) -> u8 {
        let mut duration: u8 = 0;
        if self.read_byte(MPU6050_MOTION_DUR, &mut duration) {
            return duration;
        }
        0
    }

    /**
     *
     */
    pub fn set_motion_detection_duration(&mut self, duration: u8) -> bool {
        self.write_byte_val(MPU6050_MOTION_DUR, duration)
    }

    // -----------------------------------------------------------------------
    // Zero-motion detection threshold / duration
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn get_zero_motion_detection_threshold(&mut self) -> u8 {
        let mut threshold: u8 = 0;
        if self.read_byte(MPU6050_ZERO_MOTION_THR, &mut threshold) {
            return threshold;
        }
        0
    }

    /**
     *
     */
    pub fn set_zero_motion_detection_threshold(&mut self, threshold: u8) -> bool {
        self.write_byte_val(MPU6050_ZERO_MOTION_THR, threshold)
    }

    /**
     *
     */
    pub fn get_zero_motion_detection_duration(&mut self) -> u8 {
        let mut duration: u8 = 0;
        if self.read_byte(MPU6050_ZERO_MOTION_DUR, &mut duration) {
            return duration;
        }
        0
    }

    /**
     *
     */
    pub fn set_zero_motion_detection_duration(&mut self, duration: u8) -> bool {
        self.write_byte_val(MPU6050_ZERO_MOTION_DUR, duration)
    }

    // -----------------------------------------------------------------------
    // Interrupt enable / status for motion and zero-motion
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn get_int_motion_enabled(&mut self) -> bool {
        let mut int_enable: u8 = 0;
        if !self.read_byte(MPU6050_INT_ENABLE, &mut int_enable) {
            return false;
        }
        let motion_en =
            (int_enable & MPU_INT_MOTION_DETECT_MSK) >> MPU_INT_MOTION_DETECT_POS;
        motion_en != 0
    }

    /**
     *
     */
    pub fn set_int_motion_enabled(&mut self, enable: bool) -> bool {
        let mut int_enable: u8 = 0;
        if !self.read_byte(MPU6050_INT_ENABLE, &mut int_enable) {
            return false;
        }
        int_enable &= !MPU_INT_MOTION_DETECT_MSK;
        if enable {
            int_enable |= MPU_INT_MOTION_DETECT_MSK;
        }
        self.write_byte_val(MPU6050_INT_ENABLE, int_enable)
    }

    /**
     *
     */
    pub fn get_int_motion_status(&mut self) -> bool {
        let mut int_sts: u8 = 0;
        if !self.read_byte(MPU6050_INT_STATUS, &mut int_sts) {
            return false;
        }
        let motion_sts =
            (int_sts & MPU_INT_MOTION_DETECT_MSK) >> MPU_INT_MOTION_DETECT_POS;
        motion_sts != 0
    }

    /**
     *
     */
    pub fn get_int_zero_motion_enabled(&mut self) -> bool {
        let mut int_enable: u8 = 0;
        if !self.read_byte(MPU6050_INT_ENABLE, &mut int_enable) {
            return false;
        }
        let zero_motion_en =
            (int_enable & MPU_INT_ZMOTION_DETECT_MSK) >> MPU_INT_ZMOTION_DETECT_POS;
        zero_motion_en != 0
    }

    /**
     *
     */
    pub fn set_int_zero_motion_enabled(&mut self, enable: bool) -> bool {
        let mut int_enable: u8 = 0;
        if !self.read_byte(MPU6050_INT_ENABLE, &mut int_enable) {
            return false;
        }
        int_enable &= !MPU_INT_ZMOTION_DETECT_MSK;
        if enable {
            int_enable |= MPU_INT_ZMOTION_DETECT_MSK;
        }
        self.write_byte_val(MPU6050_INT_ENABLE, int_enable)
    }

    /**
     *
     */
    pub fn get_int_zero_motion_status(&mut self) -> bool {
        let mut int_sts: u8 = 0;
        if !self.read_byte(MPU6050_INT_STATUS, &mut int_sts) {
            return false;
        }
        let zero_motion_sts =
            (int_sts & MPU_INT_ZMOTION_DETECT_MSK) >> MPU_INT_ZMOTION_DETECT_POS;
        zero_motion_sts != 0
    }

    // -----------------------------------------------------------------------
    // Accelerometer axis readings
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn get_accel_x(&mut self, print: bool) -> f32 {
        let mut data = [0u8; 2];
        let mut raw: i16 = 0;
        if self.read_multi_bytes(MPU6050_ACCEL_XOUT_H_REG, 2, &mut data) {
            raw = ((data[0] as i16) << 8) | data[1] as i16;
        }
        let fsr_sel = self.get_full_scale_accel_range();
        if fsr_sel == 0x0F {
            return 0.0;
        }
        let a_x = raw as f32 * self.accel_scale[fsr_sel as usize] * 100.0;
        if print {
            (self.print_fn)(&format!("Acceleration(X): {:.2}cm/s^2\n", a_x));
        }
        a_x
    }

    /**
     *
     */
    pub fn get_accel_y(&mut self, print: bool) -> f32 {
        let mut data = [0u8; 2];
        let mut raw: i16 = 0;
        if self.read_multi_bytes(MPU6050_ACCEL_YOUT_H_REG, 2, &mut data) {
            raw = ((data[0] as i16) << 8) | data[1] as i16;
        }
        let fsr_sel = self.get_full_scale_accel_range();
        if fsr_sel == 0x0F {
            return 0.0;
        }
        let a_y = raw as f32 * self.accel_scale[fsr_sel as usize] * 100.0;
        if print {
            (self.print_fn)(&format!("Acceleration(Y): {:.2}cm/s^2\n", a_y));
        }
        a_y
    }

    /**
     *
     */
    pub fn get_accel_z(&mut self, print: bool) -> f32 {
        let mut data = [0u8; 2];
        let mut raw: i16 = 0;
        if self.read_multi_bytes(MPU6050_ACCEL_ZOUT_H_REG, 2, &mut data) {
            raw = ((data[0] as i16) << 8) | data[1] as i16;
        }
        let fsr_sel = self.get_full_scale_accel_range();
        if fsr_sel == 0x0F {
            return 0.0;
        }
        let a_z = raw as f32 * self.accel_scale[fsr_sel as usize] * 100.0;
        if print {
            (self.print_fn)(&format!("Acceleration(Z): {:.2}cm/s^2\n", a_z));
        }
        a_z
    }

    // -----------------------------------------------------------------------
    // Gyroscope axis readings
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn get_gyro_x(&mut self, print: bool) -> f32 {
        let mut data = [0u8; 2];
        let mut raw: i16 = 0;
        if self.read_multi_bytes(MPU6050_GYRO_XOUT_H_REG, 2, &mut data) {
            raw = ((data[0] as i16) << 8) | data[1] as i16;
        }
        let fsr_sel = self.get_full_scale_gyro_range();
        if fsr_sel == 0x0F {
            return 0.0;
        }
        let g_x = raw as f32 * self.gyro_scale[fsr_sel as usize];
        if print {
            (self.print_fn)(&format!("Angular Velocity(X): {:.2}\u{00B0}/s\n", g_x));
        }
        g_x
    }

    /**
     *
     */
    pub fn get_gyro_y(&mut self, print: bool) -> f32 {
        let mut data = [0u8; 2];
        let mut raw: i16 = 0;
        if self.read_multi_bytes(MPU6050_GYRO_YOUT_H_REG, 2, &mut data) {
            raw = ((data[0] as i16) << 8) | data[1] as i16;
        }
        let fsr_sel = self.get_full_scale_gyro_range();
        if fsr_sel == 0x0F {
            return 0.0;
        }
        // NOTE: The original C++ uses _accelScale here (apparent bug); preserved for 1:1 parity.
        let g_y = raw as f32 * self.accel_scale[fsr_sel as usize] * 100.0;
        if print {
            (self.print_fn)(&format!("Angular Velocity(Y): {:.2}\u{00B0}/s\n", g_y));
        }
        g_y
    }

    /**
     *
     */
    pub fn get_gyro_z(&mut self, print: bool) -> f32 {
        let mut data = [0u8; 2];
        let mut raw: i16 = 0;
        if self.read_multi_bytes(MPU6050_GYRO_ZOUT_H_REG, 2, &mut data) {
            raw = ((data[0] as i16) << 8) | data[1] as i16;
        }
        let fsr_sel = self.get_full_scale_gyro_range();
        if fsr_sel == 0x0F {
            return 0.0;
        }
        let g_z = raw as f32 * self.gyro_scale[fsr_sel as usize];
        if print {
            (self.print_fn)(&format!("Angular Velocity(Z): {:.2}\u{00B0}/s\n", g_z));
        }
        g_z
    }

    // -----------------------------------------------------------------------
    // Raw accel / gyro (private helpers)
    // -----------------------------------------------------------------------

    /**
     *
     */
    fn get_accel(&mut self) -> Option<(i16, i16, i16)> {
        let mut accel = [0u8; 6];
        if self.read_multi_bytes(MPU6050_ACCEL_XOUT_H_REG, 6, &mut accel) {
            let a_x = ((accel[0] as i16) << 8) | accel[1] as i16;
            let a_y = ((accel[2] as i16) << 8) | accel[3] as i16;
            let a_z = ((accel[4] as i16) << 8) | accel[5] as i16;
            return Some((a_x, a_y, a_z));
        }
        None
    }

    /**
     *
     */
    fn get_gyro(&mut self) -> Option<(i16, i16, i16)> {
        let mut gyro = [0u8; 6];
        if self.read_multi_bytes(MPU6050_GYRO_XOUT_H_REG, 6, &mut gyro) {
            let g_x = ((gyro[0] as i16) << 8) | gyro[1] as i16;
            let g_y = ((gyro[2] as i16) << 8) | gyro[3] as i16;
            let g_z = ((gyro[4] as i16) << 8) | gyro[5] as i16;
            return Some((g_x, g_y, g_z));
        }
        None
    }

    // -----------------------------------------------------------------------
    // Accel / Gyro offset registers
    // -----------------------------------------------------------------------

    /**
     *
     */
    fn get_accel_offset(&mut self) -> Option<(i16, i16, i16)> {
        let mut data = [0u8; 6];
        if self.read_multi_bytes(MPU6050_XA_OFFS_USRH_REG, 6, &mut data) {
            let a_x = ((data[0] as i16) << 8) | data[1] as i16;
            let a_y = ((data[2] as i16) << 8) | data[3] as i16;
            let a_z = ((data[4] as i16) << 8) | data[5] as i16;
            return Some((a_x, a_y, a_z));
        }
        None
    }

    /**
     *
     */
    fn set_accel_offset(&mut self, a_x: i16, a_y: i16, a_z: i16) -> bool {
        let data: [u8; 6] = [
            (a_x >> 8) as u8,
            (a_x & 0xFF) as u8,
            (a_y >> 8) as u8,
            (a_y & 0xFF) as u8,
            (a_z >> 8) as u8,
            (a_z & 0xFF) as u8,
        ];
        self.write_multi_bytes(MPU6050_XA_OFFS_USRH_REG, 6, &data)
    }

    /**
     *
     */
    fn get_gyro_offset(&mut self) -> Option<(i16, i16, i16)> {
        let mut data = [0u8; 6];
        if self.read_multi_bytes(MPU6050_XG_OFFS_USRH_REG, 6, &mut data) {
            let g_x = ((data[0] as i16) << 8) | data[1] as i16;
            let g_y = ((data[2] as i16) << 8) | data[3] as i16;
            let g_z = ((data[4] as i16) << 8) | data[5] as i16;
            return Some((g_x, g_y, g_z));
        }
        None
    }

    /**
     *
     */
    fn set_gyro_offset(&mut self, g_x: i16, g_y: i16, g_z: i16) -> bool {
        let data: [u8; 6] = [
            (g_x >> 8) as u8,
            (g_x & 0xFF) as u8,
            (g_y >> 8) as u8,
            (g_y & 0xFF) as u8,
            (g_z >> 8) as u8,
            (g_z & 0xFF) as u8,
        ];
        self.write_multi_bytes(MPU6050_XG_OFFS_USRH_REG, 6, &data)
    }

    // -----------------------------------------------------------------------
    // Temperature
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn get_temp_c(&mut self, print: bool) -> f32 {
        let mut data = [0u8; 2];
        if self.read_multi_bytes(MPU6050_TEMP_OUT_H_REG, 6, &mut data) {
            let temp: i16 = ((data[0] as i16) << 8) | data[1] as i16;
            let temp_c = (temp as f32) / 340.0 + 36.53;
            if print {
                (self.print_fn)(&format!(
                    "Temperature (\u{00B0}C): {:.2}\u{00B0}C\n",
                    temp_c
                ));
            }
            return temp_c;
        }
        0.0
    }

    /**
     *
     */
    pub fn get_temp_f(&mut self, print: bool) -> f32 {
        let temp_f = (self.get_temp_c(false) * (9.0 / 5.0)) + 32.0;
        if print {
            (self.print_fn)(&format!(
                "Temperature (\u{00B0}F): {:.2}\u{00B0}F\n",
                temp_f
            ));
        }
        temp_f
    }

    // -----------------------------------------------------------------------
    // Tilt angles
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn get_tilt_x(&mut self, print: bool) -> f32 {
        if let Some((a_x, a_y, a_z)) = self.get_accel() {
            let tilt_x = (180.0 / PI)
                * ((a_x as f32)
                    / ((a_y as f32).powi(2) + (a_z as f32).powi(2)).sqrt())
                .atan();
            if print {
                (self.print_fn)(&format!("Tilt Angle(X): {:.2}\u{00B0}\n", tilt_x));
            }
            return tilt_x;
        }
        0.0
    }

    /**
     *
     */
    pub fn get_tilt_y(&mut self, print: bool) -> f32 {
        if let Some((a_x, a_y, a_z)) = self.get_accel() {
            let tilt_y = (180.0 / PI)
                * ((a_y as f32)
                    / ((a_x as f32).powi(2) + (a_z as f32).powi(2)).sqrt())
                .atan();
            if print {
                (self.print_fn)(&format!("Tilt Angle(Y): {:.2}\u{00B0}\n", tilt_y));
            }
            return tilt_y;
        }
        0.0
    }

    /**
     *
     */
    pub fn get_tilt_z(&mut self, print: bool) -> f32 {
        if let Some((a_x, a_y, a_z)) = self.get_accel() {
            let tilt_z = (180.0 / PI)
                * (((a_x as f32).powi(2) + (a_y as f32).powi(2)).sqrt()
                    / (a_z as f32))
                .atan();
            if print {
                (self.print_fn)(&format!("Tilt Angle(Z): {:.2}\u{00B0}\n", tilt_z));
            }
            return tilt_z;
        }
        0.0
    }

    // -----------------------------------------------------------------------
    // Motion status
    // -----------------------------------------------------------------------

    /**
     *
     */
    pub fn get_motion_status(&mut self, print: bool) -> bool {
        let motion_sts = self.get_int_motion_status();
        if print {
            (self.print_fn)(&format!(
                "Motion Detection Status: {}\n",
                if motion_sts { "True" } else { "False" }
            ));
        }
        motion_sts
    }

    // -----------------------------------------------------------------------
    // Platform dependent routines. Change these functions implementation
    // based on microcontroller
    // -----------------------------------------------------------------------

    /***********************************************************************************************
     * Platform dependent routines. Change these functions implementation based on microcontroller *
     ***********************************************************************************************/

    /**
     *
     */
    fn read_byte(&mut self, reg: u8, val: &mut u8) -> bool {
        self.i2c.begin_transmission(self.i2c_slave_address);
        self.i2c.write_byte(reg);
        if self.i2c.end_transmission() != 0 {
            return false;
        }
        self.i2c.request_from(self.i2c_slave_address, 1);
        if self.i2c.available() != 1 {
            return false;
        }
        *val = self.i2c.read_byte();
        true
    }

    /**
     *
     */
    fn read_multi_bytes(&mut self, reg: u8, length: u8, buf: &mut [u8]) -> bool {
        self.i2c.begin_transmission(self.i2c_slave_address);
        self.i2c.write_byte(reg);
        if self.i2c.end_transmission() != 0 {
            return false;
        }
        self.i2c.request_from(self.i2c_slave_address, length);
        if self.i2c.available() != length {
            return false;
        }
        for i in 0..length as usize {
            buf[i] = self.i2c.read_byte();
        }
        true
    }

    /**
     *
     */
    fn write_byte_reg(&mut self, reg: u8) -> bool {
        self.i2c.begin_transmission(self.i2c_slave_address);
        self.i2c.write_byte(reg);
        self.i2c.end_transmission() == 0
    }

    /**
     *
     */
    fn write_byte_val(&mut self, reg: u8, val: u8) -> bool {
        self.i2c.begin_transmission(self.i2c_slave_address);
        self.i2c.write_byte(reg);
        self.i2c.write_byte(val);
        self.i2c.end_transmission() == 0
    }

    /**
     *
     */
    fn write_address(&mut self) -> bool {
        self.i2c.begin_transmission(self.i2c_slave_address);
        self.i2c.end_transmission() == 0
    }

    /**
     *
     */
    fn write_multi_bytes(&mut self, reg: u8, length: u8, data: &[u8]) -> bool {
        self.i2c.begin_transmission(self.i2c_slave_address);
        self.i2c.write_byte(reg);
        for i in 0..length as usize {
            self.i2c.write_byte(data[i]);
        }
        self.i2c.end_transmission() == 0
    }
}
