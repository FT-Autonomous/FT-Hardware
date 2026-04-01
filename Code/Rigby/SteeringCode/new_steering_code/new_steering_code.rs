use std::time::Instant;

const POT: u8 = 3; // Analog read (A3)
const MOTOR_F: u8 = 5;
const MOTOR_B: u8 = 6;

static mut ANGLE_TARGET: f32 = 0.0;
static mut ANGLE_CURRENT: f32 = 0.0;
static mut ERROR: f32 = 0.0;  //error is dynamic
const KP: f32 = 1.0;

static mut PWM: i32 = 0;
static mut CORRECTION: i32 = 0;
const ANGLE_MAX: f32 = 39.0; // Maxium angle (you can't go any further)
static mut ANGLE_INPUT: f32 = 0.0;

fn setup() {
    // put your setup code here, to run once:
    pin_mode(MOTOR_F, PinMode::Output);
    pin_mode(MOTOR_B, PinMode::Output);
}

fn main() {
    setup();
    let mut last_time = Instant::now();

    loop { unsafe {
        // put your main code here, to run repeatedly:
        if serial_available() {
            ANGLE_INPUT = serial_parse_float();
            if ANGLE_INPUT != 0.0 {
                ANGLE_TARGET = ANGLE_INPUT.clamp(-ANGLE_MAX, ANGLE_MAX); // this allows to update the input using the serial mon.
            }
        }

        let elapsed = last_time.elapsed().as_millis();

        // Calculate RPM
        if elapsed >= 500 {
            let value = analog_read(POT);
            ANGLE_CURRENT = map(value, 0, 1023, -ANGLE_MAX as i32, ANGLE_MAX as i32) as f32; // Map out the angle to (-39 and 39 degrees the max the pot will go)
            ANGLE_CURRENT = (ANGLE_CURRENT * 10.0).round() / 10.0;

            let pwm_target = ((ANGLE_TARGET / ANGLE_MAX) * 255.0) as i32; // feed the PID a value between 0-255, it converts the target velocity from m/s to target pwn val.
            let pwm_current = ((ANGLE_CURRENT / ANGLE_MAX) * 255.0) as i32; // PWM
            ERROR = (pwm_target - pwm_current) as f32;
            CORRECTION = (ERROR * KP) as i32;
            PWM = pwm_current + CORRECTION;
            PWM = PWM.clamp(0, 255);

            print!("{}", ANGLE_TARGET);
            print!(" Goal: ");

            print!("{}", ANGLE_CURRENT);
            print!(" Current: ");

            print!("{}", CORRECTION);
            print!(" err: ");

            println!("{}", ANGLE_CURRENT);
            print!(" new:  ");

            //pulseCount = 0; // Reset pulse count
            last_time = Instant::now(); // Update time
            analog_write(MOTOR_F, PWM as u8);
            analog_write(MOTOR_B, (-(PWM as i16)) as u8);
        }

        //Serial.print(pwm);

        //  Serial.print(" ");

        // Serial.print(hold);

        // Serial.print(" ");

        //Serial.print(angle_target);
        //Serial.print("New_t ");
    }}
}

enum PinMode { Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn analog_write(_pin: u8, _val: u8) {}
fn analog_read(_pin: u8) -> i32 { 0 }
fn serial_available() -> bool { false }
fn serial_parse_float() -> f32 { 0.0 }
fn map(val: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
