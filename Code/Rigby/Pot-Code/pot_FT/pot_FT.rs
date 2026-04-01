use std::thread;
use std::time::Duration;

const POT: u8 = 3; // Analog read (A3)
const MOTOR_F: u8 = 5;
const MOTOR_B: u8 = 6;

fn setup() {
    pin_mode(MOTOR_F, PinMode::Output);
    pin_mode(MOTOR_B, PinMode::Output);
}

fn main() {
    setup();
    loop {
        let value = analog_read(POT); // Read raw value
        print!("{}", POT); // to see if there uis a change in vaue
        let angle = map(value, 0, 1023, 0, 100); // Scale to 0-100 range [Angle stuff using map function to print out values]

        print!("Potentiometer angle: ");
        println!("{}", angle);

        //delay(200); // Smoool delay
        analog_write(MOTOR_F, 150); // Turn right for 2s
        analog_write(MOTOR_B, 0);
        thread::sleep(Duration::from_secs(2));

        analog_write(MOTOR_F, 0); // Turn Left for 2s
        analog_write(MOTOR_B, 0);
        thread::sleep(Duration::from_secs(2));

        analog_write(MOTOR_F, 255); // hold for 2s
        analog_write(MOTOR_F, 255);
        thread::sleep(Duration::from_secs(2));
    }
}

enum PinMode { Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn analog_write(_pin: u8, _val: u8) {}
fn analog_read(_pin: u8) -> i32 { 0 }
fn map(val: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
