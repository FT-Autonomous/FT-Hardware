mod ft_serial;
use ft_serial::{FTSerial, SerialPort};
use std::time::Instant;

struct HwSerial;
impl SerialPort for HwSerial {
    fn available(&self) -> i32 { 0 }
    fn read(&mut self) -> u8 { 0 }
}

//Hard limits (to not break mechanically, only change if range is tested/safe)
const ANGLE_LIMIT_DEG: f32 = 80.0;
const STOP_BAND_DEG: f32 = 1.0; //How close is it to required angle before stopping (tolerance)

  //make sure adc left and right are never the same, make left < right at all times (avoids dividing by 0 later) - should be solved when actual values are added anyways
const POT_ADC_LEFT: i32 = 150; //CHANGE THIS, TEST IT BEFORE RUNNING CODE, PUT AT LEFT EXTREME AND PUT THAT ANGLE FROM POTENTIOMETER IN HERE/////////////////////
const POT_ADC_RIGHT: i32 = 870; //CHAN30GE THIS, TEST IT BEFORE RUNNING CODE, PUT AT RIGHT EXTREME AND PUT THAT ANGLE FROM POTENTIOMETER IN HERE/////////////////////

const GAIN: f32 = 3.0; //proportional gain for how far out from PWM value (error)
const PWM_MIN: i32 = 35;
const PWM_MAX: i32 = 200;

const ANGLE_SMOOTHING: f32 = 0.20; //change between 0.000...1 and 1, 1 being no smoothing as heading towards angle. (it will not move if 0)

//Limits in testing
const HOLD_LAST_VALUE: bool = true;
const SERIAL_TIMEOUT_MS: u128 = 1500;

//Pins
//these pins should be right. if the steering is turning the wrong way flip the wires or change the pwm pins here ///////////////////////////
const POT_PIN: u8 = 3; // A3

const RPWM_PIN: u8 = 5;
const LPWM_PIN: u8 = 6;
const REN_PIN: u8 = 7;
const LEN_PIN: u8 = 8;

//Starting state
static mut TARGET_DEG: f32 = 0.0;
static mut ANGLE_DEG_FILTERED: f32 = 0.0;

fn direction_check(x: f32, a: f32, b: f32) -> f32 {
    if x < a { return a; }
    if x > b { return b; }
    x
}

fn map_pot_to_deg(adc: i32) -> f32 {
    let adc = adc.clamp(
        POT_ADC_LEFT.min(POT_ADC_RIGHT),
        POT_ADC_LEFT.max(POT_ADC_RIGHT),
    ); //limits input angles to valid/safe range

    let t = (adc - POT_ADC_LEFT) as f32 / (POT_ADC_RIGHT - POT_ADC_LEFT) as f32; //make sure left < right when calibrating (to make sure this is positive)
    let deg = (-ANGLE_LIMIT_DEG) + t * (2.0 * ANGLE_LIMIT_DEG);

    deg
}

fn driver_enable(en: bool) {
    digital_write(REN_PIN, en);
    digital_write(LEN_PIN, en);
}

fn coast_stop() {
    //idk if this is really necessary, I noticed the wheel continued moving a bit when there was power but no signal going to it... (19/02/2026)
    //also being used as the general stop moving command
    analog_write(RPWM_PIN, 0);
    analog_write(LPWM_PIN, 0);
}

fn drive(pwm_signed: i32) {
    // > 0 steer right, <0 steer left
    let mut pwm = pwm_signed.abs();
    pwm = pwm.clamp(0, 255);

    if pwm_signed > 0 {
        analog_write(RPWM_PIN, pwm as u8);
        analog_write(LPWM_PIN, 0);
    } else if pwm_signed < 0 {
        analog_write(RPWM_PIN, 0);
        analog_write(LPWM_PIN, pwm as u8);
    } else {
        coast_stop();
    }
}

fn main() {
    //maybe we can make this lower, Ahmed had 9500 but I wasn't sure if that was arbitrary
    // re above: higher baudrate = faster time to react to commands, i don't see a problem with this

    pin_mode(RPWM_PIN, PinMode::Output); //in testing we generally just kept these hooked up to 5V
    pin_mode(LPWM_PIN, PinMode::Output); //keeping as may be useful for emergency stop in future
    pin_mode(REN_PIN, PinMode::Output);
    pin_mode(LEN_PIN, PinMode::Output);

    driver_enable(true);
    coast_stop();

    //initialise the filtered angle to current reading
    let adc = analog_read(POT_PIN);
    unsafe { ANGLE_DEG_FILTERED = map_pot_to_deg(adc); }

    unsafe { TARGET_DEG = 0.0; }
    let mut last_cmd = Instant::now();

    println!("Steering controller ready");

    let mut ft_serial = FTSerial::new(HwSerial, 24);
    let mut last_print = Instant::now();

    loop {
        //Update target from serial
        if let Some(cmd) = ft_serial.read_float() {
            unsafe {
                TARGET_DEG = direction_check(cmd, -ANGLE_LIMIT_DEG, ANGLE_LIMIT_DEG);
                last_cmd = Instant::now();

                print!("Target: ");
                println!("{:.2}", TARGET_DEG);
            }
        }

        //If it times out, choose behaviour (timeout behaviour is not receiving input)
        if !HOLD_LAST_VALUE {
            if last_cmd.elapsed().as_millis() > SERIAL_TIMEOUT_MS {
                unsafe { TARGET_DEG = 0.0; }
            }
        }

        //Read current angle from pot and filter it
        let adc = analog_read(POT_PIN);
        let angle_now = map_pot_to_deg(adc);
        unsafe {
            ANGLE_DEG_FILTERED = (1.0 - ANGLE_SMOOTHING) * ANGLE_DEG_FILTERED + ANGLE_SMOOTHING * angle_now;

            //compute the error (distance needed to travel)
            let err = TARGET_DEG - ANGLE_DEG_FILTERED;

            //control
            if err.abs() <= STOP_BAND_DEG {
                coast_stop();
            } else {
                //proportional speed
                let pwm_float = GAIN * err.abs();
                let mut pwm = pwm_float as i32;

                pwm = pwm.clamp(0, PWM_MAX);
                if pwm < PWM_MIN { pwm = PWM_MIN; }

                let pwm_signed = if err > 0.0 { pwm } else { -pwm }; //going left or right? neg or positive change check
                drive(pwm_signed);
            }

            if last_print.elapsed().as_millis() > 50 {
                last_print = Instant::now();
                print!("angle= ");
                print!("{:.2}", ANGLE_DEG_FILTERED);
                print!(" target= ");
                print!("{:.2}", TARGET_DEG);
                print!(" error= ");
                println!("{:.2}", err);
            }
        }
    }
}

enum PinMode { Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn digital_write(_pin: u8, _val: bool) {}
fn analog_write(_pin: u8, _val: u8) {}
fn analog_read(_pin: u8) -> i32 { 0 }
