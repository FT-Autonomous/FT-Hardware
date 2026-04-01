// This is a simple test code for the steering functionality (check your connections)

use std::thread;
use std::time::Duration;

const PIN: u8 = 3; // A3
const MOTOR_A: u8 = 5;
const MOTOR_B: u8 = 6;

fn setup() {
    // put your setup code here, to run once:
    pin_mode(PIN, PinMode::Input);
    pin_mode(MOTOR_A, PinMode::Output);
    pin_mode(MOTOR_B, PinMode::Output);
}

fn main() {
    setup();
    loop {
        // put your main code here, to run repeatedly:
        println!("one way");
        analog_write(MOTOR_A, 125);
        analog_write(MOTOR_B, 0);
        thread::sleep(Duration::from_secs(2));

        println!("other way");
        analog_write(MOTOR_A, 0);
        analog_write(MOTOR_B, 125);
        thread::sleep(Duration::from_secs(2));

        println!("stop");
        analog_write(MOTOR_A, 0);
        analog_write(MOTOR_B, 0);
        thread::sleep(Duration::from_secs(2));
    }
}

enum PinMode { Input, Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn analog_write(_pin: u8, _val: u8) {}
