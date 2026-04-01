/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Gesture Controlled Light
  Connection: Connect "Light, Proximity, and Gesture" and "Actuator" boards from the MYOSA kit with the "Controller" board and power them up. Along with that also connect the AC light bulb as shown in the connection diagram below.
  Working: Imagine you are in a sci-fi world where you want everything to be controlled with your gestures. We are developing the same application where we can control a AC Light Bulb with Gestures. Once you upload this code to the board, Sensor will detect the Gesture performed and Turn On and Off the Light Bulb accordingly.
  Connection Diagram: https://esp32io.com/images/tutorial/esp32-how-to-connect-device-to-relay.jpg [UPDATE THIS IMAGE]

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

/// Hardware abstraction stubs for Serial, Wire (I2C), Actuator, and Light/Proximity/Gesture sensor.
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn wire_begin() {}
    pub fn wire_set_clock(_freq: u32) {}
    pub fn delay(_ms: u32) {}

    pub const DISABLE: bool = false;
    pub const AC_SWITCH_IO: u8 = 1;
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

    pub struct LightProximityAndGesture;

    impl LightProximityAndGesture {
        pub fn new() -> Self { LightProximityAndGesture }
        pub fn begin(&self) -> bool { true }
        pub fn ping(&self) -> bool { true }
        pub fn enable_gesture_sensor(&self, _interrupts: bool) -> bool { true }
        pub fn get_gesture(&self) -> &str { "" }
    }
}

use hal::*;

/* Creating Object of LightProximityAndGesture and Actuator class */
fn create_actuator() -> Actuator { Actuator::new() }
fn create_sensor() -> LightProximityAndGesture { LightProximityAndGesture::new() }

/* Setup Function */
fn setup(gpio_expander: &Actuator, lpg: &LightProximityAndGesture) {

    /* Setting up communication */
    serial_begin(115200);
    wire_begin();
    wire_set_clock(100000);

    /* Initializing Actuator Board */
    loop {
        if gpio_expander.ping() {
            serial_println("4bit IO Expander Actautor (PCA9536) is connected");
            break;
        }
        serial_println("4bit IO Expander Actuator (PCA9536) is disconnected");
        delay(500);
    }

    /* Set relay IO as output */
    gpio_expander.set_mode(AC_SWITCH_IO, IO_OUTPUT);
    gpio_expander.set_state(AC_SWITCH_IO, IO_LOW);
    delay(2000);

    /* Initializing Light, Proximity, Gesture Sensor */
    loop {
        if lpg.begin() {
            serial_println("Proximity, Ambient Light, RGB & Gesture sensor is connected...");
            break;
        }
        serial_println("Proximity, Ambient Light, RGB & Gesture sensor is disconnected...");
        delay(500);
    }
    serial_println("APDS9960 initialization completed");

    /* Start running the APDS-9960 gesture sensor engine */
    if lpg.enable_gesture_sensor(DISABLE) {
        serial_println("Gesture sensor is now running");
    } else {
        serial_println("Something went wrong during gesture sensor init!");
    }

    /* Wait for initialization and calibration to finish */
    delay(500);
}

/* Global Constants */
// gesture string is returned from get_gesture() each iteration

/* Loop Function */
fn main_loop(gpio_expander: &Actuator, lpg: &LightProximityAndGesture) {

    /* Loop function continously detects the Gesture and take desired action */
    if lpg.ping() {
        let gesture = lpg.get_gesture();
        if gesture == "UP" {
            gpio_expander.set_state(AC_SWITCH_IO, IO_HIGH);
        } else if gesture == "LEFT" {
            gpio_expander.set_state(AC_SWITCH_IO, IO_HIGH);
        } else if gesture == "DOWN" {
            gpio_expander.set_state(AC_SWITCH_IO, IO_LOW);
        } else if gesture == "RIGHT" {
            gpio_expander.set_state(AC_SWITCH_IO, IO_LOW);
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let gpio_expander = create_actuator();
    let lpg = create_sensor();
    setup(&gpio_expander, &lpg);
    loop {
        main_loop(&gpio_expander, &lpg);
    }
}
