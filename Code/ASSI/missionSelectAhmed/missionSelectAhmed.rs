/*
  ahmed has trouble running missionSelect on his linux machine the way I have the code organised across 3 bitesized files
  So I am putting it all together for him
//*/

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

//Primary global variables

static mut FIRST_BLINK: bool = true;
static mut PREV_T: f64 = 0.0;
static mut CURR_T: f64 = 0.0;

static mut FIRST_BLINK2: bool = true;
static mut PREV_T2: f64 = 0.0;
static mut CURR_T2: f64 = 0.0;

//************************************

const LED_PIN_MAX: i32 = 12;
const LED_PIN_MIN: i32 = 6;  //pin range of LED array mission select

static MODE: AtomicI32 = AtomicI32::new(LED_PIN_MIN);
static mut PREV_MODE: i32 = LED_PIN_MIN;

const CYCLE_PIN: u8 = 2;
const SELECT_PIN: u8 = 3;

static SELECTED: AtomicBool = AtomicBool::new(false);
static INTERRUPTED: AtomicBool = AtomicBool::new(false);

//************************************

// ASSI specific Global variables

static mut AS_OFF: bool = false;
static mut AS_READY: bool = false;
static mut AS_DRIVING: bool = false;
static mut AS_FINISHED: bool = false;
static mut AS_EMERGENCY: bool = false;
static mut MANUAL_D: bool = false;

static mut NOT_READY: bool = false;  // this means timer elapsed without entering go.

static mut EBS: bool = false;
static mut DONE: bool = false;
static mut MOVING: bool = false;
static mut GO: bool = false;  //Serial variables: Emergency Break system, Mission done and currently moving

static mut YELLOW: u8 = 0;
static mut BLUE: u8 = 0; //yellow and blue being the colors of the corresponding LEDs
static mut INITIAL_T: f64 = 0.0;

//************************************

static mut RECEIVED: i32 = 0; // for serial communication

//************************************

fn setup() {
    assi_setup();
    //initialise all the pins needed for the ASSI LEDs and buttons

    for i in LED_PIN_MIN..LED_PIN_MAX {
        pin_mode(i as u8, PinMode::Output);
    }  //set pinmode for missionSelect LED range

    pin_mode(CYCLE_PIN, PinMode::Input);
    pin_mode(SELECT_PIN, PinMode::Input);

    attach_interrupt(CYCLE_PIN, cycle_button);
    attach_interrupt(SELECT_PIN, select_button);
}

//************************************ Interrupt functions

fn cycle_button() {
    if !INTERRUPTED.load(Ordering::SeqCst) && !SELECTED.load(Ordering::SeqCst) {  //this was set as while for some reason, I see no reason for this and dont remember it having caused an issue before so now using IF instead
        MODE.fetch_add(1, Ordering::SeqCst);
        INTERRUPTED.store(true, Ordering::SeqCst);
    }
}  // code that cycles mode when appropriate button pressed

fn select_button() {
    SELECTED.store(true, Ordering::SeqCst);
}

//************************************

fn main() {
    setup();

    loop {
        let mode = MODE.load(Ordering::SeqCst);
        let selected = SELECTED.load(Ordering::SeqCst);

        if !selected {  // if no mode has been selected yet
            blink(mode);    //blink the LED corresponding to the current mode being conidered
            //Serial.println(mode);
        } else {
            digital_write(mode as u8, true);  //display chosen mode
        }

        if mode > LED_PIN_MAX {
            MODE.store(LED_PIN_MIN, Ordering::SeqCst);
        }  //loop back around if we have cycled out of bounds

        unsafe {
            if PREV_MODE > LED_PIN_MAX {
                PREV_MODE = LED_PIN_MIN;
                INTERRUPTED.store(false, Ordering::SeqCst);
            }

            if mode != PREV_MODE {
                digital_write(PREV_MODE as u8, false);
                PREV_MODE = mode;
                FIRST_BLINK = true;
                delay(250);
                INTERRUPTED.store(false, Ordering::SeqCst);
            }
        }

        if selected {
            check_serial();
        }

        assi();
    }
}

//************************************ blink functions

fn blink(pin: i32) {
    unsafe {
        CURR_T = millis() as f64 / 1000.0;  //get time in integer seconds

        if FIRST_BLINK {           //if we only just started blinking
            digital_write(pin as u8, true);  //on
            PREV_T = CURR_T;            //save current time

            FIRST_BLINK = false;                //set false
        } else if (CURR_T - PREV_T) > 0.5 {  //if firstblink is false and the current time is 2 seconds greater than previous time
            digital_write(pin as u8, false);            //set off

            if (CURR_T - PREV_T) > 1.0 {  //delay another 2 seconds before changing states
                FIRST_BLINK = true;
            }
        }
    }
}

fn blink2(pin: i32) {
    unsafe {
        CURR_T2 = millis() as f64 / 1000.0;  //get time in integer seconds

        if FIRST_BLINK2 {          //if we only just started blinking
            digital_write(pin as u8, true);  //on
            PREV_T2 = CURR_T2;          //save current time

            FIRST_BLINK2 = false;                 //set false
        } else if (CURR_T2 - PREV_T2) > 0.5 {  //if firstblink is false and the current time is 2 seconds greater than previous time
            digital_write(pin as u8, false);              //set off

            if (CURR_T2 - PREV_T2) > 1.0 {  //delay another 2 seconds before changing states
                FIRST_BLINK2 = true;
            }
        }
    }
}

//************************************ ASSI Specific functions

fn assi_setup() {
    //initalise default/starting states
    unsafe {
        AS_OFF = false;
        AS_READY = false;
        AS_DRIVING = false;
        AS_FINISHED = false;
        AS_EMERGENCY = false;
        MANUAL_D = false;
        NOT_READY = false;

        //intilise pins

        YELLOW = 4;
        BLUE = 5;
        pin_mode(YELLOW, PinMode::Output);
        pin_mode(BLUE, PinMode::Output);
    }
}//run this function in setup to ready everything needed for the ASSI function

// Set the ASSI LEDs according to global booleans
fn assi_led() {
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
            blink2(YELLOW as i32);
            digital_write(BLUE, false);
        }

        if AS_FINISHED {
            digital_write(YELLOW, false);
            digital_write(BLUE, true);
        }

        if AS_EMERGENCY {
            blink2(BLUE as i32);
            digital_write(YELLOW, false);
        }

        if MANUAL_D {
            digital_write(YELLOW, true);
            digital_write(BLUE, true);
        }
    }
}

fn assi() {
    unsafe {
        let timer: f64;  //current T for asReady Timer
        AS_OFF = false;

        if AS_EMERGENCY == false {

            if EBS {            //if EBS is engaged
                AS_DRIVING = false;  //couldnt possibly be driving if break is pulled
                AS_READY = false;
                if !MOVING && DONE {
                    AS_FINISHED = true;
                } else {
                    AS_EMERGENCY = true;
                }
            } else {  // if EBS isnt engaged check if mission has been set

                if SELECTED.load(Ordering::SeqCst) && !NOT_READY {

                    if !AS_READY {
                        INITIAL_T = millis() as f64 / 1000.0;  //start the timer for asReady
                    }

                    AS_READY = true;

                } else {
                    AS_READY = false;
                    AS_OFF = true;
                }
            }

            if AS_READY && !AS_DRIVING {
                let timer = millis() as f64 / 1000.0 - INITIAL_T;  //update timer

                if (timer >= 5.0) && (timer < 30.0) && GO {
                    AS_READY = false;
                    AS_DRIVING = true;
                }

                if timer > 30.0 {
                    AS_EMERGENCY = true;
                    AS_READY = false;
                }
            }
        }
        send_mode();
        assi_led();  //set LEDs based on booleans
    }
}

fn report_as() {
    //TODO: send the AS status over serial after the state machine has decided it.
}

//************************************ Serial Specific functions

fn check_serial() {
    unsafe {
        if serial_available() {
            RECEIVED = serial_read() as i32;  // Serial.read() retruns a char NOT an int. so storing it this way wont store what was entered but instead the ASCII of what was entered.

            //https://theasciicode.com.ar/ascii-printable-characters/capital-letter-a-uppercase-ascii-code-65.html
            //this link displays what value which chars are stored as

            if RECEIVED != 10 {  //10 corresponds to the Enter Key which needs to be ignored
                if 65 <= RECEIVED && RECEIVED <= 70 { //if we got something between A and F (HEX INPUT)
                    RECEIVED = RECEIVED - 55;
                } else {
                    RECEIVED = RECEIVED - 48;  //the ascii for 0 is 48
                }
                println!("{}", RECEIVED);
                println!("---");
                //sendMode();
                decode(RECEIVED);
            }
        }
    }
}

fn decode(val: i32) {
    /*
      val is a decimal integer recieved over serial
      The 4bit binary of val represents the desired state for EBS, done, moving & go;
      we need to convert val to binary and then set the booleans to match those ones and zeros
      we assume that the value fits 4 bit binary
    */

    //I cant remember what these are meant to look like for testing but I dont think it's working

    unsafe {
        GO = (val & 0b0001) != 0;             // LSB (bit 0)
        MOVING = ((val >> 1) & 0b0001) != 0;  // bit 1
        DONE = ((val >> 2) & 0b0001) != 0;    // bit 2
        EBS = ((val >> 3) & 0b0001) != 0;     // MSB (bit 3)
    }
}  //set serial booleans according to integer recieved over serial

fn send_mode() {
    //bool asOff, asReady, asDriving, asFinished, asEmergency, manualD;
    unsafe {
        if AS_OFF {
            send("O");
        } else if AS_READY {
            send("R");
        } else if AS_DRIVING {
            send("D");
        } else if AS_FINISHED {
            send("E");
        }
    }
}

fn send(temp: &str) {
    println!("{}", temp);
}

enum PinMode { Output, Input }
fn pin_mode(_pin: u8, _mode: PinMode) {}
fn digital_write(_pin: u8, _val: bool) {}
fn serial_available() -> bool { false }
fn serial_read() -> u8 { 0 }
fn millis() -> u64 { 0 }
fn delay(_ms: u32) {}
fn attach_interrupt(_pin: u8, _handler: fn()) {}
