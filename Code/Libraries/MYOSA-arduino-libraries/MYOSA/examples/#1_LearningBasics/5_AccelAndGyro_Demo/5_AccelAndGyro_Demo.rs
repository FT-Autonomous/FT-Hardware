/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Accelerometer and Gyroscope Demo
  Connection: Connect the "Accelerometer and Gyroscope" board from the MYOSA kit with the "Controller" board and power them up.
  Working: Controller board prints (on Serial Monitor) the (RAW) data of 3-axis Accelerometer & Gyroscope values every 5 seconds.

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

#![no_std]
#![no_main]

/// Hardware abstraction stubs for Serial, Wire (I2C), and Accelerometer/Gyroscope sensor.
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn wire_begin() {}
    pub fn wire_set_clock(_freq: u32) {}
    pub fn delay(_ms: u32) {}

    pub struct AccelAndGyro;

    impl AccelAndGyro {
        pub fn new() -> Self { AccelAndGyro }
        pub fn begin(&self) -> bool { true }
        pub fn ping(&self) -> bool { true }
        pub fn get_accel_x(&self) {}
        pub fn get_accel_y(&self) {}
        pub fn get_accel_z(&self) {}
        pub fn get_gyro_x(&self) {}
        pub fn get_gyro_y(&self) {}
        pub fn get_gyro_z(&self) {}
        pub fn get_temp_c(&self) {}
        pub fn get_temp_f(&self) {}
        pub fn get_tilt_x(&self) {}
        pub fn get_tilt_y(&self) {}
        pub fn get_tilt_z(&self) {}
        pub fn get_motion_status(&self) {}
    }
}

use hal::*;

/* Creating Object of AccelAndGyro Class */
fn create_sensor() -> AccelAndGyro {
    AccelAndGyro::new()
}

/* Setup Function */
fn setup(ag: &AccelAndGyro) {

    /* Setting up communication */
    serial_begin(115200);
    wire_begin();
    wire_set_clock(100000);

    /* Setting up the AccelAndGyro Board. */
    loop {
        if ag.begin() == true {
            serial_println("Accelerometer and Gyroscope Sensor is connected");
            break;
        }
        serial_println("Accelerometer and Gyroscope Sensor is disconnected");
        delay(500);
    }
}

/* Loop Function */
fn main_loop(ag: &AccelAndGyro) {

    /* Loop function continously prints RAW data from the sensor every 5 seconds */
    if ag.ping() {
        ag.get_accel_x();
        ag.get_accel_y();
        ag.get_accel_z();
        ag.get_gyro_x();
        ag.get_gyro_y();
        ag.get_gyro_z();
        ag.get_temp_c();
        ag.get_temp_f();
        ag.get_tilt_x();
        ag.get_tilt_y();
        ag.get_tilt_z();
        ag.get_motion_status();
        serial_println("");
    }
    delay(5000);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let ag = create_sensor();
    setup(&ag);
    loop {
        main_loop(&ag);
    }
}
