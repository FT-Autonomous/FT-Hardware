/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  TempAndHumidity Demo
  Connection: Connect the "Temperature And Humidity" board from the MYOSA kit with the "Controller" board and power them up.
  Working: Controller board prints (on Serial Monitor) the data of Temperature, Relative Humidity and Heat Index every second.

  Synopsis of Temperature And Humidity Board
  MYOSA Platform consists of an Temperature And Humidity Board. It is equiped with Si7021 IC.
  It has +/- 3% relative humidity measurements with a range of 0-80% RH, and +/-0.4 C temperature accuracy at a range of -10 to +85 C.
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

#![no_std]
#![no_main]

/// Hardware abstraction stubs for Serial, Wire (I2C), and Temperature/Humidity sensor.
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn wire_begin() {}
    pub fn wire_set_clock(_freq: u32) {}
    pub fn delay(_ms: u32) {}

    pub struct TempAndHumidity;

    impl TempAndHumidity {
        pub fn new() -> Self { TempAndHumidity }
        pub fn begin(&self) -> bool { true }
        pub fn ping(&self) -> bool { true }
        pub fn get_firmware_version(&self) {}
        pub fn get_serial_number(&self) {}
        pub fn get_relative_humidity(&self) {}
        pub fn get_temp_c(&self) {}
        pub fn get_temp_f(&self) {}
        pub fn get_heat_index_c(&self) {}
        pub fn get_heat_index_f(&self) {}
    }
}

use hal::*;

/* Creating Object of TempAndHumidity Class */
fn create_sensor() -> TempAndHumidity {
    TempAndHumidity::new()
}

/* Setup Function */
fn setup(th: &TempAndHumidity) {

    /* Setting up communication */
    serial_begin(115200);
    wire_begin();
    wire_set_clock(100000);

    /* Setting up the TempAndHumidity Board. */
    loop {
        if th.begin() {
            serial_println("Temperature and Humidity Sensor is Connected");
            break;
        }
        serial_println("Temperature and Humidity Sensor is Disconnected");
        delay(500);
    }
    th.get_firmware_version();
    th.get_serial_number();
}

/* Loop Function */
fn main_loop(th: &TempAndHumidity) {

    /* Loop function continuously gets data and print at every second */
    if th.ping() {
        th.get_relative_humidity();
        th.get_temp_c();
        th.get_temp_f();
        th.get_heat_index_c();
        th.get_heat_index_f();
        serial_println("");
    }
    delay(1000);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let th = create_sensor();
    setup(&th);
    loop {
        main_loop(&th);
    }
}
