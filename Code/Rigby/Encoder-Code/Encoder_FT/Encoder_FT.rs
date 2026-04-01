//#include "Arduino.h"

//ArduPID C1;

const ENCODER_A: u8 = 2; // Channel A pin
const ENCODER_B: u8 = 3; // Channel B pin

const MOTOR_F: u8 = 10; // For motor stuff
const MOTOR_B: u8 = 11;

//double setpoint = 0.1
//double input;
//double output;

use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Instant;

static PULSE_COUNT: AtomicI32 = AtomicI32::new(0); // Number of pulses (Volatile int to load var from RAM and not overload the compiler as the encoder speed and direction change is based on the command driving it)
const PULSES_PER_REVOLUTION: f32 = 600.0; // Based on the encoder PPR

static mut V_TARGET: f32 = 0.0;
static mut V_CURRENT: f32 = 0.0;
static mut ERROR: f32 = 0.0;  //error is dynamic
const KP: f32 = 1.0; // kp val @ 0.5 works solo for now
const KI: f32 = 0.0;
const KD: f32 = 0.0;

static mut PWM: i32 = 0;
static mut CORRECTION: i32 = 0;
const V_MAX: f32 = 0.8; // the max speed m/s
static mut V_INPUT: f32 = 0.0;

static mut HOLD: bool = false;

fn encoder_isr() {
    // Increment pulse count on every rising edge of Channel A
    if digital_read(ENCODER_A) == true {
        if digital_read(ENCODER_B) == false {
            PULSE_COUNT.fetch_add(1, Ordering::SeqCst); // GO CLOCKWISE
        } else {
            PULSE_COUNT.fetch_add(-1, Ordering::SeqCst); // GO ANTICLOCKWISE (Could change since the wheel placment would mean the shaft goes ANTICLOCKWISE)
        }
    }
}

fn setup() {
    // Set up encoder pins
    pin_mode(ENCODER_A, PinMode::InputPullup);
    pin_mode(ENCODER_B, PinMode::InputPullup); //encoder pins

    pin_mode(MOTOR_F, PinMode::Output);
    pin_mode(MOTOR_B, PinMode::Output);  // motor stuff

    //c1.begin(&input, &output, &setpoint, p, i ,d);

    // Attach interrupt for encoder
    attach_interrupt(ENCODER_A, encoder_isr);
}

fn main() {
    setup();
    let mut last_time = Instant::now();

    loop {
        unsafe {
            if serial_available() {
                V_INPUT = read_float_from_serial();
                if V_INPUT != 0.0 {
                    V_TARGET = V_INPUT; // this allows to update the input using the serial mon.
                }
            }

            let elapsed = last_time.elapsed().as_millis();

            // Calculate RPM
            if elapsed >= 500 {
                let pulse_count = PULSE_COUNT.swap(0, Ordering::SeqCst);
                let rpm = (pulse_count as f32 / PULSES_PER_REVOLUTION) * 60.0;

                let ang_velocity = rpm * std::f32::consts::PI / 60.0;
                let linear_velocity = ang_velocity * 0.3; // LV = anugular velocity * radius (diameter is 6mm so radius is 0.3mm)
                V_CURRENT = (linear_velocity * 10.0).round() / 10.0;

                //Serial.print("RPM: ");
                //Serial.println(rpm);

                //.print(" AV: ");
                //Serial.println(Ang_velocity);

                //Serial.print("LV: ");
                //Serial.print(v_current);

                //Serial.println(v_target);

                let pwm_target = ((V_TARGET / V_MAX) * 255.0) as i32; // feed the PID a value between 0-255, it converts the target velocity from m/s to target pwn val.
                let pwm_current = ((V_CURRENT / V_MAX) * 255.0) as i32; // PWM
                ERROR = (pwm_target - pwm_current) as f32;
                CORRECTION = (ERROR * KP) as i32;
                PWM = pwm_current + CORRECTION;
                PWM = PWM.clamp(0, 255);

                print!("{} ", pwm_target);
                print!("{} ", pwm_current);
                print!("{} ", CORRECTION);
                print!("{} ", PWM);
                print!("{} ", HOLD);
                print!("{} ", V_TARGET);
                println!("{}", V_CURRENT);

                last_time = Instant::now(); // Update time
            }

            analog_write(MOTOR_F, PWM as u8);
            analog_write(MOTOR_B, 0); // Drive forward.
        }
    }
}

fn read_serial_until_newline() -> String { // type your value and press Enter
    let mut buf = [0u8; 32];
    let mut ndx: usize = 0;

    while serial_available() {
        let c = serial_read_byte();

        if c == b'\n' {
            let result = String::from_utf8_lossy(&buf[..ndx]).to_string();
            return result;
        } else if c != b'\r' {
            if ndx < 32 {
                buf[ndx] = c;
                ndx += 1;
            }
            if ndx >= 32 {
                ndx = 31;
            }
        }
    }

    String::new()
}

fn read_float_from_serial() -> f32 {
    let serial_input = read_serial_until_newline();
    if serial_input != "" {
        serial_input.parse::<f32>().unwrap_or(0.0)
    } else {
        0.0
    }
}

// to do:
// write a PID
// #1: current / max speed * 255


// serial write




// 0.8 m/s is the max speed @ 8V and 1.6 Amp

enum PinMode { Output, InputPullup }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn digital_read(_pin: u8) -> bool { false }
fn analog_write(_pin: u8, _val: u8) {}
fn serial_available() -> bool { false }
fn serial_read_byte() -> u8 { 0 }
fn attach_interrupt(_pin: u8, _handler: fn()) {}
