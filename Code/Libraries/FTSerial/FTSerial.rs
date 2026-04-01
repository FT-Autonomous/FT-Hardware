pub trait SerialPort {
    fn available(&self) -> i32;
    fn read(&mut self) -> u8;
}

pub struct FTSerial<S: SerialPort> {
    serial: S,
    buf: Vec<u8>,
    buf_size: u8,
    ndx: u8,
    reading: bool, // state for marker mode
}

impl<S: SerialPort> FTSerial<S> {
    pub fn new(serial: S, buffer_size: u8) -> Self {
        FTSerial {
            serial,
            buf: vec![0u8; buffer_size as usize],
            buf_size: buffer_size,
            ndx: 0,
            reading: false,
        }
    }

    // read until '\n' (ignores '\r'). returns "" if no complete line yet
    pub fn read_until_newline(&mut self) -> String {
        while self.serial.available() > 0 {
            let c = self.serial.read();

            if c == b'\n' {
                let result = String::from_utf8_lossy(&self.buf[..self.ndx as usize]).to_string();
                self.ndx = 0;
                return result;
            }

            if c != b'\r' {
                self.buf[self.ndx as usize] = c;
                self.ndx += 1;
                if self.ndx >= self.buf_size {
                    self.ndx = self.buf_size - 1;
                }
            }
        }
        String::new()
    }

    // read between start/end markers (default < >). returns "" if no complete message yet
    pub fn read_with_markers(&mut self, start_marker: u8, end_marker: u8) -> String {
        while self.serial.available() > 0 {
            let c = self.serial.read();

            if self.reading {
                if c != end_marker {
                    self.buf[self.ndx as usize] = c;
                    self.ndx += 1;
                    if self.ndx >= self.buf_size {
                        self.ndx = self.buf_size - 1;
                    }
                } else {
                    let result = String::from_utf8_lossy(&self.buf[..self.ndx as usize]).to_string();
                    self.reading = false;
                    self.ndx = 0;
                    return result;
                }
            } else if c == start_marker {
                self.reading = true;
                self.ndx = 0;
            }
        }
        String::new()
    }

    // reads a line via read_until_newline() and parses it as a float
    // returns true if a new value was received, and sets result to the parsed float
    pub fn read_float(&mut self) -> Option<f32> {
        let line = self.read_until_newline();
        if line.is_empty() {
            return None;
        }
        line.parse::<f32>().ok()
    }
}
