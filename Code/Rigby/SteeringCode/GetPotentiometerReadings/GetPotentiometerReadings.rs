use std::thread;
use std::time::Duration;

fn setup() {
    // Serial.begin(115200) equivalent handled by println! macro
}

fn main() {
    setup();
    loop {
        let value = analog_read(0); // A0
        println!("{}", value);
        thread::sleep(Duration::from_millis(200));
    }
}

fn analog_read(_pin: u8) -> u16 { todo!("hardware analog read") }
