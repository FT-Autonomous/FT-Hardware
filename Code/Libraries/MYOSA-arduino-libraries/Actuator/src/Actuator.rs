/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Synopsis of Actuator Board
  MYOSA Platform consists of an Actuator board. It is equiped with PCA9536 IC, a 4-bit I/O Expander with I2C operation.
  Hence, there are 4 Configurable I/O Ports available in the Actuator Board. We have utilized the ports as described below.
  1. ---> 5V Buzzer
  2. ---> AC switching Triac Circuit
  3. ---> Available for user configuration (Output Only)
  4. ---> Available for user configuration (Output Only)
  I2C Address of the board = 0x41.
  Detailed Information about Actuator board Library and usage is provided in the link below.
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

const ACTUATOR_I2C_ADDRESS: u8 = 0x41;

/* Defining Relay and Buzzer IO */
pub const AC_SWITCH_IO: PcaPin = PcaPin::IO0;
pub const BUZZER_IO: PcaPin = PcaPin::IO1;

pub const ALL_INPUT: u8 = 0xFF;
pub const ALL_OUTPUT: u8 = 0x00;
pub const ALL_LOW: u8 = 0x00;
pub const ALL_HIGH: u8 = 0xFF;
pub const ALL_NON_INVERTED: u8 = 0x00;
pub const ALL_INVERTED: u8 = 0xFF;

/*!
 * List of registers available in PCA9536 4bit I/O exanpder
 */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PcaReg {
    InputReg = 0,
    OutputReg = 1,
    PolarityReg = 2,
    ConfigReg = 3,
}

/*!
 * list of I/O pins in PCA9536 4bit I/O exanpder
 */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PcaPin {
    IO0 = 0,
    IO1 = 1,
    IO2 = 2,
    IO3 = 3,
}

/*!
 * PCA9536 4bit I/O exanpder IO pin modes
 */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PinMode {
    IoOutput = 0,
    IoInput = 1,
}

/*!
 * PCA9536 4bit I/O exanpder IO pin state control values
 */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PinState {
    IoLow = 0,
    IoHigh = 1,
}

/*!
 * PCA9536 4bit I/O exanpder IO pin polarity control values
 */
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PinPolarity {
    IoNonInverted = 0,
    IoInverted = 1,
}

/***********************************************************************************************
 * Platform dependent trait. Implement this for your microcontroller's I2C peripheral.          *
 ***********************************************************************************************/
pub trait I2CBus {
    type Error;

    /// Write `data` bytes to the device at `address`.
    fn write(&mut self, address: u8, data: &[u8]) -> Result<(), Self::Error>;

    /// Write `data` bytes then read `length` bytes from the device at `address`.
    fn write_read(
        &mut self,
        address: u8,
        data: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>;
}

pub struct Actuator<I2C> {
    i2c_slave_address: u8,
    i2c: I2C,
}

/*
 *
 */
impl<I2C: I2CBus> Actuator<I2C> {
    /*
     *
     */
    pub fn new(i2c: I2C) -> Self {
        Actuator {
            i2c_slave_address: ACTUATOR_I2C_ADDRESS,
            i2c,
        }
    }

    /*
     *
     */
    pub fn ping(&mut self) -> bool {
        self.write_address()
    }

    /*
     *
     */
    pub fn get_mode(&mut self, pin: PcaPin) -> PinMode {
        let mut mode: u8 = 0;
        self.read_byte(PcaReg::ConfigReg, &mut mode);
        if (mode >> (pin as u8)) & 0x01 != 0 {
            PinMode::IoInput
        } else {
            PinMode::IoOutput
        }
    }

    /*
     *
     */
    pub fn get_state(&mut self, pin: PcaPin) -> PinState {
        let mut state: u8 = 0;
        let reg = if self.get_mode(pin) == PinMode::IoInput {
            PcaReg::InputReg
        } else {
            PcaReg::OutputReg
        };
        self.read_byte(reg, &mut state);
        if (state >> (pin as u8)) & 0x01 != 0 {
            PinState::IoHigh
        } else {
            PinState::IoLow
        }
    }

    /*
     *
     */
    pub fn get_polarity(&mut self, pin: PcaPin) -> PinPolarity {
        let mut polarity: u8 = 0;
        self.read_byte(PcaReg::PolarityReg, &mut polarity);
        if (polarity >> (pin as u8)) & 0x01 != 0 {
            PinPolarity::IoInverted
        } else {
            PinPolarity::IoNonInverted
        }
    }

    /*
     *
     */
    pub fn set_mode_pin(&mut self, pin: PcaPin, new_mode: PinMode) {
        let mut mode_all: u8 = 0;
        self.read_byte(PcaReg::ConfigReg, &mut mode_all);
        mode_all &= !(1u8 << (pin as u8));
        mode_all |= (new_mode as u8) << (pin as u8);
        self.write_byte(PcaReg::ConfigReg, mode_all);
    }

    /*
     *
     */
    pub fn set_mode_all(&mut self, new_mode: PinMode) {
        let mode_all: u8 = if new_mode == PinMode::IoInput {
            ALL_INPUT
        } else {
            ALL_OUTPUT
        };
        self.write_byte(PcaReg::ConfigReg, mode_all);
    }

    /*
     *
     */
    pub fn set_state_pin(&mut self, pin: PcaPin, new_state: PinState) {
        let mut state_all: u8 = 0;
        self.read_byte(PcaReg::OutputReg, &mut state_all);
        state_all &= !(1u8 << (pin as u8));
        state_all |= (new_state as u8) << (pin as u8);
        self.write_byte(PcaReg::OutputReg, state_all);
    }

    /*
     *
     */
    pub fn set_state_all(&mut self, new_state: PinState) {
        let state_all: u8 = if new_state == PinState::IoHigh {
            ALL_HIGH
        } else {
            ALL_LOW
        };
        self.write_byte(PcaReg::OutputReg, state_all);
    }

    /*
     *
     */
    pub fn toggle_state_pin(&mut self, pin: PcaPin) {
        let mut state_all: u8 = 0;
        self.read_byte(PcaReg::OutputReg, &mut state_all);
        state_all ^= 1u8 << (pin as u8);
        self.write_byte(PcaReg::OutputReg, state_all);
    }

    /*
     *
     */
    pub fn toggle_state_all(&mut self) {
        let mut state_all: u8 = 0;
        self.read_byte(PcaReg::OutputReg, &mut state_all);
        state_all ^= 0xFF;
        self.write_byte(PcaReg::OutputReg, state_all);
    }

    /*
     *
     */
    pub fn set_polarity_pin(&mut self, pin: PcaPin, new_polarity: PinPolarity) {
        let mut polarity_all: u8 = 0;
        if self.get_mode(pin) == PinMode::IoInput {
            self.read_byte(PcaReg::PolarityReg, &mut polarity_all);
            polarity_all &= !(1u8 << (pin as u8));
            polarity_all |= (new_polarity as u8) << (pin as u8);
            self.write_byte(PcaReg::PolarityReg, polarity_all);
        }
    }

    /*
     *
     */
    pub fn set_polarity_all(&mut self, new_polarity: PinPolarity) {
        let mut polarity_all: u8 = 0;
        let mut polarity_msk: u8 = 0;
        self.read_byte(PcaReg::PolarityReg, &mut polarity_all);
        self.read_byte(PcaReg::ConfigReg, &mut polarity_msk);
        let polarity_new: u8 = if new_polarity == PinPolarity::IoInverted {
            ALL_INVERTED
        } else {
            ALL_NON_INVERTED
        };
        self.write_byte(
            PcaReg::PolarityReg,
            (polarity_all & !polarity_msk) | (polarity_new & polarity_msk),
        );
    }

    /***********************************************************************************************
     * Platform dependent routines. Change these functions implementation based on microcontroller *
     ***********************************************************************************************/
    /**
     *
     */
    fn read_byte(&mut self, reg: PcaReg, buf: &mut u8) -> bool {
        let mut data = [0u8; 1];
        match self
            .i2c
            .write_read(self.i2c_slave_address, &[reg as u8], &mut data)
        {
            Ok(()) => {
                *buf = data[0];
                true
            }
            Err(_) => false,
        }
    }

    /**
     *
     */
    fn write_address(&mut self) -> bool {
        self.i2c.write(self.i2c_slave_address, &[]).is_ok()
    }

    /**
     *
     */
    fn write_byte(&mut self, reg: PcaReg, val: u8) -> bool {
        self.i2c
            .write(self.i2c_slave_address, &[reg as u8, val])
            .is_ok()
    }
}
