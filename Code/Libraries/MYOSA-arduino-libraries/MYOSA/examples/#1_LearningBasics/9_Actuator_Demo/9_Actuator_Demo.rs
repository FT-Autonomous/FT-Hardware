/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Actuator Demo
  Connection: Connect the "Actuator" board from the MYOSA kit with the "Controller" board and power them up.
  Working: Turns Buzzer and AC switching ckt ON for 1 second and then turns it OFF.

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

#![no_std]
#![no_main]

/// Hardware abstraction stubs for Serial, Wire (I2C), and Actuator (GPIO expander).
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn wire_begin() {}
    pub fn wire_set_clock(_freq: u32) {}
    pub fn delay(_ms: u32) {}

    pub const AC_SWITCH_IO: u8 = 1;
    pub const BUZZER_IO: u8 = 0;
    pub const IO_OUTPUT: u8 = 1;
    pub const IO_HIGH: bool = true;
    pub const IO_LOW: bool = false;

    pub struct Actuator;

    impl Actuator {
        pub fn new() -> Self { Actuator }
        pub fn ping(&self) -> bool { true }
        pub fn set_mode(&self, _io: u8, _mode: u8) {}
        pub fn set_state(&self, _io: u8, _state: bool) {}
    }
}

use hal::*;

/* Creating Object of Actuator Class */
fn create_actuator() -> Actuator {
    Actuator::new()
}

/* Setup Function */
fn setup(gpio_expander: &Actuator) {

    /* Setting up communication */
    serial_begin(115200);
    wire_begin();
    wire_set_clock(100000);

    /* Setting up the Actuator Board. */
    loop {
        if gpio_expander.ping() {
            serial_println("4bit IO Expander Actautor (PCA9536) is connected");
            break;
        }
        serial_println("4bit IO Expander Actuator (PCA9536) is disconnected");
        delay(500);
    }

    /* Set AC SWITCH IO as output */
    gpio_expander.set_mode(AC_SWITCH_IO, IO_OUTPUT);
    gpio_expander.set_state(AC_SWITCH_IO, IO_LOW);

    /* Set BUZZER IO as output */
    gpio_expander.set_mode(BUZZER_IO, IO_OUTPUT);
    gpio_expander.set_state(BUZZER_IO, IO_LOW);
    delay(2000);

    /* Turn-on AC SWITCH for one second */
    gpio_expander.set_state(AC_SWITCH_IO, IO_HIGH);
    delay(1000);
    gpio_expander.set_state(AC_SWITCH_IO, IO_LOW);
    delay(1000);

    /* Turn-on BUZZER for one second */
    gpio_expander.set_state(BUZZER_IO, IO_HIGH);
    delay(1000);
    gpio_expander.set_state(BUZZER_IO, IO_LOW);
    delay(1000);
}

/* Loop Function */
fn main_loop() {

    /* Loop function does nothing */
    delay(1000);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let gpio_expander = create_actuator();
    setup(&gpio_expander);
    loop {
        main_loop();
    }
}
