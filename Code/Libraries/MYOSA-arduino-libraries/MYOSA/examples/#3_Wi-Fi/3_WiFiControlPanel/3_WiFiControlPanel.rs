/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  WiFi Control Panel
  Connection: Connect the "Actuator" board from the MYOSA kit with the "Controller" board power them up.
  Working: This example is intended to demonstrate the capabilities of WiFi (Controller) board. Here the controller board hosts to a  WebApp (viz a Control Panel for controlling the Actuator Board) on existing WiFi network. Hence, it lets users have a few controls to interact with the board from Mobile phone or any Digital Device through WebApp.

  Synopsis of MYOSA platform
  MYOSA Platform consists of a centralized motherboard a.k.a Controller board, 5 different sensor modules, an OLED display and an actuator board in the kit.
  Controller board is designed on ESP32 module. It is a low-power system on a chip microcontrollers with integrated Wi-Fi and Bluetooth.
  5 Sensors are as below,
  1 --> Accelerometer and Gyroscope (6-axis motion sensor)
  2 --> Temperature and Humidity Sensor
  3 --> Barometric Pressure Sensor
  4 --> Light, Proximity and Gesture Sensor
  5 --> Air Quality Sensor
  Actuator board contains a Buzzer and an AC switching circuit to turn on/off an electrical appliance.
  There is also an OLED display in the MYOSA kit.

  You can design N number of such utility examples as a part of your learning from this kit.

  Detailed Information about MYOSA platform and usage is provided in the link below.
  Detailed Guide: https://drive.google.com/file/d/1On6kzIq3ejcu9aMGr2ZB690NnFrXG2yO/view

  NOTE
  All information, including URL references, is subject to change without prior notice.
  Please always use the latest versions of software-release for best performance.
  Unless required by applicable law or agreed to in writing, this software is distributed on an
  "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied

  Modifications
  1 December, 2021 by Pegasus Automation
  (as a part of MYOSA Initiative)

  Contact Team MakeSense EduTech for any kind of feedback/issues pertaining to performance or any update request.
  Email: dev.myosa@gmail.com
*/

#![no_std]
#![no_main]

/// Hardware abstraction stubs for Serial, Wire (I2C), WiFi server, Actuator, and GPIO.
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn serial_print(_msg: &str) {}
    pub fn serial_write(_c: u8) {}
    pub fn wire_begin() {}
    pub fn wire_set_clock(_freq: u32) {}
    pub fn delay(_ms: u32) {}
    pub fn pin_mode(_pin: u8, _mode: u8) {}
    pub fn digital_write(_pin: u8, _value: bool) {}

    pub const OUTPUT: u8 = 1;
    pub const HIGH: bool = true;
    pub const LOW: bool = false;
    pub const WL_CONNECTED: u8 = 3;
    pub const BUZZER_IO: u8 = 0;
    pub const IO_OUTPUT: u8 = 1;
    pub const IO_HIGH: bool = true;
    pub const IO_LOW: bool = false;

    pub struct Actuator;

    impl Actuator {
        pub fn new() -> Self { Actuator }
        pub fn ping(&self) -> bool { true }
        pub fn set_mode(&self, _io: u8, _mode: u8) {}
        pub fn set_state(&self, _io: u8, _state: bool) {}
    }

    pub struct WiFi;

    impl WiFi {
        pub fn begin(_ssid: &str, _password: &str) {}
        pub fn status() -> u8 { WL_CONNECTED }
        pub fn local_ip() -> &'static str { "0.0.0.0" }
    }

    pub struct WiFiClient {
        _data: &'static [u8],
        _pos: usize,
    }

    impl WiFiClient {
        pub fn is_some(&self) -> bool { false }
        pub fn connected(&self) -> bool { false }
        pub fn available(&self) -> bool { false }
        pub fn read(&mut self) -> u8 { 0 }
        pub fn println(&self, _msg: &str) {}
        pub fn print(&self, _msg: &str) {}
        pub fn stop(&self) {}
    }

    pub struct WiFiServer {
        _port: u16,
    }

    impl WiFiServer {
        pub fn new(port: u16) -> Self { WiFiServer { _port: port } }
        pub fn begin(&self) {}
        pub fn available(&self) -> WiFiClient {
            WiFiClient { _data: &[], _pos: 0 }
        }
    }
}

use hal::*;

/* Enter your WiFi credentials here so that MYOSA Controller Board can connect to Internet. */
const SSID: &str = "MYOSAbyMakeSense";
const PASSWORD: &str = "LearnTheEasyWay";

/* Creating Object of Actuator class */
fn create_actuator() -> Actuator { Actuator::new() }

/* Creating an Object of the WiFiServer class */
fn create_server() -> WiFiServer { WiFiServer::new(80) }

/* Setup Function */
fn setup(gpio_expander: &Actuator, server: &WiFiServer) {

    /* Setting up the communication */
    serial_begin(115200);
    wire_begin();
    wire_set_clock(100000);

    /* Initializing Actuator Board */
    loop {
        if gpio_expander.ping() {
            serial_println("4bit IO Expander Actautor (PCA9536) is connected");
            break;
        }
        serial_println("4bit IO Expander Actuator (PCA9536) is disconnected");
        delay(500);
    }
    /* Set buzzer IO as output */
    gpio_expander.set_mode(BUZZER_IO, IO_OUTPUT);
    gpio_expander.set_state(BUZZER_IO, IO_LOW);

    pin_mode(2, OUTPUT);      // set the LED pin mode

    delay(1000);


    /* Connecting to WiFi Network */
    serial_println("");
    serial_println("");
    serial_print("Connecting to ");
    serial_println(SSID);

    WiFi::begin(SSID, PASSWORD);

    while WiFi::status() != WL_CONNECTED {
        delay(500);
        serial_print(".");
    }

    serial_println("");
    serial_println("WiFi connected.");
    serial_println("IP address: ");
    serial_println(WiFi::local_ip());

    server.begin();
}

/* Global Constants */
static mut CLIENT_PRINTING: bool = false;

/* Loop Function */
fn main_loop(gpio_expander: &Actuator, server: &WiFiServer) {

    /* Loop Function constantly check the availability of commands from the clients and takes desired action */
    let mut client = server.available();   // listen for incoming clients

    if client.is_some() {
        serial_println("New Client.");           // print a message out the serial port
        let mut current_line = [0u8; 256];       // buffer to hold incoming data from the client
        let mut line_len: usize = 0;
        while client.connected() {            // loop while the client's connected
            if client.available() {             // if there's bytes to read from the client,
                let c = client.read();             // read a byte, then
                unsafe {
                    if CLIENT_PRINTING {
                        serial_write(c);                    // print it out the serial monitor
                    }
                }
                if c == b'\n' {                    // if the byte is a newline character

                    // if the current line is blank, you got two newline characters in a row.
                    // that's the end of the client HTTP request, so send a response:
                    if line_len == 0 {
                        // HTTP headers always start with a response code (e.g. HTTP/1.1 200 OK)
                        // and a content-type so the client knows what's coming, then a blank line:
                        client.println("HTTP/1.1 200 OK");
                        client.println("Content-type:text/html");
                        client.println("");

                        // the content of the HTTP response follows the header:
                        client.println("<p style=\"font-size: 50px;text-align:center\">Welcome to the MYSOA Control Room</p>");

                        client.println("\n\n<p style=\"font-size: 40px;text-align:center\">LED Control </p>\n\n");
                        client.println("");
                        client.println("");
                        client.print(" <form action=\"H\" method=\"get\">  <p align=\"center\"> <button type=\"submit\" style=\"font-size:30px;height:200px;width:200px\" >Turn On</button> <button type=\"submit\" style=\"font-size:30px;height:200px;width:200px\" formaction=\"L\">Turn Off</button></form> </p>");

                        client.println("");
                        client.println("\n\n");
                        client.println("\n\n<p style=\"font-size: 40px;text-align:center\">Buzzer Control </p>\n\n");
                        client.println("");
                        client.println("");
                        client.print(" <form action=\"X\" method=\"get\">  <p align=\"center\"> <button type=\"submit\" style=\"font-size:30px;height:200px;width:200px\" >Turn On</button> <button type=\"submit\" style=\"font-size:30px;height:200px;width:200px\" formaction=\"Y\">Turn Off</button></form> </p>");

                        // The HTTP response ends with another blank line:
                        client.println("");
                        // break out of the while loop:
                        break;
                    } else {    // if you got a newline, then clear currentLine:
                        line_len = 0;
                    }
                } else if c != b'\r' {  // if you got anything else but a carriage return character,
                    if line_len < current_line.len() {
                        current_line[line_len] = c;      // add it to the end of the currentLine
                        line_len += 1;
                    }
                }

                // Check to see if the client request was "GET /H" or "GET /L":
                if line_len >= 6 && &current_line[line_len - 6..line_len] == b"GET /H" {
                    serial_println("Requested LED to turn ON!");
                    digital_write(2, HIGH);               // GET /H turns the LED on
                }
                if line_len >= 6 && &current_line[line_len - 6..line_len] == b"GET /L" {
                    serial_println("Requested LED to turn OFF!");
                    digital_write(2, LOW);                // GET /L turns the LED off
                }

                // Check to see if the client request was "GET /X" or "GET /Y":
                if line_len >= 6 && &current_line[line_len - 6..line_len] == b"GET /X" {
                    serial_println("Requested Buzzer to turn ON!");
                    gpio_expander.set_state(BUZZER_IO, IO_HIGH);
                }
                if line_len >= 6 && &current_line[line_len - 6..line_len] == b"GET /Y" {
                    serial_println("Requested Buzzer to turn OFF!");
                    gpio_expander.set_state(BUZZER_IO, IO_LOW);
                }
            }
        }
        // close the connection:
        client.stop();
        serial_println("Client Disconnected.");
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let gpio_expander = create_actuator();
    let server = create_server();
    setup(&gpio_expander, &server);
    loop {
        main_loop(&gpio_expander, &server);
    }
}
