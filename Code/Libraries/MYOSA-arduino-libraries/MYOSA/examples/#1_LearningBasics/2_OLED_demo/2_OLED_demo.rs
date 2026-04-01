/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  OLED Demo
  Connection: Connect the "OLED" board from the MYOSA kit with the "Controller" board and power them up.
  Working: OLED board will display welcome message and demonstrate its graphical capabilities by different symbols and animations.

  Synopsis of OLED
  MYOSA Platform consists of a beautiful OLED Display Board. It is equiped with SSD1306 IC.
  It is a very small display, about 1" in diagonal but still very readable due to high contrast.
  This display is made of 128x64 individual white OLED pixels, each one is turned on or off by the controller chip.
  I2C Address of the board = 0x3C.
  Detailed Information about OLED board Library and usage is provided in the link below.
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

/// Hardware abstraction stubs for Serial, Wire (I2C), and OLED display.
mod hal {
    pub fn serial_begin(_baud: u32) {}
    pub fn serial_println(_msg: &str) {}
    pub fn wire_begin() {}
    pub fn delay(_ms: u32) {}

    pub const SCREEN_WIDTH: i16 = 128;
    pub const SCREEN_HEIGHT: i16 = 64;
    pub const SSD1306_WHITE: u8 = 1;
    pub const SSD1306_INVERSE: u8 = 2;

    pub struct OLed {
        pub width: i16,
        pub height: i16,
    }

    impl OLed {
        pub fn new(w: i16, h: i16) -> Self { OLed { width: w, height: h } }
        pub fn begin(&self) -> bool { true }
        pub fn display(&self) {}
        pub fn clear_display(&self) {}
        pub fn draw_pixel(&self, _x: i16, _y: i16, _color: u8) {}
        pub fn draw_line(&self, _x0: i16, _y0: i16, _x1: i16, _y1: i16, _color: u8) {}
        pub fn draw_rect(&self, _x: i16, _y: i16, _w: i16, _h: i16, _color: u8) {}
        pub fn fill_rect(&self, _x: i16, _y: i16, _w: i16, _h: i16, _color: u8) {}
        pub fn draw_circle(&self, _x: i16, _y: i16, _r: i16, _color: u8) {}
        pub fn fill_circle(&self, _x: i16, _y: i16, _r: i16, _color: u8) {}
        pub fn draw_round_rect(&self, _x: i16, _y: i16, _w: i16, _h: i16, _r: i16, _color: u8) {}
        pub fn fill_round_rect(&self, _x: i16, _y: i16, _w: i16, _h: i16, _r: i16, _color: u8) {}
        pub fn draw_triangle(&self, _x0: i16, _y0: i16, _x1: i16, _y1: i16, _x2: i16, _y2: i16, _color: u8) {}
        pub fn fill_triangle(&self, _x0: i16, _y0: i16, _x1: i16, _y1: i16, _x2: i16, _y2: i16, _color: u8) {}
        pub fn draw_cube(&self, _ax: f32, _ay: f32, _az: f32) {}
        pub fn set_text_size(&self, _s: u8) {}
        pub fn set_text_color(&self, _color: u8) {}
        pub fn set_cursor(&self, _x: i16, _y: i16) {}
        pub fn cp437(&self, _enable: bool) {}
        pub fn write(&self, _c: u8) {}
        pub fn width(&self) -> i16 { self.width }
        pub fn height(&self) -> i16 { self.height }
    }
}

use hal::*;

/* Creating Object of oLed Class. Screen Width = 128, Screen Height = 64 in pixels. Defined already in library. */
fn create_display() -> OLed {
    OLed::new(SCREEN_WIDTH, SCREEN_HEIGHT)
}

/* Setup Function */
fn setup(display: &OLed) {

    /* Setting up communication */
    serial_begin(115200);
    wire_begin();

    /* Setting up the oLed Board. */
    if !display.begin() {
        serial_println("SSD1306 allocation failed");
    } else {
        // Draw a single pixel in white
        display.draw_pixel(10, 10, SSD1306_WHITE);

        // Show the display buffer on the screen. You MUST call display() after
        // drawing commands to make them visible on screen!
        display.display();
        delay(500);
        display.clear_display();
    }
}

/* Loop Function */
fn main_loop(display: &OLed) {

    /* Loop function draws different graphical images and animations. */
    testdraw_cube(display);      // Rotate Cube

    testdraw_line(display);      // Draw many lines

    testdraw_rect(display);      // Draw rectangles (outlines)

    testfill_rect(display);      // Draw rectangles (filled)

    testdraw_circle(display);    // Draw circles (outlines)

    testfill_circle(display);    // Draw circles (filled)

    testdraw_roundrect(display); // Draw rounded rectangles (outlines)

    testfill_roundrect(display); // Draw rounded rectangles (filled)

    testdraw_triangle(display);  // Draw triangles (outlines)

    testfill_triangle(display);  // Draw triangles (filled)

    testdraw_char(display);      // Draw characters of the default font

    // Clear the buffer
    display.clear_display();

    delay(5000);
}


/* Below are a few derived functions from the base functions implemented in library. */

fn testdraw_cube(display: &OLed) {
    let mut angle: f32 = 0.0;
    while angle <= 360.0 {
        display.draw_cube(angle, angle, angle);
        angle += 1.0;
    }
}

fn testdraw_line(display: &OLed) {
    display.clear_display(); // Clear display buffer

    let mut i: i16 = 0;
    while i < display.width() {
        display.draw_line(0, 0, i, display.height() - 1, SSD1306_WHITE);
        display.display(); // Update screen with each newly-drawn line
        delay(1);
        i += 4;
    }
    i = 0;
    while i < display.height() {
        display.draw_line(0, 0, display.width() - 1, i, SSD1306_WHITE);
        display.display();
        delay(1);
        i += 4;
    }
    delay(250);

    display.clear_display();

    i = 0;
    while i < display.width() {
        display.draw_line(0, display.height() - 1, i, 0, SSD1306_WHITE);
        display.display();
        delay(1);
        i += 4;
    }
    i = display.height() - 1;
    while i >= 0 {
        display.draw_line(0, display.height() - 1, display.width() - 1, i, SSD1306_WHITE);
        display.display();
        delay(1);
        i -= 4;
    }
    delay(250);

    display.clear_display();

    i = display.width() - 1;
    while i >= 0 {
        display.draw_line(display.width() - 1, display.height() - 1, i, 0, SSD1306_WHITE);
        display.display();
        delay(1);
        i -= 4;
    }
    i = display.height() - 1;
    while i >= 0 {
        display.draw_line(display.width() - 1, display.height() - 1, 0, i, SSD1306_WHITE);
        display.display();
        delay(1);
        i -= 4;
    }
    delay(250);

    display.clear_display();

    i = 0;
    while i < display.height() {
        display.draw_line(display.width() - 1, 0, 0, i, SSD1306_WHITE);
        display.display();
        delay(1);
        i += 4;
    }
    i = 0;
    while i < display.width() {
        display.draw_line(display.width() - 1, 0, i, display.height() - 1, SSD1306_WHITE);
        display.display();
        delay(1);
        i += 4;
    }

    delay(2000); // Pause for 2 seconds
}

fn testdraw_rect(display: &OLed) {
    display.clear_display();

    let mut i: i16 = 0;
    while i < display.height() / 2 {
        display.draw_rect(i, i, display.width() - 2 * i, display.height() - 2 * i, SSD1306_WHITE);
        display.display(); // Update screen with each newly-drawn rectangle
        delay(1);
        i += 2;
    }

    delay(2000);
}

fn testfill_rect(display: &OLed) {
    display.clear_display();

    let mut i: i16 = 0;
    while i < display.height() / 2 {
        // The INVERSE color is used so rectangles alternate white/black
        display.fill_rect(i, i, display.width() - i * 2, display.height() - i * 2, SSD1306_INVERSE);
        display.display(); // Update screen with each newly-drawn rectangle
        delay(1);
        i += 3;
    }

    delay(2000);
}

fn testdraw_circle(display: &OLed) {
    display.clear_display();

    let max_dim = if display.width() > display.height() { display.width() } else { display.height() };
    let mut i: i16 = 0;
    while i < max_dim / 2 {
        display.draw_circle(display.width() / 2, display.height() / 2, i, SSD1306_WHITE);
        display.display();
        delay(1);
        i += 2;
    }

    delay(2000);
}

fn testfill_circle(display: &OLed) {
    display.clear_display();

    let max_dim = if display.width() > display.height() { display.width() } else { display.height() };
    let mut i: i16 = max_dim / 2;
    while i > 0 {
        // The INVERSE color is used so circles alternate white/black
        display.fill_circle(display.width() / 2, display.height() / 2, i, SSD1306_INVERSE);
        display.display(); // Update screen with each newly-drawn circle
        delay(1);
        i -= 3;
    }

    delay(2000);
}

fn testdraw_roundrect(display: &OLed) {
    display.clear_display();

    let mut i: i16 = 0;
    while i < display.height() / 2 - 2 {
        display.draw_round_rect(i, i, display.width() - 2 * i, display.height() - 2 * i,
            display.height() / 4, SSD1306_WHITE);
        display.display();
        delay(1);
        i += 2;
    }

    delay(2000);
}

fn testfill_roundrect(display: &OLed) {
    display.clear_display();

    let mut i: i16 = 0;
    while i < display.height() / 2 - 2 {
        // The INVERSE color is used so round-rects alternate white/black
        display.fill_round_rect(i, i, display.width() - 2 * i, display.height() - 2 * i,
            display.height() / 4, SSD1306_INVERSE);
        display.display();
        delay(1);
        i += 2;
    }

    delay(2000);
}

fn testdraw_triangle(display: &OLed) {
    display.clear_display();

    let max_dim = if display.width() > display.height() { display.width() } else { display.height() };
    let mut i: i16 = 0;
    while i < max_dim / 2 {
        display.draw_triangle(
            display.width() / 2, display.height() / 2 - i,
            display.width() / 2 - i, display.height() / 2 + i,
            display.width() / 2 + i, display.height() / 2 + i, SSD1306_WHITE);
        display.display();
        delay(1);
        i += 5;
    }

    delay(2000);
}

fn testfill_triangle(display: &OLed) {
    display.clear_display();

    let max_dim = if display.width() > display.height() { display.width() } else { display.height() };
    let mut i: i16 = max_dim / 2;
    while i > 0 {
        // The INVERSE color is used so triangles alternate white/black
        display.fill_triangle(
            display.width() / 2, display.height() / 2 - i,
            display.width() / 2 - i, display.height() / 2 + i,
            display.width() / 2 + i, display.height() / 2 + i, SSD1306_INVERSE);
        display.display();
        delay(1);
        i -= 5;
    }

    delay(2000);
}

fn testdraw_char(display: &OLed) {
    display.clear_display();

    display.set_text_size(1);      // Normal 1:1 pixel scale
    display.set_text_color(SSD1306_WHITE); // Draw white text
    display.set_cursor(0, 0);     // Start at top-left corner
    display.cp437(true);          // Use full 256 char 'Code Page 437' font

    // Not all the characters will fit on the display. This is normal.
    // Library will draw what it can and the rest will be clipped.
    for i in 0u16..256 {
        if i as u8 == b'\n' {
            display.write(b' ');
        } else {
            display.write(i as u8);
        }
        delay(200);
        display.display();
    }
    delay(2000);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let display = create_display();
    setup(&display);
    loop {
        main_loop(&display);
    }
}
