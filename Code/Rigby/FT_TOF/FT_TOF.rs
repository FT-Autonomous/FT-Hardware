use std::thread;
use std::time::Duration;

const TRIG_PIN: u8 = 8;
const ECHO_PIN: u8 = 9;    // Ultrasonics Pins

const DMOTORF: u8 = 10;
const DMOTORB: u8 = 11;   // Drive Motor forward back
const SMOTOR1: u8 = 3;
const SMOTOR2: u8 = 2;     // Steering Motor 1 2

static mut CH: char = '\0';
static mut DHOLDCMD: char = '\0';
static mut SHOLDCMD: char = '\0';
static mut STEERCMD: char = '\0';
static mut DRIVECMD: char = '\0';
static mut DUPDATE: bool = false;
static mut SUPDATE: bool = false;
static mut DURATION: f32 = 0.0;
static mut DISTANCE: f32 = 0.0;
static mut DISTRD: f32 = 0.0;

fn setup() {
    pin_mode(TRIG_PIN, PinMode::Output); // trigger, output
    pin_mode(ECHO_PIN, PinMode::Input);  // echo, input

    pin_mode(DMOTORF, PinMode::Output);
    pin_mode(DMOTORB, PinMode::Output);  //
    pin_mode(SMOTOR1, PinMode::Output);
    pin_mode(SMOTOR2, PinMode::Output);
    unsafe {
        DUPDATE = false;
        SUPDATE = false;
    }
    digital_write(DMOTORF, false);
    digital_write(DMOTORB, false);
    digital_write(SMOTOR1, false);
    digital_write(SMOTOR2, false);
}

fn loop_fn() {
    digital_write(TRIG_PIN, false);
    delay_microseconds(2);
    digital_write(TRIG_PIN, true);
    delay_microseconds(10);
    digital_write(TRIG_PIN, false);
    unsafe {
        DURATION = pulse_in(ECHO_PIN, true);
        DISTANCE = (DURATION * 0.343) / 2.0;
        DISTRD = (DISTANCE / 10.0).round();
        print!("Distance: ");
        println!("{}", DISTRD);

        charget();
        charset();
        cmdset();
        if DISTRD >= 16.0 {
            println!("max lock left");
        }
        if DISTRD <= 9.9 {
            println!("max lock right");
        }
    }
    thread::sleep(Duration::from_millis(100));
}

fn charget() {
    unsafe {
        if serial_available() {
            CH = serial_read();                /// reads charachter set by laptop controller
        }
    }
}

fn charset() {
    unsafe {
        if (CH == 'w' || CH == 's' || CH == 'n') && !DUPDATE {
            DRIVECMD = CH;
            DUPDATE = true;
        }
        if (CH == 'a' || CH == 'd' || CH == 'm' || CH == 'b') && !SUPDATE {      // decipher command to steering or drive
            STEERCMD = CH;
            SUPDATE = true;
        }
    }
}

fn cmdset() {
    unsafe {
        if DRIVECMD == 'w' && DUPDATE == true {
            analog_write(DMOTORF, 50);
            analog_write(DMOTORB, 0);
            println!("forwards");     // drive forwards if w
            DUPDATE = false;
        }
        if DRIVECMD == 's' && DUPDATE == true {
            analog_write(DMOTORF, 0);   // drive backwards if s
            analog_write(DMOTORB, 50);
            println!("reverse");
            DUPDATE = false;
        }
        if DRIVECMD == 'n' && DUPDATE == true {
            analog_write(DMOTORF, 0);   // no power to drive motor n
            analog_write(DMOTORB, 0);
            println!("neutral");
            DUPDATE = false;
        }
        if STEERCMD == 'a' && SUPDATE == true {
            if DISTRD >= 15.0 {
                digital_write(SMOTOR1, false);   // max lock left
                digital_write(SMOTOR2, false);
                SUPDATE = false;
            } else if SUPDATE == true {
                digital_write(SMOTOR1, true);
                digital_write(SMOTOR2, false);    // steer right d
                println!("left");
                SUPDATE = false;
            }
        }

        if STEERCMD == 'd' {
            if DISTRD <= 9.9 {
                digital_write(SMOTOR1, false);   // max lock right
                digital_write(SMOTOR2, false);
                SUPDATE = false;
            } else if SUPDATE == true {
                digital_write(SMOTOR1, false);
                digital_write(SMOTOR2, true);    // steer right d
                println!("right");
                SUPDATE = false;
            }
        }
        if STEERCMD == 'm' && SUPDATE == true {
            digital_write(SMOTOR1, false);
            digital_write(SMOTOR2, false);     // hold current steering angle m
            println!("hold steering angle");
            SUPDATE = false;
        }
    }
}
//   if(steercmd == 'b' && supdate == true){
//   println!("centering");
//     if(distrd > 12){}
//       digital_write(smotor1, false);
//       digital_write(smotor2, false);     //steer right
//       supdate = false;
//     }
//     if(distrd < 12){
//       digital_write(smotor1, false);
//       digital_write(smotor2, false);     //steer left
//       supdate = false;
//     }
//     else{
//       digital_write(smotor1, false);
//       digital_write(smotor2, false);     // center and hold
//       supdate = false;
//     }
// }

fn main() {
    setup();
    loop {
        loop_fn();
    }
}

enum PinMode { Output, Input }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn digital_write(_pin: u8, _val: bool) {}
fn analog_write(_pin: u8, _val: u8) {}
fn delay_microseconds(_us: u32) {}
fn pulse_in(_pin: u8, _val: bool) -> f32 { 0.0 }
fn serial_available() -> bool { false }
fn serial_read() -> char { '\0' }
