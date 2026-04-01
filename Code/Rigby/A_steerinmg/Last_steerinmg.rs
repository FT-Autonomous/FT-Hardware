//Latest code as of 16/04/2026 (Passed all tests so this is the main code)
// The CODE WORKS despite the alligations that it does not, tested it again using a diffrent pot
// The issue? the wires to the pot are not secured, values get stuck to a fixed value (0 in this case)

use std::time::Instant;

const POT: u8 = 3; // Analog read (A3)
const MOTOR_B: u8 = 5; // DC Motor that contorls the steering module
const MOTOR_F: u8 = 6;

// Resrouces: https://forum.arduino.cc/t/dc-motor-pid-control-with-arduino-motor-shield-and-encoder/217946/3 & https://forum.arduino.cc/t/how-do-i-control-the-position-of-my-dc-motor-by-using-a-pot/251307/20

// Ideally ==  start @ 0, feed angle through serial mon, use the pot to read the current angle and head towards the goal, stop once you reach the goal

// One thing left to do (27/02/2026): Check the pot, the readings are not present here, idk why or how but that should
//To do: find a way to reduce the swing rate needed for the pot to read an angle
// the currecnt setup assumes that the pot movement is bigger than what acctually happens, so figure this out
// think

static mut ANGLE_TARGET: f32 = 0.0; // Ideally start @Zero
static mut ANGLE_CURRENT: f32 = 0.0; // Float works for now, round off happens down in line 56
#[allow(dead_code)]
static mut ANGLE_CURRENT_RAW: f32 = 0.0;
static mut ERROR: f32 = 0.0;

const POT_MIN: f32 = 447.0;
const POT_MAX: f32 = 591.0; // so these here are the angle values (roughly), for max L, C, max R
#[allow(dead_code)]
const CENTER: f32 = 520.0;

const KP: f32 = 10.0;
const ANGLE_MAX: f32 = 17.5;

fn setup() {
    serial_set_timeout(1000);
    pin_mode(POT, PinMode::Input);
    pin_mode(MOTOR_F, PinMode::Output);
    pin_mode(MOTOR_B, PinMode::Output);

    analog_write(MOTOR_F, 0); // the DC motor pins start@ zero
    analog_write(MOTOR_B, 0);
}

fn main() {
    setup();
    let mut last_time = Instant::now();
    let sample: u128 = 10;   // smol boy when running (10-20 ms) and big when debug

    loop { unsafe {
        let elapsed = last_time.elapsed().as_millis();

        if serial_available() {
            let angle_input = serial_parse_float(); // Read the current pot
            ANGLE_TARGET = angle_input.clamp(-ANGLE_MAX, ANGLE_MAX); // HARD LIMIT to avoied accidents
        }

        // Sample every T seconds for any update
        if elapsed >= sample {
            last_time = Instant::now(); // update
            let mut pot_ang = analog_read(POT); // Update, use int insted
            pot_ang = (pot_ang as f32).clamp(POT_MIN, POT_MAX) as i32; // map the range again just to stay in range
            //angle_current = map(pot_ang, 0 , 1023, -angle_max, angle_max); // TL;DR match the limits to the pot
            ANGLE_CURRENT = (pot_ang as f32 - POT_MIN) * (ANGLE_MAX - (-ANGLE_MAX)) / (POT_MAX - POT_MIN) + (-ANGLE_MAX); // you convert the pot reading in deg and then

            //angle_current = round(angle_current_raw * 10) / 10;
            ERROR = ANGLE_TARGET - ANGLE_CURRENT; // the diffrence is how we compute the PID
            // Now we should scale the PWM in a range with the pot:
            let mut pwm = (ERROR * KP * (255.0 / ANGLE_MAX)).abs() as i32;
            pwm = pwm.clamp(0, 255); // Possible issue with the limit of pwm, wraps value like a clock

            // Control the motor direction using simple if
            let range: f32 = 0.4;
            if ERROR >= range { //if error is greater than 1
                analog_write(MOTOR_F, 0);
                analog_write(MOTOR_B, pwm as u8);  // Go left (or right can't rememeber)
                print!("L");
            } else if ERROR <= -range { //if error is greater than -1 but not greatrer than 1
                analog_write(MOTOR_F, pwm as u8);
                analog_write(MOTOR_B, 0);  // Go left (or right can't rememeber)
                print!("R");
            } else if ERROR == 's' as i32 as f32 {
                analog_write(MOTOR_F, 0);
                analog_write(MOTOR_B, 0);  // think of it as an E-break
                print!("E stop");
            } else { //if error is less than 1 and less than -1
                analog_write(MOTOR_F, 0);
                analog_write(MOTOR_B, 0);  // STOOOOOOOP, if not vittu
                print!("Your move blud");
            }

            // Print and hope this thing works if not, i give up
            print!("Target: "); print!("{}", ANGLE_TARGET);
            print!(" | Current: "); print!("{}", ANGLE_CURRENT);
            print!(" | Err: "); print!("{}", ERROR);
            print!(" | PWM: "); print!("{}", pwm);
            print!(" | ANGLE: "); println!("{}", pot_ang);
        }
    }}
}

enum PinMode { Input, Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn analog_write(_pin: u8, _val: u8) {}
fn analog_read(_pin: u8) -> i32 { 0 }
fn serial_available() -> bool { false }
fn serial_parse_float() -> f32 { 0.0 }
fn serial_set_timeout(_ms: u32) {}
