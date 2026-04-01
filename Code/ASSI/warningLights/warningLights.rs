// as = autonomous system

use std::thread;
use std::time::Duration;

static mut AS_OFF: bool = false;
static mut AS_READY: bool = false;
static mut AS_DRIVING: bool = false;
static mut AS_FINISHED: bool = false;
static mut AS_EMERGENCY: bool = false;
static mut MANUAL_D: bool = false;
static mut YELLOW: u8 = 0;
static mut BLUE: u8 = 0;

fn assi_setup() {
    //initalise default/starting states
    unsafe {
        AS_OFF = true;
        AS_READY = false;
        AS_DRIVING = false;
        AS_FINISHED = false;
        AS_EMERGENCY = false;
        MANUAL_D = false;

        //intilise pins

        YELLOW = 5;
        BLUE = 6;
        pin_mode(YELLOW, PinMode::Output);
        pin_mode(BLUE, PinMode::Output);
    }
}

fn assi() {
    unsafe {
        if AS_OFF {
            digital_write(YELLOW, false);
            digital_write(BLUE, false);
        }

        if AS_READY {
            digital_write(YELLOW, true);
            digital_write(BLUE, false);
        }

        if AS_DRIVING {
            blink(YELLOW);
            digital_write(BLUE, false);
        }

        if AS_FINISHED {
            digital_write(YELLOW, false);
            digital_write(BLUE, true);
        }

        if AS_EMERGENCY {
            blink(BLUE);
            digital_write(YELLOW, false);
        }

        if MANUAL_D {
            digital_write(YELLOW, true);
            digital_write(BLUE, true);
        }
    }
}

fn blink(pin: u8) {
    digital_write(pin, true);
    thread::sleep(Duration::from_millis(500));
    digital_write(pin, false);
    thread::sleep(Duration::from_millis(500));
}

fn main() {
    assi_setup();
    loop {
        assi();
    }
}

enum PinMode { Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn digital_write(_pin: u8, _val: bool) {}
