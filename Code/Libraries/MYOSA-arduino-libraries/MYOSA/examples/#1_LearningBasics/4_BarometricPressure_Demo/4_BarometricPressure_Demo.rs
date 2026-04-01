/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  BarometricPressure Demo
  Connection: Connect the "Barometric Pressure" board from the MYOSA kit with the "Controller" board and power them up.
  Working: Controller board prints (on Serial Monitor) the data of Temperature, Pressure and Altitude every second.

  Synopsis of Barometric Pressure Board
  MYOSA Platform consists of a Barometric Pressure Board. It is equiped with BMP180 IC which has a pressure sensing range
  of 300-1100 hPa (9000m to -500m above sea level), with a precision up to 0.03hPa/0.25m resolution.
  It also have temperature sensing element with -40 to +85C operational range, +/-2C temperature accuracy.
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

#![no_std]
#![no_main]

/// Hardware abstraction stubs for Serial, Wire (I2C), and Barometric Pressure sensor.
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn wire_begin() {}
    pub fn wire_set_clock(_freq: u32) {}
    pub fn delay(_ms: u32) {}

    pub const ULTRA_HIGH_RESOLUTION: u8 = 3;
    pub const SEA_LEVEL_AVG_PRESSURE: f32 = 101325.0;

    pub struct BarometricPressure {
        _resolution: u8,
    }

    impl BarometricPressure {
        pub fn new(resolution: u8) -> Self { BarometricPressure { _resolution: resolution } }
        pub fn begin(&self) -> bool { true }
        pub fn ping(&self) -> bool { true }
        pub fn get_temp_c(&self) {}
        pub fn get_temp_f(&self) {}
        pub fn get_pressure_pascal(&self) {}
        pub fn get_pressure_hg(&self) {}
        pub fn get_pressure_bar(&self) {}
        pub fn get_altitude(&self, _sea_level_pressure: f32) -> f32 { 0.0 }
        pub fn get_sea_level_pressure(&self, _altitude: f32) {}
    }
}

use hal::*;

/* Creating Object of BarometricPressure Class */
fn create_sensor() -> BarometricPressure {
    BarometricPressure::new(ULTRA_HIGH_RESOLUTION)
}

/* Setup Function */
fn setup(pr: &BarometricPressure) {

    /* Setting up communication */
    serial_begin(115200);
    wire_begin();
    wire_set_clock(100000);

    /* Setting up the BarometricPressure Board. */
    loop {
        if pr.begin() == true {
            serial_println("Barometric Pressure Sensor is connected");
            break;
        }
        serial_println("Barometric Pressure Sensor is disconnected");
        delay(500);
    }
}

/* Loop Function */
fn main_loop(pr: &BarometricPressure) {

    /* Loop function continuously gets data and print at every 1 second */
    if pr.ping() {
        pr.get_temp_c();
        pr.get_temp_f();
        pr.get_pressure_pascal();
        pr.get_pressure_hg();
        pr.get_pressure_bar();
        let altitude = pr.get_altitude(SEA_LEVEL_AVG_PRESSURE);
        pr.get_sea_level_pressure(altitude);
        serial_println("");
    }
    delay(1000);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let pr = create_sensor();
    setup(&pr);
    loop {
        main_loop(&pr);
    }
}
