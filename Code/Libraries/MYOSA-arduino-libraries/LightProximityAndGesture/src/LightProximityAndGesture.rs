/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Synopsis of Light Proximity and Gesture Board
  MYOSA Platform consists of an Light Proximity and Gesture Board. It is equiped with APDS9960 IC.
  It is a digital RGB, ambient light, proximity and gesture sensor device with I2C compatible interface.
  I2C Address of the board = 0x39.
  Detailed Information about Light Proximity and Gesture board Library and usage is provided in the link below.
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

/* APDS-9960 I2C address */
pub const APDS9960_I2C_ADDRESS: u8     = 0x39;

/* Gesture parameters */
pub const GESTURE_THRESHOLD_OUT: u8    = 10;
pub const GESTURE_SENSITIVITY_1: i32   = 50;
pub const GESTURE_SENSITIVITY_2: i32   = 20;

/* Error code for returned values */
pub const ERROR: u8                    = 0xFF;

/* Acceptable device IDs */
pub const APDS9960_ID_1: u8           = 0xAB;
pub const APDS9960_ID_2: u8           = 0x9C;
pub const APDS9960_ID_3: u8           = 0xA8;

/* Misc parameters */
pub const FIFO_PAUSE_TIME: u16        = 30;       // Wait period (ms) between FIFO reads

/* APDS-9960 register addresses */
pub const APDS9960_ENABLE: u8         = 0x80;
pub const APDS9960_ATIME: u8          = 0x81;
pub const APDS9960_WTIME: u8          = 0x83;
pub const APDS9960_AILTL: u8          = 0x84;
pub const APDS9960_AILTH: u8          = 0x85;
pub const APDS9960_AIHTL: u8          = 0x86;
pub const APDS9960_AIHTH: u8          = 0x87;
pub const APDS9960_PILT: u8           = 0x89;
pub const APDS9960_PIHT: u8           = 0x8B;
pub const APDS9960_PERS: u8           = 0x8C;
pub const APDS9960_CONFIG1: u8        = 0x8D;
pub const APDS9960_PPULSE: u8         = 0x8E;
pub const APDS9960_CONTROL: u8        = 0x8F;
pub const APDS9960_CONFIG2: u8        = 0x90;
pub const APDS9960_ID: u8             = 0x92;
pub const APDS9960_STATUS: u8         = 0x93;
pub const APDS9960_CDATAL: u8         = 0x94;
pub const APDS9960_CDATAH: u8         = 0x95;
pub const APDS9960_RDATAL: u8         = 0x96;
pub const APDS9960_RDATAH: u8         = 0x97;
pub const APDS9960_GDATAL: u8         = 0x98;
pub const APDS9960_GDATAH: u8         = 0x99;
pub const APDS9960_BDATAL: u8         = 0x9A;
pub const APDS9960_BDATAH: u8         = 0x9B;
pub const APDS9960_PDATA: u8          = 0x9C;
pub const APDS9960_POFFSET_UR: u8     = 0x9D;
pub const APDS9960_POFFSET_DL: u8     = 0x9E;
pub const APDS9960_CONFIG3: u8        = 0x9F;
pub const APDS9960_GPENTH: u8         = 0xA0;
pub const APDS9960_GEXTH: u8          = 0xA1;
pub const APDS9960_GCONF1: u8         = 0xA2;
pub const APDS9960_GCONF2: u8         = 0xA3;
pub const APDS9960_GOFFSET_U: u8      = 0xA4;
pub const APDS9960_GOFFSET_D: u8      = 0xA5;
pub const APDS9960_GOFFSET_L: u8      = 0xA7;
pub const APDS9960_GOFFSET_R: u8      = 0xA9;
pub const APDS9960_GPULSE: u8         = 0xA6;
pub const APDS9960_GCONF3: u8         = 0xAA;
pub const APDS9960_GCONF4: u8         = 0xAB;
pub const APDS9960_GFLVL: u8          = 0xAE;
pub const APDS9960_GSTATUS: u8        = 0xAF;
pub const APDS9960_IFORCE: u8         = 0xE4;
pub const APDS9960_PICLEAR: u8        = 0xE5;
pub const APDS9960_CICLEAR: u8        = 0xE6;
pub const APDS9960_AICLEAR: u8        = 0xE7;
pub const APDS9960_GFIFO_U: u8        = 0xFC;
pub const APDS9960_GFIFO_D: u8        = 0xFD;
pub const APDS9960_GFIFO_L: u8        = 0xFE;
pub const APDS9960_GFIFO_R: u8        = 0xFF;

/* Enable register masks */
pub const ENABLE_MSK: u8              = 0x01;
pub const PON_EN_MSK: u8              = 0x01;
pub const AEN_EN_MSK: u8              = 0x02;
pub const PEN_EN_MSK: u8              = 0x04;
pub const WEN_EN_MSK: u8              = 0x08;
pub const AIEN_EN_MSK: u8             = 0x10;
pub const PIEN_EN_MSK: u8             = 0x20;
pub const GEN_EN_MSK: u8              = 0x40;
pub const PON_EN_POS: u8              = 0x00;
pub const AEN_EN_POS: u8              = 0x01;
pub const PEN_EN_POS: u8              = 0x02;
pub const WEN_EN_POS: u8              = 0x03;
pub const AIEN_EN_POS: u8             = 0x04;
pub const PIEN_EN_POS: u8             = 0x05;
pub const GEN_EN_POS: u8              = 0x06;

/* Gain control masks */
pub const APSD9960_GAIN_MSK: u8       = 0x03;
pub const ALS_GAIN_MSK: u8            = 0x03;
pub const PRX_GAIN_MSK: u8            = 0x0C;
pub const LED_DRIVE_MSK: u8           = 0xC0;
pub const ALS_GAIN_POS: u8            = 0x00;
pub const PRX_GAIN_POS: u8            = 0x02;
pub const LED_DRIVE_POS: u8           = 0x06;

/* Gesture config2 masks */
pub const GES_WTIME_MSK: u8           = 0x07;
pub const GES_LDRIVE_MSK: u8          = 0x18;
pub const GES_GAIN_MSK: u8            = 0x60;
pub const GES_WTIME_POS: u8           = 0x00;
pub const GES_LDRIVE_POS: u8          = 0x03;
pub const GES_GAIN_POS: u8            = 0x05;

/* Gesture config4 masks */
pub const GES_GCONFIG4_MSK: u8        = 0x01;
pub const GES_GMODE_MSK: u8           = 0x01;
pub const GES_GIEN_MSK: u8            = 0x02;
pub const GES_GMODE_POS: u8           = 0x00;
pub const GES_GIEN_POS: u8            = 0x01;

/* Config2 masks */
pub const CFG2_LED_BOOST_MSK: u8      = 0x30;
pub const CFG2_LED_BOOST_POS: u8      = 0x04;

/* Gesture status masks */
pub const GES_STATUS_MSK: u8          = 0x01;
pub const GSTS_GVALID_MSK: u8         = 0x01;
pub const GSTS_GFOV_MSK: u8           = 0x02;
pub const GSTS_GVALID_POS: u8         = 0x00;
pub const GSTS_GFOV_POS: u8           = 0x01;

/* LED Drive values */
pub const LED_DRIVE_100MA: u8          = 0;
pub const LED_DRIVE_50MA: u8           = 1;
pub const LED_DRIVE_25MA: u8           = 2;
pub const LED_DRIVE_12_5MA: u8         = 3;

/* Proximity Gain (PGAIN) values */
pub const PGAIN_1X: u8                = 0;
pub const PGAIN_2X: u8                = 1;
pub const PGAIN_4X: u8                = 2;
pub const PGAIN_8X: u8                = 3;

/* ALS Gain (AGAIN) values */
pub const AGAIN_1X: u8                = 0;
pub const AGAIN_4X: u8                = 1;
pub const AGAIN_16X: u8               = 2;
pub const AGAIN_64X: u8               = 3;

/* Gesture Gain (GGAIN) values */
pub const GGAIN_1X: u8                = 0;
pub const GGAIN_2X: u8                = 1;
pub const GGAIN_4X: u8                = 2;
pub const GGAIN_8X: u8                = 3;

/* LED Boost values */
pub const LED_BOOST_100: u8           = 0;
pub const LED_BOOST_150: u8           = 1;
pub const LED_BOOST_200: u8           = 2;
pub const LED_BOOST_300: u8           = 3;

/* Gesture wait time values */
pub const GWTIME_0MS: u8              = 0;
pub const GWTIME_2_8MS: u8            = 1;
pub const GWTIME_5_6MS: u8            = 2;
pub const GWTIME_8_4MS: u8            = 3;
pub const GWTIME_14_0MS: u8           = 4;
pub const GWTIME_22_4MS: u8           = 5;
pub const GWTIME_30_8MS: u8           = 6;
pub const GWTIME_39_2MS: u8           = 7;

/* Default values */
pub const DEFAULT_ATIME: u8           = 219;     // 103ms
pub const DEFAULT_WTIME: u8           = 246;     // 27ms
pub const DEFAULT_PROX_PPULSE: u8     = 0x87;    // 16us, 8 pulses
pub const DEFAULT_GESTURE_PPULSE: u8  = 0x89;    // 16us, 10 pulses
pub const DEFAULT_POFFSET_UR: u8      = 0;       // 0 offset
pub const DEFAULT_POFFSET_DL: u8      = 0;       // 0 offset
pub const DEFAULT_CONFIG1: u8         = 0x60;    // No 12x wait (WTIME) factor
pub const DEFAULT_LDRIVE: u8          = LED_DRIVE_100MA;
pub const DEFAULT_PGAIN: u8           = PGAIN_4X;
pub const DEFAULT_AGAIN: u8           = AGAIN_4X;
pub const DEFAULT_PILT: u8            = 0;       // Low proximity threshold
pub const DEFAULT_PIHT: u8            = 50;      // High proximity threshold
pub const DEFAULT_AILT: u16           = 0xFFFF;  // Force interrupt for calibration
pub const DEFAULT_AIHT: u16           = 0;
pub const DEFAULT_PERS: u8            = 0x11;    // 2 consecutive prox or ALS for int.
pub const DEFAULT_CONFIG2: u8         = 0x01;    // No saturation interrupts or LED boost
pub const DEFAULT_CONFIG3: u8         = 0;       // Enable all photodiodes, no SAI
pub const DEFAULT_GPENTH: u8          = 40;      // Threshold for entering gesture mode
pub const DEFAULT_GEXTH: u8           = 30;      // Threshold for exiting gesture mode
pub const DEFAULT_GCONF1: u8          = 0x40;    // 4 gesture events for int., 1 for exit
pub const DEFAULT_GGAIN: u8           = GGAIN_2X;
pub const DEFAULT_GLDRIVE: u8         = LED_DRIVE_100MA;
pub const DEFAULT_GWTIME: u8          = GWTIME_2_8MS;
pub const DEFAULT_GOFFSET: u8         = 0;       // No offset scaling for gesture mode
pub const DEFAULT_GPULSE: u8          = 0xC9;    // 32us, 10 pulses
pub const DEFAULT_GCONF3: u8          = 0;       // All photodiodes active during gesture
pub const DEFAULT_GIEN: u8            = 0;       // Disable gesture interrupts

// ============================================================================
// Enums
// ============================================================================

/*!
 *
 */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum State {
    DISABLE = 0,             /**< Disable a feature */
    ENABLE  = 1,             /**< Enable a feature */
}

/*!
 *
 */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Apds9960Mode {
    POWER             = 0,   /**< Power ON/OFF */
    AMBIENT_LIGHT     = 1,   /**< ALS Enable/Disable */
    PROXIMITY         = 2,   /**< Proximity Detect Enable/Disable */
    WAIT              = 3,   /**< Wait Enable/Disable */
    AMBIENT_LIGHT_INT = 4,   /**< ALS Interrupt Enable/Disable */
    PROXIMITY_INT     = 5,   /**< Proximity Interrupt Enable/Disable */
    GESTURE           = 6,   /**< Gesture Enable/Disable */
}

/* Direction definitions */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum Direction {
    DIR_NONE  = 0,
    DIR_LEFT  = 1,
    DIR_RIGHT = 2,
    DIR_UP    = 3,
    DIR_DOWN  = 4,
    DIR_NEAR  = 5,
    DIR_FAR   = 6,
    DIR_ALL   = 7,
    TIMEOUT   = 8,
}

/* State definitions */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum GestureState {
    NA_STATE   = 0,
    NEAR_STATE = 1,
    FAR_STATE  = 2,
    ALL_STATE  = 3,
}

// ============================================================================
// Structs
// ============================================================================

/* Container for gesture data */
#[derive(Clone, Debug)]
pub struct GestureDataType {
    pub u_data: [u8; 32],
    pub d_data: [u8; 32],
    pub l_data: [u8; 32],
    pub r_data: [u8; 32],
    pub index: u8,
    pub total_gestures: u8,
    pub in_threshold: u8,
    pub out_threshold: u8,
}

impl Default for GestureDataType {
    fn default() -> Self {
        GestureDataType {
            u_data: [0u8; 32],
            d_data: [0u8; 32],
            l_data: [0u8; 32],
            r_data: [0u8; 32],
            index: 0,
            total_gestures: 0,
            in_threshold: 0,
            out_threshold: 0,
        }
    }
}

/// RGB color result returned by get_rgb_proportion
#[derive(Clone, Copy, Debug, Default)]
pub struct RgbColor {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

// ============================================================================
// LightProximityAndGesture
// ============================================================================

pub struct LightProximityAndGesture<I2C: I2CBus, D: DelayMs> {
    _i2c_slave_address: u8,
    _is_connected: bool,
    _gesture_data: GestureDataType,
    _gesture_ud_delta: i32,
    _gesture_lr_delta: i32,
    _gesture_ud_count: i32,
    _gesture_lr_count: i32,
    _gesture_near_count: i32,
    _gesture_far_count: i32,
    _gesture_state: i32,
    _gesture_motion: i32,
    i2c: I2C,
    delay: D,
}

impl<I2C: I2CBus, D: DelayMs> LightProximityAndGesture<I2C, D> {
    /**
     *
     */
    pub fn new(i2c: I2C, delay: D) -> Self {
        LightProximityAndGesture {
            _i2c_slave_address: APDS9960_I2C_ADDRESS,
            _is_connected: false,
            _gesture_data: GestureDataType::default(),
            _gesture_ud_delta: 0,
            _gesture_lr_delta: 0,
            _gesture_ud_count: 0,
            _gesture_lr_count: 0,
            _gesture_near_count: 0,
            _gesture_far_count: 0,
            _gesture_state: 0,
            _gesture_motion: Direction::DIR_NONE as i32,
            i2c,
            delay,
        }
    }

    /**
     *
     */
    pub fn ping(&mut self) -> bool {
        let get_connect_sts = self.write_address();
        if !self._is_connected && get_connect_sts {
            self.begin();

            self.enable_ambient_light_sensor(State::DISABLE);
            self.enable_proximity_sensor(State::DISABLE);
            self.set_proximity_gain(PGAIN_2X);
            // self.enable_gesture_sensor(State::DISABLE);
            self.delay.delay_ms(500);
        }
        self._is_connected = get_connect_sts;
        get_connect_sts
    }

    /**
     *
     */
    pub fn begin(&mut self) -> bool {
        let mut device_id: u8 = 0;
        /* Read ID register and check against known values for APDS9960 */
        if !self.read_byte_reg(APDS9960_ID, &mut device_id) {
            return false;
        }
        if (device_id != APDS9960_ID_1) && (device_id != APDS9960_ID_2) && (device_id != APDS9960_ID_3) {
            return false;
        }
        /* Set ENABLE register to 0 (disable all features) */
        if !self.write_byte_val(APDS9960_ENABLE, 0) {
            return false;
        }
        /* Set default values for ambient light and proximity registers */
        if !self.write_byte_val(APDS9960_ATIME, DEFAULT_ATIME) {
            return false;
        }
        if !self.write_byte_val(APDS9960_WTIME, DEFAULT_WTIME) {
            return false;
        }
        if !self.write_byte_val(APDS9960_PPULSE, DEFAULT_PROX_PPULSE) {
            return false;
        }
        if !self.write_byte_val(APDS9960_POFFSET_UR, DEFAULT_POFFSET_UR) {
            return false;
        }
        if !self.write_byte_val(APDS9960_POFFSET_DL, DEFAULT_POFFSET_DL) {
            return false;
        }
        if !self.write_byte_val(APDS9960_CONFIG1, DEFAULT_CONFIG1) {
            return false;
        }
        if !self.set_led_drive(DEFAULT_LDRIVE) {
            return false;
        }
        if !self.set_proximity_gain(DEFAULT_PGAIN) {
            return false;
        }
        if !self.set_ambient_light_gain(DEFAULT_AGAIN) {
            return false;
        }
        if !self.write_byte_val(APDS9960_PILT, DEFAULT_PILT) {
            return false;
        }
        if !self.write_byte_val(APDS9960_PIHT, DEFAULT_PIHT) {
            return false;
        }
        if !self.set_light_int_low_threshold(DEFAULT_AILT) {
            return false;
        }
        if !self.set_light_int_high_threshold(DEFAULT_AIHT) {
            return false;
        }
        if !self.write_byte_val(APDS9960_PERS, DEFAULT_PERS) {
            return false;
        }
        if !self.write_byte_val(APDS9960_CONFIG2, DEFAULT_CONFIG2) {
            return false;
        }
        if !self.write_byte_val(APDS9960_CONFIG3, DEFAULT_CONFIG3) {
            return false;
        }
        /* Set default values for gesture sense registers */
        if !self.write_byte_val(APDS9960_GPENTH, DEFAULT_GPENTH) {
            return false;
        }
        if !self.write_byte_val(APDS9960_GEXTH, DEFAULT_GEXTH) {
            return false;
        }
        if !self.write_byte_val(APDS9960_GCONF1, DEFAULT_GCONF1) {
            return false;
        }
        if !self.set_gesture_gain(DEFAULT_GGAIN) {
            return false;
        }
        if !self.set_gesture_led_drive(DEFAULT_GLDRIVE) {
            return false;
        }
        if !self.set_gesture_wait_time(DEFAULT_GWTIME) {
            return false;
        }
        if !self.write_byte_val(APDS9960_GOFFSET_U, DEFAULT_GOFFSET) {
            return false;
        }
        if !self.write_byte_val(APDS9960_GOFFSET_D, DEFAULT_GOFFSET) {
            return false;
        }
        if !self.write_byte_val(APDS9960_GOFFSET_L, DEFAULT_GOFFSET) {
            return false;
        }
        if !self.write_byte_val(APDS9960_GOFFSET_R, DEFAULT_GOFFSET) {
            return false;
        }
        if !self.write_byte_val(APDS9960_GPULSE, DEFAULT_GPULSE) {
            return false;
        }
        if !self.write_byte_val(APDS9960_GCONF3, DEFAULT_GCONF3) {
            return false;
        }
        if !self.set_gesture_int(State::DISABLE) {
            return false;
        }
        self._is_connected = true;
        true
    }

    /**
     *
     */
    pub fn enable_power(&mut self) -> bool {
        self.set_mode(Apds9960Mode::POWER, State::ENABLE)
    }

    /**
     *
     */
    pub fn disable_power(&mut self) -> bool {
        self.set_mode(Apds9960Mode::POWER, State::DISABLE)
    }

    /**
     *
     */
    pub fn get_device_id(&mut self) -> u8 {
        let mut id: u8 = 0;
        if !self.read_byte_reg(APDS9960_ID, &mut id) {
            return ERROR;
        }
        id
    }

    /**
     *
     */
    pub fn get_mode(&mut self) -> u8 {
        let mut enable_val: u8 = 0;
        if !self.read_byte_reg(APDS9960_ENABLE, &mut enable_val) {
            return ERROR;
        }
        enable_val
    }

    /**
     *
     */
    pub fn set_mode(&mut self, mode: Apds9960Mode, state: State) -> bool {
        let mut enable_val = self.get_mode();
        if enable_val == ERROR {
            return false;
        }
        enable_val &= !(1u8 << (mode as u8));
        enable_val |= (state as u8) << (mode as u8);
        if !self.write_byte_val(APDS9960_ENABLE, enable_val) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn enable_ambient_light_sensor(&mut self, interrupt: State) -> bool {
        /* Set default gain, interrupts, enable power, and enable sensor */
        if !self.set_ambient_light_gain(DEFAULT_AGAIN) {
            return false;
        }
        if !self.set_ambient_light_int(interrupt) {
            return false;
        }
        if !self.enable_power() {
            return false;
        }
        if !self.set_mode(Apds9960Mode::AMBIENT_LIGHT, State::ENABLE) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn disable_ambient_light_sensor(&mut self) -> bool {
        if !self.set_ambient_light_int(State::DISABLE) {
            return false;
        }
        if !self.set_mode(Apds9960Mode::AMBIENT_LIGHT, State::DISABLE) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn enable_proximity_sensor(&mut self, interrupt: State) -> bool {
        /* Set default gain, LED, interrupts, enable power, and enable sensor */
        if !self.set_proximity_gain(DEFAULT_PGAIN) {
            return false;
        }
        if !self.set_led_drive(DEFAULT_LDRIVE) {
            return false;
        }
        if !self.set_proximity_int(interrupt) {
            return false;
        }
        if !self.enable_power() {
            return false;
        }
        if !self.set_mode(Apds9960Mode::PROXIMITY, State::ENABLE) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn disable_proximity_sensor(&mut self) -> bool {
        if !self.set_proximity_int(State::DISABLE) {
            return false;
        }
        if !self.set_mode(Apds9960Mode::PROXIMITY, State::DISABLE) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn enable_gesture_sensor(&mut self, interrupt: State) -> bool {
        /*
          Enable gesture mode
          Set ENABLE to 0 (power off)
          Set WTIME to 0xFF
          Set AUX to LED_BOOST_300
          Enable PON, WEN, PEN, GEN in ENABLE
        */
        self.reset_gesture_parameters();
        if !self.write_byte_val(APDS9960_WTIME, 0xFF) {
            return false;
        }
        if !self.write_byte_val(APDS9960_PPULSE, DEFAULT_GESTURE_PPULSE) {
            return false;
        }
        if !self.set_led_boost(LED_BOOST_300) {
            return false;
        }
        if !self.set_gesture_int(interrupt) {
            return false;
        }
        if !self.set_gesture_mode(1) {
            return false;
        }
        if !self.enable_power() {
            return false;
        }
        if !self.set_mode(Apds9960Mode::WAIT, State::ENABLE) {
            return false;
        }
        if !self.set_mode(Apds9960Mode::PROXIMITY, State::ENABLE) {
            return false;
        }
        if !self.set_mode(Apds9960Mode::GESTURE, State::ENABLE) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn disable_gesture_sensor(&mut self) -> bool {
        self.reset_gesture_parameters();
        if !self.set_gesture_int(State::DISABLE) {
            return false;
        }
        if !self.set_gesture_mode(0) {
            return false;
        }
        if !self.set_mode(Apds9960Mode::GESTURE, State::DISABLE) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_ambient_light_int_state(&mut self) -> bool {
        let mut enable_val: u8 = 0;
        if !self.read_byte_reg(APDS9960_ENABLE, &mut enable_val) {
            return false;
        }
        ((enable_val & AIEN_EN_MSK) >> AIEN_EN_POS) != 0
    }

    /**
     *
     */
    pub fn set_ambient_light_int(&mut self, int_state: State) -> bool {
        let mut enable_val: u8 = 0;
        /* get the existing gain value from sensor */
        if !self.read_byte_reg(APDS9960_ENABLE, &mut enable_val) {
            return false;
        }
        /* update the ALS sensor gain value */
        enable_val &= !AIEN_EN_MSK;
        enable_val |= ((int_state as u8) & ENABLE_MSK) << AIEN_EN_POS;
        /* write new gain value to register */
        if !self.write_byte_val(APDS9960_ENABLE, enable_val) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_proximity_int_state(&mut self) -> bool {
        let mut enable_val: u8 = 0;
        if !self.read_byte_reg(APDS9960_ENABLE, &mut enable_val) {
            return false;
        }
        ((enable_val & PIEN_EN_MSK) >> PIEN_EN_POS) != 0
    }

    /**
     *
     */
    pub fn set_proximity_int(&mut self, int_state: State) -> bool {
        let mut enable_val: u8 = 0;
        /* get the existing gain value from sensor */
        if !self.read_byte_reg(APDS9960_ENABLE, &mut enable_val) {
            return false;
        }
        /* update the ALS sensor gain value */
        enable_val &= !PIEN_EN_MSK;
        enable_val |= ((int_state as u8) & ENABLE_MSK) << PIEN_EN_POS;
        /* write new gain value to register */
        if !self.write_byte_val(APDS9960_ENABLE, enable_val) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_gesture_int_state(&mut self) -> bool {
        let mut gconfig4: u8 = 0;
        if !self.read_byte_reg(APDS9960_GCONF4, &mut gconfig4) {
            return false;
        }
        ((gconfig4 & GES_GIEN_MSK) >> GES_GIEN_POS) != 0
    }

    /**
     *
     */
    pub fn set_gesture_int(&mut self, int_state: State) -> bool {
        let mut gconfig4: u8 = 0;
        /* get the existing gain value from sensor */
        if !self.read_byte_reg(APDS9960_GCONF4, &mut gconfig4) {
            return false;
        }
        /* update the ALS sensor gain value */
        gconfig4 &= !GES_GIEN_MSK;
        gconfig4 |= ((int_state as u8) & GES_GCONFIG4_MSK) << GES_GIEN_POS;
        /* write new gain value to register */
        if !self.write_byte_val(APDS9960_GCONF4, gconfig4) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_ambient_light_gain(&mut self) -> u8 {
        let mut gain: u8 = 0;
        if !self.read_byte_reg(APDS9960_CONTROL, &mut gain) {
            return ERROR;
        }
        (gain & ALS_GAIN_MSK) >> ALS_GAIN_POS
    }

    /**
     *
     */
    pub fn set_ambient_light_gain(&mut self, als_gain: u8) -> bool {
        let mut gain: u8 = 0;
        /* get the existing gain value from sensor */
        if !self.read_byte_reg(APDS9960_CONTROL, &mut gain) {
            return false;
        }
        /* update the ALS sensor gain value */
        gain &= !ALS_GAIN_MSK;
        gain |= (als_gain & APSD9960_GAIN_MSK) << ALS_GAIN_POS;
        /* write new gain value to register */
        if !self.write_byte_val(APDS9960_CONTROL, gain) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_proximity_gain(&mut self) -> u8 {
        let mut gain: u8 = 0;
        if !self.read_byte_reg(APDS9960_CONTROL, &mut gain) {
            return ERROR;
        }
        (gain & PRX_GAIN_MSK) >> PRX_GAIN_POS
    }

    /**
     *
     */
    pub fn set_proximity_gain(&mut self, prx_gain: u8) -> bool {
        let mut gain: u8 = 0;
        /* get the existing gain value from sensor */
        if !self.read_byte_reg(APDS9960_CONTROL, &mut gain) {
            return false;
        }
        /* update the ALS sensor gain value */
        gain &= !PRX_GAIN_MSK;
        gain |= (prx_gain & APSD9960_GAIN_MSK) << PRX_GAIN_POS;
        /* write new gain value to register */
        if !self.write_byte_val(APDS9960_CONTROL, gain) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_led_drive(&mut self) -> u8 {
        let mut gain: u8 = 0;
        if !self.read_byte_reg(APDS9960_CONTROL, &mut gain) {
            return ERROR;
        }
        (gain & LED_DRIVE_MSK) >> LED_DRIVE_POS
    }

    /**
     *
     */
    pub fn set_led_drive(&mut self, drive: u8) -> bool {
        let mut gain: u8 = 0;
        /* get the existing gain value from sensor */
        if !self.read_byte_reg(APDS9960_CONTROL, &mut gain) {
            return false;
        }
        /* update the ALS sensor gain value */
        gain &= !LED_DRIVE_MSK;
        gain |= (drive & APSD9960_GAIN_MSK) << LED_DRIVE_POS;
        /* write new gain value to register */
        if !self.write_byte_val(APDS9960_CONTROL, gain) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_gesture_gain(&mut self) -> u8 {
        let mut gain: u8 = 0;
        if !self.read_byte_reg(APDS9960_GCONF2, &mut gain) {
            return ERROR;
        }
        (gain & GES_GAIN_MSK) >> GES_GAIN_POS
    }

    /**
     *
     */
    pub fn set_gesture_gain(&mut self, ges_gain: u8) -> bool {
        let mut gain: u8 = 0;
        /* get the existing gain value from sensor */
        if !self.read_byte_reg(APDS9960_GCONF2, &mut gain) {
            return false;
        }
        /* update the ALS sensor gain value */
        gain &= !GES_GAIN_MSK;
        gain |= (ges_gain & APSD9960_GAIN_MSK) << GES_GAIN_POS;
        /* write new gain value to register */
        if !self.write_byte_val(APDS9960_GCONF2, gain) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_gesture_led_drive(&mut self) -> u8 {
        let mut config: u8 = 0;
        if !self.read_byte_reg(APDS9960_GCONF2, &mut config) {
            return ERROR;
        }
        (config & GES_LDRIVE_MSK) >> GES_LDRIVE_POS
    }

    /**
     *
     */
    pub fn set_gesture_led_drive(&mut self, drive: u8) -> bool {
        let mut config: u8 = 0;
        /* get the existing gain value from sensor */
        if !self.read_byte_reg(APDS9960_GCONF2, &mut config) {
            return false;
        }
        /* update the ALS sensor gain value */
        config &= !GES_LDRIVE_MSK;
        config |= (drive & APSD9960_GAIN_MSK) << GES_LDRIVE_POS;
        /* write new gain value to register */
        if !self.write_byte_val(APDS9960_GCONF2, config) {
            return false;
        }
        true
    }

    /**
     *
     */
    fn get_gesture_wait_time(&mut self) -> u8 {
        let mut config: u8 = 0;
        if !self.read_byte_reg(APDS9960_GCONF2, &mut config) {
            return ERROR;
        }
        (config & GES_WTIME_MSK) >> GES_WTIME_POS
    }

    /**
     *
     */
    fn set_gesture_wait_time(&mut self, time: u8) -> bool {
        let mut config: u8 = 0;
        /* get the existing gain value from sensor */
        if !self.read_byte_reg(APDS9960_GCONF2, &mut config) {
            return false;
        }
        /* update the ALS sensor gain value */
        config &= !GES_WTIME_MSK;
        config |= (time & GES_WTIME_MSK) << GES_WTIME_POS;
        /* write new gain value to register */
        if !self.write_byte_val(APDS9960_GCONF2, config) {
            return false;
        }
        true
    }

    /**
     *
     */
    fn get_gesture_mode(&mut self) -> u8 {
        let mut gconfig4: u8 = 0;
        if !self.read_byte_reg(APDS9960_GCONF4, &mut gconfig4) {
            return ERROR;
        }
        (gconfig4 & GES_GMODE_MSK) >> GES_GMODE_POS
    }

    /**
     *
     */
    fn set_gesture_mode(&mut self, mode: u8) -> bool {
        let mut gconfig4: u8 = 0;
        /* get the existing gain value from sensor */
        if !self.read_byte_reg(APDS9960_GCONF4, &mut gconfig4) {
            return false;
        }
        /* update the ALS sensor gain value */
        gconfig4 &= !GES_GMODE_MSK;
        gconfig4 |= (mode & GES_GMODE_MSK) << GES_GMODE_POS;
        /* write new gain value to register */
        if !self.write_byte_val(APDS9960_GCONF4, gconfig4) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_light_int_low_threshold(&mut self, threshold: &mut u16) -> bool {
        let mut data = [0u8; 2];
        if !self.read_byte_reg(APDS9960_AILTL, &mut data[0]) {
            return false;
        }
        if !self.read_byte_reg(APDS9960_AILTH, &mut data[1]) {
            return false;
        }
        /* update the threshold */
        *threshold = ((data[1] as u16) << 8) | data[0] as u16;
        true
    }

    /**
     *
     */
    pub fn set_light_int_low_threshold(&mut self, threshold: u16) -> bool {
        let data = (threshold & 0xFF) as u8;
        if !self.write_byte_val(APDS9960_AILTL, data) {
            return false;
        }
        let data = ((threshold >> 8) & 0xFF) as u8;
        if !self.write_byte_val(APDS9960_AILTH, data) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_light_int_high_threshold(&mut self, threshold: &mut u16) -> bool {
        let mut data = [0u8; 2];
        if !self.read_byte_reg(APDS9960_AIHTL, &mut data[0]) {
            return false;
        }
        if !self.read_byte_reg(APDS9960_AIHTH, &mut data[1]) {
            return false;
        }
        /* update the threshold */
        *threshold = ((data[1] as u16) << 8) | data[0] as u16;
        true
    }

    /**
     *
     */
    pub fn set_light_int_high_threshold(&mut self, threshold: u16) -> bool {
        let data = (threshold & 0xFF) as u8;
        if !self.write_byte_val(APDS9960_AIHTL, data) {
            return false;
        }
        let data = ((threshold >> 8) & 0xFF) as u8;
        if !self.write_byte_val(APDS9960_AIHTH, data) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn get_proximity_int_low_threshold(&mut self, threshold: &mut u8) -> bool {
        *threshold = 0;
        self.read_byte_reg(APDS9960_PILT, threshold)
    }

    /**
     *
     */
    pub fn set_proximity_int_low_threshold(&mut self, threshold: u8) -> bool {
        self.write_byte_val(APDS9960_PILT, threshold)
    }

    /**
     *
     */
    pub fn get_proximity_int_high_threshold(&mut self, threshold: &mut u8) -> bool {
        *threshold = 0;
        self.read_byte_reg(APDS9960_PIHT, threshold)
    }

    /**
     *
     */
    pub fn set_proximity_int_high_threshold(&mut self, threshold: u8) -> bool {
        self.write_byte_val(APDS9960_PIHT, threshold)
    }

    /**
     *
     */
    fn reset_gesture_parameters(&mut self) {
        self._gesture_data.index = 0;
        self._gesture_data.total_gestures = 0;

        self._gesture_ud_delta = 0;
        self._gesture_lr_delta = 0;

        self._gesture_ud_count = 0;
        self._gesture_lr_count = 0;

        self._gesture_near_count = 0;
        self._gesture_far_count = 0;

        self._gesture_state = 0;
        self._gesture_motion = Direction::DIR_NONE as i32;
    }

    /**
     *
     */
    fn process_gesture_data(&mut self) -> bool {
        let mut u_first: u8 = 0;
        let mut d_first: u8 = 0;
        let mut l_first: u8 = 0;
        let mut r_first: u8 = 0;
        let mut u_last: u8 = 0;
        let mut d_last: u8 = 0;
        let mut l_last: u8 = 0;
        let mut r_last: u8 = 0;
        let ud_ratio_first: i32;
        let lr_ratio_first: i32;
        let ud_ratio_last: i32;
        let lr_ratio_last: i32;
        let ud_delta: i32;
        let lr_delta: i32;

        /* If we have less than 4 total gestures, that's not enough */
        if self._gesture_data.total_gestures <= 4 {
            return false;
        }

        /* Check to make sure our data isn't out of bounds */
        if (self._gesture_data.total_gestures <= 32) &&
           (self._gesture_data.total_gestures > 0)
        {
            /* Find the first value in U/D/L/R above the threshold */
            for i in 0..self._gesture_data.total_gestures as usize {
                if (self._gesture_data.u_data[i] > GESTURE_THRESHOLD_OUT) &&
                   (self._gesture_data.d_data[i] > GESTURE_THRESHOLD_OUT) &&
                   (self._gesture_data.l_data[i] > GESTURE_THRESHOLD_OUT) &&
                   (self._gesture_data.r_data[i] > GESTURE_THRESHOLD_OUT)
                {
                    u_first = self._gesture_data.u_data[i];
                    d_first = self._gesture_data.d_data[i];
                    l_first = self._gesture_data.l_data[i];
                    r_first = self._gesture_data.r_data[i];
                    break;
                }
            }
            /* If one of the _first values is 0, then there is no good data */
            if (u_first == 0) || (d_first == 0) ||
               (l_first == 0) || (r_first == 0)
            {
                return false;
            }
            /* Find the last value in U/D/L/R above the threshold */
            for i in (0..self._gesture_data.total_gestures as usize).rev() {
                if (self._gesture_data.u_data[i] > GESTURE_THRESHOLD_OUT) &&
                   (self._gesture_data.d_data[i] > GESTURE_THRESHOLD_OUT) &&
                   (self._gesture_data.l_data[i] > GESTURE_THRESHOLD_OUT) &&
                   (self._gesture_data.r_data[i] > GESTURE_THRESHOLD_OUT)
                {
                    u_last = self._gesture_data.u_data[i];
                    d_last = self._gesture_data.d_data[i];
                    l_last = self._gesture_data.l_data[i];
                    r_last = self._gesture_data.r_data[i];
                    break;
                }
            }
        }

        /* Calculate the first vs. last ratio of up/down and left/right */
        ud_ratio_first = ((u_first as i32 - d_first as i32) * 100) / (u_first as i32 + d_first as i32);
        lr_ratio_first = ((l_first as i32 - r_first as i32) * 100) / (l_first as i32 + r_first as i32);
        ud_ratio_last = ((u_last as i32 - d_last as i32) * 100) / (u_last as i32 + d_last as i32);
        lr_ratio_last = ((l_last as i32 - r_last as i32) * 100) / (l_last as i32 + r_last as i32);

        /* Determine the difference between the first and last ratios */
        ud_delta = ud_ratio_last - ud_ratio_first;
        lr_delta = lr_ratio_last - lr_ratio_first;

        /* Accumulate the UD and LR delta values */
        self._gesture_ud_delta += ud_delta;
        self._gesture_lr_delta += lr_delta;

        /* Determine U/D gesture */
        if self._gesture_ud_delta >= GESTURE_SENSITIVITY_1 {
            self._gesture_ud_count = 1;
        } else if self._gesture_ud_delta <= -GESTURE_SENSITIVITY_1 {
            self._gesture_ud_count = -1;
        } else {
            self._gesture_ud_count = 0;
        }

        /* Determine L/R gesture */
        if self._gesture_lr_delta >= GESTURE_SENSITIVITY_1 {
            self._gesture_lr_count = 1;
        } else if self._gesture_lr_delta <= -GESTURE_SENSITIVITY_1 {
            self._gesture_lr_count = -1;
        } else {
            self._gesture_lr_count = 0;
        }

        /* Determine Near/Far gesture */
        if (self._gesture_ud_count == 0) && (self._gesture_lr_count == 0) {
            if (ud_delta.abs() < GESTURE_SENSITIVITY_2) &&
               (lr_delta.abs() < GESTURE_SENSITIVITY_2)
            {
                if (ud_delta == 0) && (lr_delta == 0) {
                    self._gesture_near_count += 1;
                } else if (ud_delta != 0) || (lr_delta != 0) {
                    self._gesture_far_count += 1;
                }

                if (self._gesture_near_count >= 10) && (self._gesture_far_count >= 2) {
                    if (ud_delta == 0) && (lr_delta == 0) {
                        self._gesture_state = GestureState::NEAR_STATE as i32;
                    } else if (ud_delta != 0) && (lr_delta != 0) {
                        self._gesture_state = GestureState::FAR_STATE as i32;
                    }
                    return true;
                }
            }
        } else {
            if (ud_delta.abs() < GESTURE_SENSITIVITY_2) &&
               (lr_delta.abs() < GESTURE_SENSITIVITY_2)
            {
                if (ud_delta == 0) && (lr_delta == 0) {
                    self._gesture_near_count += 1;
                }

                if self._gesture_near_count >= 10 {
                    self._gesture_ud_count = 0;
                    self._gesture_lr_count = 0;
                    self._gesture_ud_delta = 0;
                    self._gesture_lr_delta = 0;
                }
            }
        }
        false
    }

    /**
     *
     */
    fn decode_gesture(&mut self) -> bool {
        /* Return if near or far event is detected */
        if self._gesture_state == GestureState::NEAR_STATE as i32 {
            self._gesture_motion = Direction::DIR_NEAR as i32;
            return true;
        } else if self._gesture_state == GestureState::FAR_STATE as i32 {
            self._gesture_motion = Direction::DIR_FAR as i32;
            return true;
        }

        /* Determine swipe direction */
        if (self._gesture_ud_count == -1) && (self._gesture_lr_count == 0) {
            self._gesture_motion = Direction::DIR_UP as i32;
        } else if (self._gesture_ud_count == 1) && (self._gesture_lr_count == 0) {
            self._gesture_motion = Direction::DIR_DOWN as i32;
        } else if (self._gesture_ud_count == 0) && (self._gesture_lr_count == 1) {
            self._gesture_motion = Direction::DIR_RIGHT as i32;
        } else if (self._gesture_ud_count == 0) && (self._gesture_lr_count == -1) {
            self._gesture_motion = Direction::DIR_LEFT as i32;
        } else if (self._gesture_ud_count == -1) && (self._gesture_lr_count == 1) {
            if self._gesture_ud_delta.abs() > self._gesture_lr_delta.abs() {
                self._gesture_motion = Direction::DIR_UP as i32;
            } else {
                self._gesture_motion = Direction::DIR_RIGHT as i32;
            }
        } else if (self._gesture_ud_count == 1) && (self._gesture_lr_count == -1) {
            if self._gesture_ud_delta.abs() > self._gesture_lr_delta.abs() {
                self._gesture_motion = Direction::DIR_DOWN as i32;
            } else {
                self._gesture_motion = Direction::DIR_LEFT as i32;
            }
        } else if (self._gesture_ud_count == -1) && (self._gesture_lr_count == -1) {
            if self._gesture_ud_delta.abs() > self._gesture_lr_delta.abs() {
                self._gesture_motion = Direction::DIR_UP as i32;
            } else {
                self._gesture_motion = Direction::DIR_LEFT as i32;
            }
        } else if (self._gesture_ud_count == 1) && (self._gesture_lr_count == 1) {
            if self._gesture_ud_delta.abs() > self._gesture_lr_delta.abs() {
                self._gesture_motion = Direction::DIR_DOWN as i32;
            } else {
                self._gesture_motion = Direction::DIR_RIGHT as i32;
            }
        } else {
            return false;
        }
        true
    }

    /**
     *
     */
    fn get_led_boost(&mut self) -> u8 {
        let mut config2: u8 = 0;
        if !self.read_byte_reg(APDS9960_CONFIG2, &mut config2) {
            return ERROR;
        }
        (config2 & CFG2_LED_BOOST_MSK) >> CFG2_LED_BOOST_POS
    }

    /**
     *
     */
    fn set_led_boost(&mut self, boost: u8) -> bool {
        let mut config2: u8 = 0;
        if !self.read_byte_reg(APDS9960_CONFIG2, &mut config2) {
            return false;
        }
        config2 &= !CFG2_LED_BOOST_MSK;
        config2 |= (boost & 0x03) << CFG2_LED_BOOST_POS;
        if !self.write_byte_val(APDS9960_CONFIG2, config2) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn clear_ambient_light_int(&mut self) -> bool {
        if !self.write_byte_cmd(APDS9960_AICLEAR) {
            return false;
        }
        true
    }

    /**
     *
     */
    pub fn clear_proximity_int(&mut self) -> bool {
        if !self.write_byte_cmd(APDS9960_PICLEAR) {
            return false;
        }
        true
    }

    /**
     *
     */
    fn read_ambient_light(&mut self, val: &mut u16) -> bool {
        let mut data = [0u8; 2];
        if !self.read_byte_reg(APDS9960_CDATAL, &mut data[0]) {
            return false;
        }
        if !self.read_byte_reg(APDS9960_CDATAH, &mut data[1]) {
            return false;
        }
        *val = ((data[1] as u16) << 8) | data[0] as u16;
        true
    }

    /**
     *
     */
    fn read_red_light(&mut self, val: &mut u16) -> bool {
        let mut data = [0u8; 2];
        if !self.read_byte_reg(APDS9960_RDATAL, &mut data[0]) {
            return false;
        }
        if !self.read_byte_reg(APDS9960_RDATAH, &mut data[1]) {
            return false;
        }
        *val = ((data[1] as u16) << 8) | data[0] as u16;
        true
    }

    /**
     *
     */
    fn read_green_light(&mut self, val: &mut u16) -> bool {
        let mut data = [0u8; 2];
        if !self.read_byte_reg(APDS9960_GDATAL, &mut data[0]) {
            return false;
        }
        if !self.read_byte_reg(APDS9960_GDATAH, &mut data[1]) {
            return false;
        }
        *val = ((data[1] as u16) << 8) | data[0] as u16;
        true
    }

    /**
     *
     */
    fn read_blue_light(&mut self, val: &mut u16) -> bool {
        let mut data = [0u8; 2];
        if !self.read_byte_reg(APDS9960_BDATAL, &mut data[0]) {
            return false;
        }
        if !self.read_byte_reg(APDS9960_BDATAH, &mut data[1]) {
            return false;
        }
        *val = ((data[1] as u16) << 8) | data[0] as u16;
        true
    }

    /**
     *
     */
    fn read_proximity(&mut self, val: &mut u8) -> bool {
        self.read_byte_reg(APDS9960_PDATA, val)
    }

    /**
     *
     */
    fn is_gesture_available(&mut self) -> bool {
        let mut gsts: u8 = 0;
        if !self.read_byte_reg(APDS9960_GSTATUS, &mut gsts) {
            return false;
        }
        ((gsts & GSTS_GVALID_MSK) >> GSTS_GVALID_POS) != 0
    }

    /**
     *
     */
    fn read_gesture(&mut self) -> i32 {
        let mut fifo_level: u8 = 0;
        let mut fifo_data = [0u8; 128];
        let mut gstatus: u8;
        let mut bytes_read: i8;
        let motion: i32;
        let mut time_cnt: u32 = 0;

        /* Make sure that power and gesture is on and data is valid */
        if !self.is_gesture_available() || (self.get_mode() & (GEN_EN_MSK | PON_EN_MSK)) == 0 {
            return Direction::DIR_NONE as i32;
        }

        /* Keep looping as long as gesture data is valid */
        loop {
            /* Wait some time to collect next batch of FIFO data */
            self.delay.delay_ms(FIFO_PAUSE_TIME);

            /* increment timeout count */
            time_cnt += 1;

            /* Get the contents of the STATUS register. Is data still valid? */
            gstatus = 0;
            if !self.read_byte_reg(APDS9960_GSTATUS, &mut gstatus) {
                return ERROR as i32;
            }

            /* If we have valid data, read in FIFO */
            if ((gstatus & GSTS_GVALID_MSK) == GSTS_GVALID_MSK) && (time_cnt <= 10) {
                /* Read the current FIFO level */
                if !self.read_byte_reg(APDS9960_GFLVL, &mut fifo_level) {
                    return ERROR as i32;
                }

                /* If there's stuff in the FIFO, read it into our data block */
                if fifo_level > 0 {
                    bytes_read = self.read_multi_bytes_reg(
                        APDS9960_GFIFO_U,
                        fifo_level * 4,
                        &mut fifo_data,
                    );
                    if bytes_read == -1 {
                        return ERROR as i32;
                    }

                    /* If at least 1 set of data, sort the data into U/D/L/R */
                    if bytes_read >= 4 {
                        let mut i = 0i8;
                        while i < bytes_read {
                            let idx = self._gesture_data.index as usize;
                            self._gesture_data.u_data[idx] = fifo_data[i as usize];
                            self._gesture_data.d_data[idx] = fifo_data[(i + 1) as usize];
                            self._gesture_data.l_data[idx] = fifo_data[(i + 2) as usize];
                            self._gesture_data.r_data[idx] = fifo_data[(i + 3) as usize];
                            self._gesture_data.index += 1;
                            self._gesture_data.total_gestures += 1;
                            i += 4;
                        }

                        /* Filter and process gesture data. Decode near/far state */
                        if self.process_gesture_data() {
                            if self.decode_gesture() {
                                //***TODO: U-Turn Gestures
                            }
                        }

                        /* Reset data */
                        self._gesture_data.index = 0;
                        self._gesture_data.total_gestures = 0;
                    }
                }
            } else {
                if time_cnt >= 10 {
                    motion = Direction::TIMEOUT as i32;
                } else {
                    /* Determine best guessed gesture and clean up */
                    self.delay.delay_ms(FIFO_PAUSE_TIME);
                    self.decode_gesture();
                    motion = self._gesture_motion;
                    self.reset_gesture_parameters();
                }
                return motion;
            }
        }
    }

    /**
     *
     */
    pub fn get_rgb_proportion(&mut self) -> RgbColor {
        let mut color = RgbColor { red: 0, green: 0, blue: 0 };
        if self.read_red_light(&mut color.red) {
            if self.read_green_light(&mut color.green) {
                self.read_blue_light(&mut color.blue);
            }
        }
        color
    }

    /**
     *
     */
    pub fn get_ambient_light(&mut self) -> u16 {
        let mut ambient_light: u16 = 0;
        if self.read_ambient_light(&mut ambient_light) {
            return ambient_light;
        }
        0
    }

    /**
     *
     */
    pub fn get_red_proportion(&mut self) -> u16 {
        let mut red_light: u16 = 0;
        if !self.read_red_light(&mut red_light) {
            return 0;
        }
        red_light
    }

    /**
     *
     */
    pub fn get_green_proportion(&mut self) -> u16 {
        let mut green_light: u16 = 0;
        if !self.read_green_light(&mut green_light) {
            return 0;
        }
        green_light
    }

    /**
     *
     */
    pub fn get_blue_proportion(&mut self) -> u16 {
        let mut blue_light: u16 = 0;
        if !self.read_blue_light(&mut blue_light) {
            return 0;
        }
        blue_light
    }

    /**
     *
     */
    pub fn get_proximity(&mut self) -> f32 {
        let mut proximity_data: u8 = 0;
        if self.read_proximity(&mut proximity_data) {
            return proximity_data as f32;
        }
        0.0
    }

    /**
     *
     */
    pub fn get_gesture(&mut self) -> &'static str {
        let mut gesture: &str = "NONE";
        if self.is_gesture_available() {
            match self.read_gesture() {
                x if x == Direction::DIR_UP as i32    => { gesture = "UP"; }
                x if x == Direction::DIR_DOWN as i32  => { gesture = "DOWN"; }
                x if x == Direction::DIR_LEFT as i32  => { gesture = "LEFT"; }
                x if x == Direction::DIR_RIGHT as i32 => { gesture = "RIGHT"; }
                x if x == Direction::DIR_NEAR as i32  => { gesture = "NEAR"; }
                x if x == Direction::DIR_FAR as i32   => { gesture = "FAR"; }
                x if x == Direction::TIMEOUT as i32   => { gesture = "TIMEOUT"; }
                _ => { gesture = "NONE"; }
            }
        }
        gesture
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
    fn read_multi_bytes_reg(&mut self, reg: u8, length: u8, result: &mut [u8]) -> i8 {
        let mut n_data: i8 = 0;
        self.i2c.begin_transmission(self._i2c_slave_address);
        self.i2c.write_byte(reg);
        if self.i2c.end_transmission() != 0 {
            return -1;
        }
        self.i2c.request_from(self._i2c_slave_address, length);
        while self.i2c.available() > 0 {
            if n_data >= length as i8 {
                return -1;
            }
            result[n_data as usize] = self.i2c.read_byte();
            n_data += 1;
        }
        n_data
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
