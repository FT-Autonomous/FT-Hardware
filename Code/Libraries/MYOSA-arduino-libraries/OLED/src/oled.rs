/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

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

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::f32::consts::PI;

// Hardware-abstraction traits (same as Adafruit_SSD1306.rs)
pub trait I2cBus {
    fn begin_transmission(&mut self, addr: u8);
    fn write_byte(&mut self, data: u8);
    fn write_bytes(&mut self, data: &[u8]);
    fn end_transmission(&mut self) -> u8;
    fn request_from(&mut self, addr: u8, quantity: u8) -> u8;
    fn read(&mut self) -> u8;
    fn available(&self) -> u8;
    fn set_clock(&mut self, freq: u32);
}

pub trait SpiBus {
    fn begin(&mut self);
    fn begin_transaction(&mut self, clock: u32, bit_order: u8, data_mode: u8);
    fn transfer(&mut self, data: u8) -> u8;
    fn end_transaction(&mut self);
}

pub trait GpioPin {
    fn pin_mode(&mut self, mode: u8);
    fn digital_write(&mut self, val: u8);
    fn digital_read(&self) -> u8;
}

pub trait DelayProvider {
    fn delay_ms(&self, ms: u32);
}

pub trait DrawPixel {
    fn draw_pixel(&mut self, x: i16, y: i16, color: u16);
}

// Forward-declared types from Adafruit_SSD1306 and Adafruit_GFX
// In practice these would be imported; here we define the interface oLed needs

const OLED_I2C_ADDRESS: u8 = 0x3C;
const SCREEN_WIDTH: u8 = 128;
const SCREEN_HEIGHT: u8 = 64;

const SSD1306_SWITCHCAPVCC: u8 = 0x02;
const WHITE: u16 = 1;

#[derive(Clone, Copy)]
struct point3D_t {
    x: f32,
    y: f32,
    z: f32,
}

/**
 * oLed wraps Adafruit_SSD1306 display with additional 3D cube drawing
 * and MYOSA logo display functionality.
 *
 * NOTE: This Rust port takes a different composition approach vs the C++
 * inheritance chain (oLed : Adafruit_SSD1306 : Adafruit_GFX).
 * Here, oLed holds an Adafruit_SSD1306 and an Adafruit_GFX and
 * delegates drawing calls to them.
 */
pub struct oLed {
    // In a full port, these would be the actual Adafruit_SSD1306 and Adafruit_GFX structs.
    // For this port, we define the interface and stub the hardware calls.
    _width: u8,
    _height: u8,
    _rst_pin: i8,
    _i2c_addr: u8,
    _buffer: Vec<u8>,
    _cursor_x: i16,
    _cursor_y: i16,
    _text_color: u16,
    _text_bg: u16,
    _text_size_x: u8,
    _text_size_y: u8,
    _rotation: u8,
    _vertices: [point3D_t; 8],
    _mV: [point3D_t; 8],
    _edges: [[u8; 2]; 12],
}

impl oLed {
    /**
     *
     */
    pub fn new_i2c(w: u8, h: u8, rst_pin: i8) -> Self {
        let buf_size = (w as usize) * (h as usize) / 8;
        oLed {
            _width: w,
            _height: h,
            _rst_pin: rst_pin,
            _i2c_addr: OLED_I2C_ADDRESS,
            _buffer: vec![0u8; buf_size],
            _cursor_x: 0,
            _cursor_y: 0,
            _text_color: WHITE,
            _text_bg: 0,
            _text_size_x: 1,
            _text_size_y: 1,
            _rotation: 0,
            _vertices: [point3D_t { x: 0.0, y: 0.0, z: 0.0 }; 8],
            _mV: [point3D_t { x: 0.0, y: 0.0, z: 0.0 }; 8],
            _edges: [[0u8; 2]; 12],
        }
    }

    /**
     *
     */
    pub fn new_soft_spi(w: u8, h: u8, _mosi_pin: i8, _sclk_pin: i8,
                        _dc_pin: i8, _rst_pin: i8, _cs_pin: i8) -> Self {
        let buf_size = (w as usize) * (h as usize) / 8;
        oLed {
            _width: w,
            _height: h,
            _rst_pin,
            _i2c_addr: 0,
            _buffer: vec![0u8; buf_size],
            _cursor_x: 0,
            _cursor_y: 0,
            _text_color: WHITE,
            _text_bg: 0,
            _text_size_x: 1,
            _text_size_y: 1,
            _rotation: 0,
            _vertices: [point3D_t { x: 0.0, y: 0.0, z: 0.0 }; 8],
            _mV: [point3D_t { x: 0.0, y: 0.0, z: 0.0 }; 8],
            _edges: [[0u8; 2]; 12],
        }
    }

    /**
     *
     */
    pub fn new_hw_spi(w: u8, h: u8, _dc_pin: i8, _rst_pin: i8, _cs_pin: i8) -> Self {
        let buf_size = (w as usize) * (h as usize) / 8;
        oLed {
            _width: w,
            _height: h,
            _rst_pin,
            _i2c_addr: 0,
            _buffer: vec![0u8; buf_size],
            _cursor_x: 0,
            _cursor_y: 0,
            _text_color: WHITE,
            _text_bg: 0,
            _text_size_x: 1,
            _text_size_y: 1,
            _rotation: 0,
            _vertices: [point3D_t { x: 0.0, y: 0.0, z: 0.0 }; 8],
            _mV: [point3D_t { x: 0.0, y: 0.0, z: 0.0 }; 8],
            _edges: [[0u8; 2]; 12],
        }
    }

    /**
     *
     */
    pub fn begin(&mut self) -> bool {
        /* update the vertices */
        self._vertices[0].x = -1.0; self._vertices[0].y =  1.0; self._vertices[0].z = -1.0;
        self._vertices[1].x =  1.0; self._vertices[1].y =  1.0; self._vertices[1].z = -1.0;
        self._vertices[2].x =  1.0; self._vertices[2].y = -1.0; self._vertices[2].z = -1.0;
        self._vertices[3].x = -1.0; self._vertices[3].y = -1.0; self._vertices[3].z = -1.0;
        self._vertices[4].x = -1.0; self._vertices[4].y =  1.0; self._vertices[4].z =  1.0;
        self._vertices[5].x =  1.0; self._vertices[5].y =  1.0; self._vertices[5].z =  1.0;
        self._vertices[6].x =  1.0; self._vertices[6].y = -1.0; self._vertices[6].z =  1.0;
        self._vertices[7].x = -1.0; self._vertices[7].y = -1.0; self._vertices[7].z =  1.0;
        /* update the back edges */
        self._edges[0][0] = 0; self._edges[0][1] = 1; self._edges[1][0] = 1; self._edges[1][1] = 2;
        self._edges[2][0] = 2; self._edges[2][1] = 3; self._edges[3][0] = 3; self._edges[3][1] = 0;
        /* update the Front edges */
        self._edges[4][0] = 5; self._edges[4][1] = 4; self._edges[5][0] = 4; self._edges[5][1] = 7;
        self._edges[6][0] = 7; self._edges[6][1] = 6; self._edges[7][0] = 6; self._edges[7][1] = 5;
        /* update the Front-to-back edges */
        self._edges[8][0] = 0; self._edges[8][1] = 4; self._edges[9][0] = 1; self._edges[9][1] = 5;
        self._edges[10][0] = 2; self._edges[10][1] = 6; self._edges[11][0] = 3; self._edges[11][1] = 7;
        /* begin OLED display */
        if ssd1306_begin(SSD1306_SWITCHCAPVCC, OLED_I2C_ADDRESS) == false {
            return false;
        }
        self.clearDisplay();
        self.displayLogo();
        return true;
    }

    /**
     *
     */
    fn displayLogo(&mut self) {
        self.setRotation(0);
        self.clearDisplay();
        self.setTextColor(WHITE); // or BLACK);
        self.setTextSize(2);      // printable sizes from 1 to 8; typical use is 1, 2 or 4
        self.setCursor(6, 6);     // begin text at this location
        self.print("Welcome!!!");
        self.display();
        delay(2000);

        self.setTextSize(3);
        self.setCursor(5, 33);     // begin text at this location
        self.print("MYOSA");
        self.display();
        delay(1000);

        self.setTextSize(1);
        self.setCursor(98, 48);     // begin text at this location
        self.print("v3.0");
        self.display();
        delay(2500);
        //drawBitmap(0, 32, logo16_glcd_bmp, 256, 64, 1); //draw logo
        //self.display();
        //delay(2000);
        self.clearDisplay();
        //setCursor(0,0);
        //self.display();
    }

    /**
     *
     */
    pub fn drawCube(&mut self, xAngle: f32, yAngle: f32, zAngle: f32) {
        let mut e: [u8; 2];

        /* clear the display */
        self.clearDisplay();
        /* Calculate the points */
        for nVertex in 0u8..8u8 {
            self._mV[nVertex as usize].x = self._vertices[nVertex as usize].x;
            self._mV[nVertex as usize].y = self._vertices[nVertex as usize].y;
            self._mV[nVertex as usize].z = self._vertices[nVertex as usize].z;
            /* Calulate the 3D points */
            oLed::rotateXYZ(&mut self._mV[nVertex as usize], xAngle, yAngle, zAngle);
            /* project 3D to 2D */
            oLed::project3Dto2D(&mut self._mV[nVertex as usize], 128, 64, 64, 4);
        }
        /* Plot the line to make cube */
        for nEdge in 0u8..12u8 {
            e[0] = self._edges[nEdge as usize][0];
            e[1] = self._edges[nEdge as usize][1];
            self.drawLine(
                self._mV[e[0] as usize].x as i16, self._mV[e[0] as usize].y as i16,
                self._mV[e[1] as usize].x as i16, self._mV[e[1] as usize].y as i16,
                1,
            );
        }
        self.display();
    }

    /**
     *
     */
    fn rotateXYZ(point: &mut point3D_t, xAngle: f32, yAngle: f32, zAngle: f32) {
        let mut rad: f32;
        let mut cosa: f32;
        let mut sina: f32;
        let mut x: f32;
        let mut y: f32;
        let mut z: f32;

        /* Rotates this point around the X axis the given number of degrees */
        rad     = xAngle * PI / 180.0;
        cosa    = rad.cos();
        sina    = rad.sin();
        y = point.y * cosa - point.z * sina;
        z = point.y * sina + point.z * cosa;
        point.y = y;
        point.z = z;

        /* Rotates this point around the Y axis the given number of degrees */
        rad     = yAngle * PI / 180.0;
        cosa    = rad.cos();
        sina    = rad.sin();
        z = point.z * cosa - point.x * sina;
        x = point.z * sina + point.x * cosa;
        point.z = z;
        point.x = x;

        /* Rotates this point around the Z axis the given number of degrees */
        rad     = zAngle * PI / 180.0;
        cosa    = rad.cos();
        sina    = rad.sin();
        x = point.x * cosa - point.y * sina;
        y = point.x * sina + point.y * cosa;
        point.x = x;
        point.y = y;
    }

    /**
     *
     */
    fn project3Dto2D(point: &mut point3D_t, win_width: u16, win_height: u16, fov: u16, viewer_distance: u16) {
        let x: f32;
        let y: f32;
        let factor: f32;

        factor  =  fov as f32 / (viewer_distance as f32 + point.z);
        x       =  point.x * factor + win_width as f32 / 2.0;
        y       = -point.y * factor + win_height as f32 / 2.0;

        /* update the axes */
        point.x = x;
        point.y = y;
    }

    /**
     *
     */
    pub fn drawPixel(&mut self, x: i16, y: i16, color: u16) {
        ssd1306_draw_pixel(x, y, color);
    }

    /**
     *
     */
    pub fn drawLine(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, color: u16) {
        gfx_draw_line(x0, y0, x1, y1, color);
    }

    /**
     *
     */
    pub fn drawRect(&mut self, x: i16, y: i16, w: i16, h: i16, color: u16) {
        gfx_draw_rect(x, y, w, h, color);
    }

    /**
     *
     */
    pub fn fillRect(&mut self, x: i16, y: i16, w: i16, h: i16, color: u16) {
        gfx_fill_rect(x, y, w, h, color);
    }

    /**
     *
     */
    pub fn drawCircle(&mut self, x0: i16, y0: i16, r: i16, color: u16) {
        gfx_draw_circle(x0, y0, r, color);
    }

    /**
     *
     */
    pub fn drawCircleHelper(&mut self, x0: i16, y0: i16, r: i16, cornername: u8, color: u16) {
        gfx_draw_circle_helper(x0, y0, r, cornername, color);
    }

    /**
     *
     */
    pub fn fillCircle(&mut self, x0: i16, y0: i16, r: i16, color: u16) {
        gfx_fill_circle(x0, y0, r, color);
    }

    /**
     *
     */
    pub fn fillCircleHelper(&mut self, x0: i16, y0: i16, r: i16, cornername: u8, delta: i16, color: u16) {
        gfx_fill_circle_helper(x0, y0, r, cornername, delta, color);
    }

    /**
     *
     */
    pub fn drawTriangle(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, x2: i16, y2: i16, color: u16) {
        gfx_draw_triangle(x0, y0, x1, y1, x2, y2, color);
    }

    /**
     *
     */
    pub fn fillTriangle(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, x2: i16, y2: i16, color: u16) {
        gfx_fill_triangle(x0, y0, x1, y1, x2, y2, color);
    }

    /**
     *
     */
    pub fn drawRoundRect(&mut self, x0: i16, y0: i16, w: i16, h: i16, radius: i16, color: u16) {
        gfx_draw_round_rect(x0, y0, w, h, radius, color);
    }

    /**
     *
     */
    pub fn fillRoundRect(&mut self, x0: i16, y0: i16, w: i16, h: i16, radius: i16, color: u16) {
        gfx_fill_round_rect(x0, y0, w, h, radius, color);
    }

    /**
     *
     */
    pub fn drawBitmap_const(&mut self, x: i16, y: i16, bitmap: &[u8], w: i16, h: i16, color: u16) {
        gfx_draw_bitmap(x, y, bitmap, w, h, color);
    }

    /**
     *
     */
    pub fn drawBitmap_const_bg(&mut self, x: i16, y: i16, bitmap: &[u8], w: i16, h: i16, color: u16, bg: u16) {
        gfx_draw_bitmap_bg(x, y, bitmap, w, h, color, bg);
    }

    /**
     *
     */
    pub fn drawBitmap_mut(&mut self, x: i16, y: i16, bitmap: &mut [u8], w: i16, h: i16, color: u16) {
        gfx_draw_bitmap(x, y, bitmap, w, h, color);
    }

    /**
     *
     */
    pub fn drawBitmap_mut_bg(&mut self, x: i16, y: i16, bitmap: &mut [u8], w: i16, h: i16, color: u16, bg: u16) {
        gfx_draw_bitmap_bg(x, y, bitmap, w, h, color, bg);
    }

    /**
     *
     */
    pub fn drawXBitmap(&mut self, x: i16, y: i16, bitmap: &[u8], w: i16, h: i16, color: u16) {
        gfx_draw_x_bitmap(x, y, bitmap, w, h, color);
    }

    /**
     *
     */
    pub fn drawChar(&mut self, x: i16, y: i16, c: u8, color: u16, bg: u16, size: u8) {
        gfx_draw_char(x, y, c, color, bg, size, size);
    }

    /**
     *
     */
    pub fn drawChar_xy(&mut self, x: i16, y: i16, c: u8, color: u16, bg: u16, size_x: u8, size_y: u8) {
        gfx_draw_char(x, y, c, color, bg, size_x, size_y);
    }

    /**
     *
     */
    pub fn setCursor(&mut self, x: i16, y: i16) {
        self._cursor_x = x;
        self._cursor_y = y;
    }

    /**
     *
     */
    pub fn setTextColor(&mut self, c: u16) {
        self._text_color = c;
        self._text_bg = c;
    }

    /**
     *
     */
    pub fn setTextColor_bg(&mut self, c: u16, bg: u16) {
        self._text_color = c;
        self._text_bg = bg;
    }

    /**
     *
     */
    pub fn setTextSize(&mut self, s: u8) {
        self._text_size_x = if s > 0 { s } else { 1 };
        self._text_size_y = if s > 0 { s } else { 1 };
    }

    /**
     *
     */
    pub fn setRotation(&mut self, r: u8) {
        self._rotation = r & 3;
    }

    /**
     *
     */
    pub fn clearDisplay(&mut self) {
        for byte in self._buffer.iter_mut() {
            *byte = 0;
        }
    }

    /**
     *
     */
    pub fn display(&mut self) {
        ssd1306_display(&self._buffer);
    }

    /**
     *
     */
    pub fn print(&mut self, s: &str) {
        for c in s.bytes() {
            gfx_write(c);
        }
    }
}

/***********************************************************************************************
 * Platform dependent routines. Change these functions implementation based on microcontroller *
 ***********************************************************************************************/

fn ssd1306_begin(_switchcap_vcc: u8, _addr: u8) -> bool {
    // Adafruit_SSD1306::begin(SSD1306_SWITCHCAPVCC, OLED_I2C_ADDRESS)
    true
}

fn ssd1306_draw_pixel(_x: i16, _y: i16, _color: u16) {
    // Adafruit_SSD1306::drawPixel(x, y, color)
}

fn ssd1306_display(_buffer: &[u8]) {
    // Adafruit_SSD1306::display()
}

fn gfx_draw_line(_x0: i16, _y0: i16, _x1: i16, _y1: i16, _color: u16) {
    // Adafruit_GFX::drawLine(x0, y0, x1, y1, color)
}

fn gfx_draw_rect(_x: i16, _y: i16, _w: i16, _h: i16, _color: u16) {
    // Adafruit_GFX::drawRect(x, y, w, h, color)
}

fn gfx_fill_rect(_x: i16, _y: i16, _w: i16, _h: i16, _color: u16) {
    // Adafruit_GFX::fillRect(x, y, w, h, color)
}

fn gfx_draw_circle(_x0: i16, _y0: i16, _r: i16, _color: u16) {
    // Adafruit_GFX::drawCircle(x0, y0, r, color)
}

fn gfx_draw_circle_helper(_x0: i16, _y0: i16, _r: i16, _cornername: u8, _color: u16) {
    // Adafruit_GFX::drawCircleHelper(x0, y0, r, cornername, color)
}

fn gfx_fill_circle(_x0: i16, _y0: i16, _r: i16, _color: u16) {
    // Adafruit_GFX::fillCircle(x0, y0, r, color)
}

fn gfx_fill_circle_helper(_x0: i16, _y0: i16, _r: i16, _cornername: u8, _delta: i16, _color: u16) {
    // Adafruit_GFX::fillCircleHelper(x0, y0, r, cornername, delta, color)
}

fn gfx_draw_triangle(_x0: i16, _y0: i16, _x1: i16, _y1: i16, _x2: i16, _y2: i16, _color: u16) {
    // Adafruit_GFX::drawTriangle(x0, y0, x1, y1, x2, y2, color)
}

fn gfx_fill_triangle(_x0: i16, _y0: i16, _x1: i16, _y1: i16, _x2: i16, _y2: i16, _color: u16) {
    // Adafruit_GFX::fillTriangle(x0, y0, x1, y1, x2, y2, color)
}

fn gfx_draw_round_rect(_x0: i16, _y0: i16, _w: i16, _h: i16, _radius: i16, _color: u16) {
    // Adafruit_GFX::drawRoundRect(x0, y0, w, h, radius, color)
}

fn gfx_fill_round_rect(_x0: i16, _y0: i16, _w: i16, _h: i16, _radius: i16, _color: u16) {
    // Adafruit_GFX::fillRoundRect(x0, y0, w, h, radius, color)
}

fn gfx_draw_bitmap(_x: i16, _y: i16, _bitmap: &[u8], _w: i16, _h: i16, _color: u16) {
    // Adafruit_GFX::drawBitmap(x, y, bitmap, w, h, color)
}

fn gfx_draw_bitmap_bg(_x: i16, _y: i16, _bitmap: &[u8], _w: i16, _h: i16, _color: u16, _bg: u16) {
    // Adafruit_GFX::drawBitmap(x, y, bitmap, w, h, color, bg)
}

fn gfx_draw_x_bitmap(_x: i16, _y: i16, _bitmap: &[u8], _w: i16, _h: i16, _color: u16) {
    // Adafruit_GFX::drawXBitmap(x, y, bitmap, w, h, color)
}

fn gfx_draw_char(_x: i16, _y: i16, _c: u8, _color: u16, _bg: u16, _size_x: u8, _size_y: u8) {
    // Adafruit_GFX::drawChar(x, y, c, color, bg, size_x, size_y)
}

fn gfx_write(_c: u8) {
    // Adafruit_GFX::write(c)
}

fn delay(_ms: u32) {
    // Arduino delay(ms)
}
