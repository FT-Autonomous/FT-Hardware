use std::io::{self, BufRead};
use std::thread;
use std::time::Duration;

const LPWM: u8 = 9;
const RPWM: u8 = 10;
const L_EN: u8 = 25;
const R_EN: u8 = 26;
const LED: u8 = 27;

fn setup() -> bool {
    // 1. Immediately force all pins LOW
    pin_mode(LPWM, PinMode::Output);
    pin_mode(RPWM, PinMode::Output);
    pin_mode(L_EN, PinMode::Output);
    pin_mode(R_EN, PinMode::Output);
    pin_mode(LED, PinMode::Output);

    digital_write(LPWM, false);
    digital_write(RPWM, false);
    digital_write(L_EN, false); // Keep driver disabled initially
    digital_write(R_EN, false);
    digital_write(LED, false); // LED starts OFF

    println!("SYSTEM INACTIVE. Type 'ARM' to begin...");

    // 2. Wait for Serial Command
    let stdin = io::stdin();
    let mut is_armed = false;
    while !is_armed {
        let mut input = String::new();
        if stdin.lock().read_line(&mut input).is_ok() {
            let input = input.trim();
            if input == "ARM" {
                is_armed = true;
                println!("!!! SYSTEM ARMED - MOTORS LIVE !!!");
            }
        }
        thread::sleep(Duration::from_millis(100));
    }

    // 3. Enable the driver only after arming
    digital_write(L_EN, true);
    digital_write(R_EN, true);
    digital_write(LED, true); // LED turns on when system armed

    is_armed
}

fn main() {
    setup();
    loop {
        analog_write(LPWM, 255);
        analog_write(RPWM, 0);
        thread::sleep(Duration::from_secs(1));
        analog_write(LPWM, 0);
        analog_write(RPWM, 255);
        thread::sleep(Duration::from_secs(1));
    }
}

enum PinMode { Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn digital_write(_pin: u8, _val: bool) {}
fn analog_write(_pin: u8, _val: u8) {}
