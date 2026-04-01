/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Master Code
  Connection: Connect all the boards from the MYOSA kit with the "Controller" board and power them up.
  Working: Controller board will display data (on the OLED board) from all the modules in cyclic fashion demonstrating complete RAW capabilities of the kit.

  Synopsis of MYOSA platform
  MYOSA Platform consists of a centralized motherboard a.k.a Controller board, 5 different sensor modules, an OLED display and an actuator board in the kit.
  Controller board is designed on ESP32 module. It is a low-power system on a chip microcontrollers with integrated Wi-Fi and Bluetooth.
  5 Sensors are as below,
  1 --> Accelerometer and Gyroscope (6-axis motion sensor)
  2 --> Temperature and Humidity Sensor
  3 --> Barometric Pressure Sensor
  4 --> Light, Proximity and Gesture Sensor
  5 --> Air Quality Sensor
  Actuator board contains a Buzzer and an AC switching circuit to turn on/off an electrical appliance.
  There is also an OLED display in the MYOSA kit.

  You can design N number of such utility examples as a part of your learning from this kit.

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

/// Hardware abstraction stubs for Serial, Wire (I2C), millis timer, and MYOSA master class.
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn wire_begin() {}
    pub fn wire_set_clock(_freq: u32) {}
    pub fn millis() -> u32 { 0 }

    pub struct MYOSA;

    impl MYOSA {
        pub fn new() -> Self { MYOSA }
        pub fn begin(&self) -> &str { "MYOSA initialized" }
        pub fn print_accel_and_gyro(&self) {}
        pub fn print_air_quality(&self) {}
        pub fn print_barometric_pressure(&self) {}
        pub fn print_light_proximity_and_gesture(&self) {}
        pub fn print_temp_and_humidity(&self) {}
        pub fn send_ble_data(&self) {}
    }
}

use hal::*;

/* Create Object of MYOSA class */
fn create_myosa() -> MYOSA {
    MYOSA::new()
}

/* Set the timer to zero */
static mut PREVIOUS_MILLIS: u32 = 0;        // will store last time screen was updated

/* Global Constants */
const PER_MODULE_INTERVAL: u32 = 1500;           // interval at which screen will update next data (milliseconds)
static mut N_SCREEN: u8 = 0;

/* Setup Function */
fn setup(myosa: &MYOSA) {

    /* Setting up communication */
    serial_begin(115200);
    wire_begin();
    wire_set_clock(100000);

    /* This function initializes all the modules attached. */
    serial_println(myosa.begin());
}

/* Loop Function */
fn main_loop(myosa: &MYOSA) {

    /* Loop Function make use of all the connected modules in cyclic form and prints the data/action in OLED display. */
    let current_millis = millis();
    unsafe {
        if current_millis - PREVIOUS_MILLIS >= PER_MODULE_INTERVAL {
            PREVIOUS_MILLIS = current_millis;
            match N_SCREEN {
                0 => {
                    myosa.print_accel_and_gyro();
                    N_SCREEN = 1;
                }
                1 => {
                    myosa.print_accel_and_gyro();
                    N_SCREEN = 2;
                }
                2 => {
                    myosa.print_accel_and_gyro();
                    N_SCREEN = 3;
                }
                3 => {
                    myosa.print_accel_and_gyro();
                    N_SCREEN = 4;
                }
                4 => {
                    myosa.print_air_quality();
                    N_SCREEN = 5;
                }
                5 => {
                    myosa.print_barometric_pressure();
                    N_SCREEN = 6;
                }
                6 => {
                    myosa.print_barometric_pressure();
                    N_SCREEN = 7;
                }
                7 => {
                    myosa.print_light_proximity_and_gesture();
                    N_SCREEN = 8;
                }
                8 => {
                    myosa.print_light_proximity_and_gesture();
                    N_SCREEN = 9;
                }
                9 => {
                    myosa.print_temp_and_humidity();
                    N_SCREEN = 10;
                }
                10 => {
                    myosa.print_temp_and_humidity();
                    N_SCREEN = 0;
                }
                _ => {
                    N_SCREEN = 0;
                }
            }
            myosa.send_ble_data();
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let myosa = create_myosa();
    setup(&myosa);
    loop {
        main_loop(&myosa);
    }
}
