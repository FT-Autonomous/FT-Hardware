mod ft_serial;
use ft_serial::{FTSerial, SerialPort};

struct HwSerial;
impl SerialPort for HwSerial {
    fn available(&self) -> i32 { 0 }
    fn read(&mut self) -> u8 { 0 }
}

const MOTOR_L: u8 = 5;
const MOTOR_R: u8 = 6;

const HIGH: u8 = 255;
const LOW: u8 = 0;

fn steer(temp: char) {
    if temp == 'L' {  //Move left
        analog_write(MOTOR_L, HIGH);
        analog_write(MOTOR_R, LOW);
    } else if temp == 'R' {  //Move right
        analog_write(MOTOR_R, HIGH);
        analog_write(MOTOR_L, LOW);
    } else if temp == 's' {  //Stop if not explicitly left or right
        analog_write(MOTOR_L, LOW);
        analog_write(MOTOR_R, LOW);
    }
}

fn main() {
    //we could speed up if lagging? - 115200
    pin_mode(MOTOR_L, PinMode::Output);
    pin_mode(MOTOR_R, PinMode::Output);

    steer('s');  //stopped by default

    let mut ft_serial = FTSerial::new(HwSerial, 32);

    loop {
        let line = ft_serial.read_until_newline();
        if line.len() > 0 {
            let received = line.chars().next().unwrap();
            print!("Got: ");
            print!("{}", received);
            print!(" ASCII=");
            println!("{}", received as u32);
            steer(received);
        }
    }
}

enum PinMode { Output }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn analog_write(_pin: u8, _val: u8) {}
