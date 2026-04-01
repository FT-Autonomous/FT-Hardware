use std::thread;
use std::time::Duration;

const PIN_A: u8 = 10;
const PIN_B: u8 = 11;

const ST: u64 = 2000; // delay
const A_VAL: i32 = 100;
const B_VAL: i32 = 100;

fn a(state: i32) {
    let state = (255 * state / 10) as u8;
    analog_write(PIN_A, state);
}//set A to state percent power

fn b(state: i32) {
    let state = (255 * state / 10) as u8;
    analog_write(PIN_B, state);
}

fn setup() {
    // put your setup code here, to run once:
    println!("----------------------");
    pin_mode(PIN_A, PinMode::Output);
    pin_mode(PIN_B, PinMode::Output);
}

fn main() {
    setup();
    loop {
        // put your main code here, to run repeatedly:
        a(0);
        b(0);
        println!("both low");

        thread::sleep(Duration::from_millis(ST));

        a(A_VAL);
        println!("A high: Extend");

        thread::sleep(Duration::from_millis(ST));

        a(0);
        println!("both low");

        thread::sleep(Duration::from_millis(ST));

        b(B_VAL);
        println!("b high: Retract");

        thread::sleep(Duration::from_millis(ST));

        println!("----------------------");
    }
}

enum PinMode { Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn analog_write(_pin: u8, _val: u8) {}
