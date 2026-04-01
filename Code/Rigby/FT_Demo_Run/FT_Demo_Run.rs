use std::thread;
use std::time::Duration;

const DMOTORF: u8 = 10;
const DMOTORB: u8 = 11;

fn setup() {
    // put your setup code here, to run once:
    pin_mode(DMOTORF, PinMode::Output);
    pin_mode(DMOTORB, PinMode::Output);

    thread::sleep(Duration::from_secs(5));

    analog_write(DMOTORF, 120); // speed - no more than 120
    analog_write(DMOTORB, 0);

    thread::sleep(Duration::from_secs(5)); // this is how long to run for 1000 = 1 sec

    analog_write(DMOTORF, 0);
    analog_write(DMOTORB, 0);
}

fn main() {
    setup();
    // put your main code here, to run repeatedly:
    loop {}
}

enum PinMode { Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn analog_write(_pin: u8, _val: u8) {}
