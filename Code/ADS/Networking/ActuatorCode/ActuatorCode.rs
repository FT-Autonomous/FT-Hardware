use std::sync::atomic::{AtomicI64, Ordering};
use std::thread;
use std::time::Duration;

const SSID: &str = "Test";
const PASS: &str = "wordpass";

static mut PAUSED: bool = true;
static mut CYCLE_TEST: bool = false;

const PIN_A: u8 = 10;
const PIN_B: u8 = 11;

const HALL_PIN_R: u8 = 3;
static COUNT_R: AtomicI64 = AtomicI64::new(0);
static mut COUNT: i64 = 0;

fn right_counter_interrupt() {
    COUNT_R.fetch_add(1, Ordering::SeqCst);
}

static mut ST: i32 = 2000; // delay
static mut A_VAL: i32 = 100;
static mut B_VAL: i32 = 100;

fn a(state: i32) {
    let state = (255 * state / 10) as u8;
    analog_write(PIN_A, state);
}//set A to state percent power

fn b(state: i32) {
    let state = (255 * state / 10) as u8;
    analog_write(PIN_B, state);
}

fn hall_update() {
    unsafe {
        let cr = COUNT_R.load(Ordering::SeqCst);
        if COUNT != cr {
            println!("{}", cr);
            COUNT = cr;
            send_update();
        }
    }
}

// ---- WiFi Networking ----

fn wifi_setup() {
    wifi_begin(SSID, PASS);
    let ip = wifi_local_ip();  //get the IP
    let mut vip = wifi_local_ip();  //get the IP
    println!("{}", ip);

    while ip == vip {
        wifi_begin(SSID, PASS);
        vip = wifi_local_ip();  //get the IP
        println!("{}", vip);
        if vip != ip {
            break;
        }
    }

    server_begin();
}

fn wifi_loop() {
    while let Some((data, client)) = server_accept() {
        command(data, client);
    }
}

fn command(data: char, mut client: WifiClient) {
    unsafe {
        if data == 'p' {
            PAUSED = !PAUSED;
        } else if data == 's' {
            //prepare to receive 3 value updates: A B and t
            A_VAL = get_int(&mut client);
            print!("aVal set to: ");
            println!("{}", A_VAL);

            B_VAL = get_int(&mut client);
            print!("bVal set to: ");
            println!("{}", B_VAL);

            ST = get_int(&mut client);
            print!("delay set to: ");
            ST = ST * 1000;
            println!("{}", ST);
        } else if data == 'r' {
            COUNT = 0;  //set counter to 0
            COUNT_R.store(0, Ordering::SeqCst);
        } else if data == 'l' {
            CYCLE_TEST = !CYCLE_TEST;
        } else if data == 'a' {
            //set aVal to incoming integer
            A_VAL = get_int(&mut client);
            print!("aVal set to: ");
            println!("{}", A_VAL);
            CYCLE_TEST = false;
        } else if data == 'b' {
            //set bVal to incoming integer
            B_VAL = get_int(&mut client);
            print!("bVal set to: ");
            println!("{}", B_VAL);
            CYCLE_TEST = false;
        }
    }
}

fn get_int(client: &mut WifiClient) -> i32 {
    let mut temp: i32 = -1;
    while temp == -1 {
        temp = client.read();
    }
    temp
}

fn send_update() {
    unsafe {
        if let Some(mut client) = server_available() {
            if client.connected() {
                client.write_byte(b'c');
                client.write_int(COUNT as i32);
            }
        }
    }
}

// ---- Main ----

fn setup() {
    // put your setup code here, to run once:
    println!("----------------------");
    pin_mode(PIN_A, PinMode::Output);
    pin_mode(PIN_B, PinMode::Output);

    pin_mode(HALL_PIN_R, PinMode::InputPullup);
    attach_interrupt(HALL_PIN_R, right_counter_interrupt);  //rightEncoderInterrupt will run when the pin CHANGES VALUE
    wifi_setup();
}

fn main() {
    setup();
    loop {
        // put your main code here, to run repeatedly:
        wifi_loop();

        unsafe {
            if !PAUSED {
                a(0);
                b(0);
                println!("both low");

                hall_update();
                thread::sleep(Duration::from_millis(ST as u64));

                a(A_VAL);
                println!("A high: Extend");

                hall_update();
                thread::sleep(Duration::from_millis(ST as u64));

                a(0);
                println!("both low");

                hall_update();
                thread::sleep(Duration::from_millis(ST as u64));

                b(B_VAL);
                println!("b high: Retract");

                hall_update();
                thread::sleep(Duration::from_millis(ST as u64));

                println!("----------------------");
            }
        }
    }
}

struct WifiClient;
impl WifiClient {
    fn read(&mut self) -> i32 { -1 }
    fn connected(&self) -> bool { false }
    fn write_byte(&mut self, _b: u8) {}
    fn write_int(&mut self, _val: i32) {}
}
enum PinMode { Output, InputPullup }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn analog_write(_pin: u8, _val: u8) {}
fn wifi_begin(_ssid: &str, _pass: &str) {}
fn wifi_local_ip() -> String { String::from("0.0.0.0") }
fn server_begin() {}
fn server_accept() -> Option<(char, WifiClient)> { None }
fn server_available() -> Option<WifiClient> { None }
fn attach_interrupt(_pin: u8, _handler: fn()) {}
