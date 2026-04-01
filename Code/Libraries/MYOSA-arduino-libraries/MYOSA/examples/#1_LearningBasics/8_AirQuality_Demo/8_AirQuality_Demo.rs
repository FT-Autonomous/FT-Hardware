/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Air Quality Demo
  Connection: Connect the "Air Quality" board from the MYOSA kit with the "Controller" board and power them up.
  Working: Controller board prints (on Serial Monitor) the data of Total Volatile Organic Compounds (TVOCs) and equivalent carbon dioxide (eCO2) every second.

  Synopsis of Air Quality
  MYOSA Platform consists of an environmental Air Quality Board. It is equiped with CCS811 IC.
  It is a digital gas sesnor that senses wide range of TVOCs and eCO2. It is is intended for indoor air quality monitoring purposes.
  I2C Address of the board = 0x5B.
  Detailed Information about Air Quality board Library and usage is provided in the link below.
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

/// Hardware abstraction stubs for Serial, Wire (I2C), and Air Quality sensor.
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn serial_print(_msg: &str) {}
    pub fn wire_begin() {}
    pub fn wire_set_clock(_freq: u32) {}
    pub fn delay(_ms: u32) {}

    pub const CCS811_I2C_ADDRESS1: u8 = 0x5B;
    pub const REF_RESISTANCE: f32 = 10000.0;
    pub const SENSOR_SUCCESS: u8 = 0;

    pub struct AirQuality {
        _addr: u8,
        _ref_resistance: f32,
    }

    impl AirQuality {
        pub fn new(addr: u8, ref_resistance: f32) -> Self {
            AirQuality { _addr: addr, _ref_resistance: ref_resistance }
        }
        pub fn begin(&self) -> u8 { SENSOR_SUCCESS }
        pub fn ping(&self) -> bool { true }
        pub fn get_hw_id(&self) -> u8 { 0x81 }
        pub fn get_hw_version(&self) -> u8 { 0 }
        pub fn get_fw_boot_version(&self) -> u16 { 0 }
        pub fn get_fw_app_version(&self) -> u16 { 0 }
        pub fn is_data_available(&self) -> bool { true }
        pub fn read_algorithm_results(&self) -> u8 { SENSOR_SUCCESS }
        pub fn get_co2(&self) {}
        pub fn get_tvoc(&self) {}
    }
}

use hal::*;

/* Creating Object of Air Quality Class */
fn create_sensor() -> AirQuality {
    AirQuality::new(CCS811_I2C_ADDRESS1, REF_RESISTANCE)
}

/* Setup Function */
fn setup(aq: &AirQuality) {

    /* Setting up communication */
    serial_begin(115200);
    wire_begin();
    wire_set_clock(100000);

    /* Setting up the Air Quality Board. */
    loop {
        if aq.begin() == SENSOR_SUCCESS {
            serial_println("Air Quality sensor CCS811 is connected...");
            break;
        }
        serial_println("Air Quality sensor ccs811 is disconnected...");
        delay(500);
    }

    serial_println("\nDevice Specifications");
    serial_print("DEVICE ID       : 0x");
    // In a real implementation, format aq.get_hw_id() as hex
    let _ = aq.get_hw_id();
    serial_println("");
    serial_print("HW VERSION      : ");
    let _ = aq.get_hw_version();
    serial_println("");
    serial_print("FW BOOT VERSION : ");
    let _ = aq.get_fw_boot_version();
    serial_println("");
    serial_print("FW APP VERSION  : ");
    let _ = aq.get_fw_app_version();
    serial_println("");
    serial_println("");
}

/* Loop function */
fn main_loop(aq: &AirQuality) {

    /* Loop function continously prints data from the sensor every 1 second */
    if aq.ping() {
        /* Check if data is ready or not */
        if aq.is_data_available() {
            if aq.read_algorithm_results() == SENSOR_SUCCESS {
                aq.get_co2();
                aq.get_tvoc();
                serial_println("");
            }
        }
    }
    delay(1010);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let aq = create_sensor();
    setup(&aq);
    loop {
        main_loop(&aq);
    }
}
