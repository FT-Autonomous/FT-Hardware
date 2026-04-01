/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Light Proximity Demo
  Connection: Connect the "Light Proximity and Gesture" board from the MYOSA kit with the "Controller" board and power them up.
  Working: Controller board prints (on Serial Monitor) the data of Ambient Light, RGB Proportion and Proximity every second.

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

#![no_std]
#![no_main]

/// Hardware abstraction stubs for Serial, Wire (I2C), and Light/Proximity/Gesture sensor.
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn wire_begin() {}
    pub fn wire_set_clock(_freq: u32) {}
    pub fn delay(_ms: u32) {}

    pub const DISABLE: bool = false;
    pub const PGAIN_2X: u8 = 2;

    pub struct LightProximityAndGesture;

    impl LightProximityAndGesture {
        pub fn new() -> Self { LightProximityAndGesture }
        pub fn begin(&self) -> bool { true }
        pub fn ping(&self) -> bool { true }
        pub fn enable_ambient_light_sensor(&self, _interrupts: bool) -> bool { true }
        pub fn enable_proximity_sensor(&self, _interrupts: bool) -> bool { true }
        pub fn set_proximity_gain(&self, _gain: u8) -> bool { true }
        pub fn get_ambient_light(&self) -> i32 { 0 }
        pub fn get_rgb_proportion(&self) -> [u16; 3] { [0, 0, 0] }
        pub fn get_proximity(&self) {}
    }
}

use hal::*;

/* Creating Object of LightProximityAndGesture Class */
fn create_sensor() -> LightProximityAndGesture {
    LightProximityAndGesture::new()
}

/* Setup Function */
fn setup(lpg: &LightProximityAndGesture) {

    /* Setting up communication */
    serial_begin(115200);
    wire_begin();
    wire_set_clock(100000);

    /* Setting up the LightProximityAndGesture Board. */
    loop {
        if lpg.begin() {
            serial_println("Proximity, Ambient Light, RGB & Gesture sensor is connected...");
            break;
        }
        serial_println("Proximity, Ambient Light, RGB & Gesture sensor is disconnected...");
        delay(500);
    }
    serial_println("APDS9960 initialization completed");

    /* Start running the Ambient light sensor engine (no interrupts) */
    if lpg.enable_ambient_light_sensor(DISABLE) {
        serial_println("Light sensor is now running");
    } else {
        serial_println("Something went wrong during light sensor init!");
    }

    /* Start running the Proximity sensor engine (no interrupts) */
    if lpg.enable_proximity_sensor(DISABLE) {
        serial_println("Proximity sensor is now running");
    } else {
        serial_println("Something went wrong during sensor init!");
    }

    /* Adjust the Proximity sensor gain */
    if !lpg.set_proximity_gain(PGAIN_2X) {
        serial_println("Something went wrong trying to set PGAIN");
    }

    /* Wait for initialization and calibration to finish */
    delay(500);
}

/* Loop Function */
fn main_loop(lpg: &LightProximityAndGesture) {

    /* Loop function continuously gets data and print at every second */
    if lpg.ping() {
        lpg.get_ambient_light();
        let _rgb_proportion = lpg.get_rgb_proportion();
        lpg.get_proximity();
        serial_println("");
    }
    delay(1000);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let lpg = create_sensor();
    setup(&lpg);
    loop {
        main_loop(&lpg);
    }
}
