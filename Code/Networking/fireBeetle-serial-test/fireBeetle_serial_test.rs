// FireBeetle ESP32 Serial Test
// Board: DFRobot FireBeetle ESP32 (DFR0478) -- select "FireBeetle-ESP32" or "ESP32 Dev Module" in Arduino IDE
// Install: Add https://raw.githubusercontent.com/DFRobot/BoardManagerForDFRobot/master/package_DFRobot_index.json
//          to Arduino IDE > Preferences > Additional Board Manager URLs
//
// Refs:
//   https://www.dfrobot.com/product-1590.html
//   https://arduino.github.io/arduino-cli/0.32/getting-started/
//   https://forum.arduino.cc/t/serial-input-basics-updated/382007

mod ft_serial;
use ft_serial::{FTSerial, SerialPort};
use std::thread;
use std::time::Duration;

const BAUD_RATE: u32 = 115200;
const MAX_MSG_LEN: u8 = 64;  // ESP32 has plenty of RAM -- doubled from original 32

struct HwSerial;
impl SerialPort for HwSerial {
    fn available(&self) -> i32 { 0 }
    fn read(&mut self) -> u8 { 0 }
}

fn setup() {
    thread::sleep(Duration::from_secs(1)); // let ESP32 boot messages flush before we print
    println!("FireBeetle serial test active");
}

fn print_serial(ft_serial: &mut FTSerial<HwSerial>) {
    let serial_string = ft_serial.read_until_newline();
    if serial_string != "" {
        print!("received: ");
        println!("{}", serial_string);
    }
}

fn main() {
    setup();
    let mut ft_serial = FTSerial::new(HwSerial, MAX_MSG_LEN);

    loop {
        if serial_available() {
            print_serial(&mut ft_serial);
        }
        // yield(); // feed the ESP32 watchdog
    }
}

fn serial_available() -> bool { false }
