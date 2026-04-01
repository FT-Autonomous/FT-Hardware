/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Blink Demo
  Connection: Connect the "Controller" board from the MYOSA kit power it up.
  Working: Blue status LED on the Controller board starts blinking for every 1 second.

  Synopsis of "Controller" board
  MYOSA Platform consists of a centralized motherboard also known as Controller board. It is board designed of the ESP-Wroom32 module.
  ESP32 is a low-power system on a chip microcontrollers with integrated Wi-Fi and Bluetooth.
  It also has lot many GPIO pins along with famous communication protocols like i2c, uart, spi, etc. which can be used for interfacing with sensors and other modules.
  Detailed Information about MYOSA platform and usage is provided in the link below.
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

/// Hardware abstraction stubs for ESP32 GPIO and timing.
mod hal {
    pub const OUTPUT: u8 = 1;
    pub const HIGH: bool = true;
    pub const LOW: bool = false;

    pub fn pin_mode(_pin: u8, _mode: u8) {}
    pub fn digital_write(_pin: u8, _value: bool) {}
    pub fn delay(_ms: u32) {}
}

use hal::*;

/* Setup Function */
fn setup() {

    /* Setting up Status LED pin as OUTPUT pin */
    pin_mode(2, OUTPUT);
}

/* Loop Function */
fn main_loop() {

    /* Loop function continously blinks the status LED on the Controller Board */
    digital_write(2, HIGH);
    delay(1000);
    digital_write(2, LOW);
    delay(1000);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    setup();
    loop {
        main_loop();
    }
}
