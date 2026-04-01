use std::thread;
use std::time::Duration;

const POT: u8 = 3; // Analog read (A3)

fn setup() {
    // Serial.begin(9600)
}

fn main() {
    setup();
    loop {
        let value = analog_read(POT); // Read raw value
        print!("{}", POT); // to see if there uis a change in vaue
        let angle = map(value, 0, 1023, 0, 100); // Scale to 0-100 range [Angle stuff using map function to print out values]

        print!("Potentiometer angle: ");
        println!("{}", angle);

        thread::sleep(Duration::from_millis(200)); // Smoool delay
    }
}

fn analog_read(_pin: u8) -> i32 { 0 }
fn map(val: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
