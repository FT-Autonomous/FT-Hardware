// Useful links:
// https://forum.arduino.cc/t/serial-input-basics-updated/382007

#[allow(dead_code)]
static mut EBS_ACTIVE: bool = false;
#[allow(dead_code)]
static mut MISSION_SELECTED: bool = false;
#[allow(dead_code)]
static mut ASMS_ACTIVE: bool = false;
#[allow(dead_code)]
static mut ASB_OK: bool = false;
#[allow(dead_code)]
static mut TS_ACTIVE: bool = false;
#[allow(dead_code)]
static mut R2D: bool = false;
#[allow(dead_code)]
static mut BRAKES_ENGAGED: bool = false;
#[allow(dead_code)]
static mut MISSION_FINISHED: bool = false;
#[allow(dead_code)]
static mut VEHICLE_STANDSTILL: bool = false;
#[allow(dead_code)]
static mut SDC_OPEN: bool = false;

const NUM_CHARS: usize = 32;
static mut RECIEVED_CHARS: [u8; NUM_CHARS] = [0; NUM_CHARS];

static mut NEW_DATA: bool = false;

fn setup() {
    println!("<Arduino is ready>");
}

fn recv_with_start_end_markers() {
    static mut RECV_IN_PROGRESS: bool = false;
    static mut NDX: usize = 0;
    let start_marker = b'<';
    let end_marker = b'>';

    unsafe {
        while RECV_IN_PROGRESS == true {
            let rc = serial_read();
            if rc != end_marker {
                RECIEVED_CHARS[NDX] = rc;
                NDX += 1;
                if NDX >= NUM_CHARS {
                    NDX = NUM_CHARS - 1;
                }
            } else {
                RECIEVED_CHARS[NDX] = b'\0'; // terminate the string
                RECV_IN_PROGRESS = false;
                NDX = 0;
                NEW_DATA = true;
            }
        }

        let rc = serial_read();
        if rc == start_marker {
            RECV_IN_PROGRESS = true;
        }
    }
}

fn show_new_data() {
    unsafe {
        if NEW_DATA == true {
            print!("This just in...");
            let len = RECIEVED_CHARS.iter().position(|&c| c == 0).unwrap_or(NUM_CHARS);
            println!("{}", String::from_utf8_lossy(&RECIEVED_CHARS[..len]));
            NEW_DATA = false;
        }
    }
}

fn main() {
    setup();
    loop {
        recv_with_start_end_markers();
        show_new_data();
    }
}

fn serial_read() -> u8 { 0 }
