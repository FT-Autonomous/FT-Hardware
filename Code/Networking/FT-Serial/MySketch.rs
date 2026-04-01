
// Links that helped: https://arduino.github.io/arduino-cli/0.32/getting-started/
// https://forum.arduino.cc/t/two-ways-communication-between-python3-and-arduino/1219738
// https://forum.arduino.cc/t/serial-input-basics-updated/382007

fn setup() {
    println!("Bruz");
}

// loop is very close to how the encoder PID serial is setup
// EDIT: not sure if this still applies post redoing serial
fn main() {
    setup();
    loop {
        if serial_available() {
            print_serial();
        }
    }
}

// this function mainly exists as proof of concept that we are no longer
// limited to single char messages to the arduino and to be easily transferable
fn read_serial_with_start_end_markers() -> String {
    static mut READING: bool = false;
    static mut NDX: usize = 0;

    let start_marker = b'<';
    let end_marker = b'>';

    const NUM_CHARS: usize = 32;
    let mut received_characters = [0u8; NUM_CHARS];
    let mut new_data = false;

    while serial_available() && !new_data {
        let read_character = serial_read_byte();

        unsafe {
            if READING {
                if read_character != end_marker {
                    received_characters[NDX] = read_character;
                    NDX += 1;
                    if NDX >= NUM_CHARS {
                        NDX = NUM_CHARS - 1;
                    }
                } else {
                    received_characters[NDX] = b'\0';
                    READING = false;
                    NDX = 0;
                    new_data = true;
                }
            } else if read_character == start_marker {
                READING = true;
            }
        }
    }

    if new_data {
        let len = received_characters.iter().position(|&c| c == 0).unwrap_or(NUM_CHARS);
        String::from_utf8_lossy(&received_characters[..len]).to_string()
    } else {
        String::new()
    }
}

fn read_serial_until_newline() -> String { // type your value and press enter
    static mut NDX: usize = 0;
    static mut RECEIVED_CHARACTERS: [u8; 32] = [0; 32];
    const NUM_CHARS: usize = 32;

    while serial_available() {
        let read_character = serial_read_byte();

        unsafe {
            if read_character == b'\n' {
                RECEIVED_CHARACTERS[NDX] = b'\0';
                let result = String::from_utf8_lossy(&RECEIVED_CHARACTERS[..NDX]).to_string();
                NDX = 0;
                return result;
            } else if read_character != b'\r' {
                RECEIVED_CHARACTERS[NDX] = read_character;
                NDX += 1;
                if NDX >= NUM_CHARS {
                    NDX = NUM_CHARS - 1;
                }
            }
        }
    }

    String::new()
}

fn print_serial() {
    let serial_string = read_serial_until_newline();
    if serial_string != "" {
        print!("received: ");
        println!("{}", serial_string);
    }
}

fn serial_available() -> bool { false }
fn serial_read_byte() -> u8 { 0 }
