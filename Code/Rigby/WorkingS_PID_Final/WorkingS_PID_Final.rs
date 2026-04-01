use std::time::Instant;

const POT: u8 = 3; // Analog read (A3)
const MOTOR_B: u8 = 5; // DC Motor that contorls the steering module
const MOTOR_F: u8 = 6;

// Resrouces: https://forum.arduino.cc/t/dc-motor-pid-control-with-arduino-motor-shield-and-encoder/217946/3 & https://forum.arduino.cc/t/how-do-i-control-the-position-of-my-dc-motor-by-using-a-pot/251307/20
// Main issue(s): so when you match the current and present angle Motor stops thats good, but it seems like there is an issue with the pot readings going haywire and
// Ok to test there seems to be an issue with the timing variable, the == 0 caused a perma reset so it never updated
// Ideally ==  start @ 0, feed angle through serial mon, use the pot to read the current angle and head towards the goal, stop once you reach the goal
// Pot + Motor controller + DC motor + Voltage generator (12V) + Arduino == Should work
// Youtube vids and arduino forms are worth looking at, They have hidden gems that you can adapt to work like below:

//Update: it worked!!, for future refrence always check if the electronics are kaput, that will save you time, and sanity and and and and and and ....
// One thing left to do: Check the pot, the readings are not present here, idk why or how but that should

static mut ANGLE_TARGET: f32 = 0.0; // Ideally start @Zero
static mut ANGLE_CURRENT: f32 = 0.0; // Float works for now, round off happens down in line 56
static mut ANGLE_CURRENT_RAW: f32 = 0.0;
static mut ERROR: f32 = 0.0;

const KP: f32 = 1.2;
const ANGLE_MAX: f32 = 10.0;

fn setup() {
    pin_mode(POT, PinMode::Input);
    pin_mode(MOTOR_F, PinMode::Output);
    pin_mode(MOTOR_B, PinMode::Output);

    analog_write(MOTOR_F, 0); // the DC motor pins start@ zero
    analog_write(MOTOR_B, 0);
}

fn main() {
    setup();
    let mut last_time = Instant::now();
    let sample: u128 = 20;   // smol boy when running (10-20 ms) and big when debug

    loop { unsafe {
        let elapsed = last_time.elapsed().as_millis();

        if serial_available() {
            let angle_input = serial_parse_float(); // Read the current pot
            ANGLE_TARGET = angle_input.clamp(-ANGLE_MAX, ANGLE_MAX); // HARD LIMIT to avoied accidents
        }

        // Sample every T seconds for any update
        if elapsed >= sample {
            last_time = Instant::now(); // update
            let pot_ang = analog_read(POT) as f32; // I hate this (hear me out float == decimal == round it off == better? idk its convoluted but it works)
            ANGLE_CURRENT = map_float(pot_ang, 0.0, 1023.0, -ANGLE_MAX, ANGLE_MAX); // TL;DR match the limits to the pot

            ANGLE_CURRENT = (ANGLE_CURRENT_RAW * 10.0).round() / 10.0;
            ERROR = ANGLE_TARGET - ANGLE_CURRENT; // the diffrence is how we compute the PID
            // Now we should scale the PWM in a range with the pot:
            let mut pwm = (ERROR * KP * (255.0 / ANGLE_MAX)).abs() as i32;
            pwm = pwm.clamp(0, 255); // Possible issue with the limit of pwm, wraps value like a clock

            // if(pwm > 255)
            //   pwm = 255;
            // else if(pwm < 0)
            //   pwm = 0;

            // An idea: (if this is does not work im *redacted* myself)
            // Control the motor direction using simple if
            let range: f32 = 1.0;
            if ERROR >= range { //if error is greater than 1
                analog_write(MOTOR_F, 0);
                analog_write(MOTOR_B, pwm as u8);  // Go left (or right can't rememeber)
            } else if ERROR <= -range { //if error is greater than -1 but not greatrer than 1
                analog_write(MOTOR_F, pwm as u8);
                analog_write(MOTOR_B, 0);  // Go left (or right can't rememeber)
            } else { //if error is less than 1 and less than -1
                analog_write(MOTOR_F, 0);
                analog_write(MOTOR_B, 0);  // STOOOOOOOP, if not vittu
            }

            // Print and hope this thing works if not, i give up
            print!("Target: "); print!("{}", ANGLE_TARGET);
            print!(" | Current: "); print!("{}", ANGLE_CURRENT_RAW);
            print!(" | Err: "); print!("{}", ERROR);
            print!(" | PWM: "); println!("{}", pwm);
        }
    }}
}

enum PinMode { Input, Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn analog_write(_pin: u8, _val: u8) {}
fn analog_read(_pin: u8) -> i32 { 0 }
fn serial_available() -> bool { false }
fn serial_parse_float() -> f32 { 0.0 }
fn map_float(val: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
