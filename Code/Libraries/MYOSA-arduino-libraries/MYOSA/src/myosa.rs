/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

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

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::fmt::Write as FmtWrite;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const RELAY_IO: u8 = 0; // IO0
pub const BUZZER_IO: u8 = 1; // IO1

/// OLED display width, in pixels
pub const SCREEN_WIDTH: u16 = 128;
/// OLED display height, in pixels
pub const SCREEN_HEIGHT: u16 = 64;
/// Reset pin # (or -1 if sharing Arduino reset pin)
pub const OLED_RESET: i8 = 4;

pub const NUM_SERVICES: usize = 6;
pub const MAX_CHARACTERISTICS: usize = 5;
pub const MAX_EVENTS: usize = 2;

pub const IO_OUTPUT: u8 = 1;
pub const IO_LOW: u8 = 0;
pub const IO_HIGH: u8 = 1;

// ---------------------------------------------------------------------------
// Hardware-abstraction traits
// ---------------------------------------------------------------------------

/// Trait for serial / debug output
pub trait SerialPort {
    fn print(&mut self, s: &str);
    fn println(&mut self, s: &str);
}

/// Trait for an accelerometer + gyroscope sensor (e.g. MPU6050)
pub trait AccelAndGyroSensor {
    fn begin(&mut self) -> bool;
    fn ping(&self) -> bool;
    fn get_accel_x(&self) -> f32;
    fn get_accel_y(&self) -> f32;
    fn get_accel_z(&self) -> f32;
    fn get_gyro_x(&self) -> f32;
    fn get_gyro_y(&self) -> f32;
    fn get_gyro_z(&self) -> f32;
    fn get_tilt_x(&self) -> f32;
    fn get_tilt_y(&self) -> f32;
    fn get_tilt_z(&self) -> f32;
    fn get_temp_c(&self) -> f32;
    fn get_temp_f(&self) -> f32;
}

/// Trait for an air-quality sensor (e.g. CCS811)
pub trait AirQualitySensor {
    fn begin(&mut self) -> bool;
    fn ping(&self) -> bool;
    fn is_data_available(&self) -> bool;
    fn read_algorithm_results(&mut self) -> SensorStatus;
    fn get_co2(&self) -> u16;
    fn get_tvoc(&self) -> u16;
}

/// Trait for a barometric pressure sensor (e.g. BMP180/280)
pub trait BarometricPressureSensor {
    fn begin(&mut self) -> bool;
    fn ping(&self) -> bool;
    fn get_temp_c(&self) -> f32;
    fn get_temp_f(&self) -> f32;
    fn get_pressure_pascal(&self) -> f32;
    fn get_pressure_hg(&self) -> f32;
    fn get_pressure_bar(&self) -> f32;
    fn get_altitude(&self, sea_level_pressure: f32) -> f32;
}

/// Trait for a light / proximity / gesture sensor (e.g. APDS-9960)
pub trait LightProximityAndGestureSensor {
    fn begin(&mut self) -> bool;
    fn ping(&self) -> bool;
    fn enable_ambient_light_sensor(&mut self, interrupts: bool) -> bool;
    fn enable_proximity_sensor(&mut self, interrupts: bool) -> bool;
    fn set_proximity_gain(&mut self, gain: u8) -> bool;
    fn get_ambient_light(&self) -> u16;
    fn get_proximity(&self) -> f32;
    fn get_red_proportion(&self) -> u16;
    fn get_green_proportion(&self) -> u16;
    fn get_blue_proportion(&self) -> u16;
}

/// Trait for a GPIO expander / actuator (e.g. PCA9536)
pub trait ActuatorDevice {
    fn ping(&self) -> bool;
    fn set_mode(&mut self, pin: u8, mode: u8);
    fn set_state(&mut self, pin: u8, state: u8);
}

/// Trait for a temperature & humidity sensor (e.g. DHT / SHT)
pub trait TempAndHumiditySensor {
    fn begin(&mut self) -> bool;
    fn ping(&self) -> bool;
    fn get_temp_c(&self) -> f32;
    fn get_temp_f(&self) -> f32;
    fn get_relative_humidity(&self) -> f32;
    fn get_heat_index_c(&self) -> f32;
    fn get_heat_index_f(&self) -> f32;
}

/// Trait for an OLED display (backed by Adafruit_SSD1306 / oLed in C++)
pub trait OledDisplay {
    fn begin(&mut self) -> bool;
    fn clear_display(&mut self);
    fn display(&mut self);
    fn set_text_size(&mut self, size: u8);
    fn set_text_color(&mut self, color: u16);
    fn set_cursor(&mut self, x: i16, y: i16);
    fn get_cursor_x(&self) -> i16;
    fn get_cursor_y(&self) -> i16;
    fn set_font(&mut self, font: Option<&GFXfont>);
    fn print(&mut self, s: &str);
    fn println(&mut self, s: &str);
    fn print_float(&mut self, val: f32, decimals: u8);
    fn print_int(&mut self, val: i32);
    fn draw_circle(&mut self, x0: i16, y0: i16, r: i16, color: u16);
    fn get_text_bounds(&self, s: &str, x: i16, y: i16) -> (i16, i16, u16, u16);
}

/// Trait representing BLE characteristic operations
pub trait BleCharacteristic {
    fn set_value(&mut self, value: &str);
    fn get_value(&self) -> String;
    fn notify(&mut self);
}

/// Trait representing BLE operations at a higher level
pub trait BleDevice {
    fn init(&mut self, name: &str);
    fn start_advertising(&mut self);
    fn get_characteristic(&mut self, service: usize, char_idx: usize) -> &mut dyn BleCharacteristic;
}

// ---------------------------------------------------------------------------
// Supporting types (ported from C++ structs / enums)
// ---------------------------------------------------------------------------

/// Mirrors SENSOR_SUCCESS in the C++ code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorStatus {
    Success,
    Error,
}

/// Placeholder for the GFXfont struct (see Adafruit_GFX.rs for full definition)
pub struct GFXfont;

pub const SEA_LEVEL_AVG_PRESSURE: f32 = 101325.0;

/// BLE event data, mirrors the C++ `eventData` struct
#[derive(Debug, Clone)]
pub struct EventData {
    pub is_enable: bool,
    pub service_number: usize,
    pub char_number: usize,
    pub para_number: usize,
    pub min: f64,
    pub max: f64,
    pub is_inclusive: bool,
    pub is_strict: bool,
    pub action: char,
}

impl Default for EventData {
    fn default() -> Self {
        EventData {
            is_enable: false,
            service_number: 0,
            char_number: 0,
            para_number: 0,
            min: 0.0,
            max: 0.0,
            is_inclusive: false,
            is_strict: false,
            action: '\0',
        }
    }
}

// ---------------------------------------------------------------------------
// MYOSA struct
// ---------------------------------------------------------------------------

/// Main MYOSA platform struct.
///
/// Generic over the concrete hardware implementations so that the same
/// logic can run against real hardware *or* test doubles.
pub struct MYOSA<Ag, Aq, Pr, Lpg, Act, Th, Disp, Ble, Ser>
where
    Ag: AccelAndGyroSensor,
    Aq: AirQualitySensor,
    Pr: BarometricPressureSensor,
    Lpg: LightProximityAndGestureSensor,
    Act: ActuatorDevice,
    Th: TempAndHumiditySensor,
    Disp: OledDisplay,
    Ble: BleDevice,
    Ser: SerialPort,
{
    /* Create sensor objects */
    pub ag: Ag,
    pub aq: Aq,
    pub pr: Pr,
    pub lpg: Lpg,
    pub gpio_expander: Act,
    pub th: Th,
    pub display: Disp,
    pub ble: Ble,
    pub serial: Ser,

    /* Variables */
    pub altitude: f32,

    /* private */
    event_arr: [EventData; MAX_EVENTS],
    event_cnt: usize,
    ref_resistance: f32,
}

// ---------------------------------------------------------------------------
// Free helper functions (ported from file-scope C++ helpers)
// ---------------------------------------------------------------------------

/// Parse the first character (event type) from a comma-separated input
/// string and return (event_type, remaining_string).
fn parse_event_type(input: &str) -> (char, &str) {
    let first_char = input.chars().next().unwrap_or('\0');
    let remaining = match input.find(',') {
        Some(idx) => &input[idx + 1..],
        None => "",
    };

    // Serial.print("First character: ");
    // Serial.println(firstChar);
    // Serial.print("Remaining string: ");
    // Serial.println(remainingString);

    (first_char, remaining)
}

/// Create or update an EventData from a comma-separated payload string.
fn create_event(input: &str) -> EventData {
    let mut temp_event = EventData {
        is_enable: true,
        ..Default::default()
    };

    let mut output_index: usize = 0;
    let mut value = String::new();

    for ch in input.chars() {
        if ch != ',' {
            value.push(ch);
        } else {
            match output_index {
                0 => temp_event.service_number = value.parse::<usize>().unwrap_or(0),
                1 => temp_event.char_number = value.parse::<usize>().unwrap_or(0),
                2 => temp_event.para_number = value.parse::<usize>().unwrap_or(0),
                3 => temp_event.min = value.parse::<f64>().unwrap_or(0.0),
                4 => temp_event.max = value.parse::<f64>().unwrap_or(0.0),
                5 => temp_event.is_inclusive = value.starts_with('1'),
                6 => temp_event.is_strict = value.starts_with('1'),
                _ => {}
            }
            output_index += 1;
            value.clear();
        }
    }
    if output_index == 7 {
        temp_event.action = value.chars().next().unwrap_or('\0');
    }
    temp_event
}

// ---------------------------------------------------------------------------
// MYOSA impl
// ---------------------------------------------------------------------------

impl<Ag, Aq, Pr, Lpg, Act, Th, Disp, Ble, Ser>
    MYOSA<Ag, Aq, Pr, Lpg, Act, Th, Disp, Ble, Ser>
where
    Ag: AccelAndGyroSensor,
    Aq: AirQualitySensor,
    Pr: BarometricPressureSensor,
    Lpg: LightProximityAndGestureSensor,
    Act: ActuatorDevice,
    Th: TempAndHumiditySensor,
    Disp: OledDisplay,
    Ble: BleDevice,
    Ser: SerialPort,
{
    /// Construct a new MYOSA instance from pre-created peripheral objects.
    pub fn new(
        ag: Ag,
        aq: Aq,
        pr: Pr,
        lpg: Lpg,
        gpio_expander: Act,
        th: Th,
        display: Disp,
        ble: Ble,
        serial: Ser,
    ) -> Self {
        MYOSA {
            ag,
            aq,
            pr,
            lpg,
            gpio_expander,
            th,
            display,
            ble,
            serial,
            altitude: 0.0,
            event_arr: [EventData::default(), EventData::default()],
            event_cnt: 0,
            ref_resistance: 10_000.0,
        }
    }

    /**
     *
     */
    pub fn begin(&mut self) -> bool {
        self.ble.init("MYOSA_1");
        self.ble.start_advertising();
        self.serial.println("BLE server started!");

        if self.display.begin() {
            self.serial.println("OLED initializated");
        }
        if self.ag.begin() {
            self.serial.println("AccelAndGyro initializated");
        }
        if self.aq.begin() {
            self.serial.println("AirQuality initializated");
        }
        if self.pr.begin() {
            self.serial.println("BarometricPressure initializated");
        }
        if self.lpg.begin() {
            self.serial.println("LightProximityAndGesture initializated");
            if self.lpg.enable_ambient_light_sensor(false) {
                self.serial.println("Light sensor is now running");
            }
            if self.lpg.enable_proximity_sensor(false) {
                self.serial.println("Proximity sensor is now running");
            }
            /* Adjust the Proximity sensor gain */
            const PGAIN_2X: u8 = 1;
            if !self.lpg.set_proximity_gain(PGAIN_2X) {
                self.serial.println("Something went wrong trying to set PGAIN");
            }
        }
        if self.gpio_expander.ping() {
            /* Set relay IO as output */
            self.gpio_expander.set_mode(RELAY_IO, IO_OUTPUT);
            self.gpio_expander.set_state(RELAY_IO, IO_LOW);
            /* Set buzzer IO as output */
            self.gpio_expander.set_mode(BUZZER_IO, IO_OUTPUT);
            self.gpio_expander.set_state(BUZZER_IO, IO_LOW);
            self.serial.println("gpioExpander initializated");
        }
        if self.th.begin() {
            self.serial.println("TempAndHumidity initializated");
        }

        true
    }

    /**
     *
     */
    pub fn turn_on_relay(&mut self) {
        self.gpio_expander.set_mode(RELAY_IO, IO_OUTPUT);
        self.gpio_expander.set_state(RELAY_IO, IO_HIGH);
    }

    /**
     *
     */
    pub fn turn_off_relay(&mut self) {
        self.gpio_expander.set_mode(RELAY_IO, IO_OUTPUT);
        self.gpio_expander.set_state(RELAY_IO, IO_LOW);
    }

    /**
     *
     */
    pub fn turn_on_buzzer(&mut self, print: i32) {
        self.gpio_expander.set_mode(BUZZER_IO, IO_OUTPUT);
        self.gpio_expander.set_state(BUZZER_IO, IO_HIGH);

        if print != 0 {
            self.display.clear_display();
            self.display.set_text_size(1);
            self.display.set_text_color(1); // WHITE
            self.display.set_font(None); // VeraMonoBold7pt7b placeholder
            self.display.set_cursor(0, 9);
            self.draw_centre_string("Actuator");
            if self.gpio_expander.ping() {
                self.display.set_font(None); // VeraMono7pt7b placeholder
                self.display.print("Buzzer: Turn On");
                self.display.println("");
                self.display.display();
            }
        }
    }

    /**
     *
     */
    pub fn turn_off_buzzer(&mut self, _print: i32) {
        self.gpio_expander.set_mode(BUZZER_IO, IO_OUTPUT);
        self.gpio_expander.set_state(BUZZER_IO, IO_LOW);

        // display.clearDisplay();
        self.display.set_text_size(1);
        self.display.set_text_color(1); // WHITE
        self.display.set_font(None); // VeraMonoBold7pt7b placeholder
        // display.setCursor(0, 9);
        // drawCentreString("Actuator");
        if self.gpio_expander.ping() {
            self.display.set_font(None); // VeraMono7pt7b placeholder
            self.display.print("Buzzer: Turn Off");
            self.display.println("");
            self.display.display();
        }
    }

    /**
     *
     */
    pub fn draw_centre_string(&mut self, buf: &str) {
        let cx = self.display.get_cursor_x();
        let cy = self.display.get_cursor_y();
        let (x1, y1, w, h) = self.display.get_text_bounds(buf, cx, cy);
        self.display
            .set_cursor(((SCREEN_WIDTH as i16) - w as i16) / 2, y1 + h as i16);
        self.display.println(buf);
    }

    /**
     *
     */
    pub fn draw_subscript_symbol(&mut self, buf: &str) {
        self.display.set_font(None);
        let cx = self.display.get_cursor_x();
        let cy = self.display.get_cursor_y();
        self.display.set_cursor(cx, cy + 3);
        self.display.print(buf);
        let cx = self.display.get_cursor_x();
        let cy = self.display.get_cursor_y();
        self.display.set_cursor(cx, cy - 3);
        self.display.set_font(None); // VeraMono7pt7b placeholder
    }

    /**
     *
     */
    pub fn draw_superscript_symbol(&mut self, buf: &str) {
        self.display.set_font(None);
        let cx = self.display.get_cursor_x();
        let cy = self.display.get_cursor_y();
        self.display.set_cursor(cx, cy - 6);
        self.display.print(buf);
        let cx = self.display.get_cursor_x();
        let cy = self.display.get_cursor_y();
        self.display.set_cursor(cx, cy + 6);
        self.display.set_font(None); // VeraMono7pt7b placeholder
    }

    /**
     *
     */
    pub fn draw_degree_symbol(&mut self) {
        let cx = self.display.get_cursor_x();
        let cy = self.display.get_cursor_y();
        self.display.draw_circle(cx + 4, cy - 8, 2, 1);
        self.display.set_cursor(cx + 6, cy);
    }

    /**
     *
     */
    pub fn send_ble_data(&mut self) {
        for i in 0..(NUM_SERVICES - 1) {
            let char_iterator: usize = match i {
                0 => 4,
                1 => 2,
                _ => 3,
            };

            for j in 0..char_iterator {
                let mut payload = String::new();
                let mut has_data = false;

                if self.ag.ping() {
                    match (i, j) {
                        (0, 0) => {
                            let _ = write!(
                                payload,
                                "{:.2}, {:.2}, {:.2}",
                                self.ag.get_accel_x(),
                                self.ag.get_accel_y(),
                                self.ag.get_accel_z()
                            );
                            has_data = true;
                        }
                        (0, 1) => {
                            let _ = write!(
                                payload,
                                "{:.2}, {:.2}, {:.2}",
                                self.ag.get_gyro_x(),
                                self.ag.get_gyro_y(),
                                self.ag.get_gyro_z()
                            );
                            has_data = true;
                        }
                        (0, 2) => {
                            let _ = write!(
                                payload,
                                "{:.2}, {:.2}, {:.2}",
                                self.ag.get_tilt_x(),
                                self.ag.get_tilt_y(),
                                self.ag.get_tilt_z()
                            );
                            has_data = true;
                        }
                        (0, 3) => {
                            let _ = write!(
                                payload,
                                "{:.2}, {:.2}",
                                self.ag.get_temp_c(),
                                self.ag.get_temp_f()
                            );
                            has_data = true;
                        }
                        _ => {}
                    }
                }
                if self.aq.ping() {
                    match (i, j) {
                        (1, 0) => {
                            let _ = write!(payload, "{}", self.aq.get_co2());
                            has_data = true;
                        }
                        (1, 1) => {
                            let _ = write!(payload, "{}", self.aq.get_tvoc());
                            has_data = true;
                        }
                        _ => {}
                    }
                }
                if self.pr.ping() {
                    match (i, j) {
                        (2, 0) => {
                            let _ = write!(
                                payload,
                                "{:.2}, {:.2}",
                                self.pr.get_temp_c(),
                                self.pr.get_temp_f()
                            );
                            has_data = true;
                        }
                        (2, 1) => {
                            let _ = write!(
                                payload,
                                "{:.2}, {:.2}, {:.2}",
                                self.pr.get_pressure_pascal(),
                                self.pr.get_pressure_hg(),
                                self.pr.get_pressure_bar()
                            );
                            has_data = true;
                        }
                        (2, 2) => {
                            let _ = write!(
                                payload,
                                "{:.2}",
                                self.pr.get_altitude(SEA_LEVEL_AVG_PRESSURE)
                            );
                            has_data = true;
                        }
                        _ => {}
                    }
                }
                if self.lpg.ping() {
                    match (i, j) {
                        (3, 0) => {
                            let _ = write!(payload, "{}", self.lpg.get_ambient_light());
                            has_data = true;
                        }
                        (3, 1) => {
                            let _ = write!(payload, "{:.2}", self.lpg.get_proximity());
                            has_data = true;
                        }
                        (3, 2) => {
                            let _ = write!(
                                payload,
                                "{}, {}, {}",
                                self.lpg.get_red_proportion(),
                                self.lpg.get_green_proportion(),
                                self.lpg.get_blue_proportion()
                            );
                            has_data = true;
                        }
                        _ => {}
                    }
                }
                if self.th.ping() {
                    match (i, j) {
                        (4, 0) => {
                            let _ = write!(
                                payload,
                                "{:.2}, {:.2}",
                                self.th.get_temp_c(),
                                self.th.get_temp_f()
                            );
                            has_data = true;
                        }
                        (4, 1) => {
                            let _ = write!(payload, "{:.2}", self.th.get_relative_humidity());
                            has_data = true;
                        }
                        (4, 2) => {
                            let _ = write!(
                                payload,
                                "{:.2}, {:.2}",
                                self.th.get_heat_index_c(),
                                self.th.get_heat_index_f()
                            );
                            has_data = true;
                        }
                        _ => {}
                    }
                }

                if has_data {
                    let msg = format!(
                        "Notifying value of characteristic {} in service {}: {}",
                        j, i, payload
                    );
                    self.serial.println(&msg);
                    let ch = self.ble.get_characteristic(i, j);
                    ch.set_value(&payload);
                    ch.notify();
                }
            }
        }

        // Check for events
        for idx in 0..MAX_EVENTS {
            let ev = self.event_arr[idx].clone();
            if ev.is_enable {
                let raw_value = self.ble.get_characteristic(ev.service_number, ev.char_number).get_value();
                self.serial.println(&raw_value);

                let mut val: f64 = 0.0;
                let mut cnt: usize = 0;
                for token in raw_value.split(',') {
                    let parsed = token.trim().parse::<f64>().unwrap_or(0.0);
                    if ev.para_number == cnt {
                        val = parsed;
                        break;
                    }
                    cnt += 1;
                }
                let msg = format!("{}", val);
                self.serial.println(&msg);

                let trigger = if ev.is_inclusive {
                    if ev.is_strict {
                        val >= ev.min && val <= ev.max
                    } else {
                        val > ev.min && val < ev.max
                    }
                } else {
                    if ev.is_strict {
                        val <= ev.min && val >= ev.max
                    } else {
                        val < ev.min && val > ev.max
                    }
                };

                if trigger {
                    if ev.action == 'b' {
                        self.turn_on_buzzer(0);
                    } else {
                        self.turn_on_relay();
                    }
                } else {
                    if ev.action == 'b' {
                        self.turn_off_buzzer(0);
                    } else {
                        self.turn_off_relay();
                    }
                }
            }
        }
    }

    /// Handle an incoming BLE write payload (mirrors MyCallbacks::onWrite).
    pub fn handle_ble_write(&mut self, new_value: &str) {
        if !new_value.is_empty() {
            self.serial.println("*********");
            let msg = format!("New value: {}", new_value);
            self.serial.println(&msg);
            self.serial.println("");
            self.serial.println("*********");

            let (event_type, payload) = parse_event_type(new_value);

            match event_type {
                'c' => {
                    let ev = create_event(payload);
                    if ev.action == 'b' {
                        self.event_arr[0] = ev;
                    } else {
                        self.event_arr[1] = ev;
                    }
                    self.event_cnt += 1;
                }
                'u' => {
                    let ev = create_event(payload);
                    if ev.action == 'b' {
                        self.event_arr[0] = ev;
                    } else {
                        self.event_arr[1] = ev;
                    }
                }
                'd' => {
                    if payload.starts_with('b') {
                        self.event_arr[0].is_enable = false;
                    } else {
                        self.event_arr[1].is_enable = false;
                    }
                    if self.event_cnt > 0 {
                        self.event_cnt -= 1;
                    }
                }
                _ => {}
            }

            for i in 0..MAX_EVENTS {
                self.print_event_data(&self.event_arr[i].clone());
            }
        }
    }

    fn print_event_data(&mut self, data: &EventData) {
        self.serial.println(&format!("isEnable: {}", data.is_enable));
        self.serial.println(&format!("serviceNumber: {}", data.service_number));
        self.serial.println(&format!("charNumber: {}", data.char_number));
        self.serial.println(&format!("paraNumber: {}", data.para_number));
        self.serial.println(&format!("min: {}", data.min));
        self.serial.println(&format!("max: {}", data.max));
        self.serial.println(&format!("isInclusive: {}", data.is_inclusive));
        self.serial.println(&format!("isStrict: {}", data.is_strict));
        self.serial.println(&format!("action: {}", data.action));
        self.serial.println("");
    }

    /**
     *
     */
    pub fn print_accel_and_gyro(&mut self) {
        // static counter emulated via an internal field would normally
        // be required; here we accept a mutable counter reference.
        // For a direct port, the caller should maintain the counter.
        self.print_accel_and_gyro_page(0);
    }

    pub fn print_accel_and_gyro_page(&mut self, n_cnt: u8) {
        self.display.clear_display();
        self.display.set_text_size(1);
        self.display.set_text_color(1); // WHITE
        self.display.set_font(None); // VeraMonoBold7pt7b
        self.display.set_cursor(0, 9);
        self.draw_centre_string("AccelGyro: Raw");
        if self.ag.ping() {
            self.display.set_font(None); // VeraMono7pt7b
            match n_cnt {
                0 => {
                    self.display.print("aX:");
                    self.display.print_float(self.ag.get_accel_x(), 1);
                    self.display.print("cm/s");
                    self.draw_superscript_symbol("2");
                    self.display.println("");

                    self.display.print("aY:");
                    self.display.print_float(self.ag.get_accel_y(), 1);
                    self.display.print("cm/s");
                    self.draw_superscript_symbol("2");
                    self.display.println("");

                    self.display.print("aZ:");
                    self.display.print_float(self.ag.get_accel_z(), 1);
                    self.display.print("cm/s");
                    self.draw_superscript_symbol("2");
                }
                1 => {
                    self.display.print("gX:");
                    self.display.print_float(self.ag.get_gyro_x(), 1);
                    self.draw_degree_symbol();
                    self.display.println("/s");
                    self.display.print("gY:");
                    self.display.print_float(self.ag.get_gyro_y(), 1);
                    self.draw_degree_symbol();
                    self.display.println("/s");
                    self.display.print("gZ:");
                    self.display.print_float(self.ag.get_gyro_z(), 1);
                    self.draw_degree_symbol();
                    self.display.println("/s");
                }
                2 => {
                    self.display.print("tiltX:");
                    self.display.print_float(self.ag.get_tilt_x(), 1);
                    self.draw_degree_symbol();
                    self.display.println("");
                    self.display.print("tiltY:");
                    self.display.print_float(self.ag.get_tilt_y(), 1);
                    self.draw_degree_symbol();
                    self.display.println("");
                    self.display.print("tiltZ:");
                    self.display.print_float(self.ag.get_tilt_z(), 1);
                    self.draw_degree_symbol();
                    self.display.println("");
                }
                _ => {
                    self.display.print("Temp:");
                    self.display.print_float(self.ag.get_temp_c(), 1);
                    self.draw_degree_symbol();
                    self.display.println("C");
                    self.display.print("Temp:");
                    self.display.print_float(self.ag.get_temp_f(), 1);
                    self.draw_degree_symbol();
                    self.display.println("F");
                }
            }
        }
        self.display.display();
    }

    /**
     *
     */
    pub fn print_air_quality(&mut self) {
        self.display.clear_display();
        self.display.set_text_size(1);
        self.display.set_text_color(1); // WHITE
        self.display.set_font(None); // VeraMonoBold7pt7b
        self.display.set_cursor(0, 9);
        self.draw_centre_string("Air Quality");
        if self.aq.ping() {
            /* Check if data is ready or not */
            if self.aq.is_data_available() {
                if self.aq.read_algorithm_results() == SensorStatus::Success {
                    self.display.set_font(None); // VeraMono7pt7b
                    self.display.print("eCO2 :");
                    self.display.print_int(self.aq.get_co2() as i32);
                    self.display.println("ppm");
                    self.display.print("TVOC :");
                    self.display.print_int(self.aq.get_tvoc() as i32);
                    self.display.println("ppb");
                }
            }
        }
        self.display.display();
    }

    /**
     *
     */
    pub fn print_barometric_pressure(&mut self, n_cnt: u8) {
        self.display.clear_display();
        self.display.set_text_size(1);
        self.display.set_text_color(1); // WHITE
        self.display.set_font(None); // VeraMonoBold7pt7b
        self.display.set_cursor(0, 9);
        self.draw_centre_string("Pressure");
        if self.pr.ping() {
            self.display.set_font(None); // VeraMono7pt7b
            if n_cnt == 0 {
                self.display.print("Temp:");
                self.display.print_float(self.pr.get_temp_c(), 1);
                self.draw_degree_symbol();
                self.display.println("C");
                self.display.print("Temp:");
                self.display.print_float(self.pr.get_temp_f(), 1);
                self.draw_degree_symbol();
                self.display.println("F");
                self.display.print("Alti:");
                self.display.print_float(self.pr.get_altitude(SEA_LEVEL_AVG_PRESSURE), 1);
                self.display.println("m");
            } else {
                self.display.print("Pres:");
                self.display.print_float(self.pr.get_pressure_pascal(), 1);
                self.display.println("kPa");
                self.display.print("Pres:");
                self.display.print_float(self.pr.get_pressure_hg(), 1);
                self.display.println("mmHg");
                self.display.print("Pres:");
                self.display.print_float(self.pr.get_pressure_bar(), 1);
                self.display.println("mbar");
            }
        }
        self.display.display();
    }

    /**
     *
     */
    pub fn print_light_proximity_and_gesture(&mut self, n_cnt: u8) {
        self.display.clear_display();
        self.display.set_text_size(1);
        self.display.set_text_color(1); // WHITE
        self.display.set_font(None); // VeraMonoBold7pt7b
        self.display.set_cursor(0, 9);
        self.draw_centre_string("Light Prox RGB");
        if self.lpg.ping() {
            self.display.set_font(None); // VeraMono7pt7b
            if n_cnt == 0 {
                self.display.print("Ambient:");
                self.display.print_int(self.lpg.get_ambient_light() as i32);
                self.display.println("Lux");
                self.display.print("Proximity:");
                self.display.print_float(self.lpg.get_proximity(), 1);
                self.display.println("");
            } else {
                self.display.print("Red  :");
                self.display.print_int(self.lpg.get_red_proportion() as i32);
                self.display.println("%");
                self.display.print("Green:");
                self.display.print_int(self.lpg.get_green_proportion() as i32);
                self.display.println("%");
                self.display.print("Blue :");
                self.display.print_int(self.lpg.get_blue_proportion() as i32);
                self.display.println("%");
            }
        }
        self.display.display();
    }

    /**
     *
     */
    pub fn print_temp_and_humidity(&mut self, n_cnt: u8) {
        self.display.clear_display();
        self.display.set_text_size(1);
        self.display.set_text_color(1); // WHITE
        self.display.set_font(None); // VeraMonoBold7pt7b
        self.display.set_cursor(0, 9);
        self.draw_centre_string("Temp Humidity");
        if self.th.ping() {
            self.display.set_font(None); // VeraMono7pt7b
            if n_cnt == 0 {
                self.display.print("RH  :");
                self.display.print_float(self.th.get_relative_humidity(), 1);
                self.display.println("%");
                self.display.print("Temp:");
                self.display.print_float(self.th.get_temp_c(), 1);
                self.draw_degree_symbol();
                self.display.println("C");
                self.display.print("Temp:");
                self.display.print_float(self.th.get_temp_f(), 1);
                self.draw_degree_symbol();
                self.display.println("F");
            } else {
                self.display.print("HI :");
                self.display.print_float(self.th.get_heat_index_c(), 1);
                self.draw_degree_symbol();
                self.display.println("C");
                self.display.print("HI :");
                self.display.print_float(self.th.get_heat_index_f(), 1);
                self.draw_degree_symbol();
                self.display.println("F");
            }
        }
        self.display.display();
    }
}
