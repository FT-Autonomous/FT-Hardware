/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Scanning Near By WiFi
  Connection: Connect the "Controller" board and power it up.
  Working: This example scans the Nearby WiFi networks along with relevant details.

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

/// Hardware abstraction stubs for Serial and WiFi scanning.
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn serial_print(_msg: &str) {}
    pub fn serial_print_i32(_val: i32) {}
    pub fn delay(_ms: u32) {}

    pub const WIFI_STA: u8 = 1;
    pub const WIFI_AUTH_OPEN: u8 = 0;

    pub struct WiFi;

    impl WiFi {
        pub fn mode(_m: u8) {}
        pub fn disconnect() {}
        pub fn scan_networks() -> i32 { 0 }
        pub fn ssid(_i: i32) -> &'static str { "" }
        pub fn rssi(_i: i32) -> i32 { 0 }
        pub fn encryption_type(_i: i32) -> u8 { 0 }
    }
}

use hal::*;

/* Setup Function */
fn setup() {

    /* Setting up the communication */
    serial_begin(115200);

    /* Set WiFi to station mode */
    WiFi::mode(WIFI_STA);
    WiFi::disconnect();
    delay(100);

    serial_println("Setup done");
}

/* Loop Function */
fn main_loop() {

    /* Loop function continously scans the available WiFi networks with detailed RSSI values every 5 seconds. */
    serial_println("Scanning...");

    let n = WiFi::scan_networks();

    if n == 0 {
        serial_println("No nearby networks found");
    } else {
        serial_print_i32(n);
        serial_println(" networks found\n");
        for i in 0..n {
            // Print SSID and RSSI for each network found
            serial_print_i32(i + 1);
            serial_print(": ");
            serial_print(WiFi::ssid(i));
            serial_print(" (");
            let rssi = WiFi::rssi(i);
            serial_print_i32(rssi);
            serial_print(")");
            if rssi > -85 {
                serial_print(" Good Signal Strength ");
            } else if rssi > -100 {
                serial_print(" Fair Signal Strength ");
            } else if rssi > -110 {
                serial_print(" Poor Signal Strength ");
            } else if rssi > -120 {
                serial_print(" No Signal ");
            }
            if WiFi::encryption_type(i) == WIFI_AUTH_OPEN {
                serial_println(" ");
            } else {
                serial_println("*");
            }

            delay(200);
        }
    }
    serial_println("\nScanning done");
    serial_println("");

    // Wait a bit before scanning again
    delay(5000);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    setup();
    loop {
        main_loop();
    }
}
