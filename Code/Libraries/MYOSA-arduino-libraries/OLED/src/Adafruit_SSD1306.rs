/*!
 * @file Adafruit_SSD1306.rs
 *
 * This is part of for Adafruit's SSD1306 library for monochrome
 * OLED displays: http://www.adafruit.com/category/63_98
 *
 * These displays use I2C or SPI to communicate. I2C requires 2 pins
 * (SCL+SDA) and optionally a RESET pin. SPI requires 4 pins (MOSI, SCK,
 * select, data/command) and optionally a reset pin. Hardware SPI or
 * 'bitbang' software SPI are both supported.
 *
 * Adafruit invests time and resources providing this open source code,
 * please support Adafruit and open-source hardware by purchasing
 * products from Adafruit!
 *
 * Written by Limor Fried/Ladyada for Adafruit Industries, with
 * contributions from the open source community.
 *
 * BSD license, all text above, and the splash screen header file,
 * must be included in any redistribution.
 *
 */

/*!
 * @mainpage Rust port of Arduino library for monochrome OLEDs based on SSD1306 drivers.
 *
 * @section intro_sec Introduction
 *
 * This is documentation for Adafruit's SSD1306 library for monochrome
 * OLED displays: http://www.adafruit.com/category/63_98
 *
 * These displays use I2C or SPI to communicate. I2C requires 2 pins
 * (SCL+SDA) and optionally a RESET pin. SPI requires 4 pins (MOSI, SCK,
 * select, data/command) and optionally a reset pin. Hardware SPI or
 * 'bitbang' software SPI are both supported.
 *
 * Adafruit invests time and resources providing this open source code,
 * please support Adafruit and open-source hardware by purchasing
 * products from Adafruit!
 *
 * @section dependencies Dependencies
 *
 * This library depends on <a
 * href="https://github.com/adafruit/Adafruit-GFX-Library"> Adafruit_GFX</a>
 * being present on your system. Please make sure you have installed the latest
 * version before using this library.
 *
 * @section author Author
 *
 * Written by Limor Fried/Ladyada for Adafruit Industries, with
 * contributions from the open source community.
 *
 * @section license License
 *
 * BSD license, all text above, and the splash screen included below,
 * must be included in any redistribution.
 *
 */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Color constants
// ---------------------------------------------------------------------------

/// The following "raw" color names are kept for backwards client compatability
/// They can be disabled by predefining this macro before including the Adafruit
/// header client code will then need to be modified to use the scoped enum
/// values directly

/// Draw 'off' pixels
pub const SSD1306_BLACK: u16 = 0;
/// Draw 'on' pixels
pub const SSD1306_WHITE: u16 = 1;
/// Invert pixels
pub const SSD1306_INVERSE: u16 = 2;

/// Convenience aliases (matching NO_ADAFRUIT_SSD1306_COLOR_COMPATIBILITY unset)
pub const BLACK: u16 = SSD1306_BLACK;
pub const WHITE: u16 = SSD1306_WHITE;
pub const INVERSE: u16 = SSD1306_INVERSE;

// ---------------------------------------------------------------------------
// SSD1306 command constants
// ---------------------------------------------------------------------------

pub const SSD1306_MEMORYMODE: u8 = 0x20;          ///< See datasheet
pub const SSD1306_COLUMNADDR: u8 = 0x21;          ///< See datasheet
pub const SSD1306_PAGEADDR: u8 = 0x22;            ///< See datasheet
pub const SSD1306_SETCONTRAST: u8 = 0x81;         ///< See datasheet
pub const SSD1306_CHARGEPUMP: u8 = 0x8D;          ///< See datasheet
pub const SSD1306_SEGREMAP: u8 = 0xA0;            ///< See datasheet
pub const SSD1306_DISPLAYALLON_RESUME: u8 = 0xA4; ///< See datasheet
pub const SSD1306_DISPLAYALLON: u8 = 0xA5;        ///< Not currently used
pub const SSD1306_NORMALDISPLAY: u8 = 0xA6;       ///< See datasheet
pub const SSD1306_INVERTDISPLAY: u8 = 0xA7;       ///< See datasheet
pub const SSD1306_SETMULTIPLEX: u8 = 0xA8;        ///< See datasheet
pub const SSD1306_DISPLAYOFF: u8 = 0xAE;          ///< See datasheet
pub const SSD1306_DISPLAYON: u8 = 0xAF;           ///< See datasheet
pub const SSD1306_COMSCANINC: u8 = 0xC0;          ///< Not currently used
pub const SSD1306_COMSCANDEC: u8 = 0xC8;          ///< See datasheet
pub const SSD1306_SETDISPLAYOFFSET: u8 = 0xD3;    ///< See datasheet
pub const SSD1306_SETDISPLAYCLOCKDIV: u8 = 0xD5;  ///< See datasheet
pub const SSD1306_SETPRECHARGE: u8 = 0xD9;        ///< See datasheet
pub const SSD1306_SETCOMPINS: u8 = 0xDA;          ///< See datasheet
pub const SSD1306_SETVCOMDETECT: u8 = 0xDB;       ///< See datasheet

pub const SSD1306_SETLOWCOLUMN: u8 = 0x00;  ///< Not currently used
pub const SSD1306_SETHIGHCOLUMN: u8 = 0x10; ///< Not currently used
pub const SSD1306_SETSTARTLINE: u8 = 0x40;  ///< See datasheet

pub const SSD1306_EXTERNALVCC: u8 = 0x01;  ///< External display voltage source
pub const SSD1306_SWITCHCAPVCC: u8 = 0x02; ///< Gen. display voltage from 3.3V

pub const SSD1306_RIGHT_HORIZONTAL_SCROLL: u8 = 0x26;              ///< Init rt scroll
pub const SSD1306_LEFT_HORIZONTAL_SCROLL: u8 = 0x27;               ///< Init left scroll
pub const SSD1306_VERTICAL_AND_RIGHT_HORIZONTAL_SCROLL: u8 = 0x29; ///< Init diag scroll
pub const SSD1306_VERTICAL_AND_LEFT_HORIZONTAL_SCROLL: u8 = 0x2A;  ///< Init diag scroll
pub const SSD1306_DEACTIVATE_SCROLL: u8 = 0x2E;                    ///< Stop scroll
pub const SSD1306_ACTIVATE_SCROLL: u8 = 0x2F;                      ///< Start scroll
pub const SSD1306_SET_VERTICAL_SCROLL_AREA: u8 = 0xA3;             ///< Set scroll range

/// Use common Arduino core default
pub const WIRE_MAX: usize = 32;

// ---------------------------------------------------------------------------
// Hardware-abstraction traits
// ---------------------------------------------------------------------------

/// Trait abstracting I2C bus operations needed by the SSD1306 driver.
pub trait I2cBus {
    fn begin(&mut self);
    fn begin_transmission(&mut self, addr: u8);
    fn write_byte(&mut self, data: u8);
    fn end_transmission(&mut self);
    fn set_clock(&mut self, freq: u32);
}

/// Trait abstracting SPI bus operations needed by the SSD1306 driver.
pub trait SpiBus {
    fn begin(&mut self);
    fn transfer(&mut self, data: u8) -> u8;
    fn begin_transaction(&mut self);
    fn end_transaction(&mut self);
}

/// Trait abstracting GPIO pin control.
pub trait GpioPin {
    fn pin_mode_output(&mut self, pin: i8);
    fn digital_write(&mut self, pin: i8, value: bool);
}

/// Trait for a delay provider.
pub trait DelayProvider {
    fn delay_ms(&mut self, ms: u32);
}

// ---------------------------------------------------------------------------
// SSD1306 bus configuration enum
// ---------------------------------------------------------------------------

/// Describes which bus the SSD1306 is connected through.
pub enum SSD1306Bus<I: I2cBus, S: SpiBus> {
    /// I2C connection
    I2C {
        wire: I,
        wire_clk: u32,
        restore_clk: u32,
    },
    /// Hardware SPI connection
    HardwareSPI {
        spi: S,
        dc_pin: i8,
        cs_pin: i8,
    },
    /// Software (bitbang) SPI connection
    SoftSPI {
        mosi_pin: i8,
        clk_pin: i8,
        dc_pin: i8,
        cs_pin: i8,
    },
}

// ---------------------------------------------------------------------------
// Adafruit_SSD1306 struct
// ---------------------------------------------------------------------------

use super::Adafruit_GFX::{Adafruit_GFX, DrawPixel};

/*!
    @brief  Class that stores state and functions for interacting with
            SSD1306 OLED displays.
*/
pub struct Adafruit_SSD1306<I: I2cBus, S: SpiBus, G: GpioPin, D: DelayProvider> {
    pub gfx: Adafruit_GFX,
    bus: SSD1306Bus<I, S>,
    gpio: G,
    delay: D,
    buffer: Vec<u8>,
    i2caddr: u8,
    vccstate: u8,
    page_end: i8,
    rst_pin: i8,
    contrast: u8,
}

// CONSTRUCTORS, DESTRUCTOR ------------------------------------------------

impl<I: I2cBus, S: SpiBus, G: GpioPin, D: DelayProvider> Adafruit_SSD1306<I, S, G, D> {
    /*!
        @brief  Constructor for I2C-interfaced SSD1306 displays.
        @param  w
                Display width in pixels
        @param  h
                Display height in pixels
        @param  wire
                I2C bus instance
        @param  rst_pin
                Reset pin (using Arduino pin numbering), or -1 if not used
        @param  clk_during
                Speed (in Hz) for Wire transmissions in SSD1306 library calls.
        @param  clk_after
                Speed (in Hz) for Wire transmissions following SSD1306 library calls.
        @return Adafruit_SSD1306 object.
        @note   Call the object's begin() function before use -- buffer
                allocation is performed there!
    */
    pub fn new_i2c(
        w: u8,
        h: u8,
        wire: I,
        gpio: G,
        delay: D,
        rst_pin: i8,
        clk_during: u32,
        clk_after: u32,
    ) -> Self
    where
        S: Default,
    {
        Adafruit_SSD1306 {
            gfx: Adafruit_GFX::new(w as i16, h as i16),
            bus: SSD1306Bus::I2C {
                wire,
                wire_clk: clk_during,
                restore_clk: clk_after,
            },
            gpio,
            delay,
            buffer: Vec::new(),
            i2caddr: 0,
            vccstate: 0,
            page_end: 0,
            rst_pin,
            contrast: 0,
        }
    }

    /*!
        @brief  Constructor for SPI SSD1306 displays, using software (bitbang)
                SPI.
    */
    pub fn new_soft_spi(
        w: u8,
        h: u8,
        mosi_pin: i8,
        sclk_pin: i8,
        dc_pin: i8,
        rst_pin: i8,
        cs_pin: i8,
        gpio: G,
        delay: D,
    ) -> Self
    where
        I: Default,
        S: Default,
    {
        Adafruit_SSD1306 {
            gfx: Adafruit_GFX::new(w as i16, h as i16),
            bus: SSD1306Bus::SoftSPI {
                mosi_pin,
                clk_pin: sclk_pin,
                dc_pin,
                cs_pin,
            },
            gpio,
            delay,
            buffer: Vec::new(),
            i2caddr: 0,
            vccstate: 0,
            page_end: 0,
            rst_pin,
            contrast: 0,
        }
    }

    /*!
        @brief  Constructor for SPI SSD1306 displays, using native hardware SPI.
    */
    pub fn new_hw_spi(
        w: u8,
        h: u8,
        spi: S,
        dc_pin: i8,
        rst_pin: i8,
        cs_pin: i8,
        gpio: G,
        delay: D,
    ) -> Self
    where
        I: Default,
    {
        Adafruit_SSD1306 {
            gfx: Adafruit_GFX::new(w as i16, h as i16),
            bus: SSD1306Bus::HardwareSPI {
                spi,
                dc_pin,
                cs_pin,
            },
            gpio,
            delay,
            buffer: Vec::new(),
            i2caddr: 0,
            vccstate: 0,
            page_end: 0,
            rst_pin,
            contrast: 0,
        }
    }

    // LOW-LEVEL UTILS ---------------------------------------------------------

    // Issue single byte out SPI, either soft or hardware as appropriate.
    // SPI transaction/selection must be performed in calling function.
    fn spi_write(&mut self, d: u8) {
        match &mut self.bus {
            SSD1306Bus::HardwareSPI { spi, .. } => {
                let _ = spi.transfer(d);
            }
            SSD1306Bus::SoftSPI {
                mosi_pin,
                clk_pin,
                ..
            } => {
                let mosi = *mosi_pin;
                let clk = *clk_pin;
                for shift in (0..8).rev() {
                    self.gpio.digital_write(mosi, (d & (1 << shift)) != 0);
                    self.gpio.digital_write(clk, true);  // Clock high
                    self.gpio.digital_write(clk, false); // Clock low
                }
            }
            _ => {}
        }
    }

    // Issue single command to SSD1306, using I2C or hard/soft SPI as needed.
    // Because command calls are often grouped, SPI transaction and selection
    // must be started/ended in calling function for efficiency.
    // This is a private function, not exposed (see ssd1306_command() instead).
    fn ssd1306_command1(&mut self, c: u8) {
        match &mut self.bus {
            SSD1306Bus::I2C { wire, .. } => {
                wire.begin_transmission(self.i2caddr);
                wire.write_byte(0x00); // Co = 0, D/C = 0
                wire.write_byte(c);
                wire.end_transmission();
            }
            SSD1306Bus::HardwareSPI { dc_pin, .. } | SSD1306Bus::SoftSPI { dc_pin, .. } => {
                let pin = *dc_pin;
                self.gpio.digital_write(pin, false); // Command mode
                self.spi_write(c);
            }
        }
    }

    // Issue list of commands to SSD1306, same rules as above re: transactions.
    // This is a private function, not exposed.
    fn ssd1306_command_list(&mut self, commands: &[u8]) {
        match &mut self.bus {
            SSD1306Bus::I2C { wire, .. } => {
                wire.begin_transmission(self.i2caddr);
                wire.write_byte(0x00); // Co = 0, D/C = 0
                let mut bytes_out: usize = 1;
                for &c in commands {
                    if bytes_out >= WIRE_MAX {
                        wire.end_transmission();
                        wire.begin_transmission(self.i2caddr);
                        wire.write_byte(0x00); // Co = 0, D/C = 0
                        bytes_out = 1;
                    }
                    wire.write_byte(c);
                    bytes_out += 1;
                }
                wire.end_transmission();
            }
            SSD1306Bus::HardwareSPI { dc_pin, .. } | SSD1306Bus::SoftSPI { dc_pin, .. } => {
                let pin = *dc_pin;
                self.gpio.digital_write(pin, false); // Command mode
                for &c in commands {
                    self.spi_write(c);
                }
            }
        }
    }

    fn transaction_start(&mut self) {
        match &mut self.bus {
            SSD1306Bus::I2C { wire, wire_clk, .. } => {
                wire.set_clock(*wire_clk);
            }
            SSD1306Bus::HardwareSPI { spi, cs_pin, .. } => {
                spi.begin_transaction();
                let pin = *cs_pin;
                self.gpio.digital_write(pin, false); // Select
            }
            SSD1306Bus::SoftSPI { cs_pin, .. } => {
                let pin = *cs_pin;
                self.gpio.digital_write(pin, false); // Select
            }
        }
    }

    fn transaction_end(&mut self) {
        match &mut self.bus {
            SSD1306Bus::I2C { wire, restore_clk, .. } => {
                wire.set_clock(*restore_clk);
            }
            SSD1306Bus::HardwareSPI { spi, cs_pin, .. } => {
                let pin = *cs_pin;
                self.gpio.digital_write(pin, true); // Deselect
                spi.end_transaction();
            }
            SSD1306Bus::SoftSPI { cs_pin, .. } => {
                let pin = *cs_pin;
                self.gpio.digital_write(pin, true); // Deselect
            }
        }
    }

    // A public version of ssd1306_command1(), for existing user code that
    // might rely on that function. This encapsulates the command transfer
    // in a transaction start/end, similar to old library's handling of it.
    /*!
        @brief  Issue a single low-level command directly to the SSD1306
                display, bypassing the library.
        @param  c
                Command to issue (0x00 to 0xFF, see datasheet).
        @return None (void).
    */
    pub fn ssd1306_command(&mut self, c: u8) {
        self.transaction_start();
        self.ssd1306_command1(c);
        self.transaction_end();
    }

    // ALLOCATE & INIT DISPLAY -------------------------------------------------

    /*!
        @brief  Allocate RAM for image buffer, initialize peripherals and pins.
        @param  vcs
                VCC selection. Pass SSD1306_SWITCHCAPVCC to generate the display
                voltage (step up) from the 3.3V source, or SSD1306_EXTERNALVCC
                otherwise.
        @param  addr
                I2C address of corresponding SSD1306 display (or pass 0 to use
                default of 0x3C for 128x32 display, 0x3D for all others).
        @param  reset
                If true, and if the reset pin passed to the constructor is
                valid, a hard reset will be performed before initializing the
                display.
        @param  periph_begin
                If true, and if a hardware peripheral is being used (I2C or SPI,
                but not software SPI), call that peripheral's begin() function.
        @return true on successful allocation/init, false otherwise.
        @note   MUST call this function before any drawing or updates!
    */
    pub fn begin(
        &mut self,
        vcs: u8,
        addr: u8,
        reset: bool,
        periph_begin: bool,
    ) -> bool {
        let w = self.gfx.WIDTH;
        let h = self.gfx.HEIGHT;

        if self.buffer.is_empty() {
            let buf_size = (w as usize) * (((h as usize) + 7) / 8);
            self.buffer = vec![0u8; buf_size];
        }

        self.clear_display();

        // splash screen would be drawn here (omitted -- platform-specific bitmap)

        self.vccstate = vcs;

        // Setup pin directions
        match &mut self.bus {
            SSD1306Bus::I2C { wire, .. } => {
                self.i2caddr = if addr != 0 {
                    addr
                } else if h == 32 {
                    0x3C
                } else {
                    0x3D
                };
                if periph_begin {
                    wire.begin();
                }
            }
            SSD1306Bus::HardwareSPI { spi, dc_pin, cs_pin, .. } => {
                let dc = *dc_pin;
                let cs = *cs_pin;
                self.gpio.pin_mode_output(dc);
                self.gpio.pin_mode_output(cs);
                self.gpio.digital_write(cs, true); // Deselect
                if periph_begin {
                    spi.begin();
                }
            }
            SSD1306Bus::SoftSPI { mosi_pin, clk_pin, dc_pin, cs_pin } => {
                let dc = *dc_pin;
                let cs = *cs_pin;
                let mosi = *mosi_pin;
                let clk = *clk_pin;
                self.gpio.pin_mode_output(dc);
                self.gpio.pin_mode_output(cs);
                self.gpio.digital_write(cs, true); // Deselect
                self.gpio.pin_mode_output(mosi);
                self.gpio.pin_mode_output(clk);
                self.gpio.digital_write(clk, false); // Clock low
            }
        }

        // Reset SSD1306 if requested and reset pin specified in constructor
        if reset && (self.rst_pin >= 0) {
            self.gpio.pin_mode_output(self.rst_pin);
            self.gpio.digital_write(self.rst_pin, true);
            self.delay.delay_ms(1);                      // VDD goes high at start, pause for 1 ms
            self.gpio.digital_write(self.rst_pin, false); // Bring reset low
            self.delay.delay_ms(10);                      // Wait 10 ms
            self.gpio.digital_write(self.rst_pin, true);  // Bring out of reset
        }

        self.transaction_start();

        // Init sequence
        let init1: [u8; 4] = [
            SSD1306_DISPLAYOFF,        // 0xAE
            SSD1306_SETDISPLAYCLOCKDIV, // 0xD5
            0x80,                       // the suggested ratio 0x80
            SSD1306_SETMULTIPLEX,       // 0xA8
        ];
        self.ssd1306_command_list(&init1);
        self.ssd1306_command1((h - 1) as u8);

        let init2: [u8; 4] = [
            SSD1306_SETDISPLAYOFFSET,      // 0xD3
            0x0,                            // no offset
            SSD1306_SETSTARTLINE | 0x0,     // line #0
            SSD1306_CHARGEPUMP,             // 0x8D
        ];
        self.ssd1306_command_list(&init2);

        self.ssd1306_command1(if self.vccstate == SSD1306_EXTERNALVCC { 0x10 } else { 0x14 });

        let init3: [u8; 4] = [
            SSD1306_MEMORYMODE, // 0x20
            0x00,               // 0x0 act like ks0108
            SSD1306_SEGREMAP | 0x1,
            SSD1306_COMSCANDEC,
        ];
        self.ssd1306_command_list(&init3);

        let mut com_pins: u8 = 0x02;
        self.contrast = 0x8F;

        if w == 128 && h == 32 {
            com_pins = 0x02;
            self.contrast = 0x8F;
        } else if w == 128 && h == 64 {
            com_pins = 0x12;
            self.contrast = if self.vccstate == SSD1306_EXTERNALVCC { 0x9F } else { 0xCF };
        } else if w == 96 && h == 16 {
            com_pins = 0x2; // ada x12
            self.contrast = if self.vccstate == SSD1306_EXTERNALVCC { 0x10 } else { 0xAF };
        }
        // Other screen varieties -- TBD

        self.ssd1306_command1(SSD1306_SETCOMPINS);
        self.ssd1306_command1(com_pins);
        self.ssd1306_command1(SSD1306_SETCONTRAST);
        self.ssd1306_command1(self.contrast);

        self.ssd1306_command1(SSD1306_SETPRECHARGE); // 0xd9
        self.ssd1306_command1(if self.vccstate == SSD1306_EXTERNALVCC { 0x22 } else { 0xF1 });

        let init5: [u8; 6] = [
            SSD1306_SETVCOMDETECT,       // 0xDB
            0x40,
            SSD1306_DISPLAYALLON_RESUME, // 0xA4
            SSD1306_NORMALDISPLAY,       // 0xA6
            SSD1306_DEACTIVATE_SCROLL,
            SSD1306_DISPLAYON,           // Main screen turn on
        ];
        self.ssd1306_command_list(&init5);

        self.transaction_end();

        true // Success
    }

    // DRAWING FUNCTIONS -------------------------------------------------------

    /*!
        @brief  Set/clear/invert a single pixel. This is also invoked by the
                Adafruit_GFX library in generating many higher-level graphics
                primitives.
        @param  x
                Column of display -- 0 at left to (screen width - 1) at right.
        @param  y
                Row of display -- 0 at top to (screen height -1) at bottom.
        @param  color
                Pixel color, one of: SSD1306_BLACK, SSD1306_WHITE or SSD1306_INVERSE.
        @return None (void).
        @note   Changes buffer contents only, no immediate effect on display.
                Follow up with a call to display(), or with other graphics
                commands as needed by one's own application.
    */
    pub fn draw_pixel(&mut self, x: i16, y: i16, color: u16) {
        let w = self.gfx.WIDTH;
        let h = self.gfx.HEIGHT;
        if x >= 0 && x < self.gfx.width() && y >= 0 && y < self.gfx.height() {
            let mut rx = x;
            let mut ry = y;
            // Pixel is in-bounds. Rotate coordinates if needed.
            match self.gfx.get_rotation() {
                1 => {
                    core::mem::swap(&mut rx, &mut ry);
                    rx = w - rx - 1;
                }
                2 => {
                    rx = w - rx - 1;
                    ry = h - ry - 1;
                }
                3 => {
                    core::mem::swap(&mut rx, &mut ry);
                    ry = h - ry - 1;
                }
                _ => {}
            }
            let idx = (rx + (ry / 8) * w) as usize;
            let bit = 1u8 << (ry & 7);
            match color {
                SSD1306_WHITE => {
                    self.buffer[idx] |= bit;
                }
                SSD1306_BLACK => {
                    self.buffer[idx] &= !bit;
                }
                SSD1306_INVERSE => {
                    self.buffer[idx] ^= bit;
                }
                _ => {}
            }
        }
    }

    /*!
        @brief  Clear contents of display buffer (set all pixels to off).
        @return None (void).
        @note   Changes buffer contents only, no immediate effect on display.
                Follow up with a call to display(), or with other graphics
                commands as needed by one's own application.
    */
    pub fn clear_display(&mut self) {
        let w = self.gfx.WIDTH as usize;
        let h = self.gfx.HEIGHT as usize;
        let size = w * ((h + 7) / 8);
        for b in self.buffer[..size].iter_mut() {
            *b = 0;
        }
    }

    /*!
        @brief  Draw a horizontal line. This is also invoked by the Adafruit_GFX
                library in generating many higher-level graphics primitives.
        @param  x
                Leftmost column -- 0 at left to (screen width - 1) at right.
        @param  y
                Row of display -- 0 at top to (screen height -1) at bottom.
        @param  w
                Width of line, in pixels.
        @param  color
                Line color, one of: SSD1306_BLACK, SSD1306_WHITE or SSD1306_INVERT.
        @return None (void).
        @note   Changes buffer contents only, no immediate effect on display.
    */
    pub fn draw_fast_h_line(&mut self, mut x: i16, mut y: i16, w: i16, color: u16) {
        let mut b_swap = false;
        let width = self.gfx.WIDTH;
        let height = self.gfx.HEIGHT;
        let mut w = w;

        match self.gfx.rotation {
            1 => {
                // 90 degree rotation, swap x & y for rotation, then invert x
                b_swap = true;
                core::mem::swap(&mut x, &mut y);
                x = width - x - 1;
            }
            2 => {
                // 180 degree rotation, invert x and y, then shift y around for height.
                x = width - x - 1;
                y = height - y - 1;
                x -= w - 1;
            }
            3 => {
                // 270 degree rotation, swap x & y for rotation,
                // then invert y and adjust y for w (not to become h)
                b_swap = true;
                core::mem::swap(&mut x, &mut y);
                y = height - y - 1;
                y -= w - 1;
            }
            _ => {}
        }

        if b_swap {
            self.draw_fast_v_line_internal(x, y, w, color);
        } else {
            self.draw_fast_h_line_internal(x, y, w, color);
        }
    }

    fn draw_fast_h_line_internal(&mut self, mut x: i16, y: i16, mut w: i16, color: u16) {
        let width = self.gfx.WIDTH;
        let height = self.gfx.HEIGHT;

        if y >= 0 && y < height {
            if x < 0 {
                w += x;
                x = 0;
            }
            if (x + w) > width {
                w = width - x;
            }
            if w > 0 {
                let mask = 1u8 << (y & 7);
                let mut idx = ((y / 8) * width + x) as usize;
                match color {
                    SSD1306_WHITE => {
                        let mut remaining = w;
                        while remaining > 0 {
                            self.buffer[idx] |= mask;
                            idx += 1;
                            remaining -= 1;
                        }
                    }
                    SSD1306_BLACK => {
                        let inv_mask = !mask;
                        let mut remaining = w;
                        while remaining > 0 {
                            self.buffer[idx] &= inv_mask;
                            idx += 1;
                            remaining -= 1;
                        }
                    }
                    SSD1306_INVERSE => {
                        let mut remaining = w;
                        while remaining > 0 {
                            self.buffer[idx] ^= mask;
                            idx += 1;
                            remaining -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /*!
        @brief  Draw a vertical line. This is also invoked by the Adafruit_GFX
                library in generating many higher-level graphics primitives.
        @param  x
                Column of display -- 0 at left to (screen width -1) at right.
        @param  y
                Topmost row -- 0 at top to (screen height - 1) at bottom.
        @param  h
                Height of line, in pixels.
        @param  color
                Line color, one of: SSD1306_BLACK, SSD1306_WHITE or SSD1306_INVERT.
        @return None (void).
        @note   Changes buffer contents only, no immediate effect on display.
    */
    pub fn draw_fast_v_line(&mut self, mut x: i16, mut y: i16, mut h: i16, color: u16) {
        let mut b_swap = false;
        let width = self.gfx.WIDTH;
        let height = self.gfx.HEIGHT;

        match self.gfx.rotation {
            1 => {
                // 90 degree rotation, swap x & y for rotation,
                // then invert x and adjust x for h (now to become w)
                b_swap = true;
                core::mem::swap(&mut x, &mut y);
                x = width - x - 1;
                x -= h - 1;
            }
            2 => {
                // 180 degree rotation, invert x and y, then shift y around for height.
                x = width - x - 1;
                y = height - y - 1;
                y -= h - 1;
            }
            3 => {
                // 270 degree rotation, swap x & y for rotation, then invert y
                b_swap = true;
                core::mem::swap(&mut x, &mut y);
                y = height - y - 1;
            }
            _ => {}
        }

        if b_swap {
            self.draw_fast_h_line_internal(x, y, h, color);
        } else {
            self.draw_fast_v_line_internal(x, y, h, color);
        }
    }

    fn draw_fast_v_line_internal(&mut self, x: i16, mut y: i16, mut h: i16, color: u16) {
        let width = self.gfx.WIDTH;
        let height = self.gfx.HEIGHT;

        if x >= 0 && x < width {
            if y < 0 {
                h += y;
                y = 0;
            }
            if (y + h) > height {
                h = height - y;
            }
            if h > 0 {
                let mut uy = y as u8;
                let mut uh = h as u8;
                let mut p_buf = ((uy as i16 / 8) * width + x) as usize;

                // do the first partial byte, if necessary - this requires some masking
                let modval = (uy & 7) as u8;
                if modval != 0 {
                    let modval = 8 - modval;
                    // note - lookup table results in a nearly 10% performance
                    // improvement in fill* functions
                    static PREMASK: [u8; 8] = [0x00, 0x80, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC, 0xFE];
                    let mut mask = PREMASK[modval as usize];
                    // adjust the mask if we're not going to reach the end of this byte
                    if uh < modval {
                        mask &= 0xFF >> (modval - uh);
                    }

                    match color {
                        SSD1306_WHITE => {
                            self.buffer[p_buf] |= mask;
                        }
                        SSD1306_BLACK => {
                            self.buffer[p_buf] &= !mask;
                        }
                        SSD1306_INVERSE => {
                            self.buffer[p_buf] ^= mask;
                        }
                        _ => {}
                    }
                    p_buf += width as usize;
                }

                if uh >= modval {
                    // More to go?
                    uh -= modval;
                    // Write solid bytes while we can - effectively 8 rows at a time
                    if uh >= 8 {
                        if color == SSD1306_INVERSE as u16 {
                            // separate copy of the code so we don't impact performance of
                            // black/white write version with an extra comparison per loop
                            loop {
                                self.buffer[p_buf] ^= 0xFF; // Invert byte
                                p_buf += width as usize;     // Advance pointer 8 rows
                                uh -= 8;                     // Subtract 8 rows from height
                                if uh < 8 {
                                    break;
                                }
                            }
                        } else {
                            // store a local value to work with
                            let val: u8 = if color != SSD1306_BLACK as u16 { 255 } else { 0 };
                            loop {
                                self.buffer[p_buf] = val;    // Set byte
                                p_buf += width as usize;     // Advance pointer 8 rows
                                uh -= 8;                     // Subtract 8 rows from height
                                if uh < 8 {
                                    break;
                                }
                            }
                        }
                    }

                    if uh > 0 {
                        // Do the final partial byte, if necessary
                        let modval = uh & 7;
                        // this time we want to mask the low bits of the byte,
                        // vs the high bits we did above
                        static POSTMASK: [u8; 8] = [0x00, 0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3F, 0x7F];
                        let mask = POSTMASK[modval as usize];
                        match color {
                            SSD1306_WHITE => {
                                self.buffer[p_buf] |= mask;
                            }
                            SSD1306_BLACK => {
                                self.buffer[p_buf] &= !mask;
                            }
                            SSD1306_INVERSE => {
                                self.buffer[p_buf] ^= mask;
                            }
                            _ => {}
                        }
                    }
                }
            } // endif positive height
        }   // endif x in bounds
    }

    /*!
        @brief  Return color of a single pixel in display buffer.
        @param  x
                Column of display -- 0 at left to (screen width - 1) at right.
        @param  y
                Row of display -- 0 at top to (screen height -1) at bottom.
        @return true if pixel is set (usually SSD1306_WHITE, unless display invert
       mode is enabled), false if clear (SSD1306_BLACK).
        @note   Reads from buffer contents; may not reflect current contents of
                screen if display() has not been called.
    */
    pub fn get_pixel(&self, x: i16, y: i16) -> bool {
        let w = self.gfx.WIDTH;
        let h = self.gfx.HEIGHT;
        if x >= 0 && x < self.gfx.width() && y >= 0 && y < self.gfx.height() {
            let mut rx = x;
            let mut ry = y;
            // Pixel is in-bounds. Rotate coordinates if needed.
            match self.gfx.get_rotation() {
                1 => {
                    core::mem::swap(&mut rx, &mut ry);
                    rx = w - rx - 1;
                }
                2 => {
                    rx = w - rx - 1;
                    ry = h - ry - 1;
                }
                3 => {
                    core::mem::swap(&mut rx, &mut ry);
                    ry = h - ry - 1;
                }
                _ => {}
            }
            return (self.buffer[(rx + (ry / 8) * w) as usize] & (1 << (ry & 7))) != 0;
        }
        false // Pixel out of bounds
    }

    /*!
        @brief  Get base address of display buffer for direct reading or writing.
        @return Reference to an unsigned 8-bit slice, column-major, columns padded
                to full byte boundary if needed.
    */
    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    // REFRESH DISPLAY ---------------------------------------------------------

    /*!
        @brief  Push data currently in RAM to SSD1306 display.
        @return None (void).
        @note   Drawing operations are not visible until this function is
                called. Call after each graphics command, or after a whole set
                of graphics commands, as best needed by one's own application.
    */
    pub fn display(&mut self) {
        let w = self.gfx.WIDTH;
        let h = self.gfx.HEIGHT;

        self.transaction_start();
        let dlist1: [u8; 5] = [
            SSD1306_PAGEADDR,
            0,        // Page start address
            0xFF,     // Page end (not really, but works here)
            SSD1306_COLUMNADDR,
            0,        // Column start address
        ];
        self.ssd1306_command_list(&dlist1);
        self.ssd1306_command1((w - 1) as u8); // Column end address

        let count = (w as usize) * (((h as usize) + 7) / 8);

        match &mut self.bus {
            SSD1306Bus::I2C { wire, .. } => {
                wire.begin_transmission(self.i2caddr);
                wire.write_byte(0x40);
                let mut bytes_out: usize = 1;
                for i in 0..count {
                    if bytes_out >= WIRE_MAX {
                        wire.end_transmission();
                        wire.begin_transmission(self.i2caddr);
                        wire.write_byte(0x40);
                        bytes_out = 1;
                    }
                    wire.write_byte(self.buffer[i]);
                    bytes_out += 1;
                }
                wire.end_transmission();
            }
            SSD1306Bus::HardwareSPI { dc_pin, .. } | SSD1306Bus::SoftSPI { dc_pin, .. } => {
                let pin = *dc_pin;
                self.gpio.digital_write(pin, true); // Data mode
                for i in 0..count {
                    self.spi_write(self.buffer[i]);
                }
            }
        }
        self.transaction_end();
    }

    // SCROLLING FUNCTIONS -----------------------------------------------------

    /*!
        @brief  Activate a right-handed scroll for all or part of the display.
        @param  start
                First row.
        @param  stop
                Last row.
        @return None (void).
    */
    // To scroll the whole display, run: display.startscrollright(0x00, 0x0F)
    pub fn start_scroll_right(&mut self, start: u8, stop: u8) {
        self.transaction_start();
        let scroll_list_a: [u8; 2] = [SSD1306_RIGHT_HORIZONTAL_SCROLL, 0x00];
        self.ssd1306_command_list(&scroll_list_a);
        self.ssd1306_command1(start);
        self.ssd1306_command1(0x00);
        self.ssd1306_command1(stop);
        let scroll_list_b: [u8; 3] = [0x00, 0xFF, SSD1306_ACTIVATE_SCROLL];
        self.ssd1306_command_list(&scroll_list_b);
        self.transaction_end();
    }

    /*!
        @brief  Activate a left-handed scroll for all or part of the display.
        @param  start
                First row.
        @param  stop
                Last row.
        @return None (void).
    */
    // To scroll the whole display, run: display.startscrollleft(0x00, 0x0F)
    pub fn start_scroll_left(&mut self, start: u8, stop: u8) {
        self.transaction_start();
        let scroll_list_a: [u8; 2] = [SSD1306_LEFT_HORIZONTAL_SCROLL, 0x00];
        self.ssd1306_command_list(&scroll_list_a);
        self.ssd1306_command1(start);
        self.ssd1306_command1(0x00);
        self.ssd1306_command1(stop);
        let scroll_list_b: [u8; 3] = [0x00, 0xFF, SSD1306_ACTIVATE_SCROLL];
        self.ssd1306_command_list(&scroll_list_b);
        self.transaction_end();
    }

    /*!
        @brief  Activate a diagonal scroll for all or part of the display.
        @param  start
                First row.
        @param  stop
                Last row.
        @return None (void).
    */
    // display.startscrolldiagright(0x00, 0x0F)
    pub fn start_scroll_diag_right(&mut self, start: u8, stop: u8) {
        let h = self.gfx.HEIGHT;
        self.transaction_start();
        let scroll_list_a: [u8; 2] = [SSD1306_SET_VERTICAL_SCROLL_AREA, 0x00];
        self.ssd1306_command_list(&scroll_list_a);
        self.ssd1306_command1(h as u8);
        let scroll_list_b: [u8; 2] = [SSD1306_VERTICAL_AND_RIGHT_HORIZONTAL_SCROLL, 0x00];
        self.ssd1306_command_list(&scroll_list_b);
        self.ssd1306_command1(start);
        self.ssd1306_command1(0x00);
        self.ssd1306_command1(stop);
        let scroll_list_c: [u8; 2] = [0x01, SSD1306_ACTIVATE_SCROLL];
        self.ssd1306_command_list(&scroll_list_c);
        self.transaction_end();
    }

    /*!
        @brief  Activate alternate diagonal scroll for all or part of the display.
        @param  start
                First row.
        @param  stop
                Last row.
        @return None (void).
    */
    // To scroll the whole display, run: display.startscrolldiagleft(0x00, 0x0F)
    pub fn start_scroll_diag_left(&mut self, start: u8, stop: u8) {
        let h = self.gfx.HEIGHT;
        self.transaction_start();
        let scroll_list_a: [u8; 2] = [SSD1306_SET_VERTICAL_SCROLL_AREA, 0x00];
        self.ssd1306_command_list(&scroll_list_a);
        self.ssd1306_command1(h as u8);
        let scroll_list_b: [u8; 2] = [SSD1306_VERTICAL_AND_LEFT_HORIZONTAL_SCROLL, 0x00];
        self.ssd1306_command_list(&scroll_list_b);
        self.ssd1306_command1(start);
        self.ssd1306_command1(0x00);
        self.ssd1306_command1(stop);
        let scroll_list_c: [u8; 2] = [0x01, SSD1306_ACTIVATE_SCROLL];
        self.ssd1306_command_list(&scroll_list_c);
        self.transaction_end();
    }

    /*!
        @brief  Cease a previously-begun scrolling action.
        @return None (void).
    */
    pub fn stop_scroll(&mut self) {
        self.transaction_start();
        self.ssd1306_command1(SSD1306_DEACTIVATE_SCROLL);
        self.transaction_end();
    }

    // OTHER HARDWARE SETTINGS -------------------------------------------------

    /*!
        @brief  Enable or disable display invert mode (white-on-black vs
                black-on-white).
        @param  i
                If true, switch to invert mode (black-on-white), else normal
                mode (white-on-black).
        @return None (void).
        @note   This has an immediate effect on the display, no need to call the
                display() function -- buffer contents are not changed, rather a
                different pixel mode of the display hardware is used. When
                enabled, drawing SSD1306_BLACK (value 0) pixels will actually draw
       white, SSD1306_WHITE (value 1) will draw black.
    */
    pub fn invert_display(&mut self, i: bool) {
        self.transaction_start();
        self.ssd1306_command1(if i { SSD1306_INVERTDISPLAY } else { SSD1306_NORMALDISPLAY });
        self.transaction_end();
    }

    /*!
        @brief  Dim the display.
        @param  dim
                true to enable lower brightness mode, false for full brightness.
        @return None (void).
        @note   This has an immediate effect on the display, no need to call the
                display() function -- buffer contents are not changed.
    */
    pub fn dim(&mut self, dim: bool) {
        // the range of contrast to too small to be really useful
        // it is useful to dim the display
        self.transaction_start();
        self.ssd1306_command1(SSD1306_SETCONTRAST);
        self.ssd1306_command1(if dim { 0 } else { self.contrast });
        self.transaction_end();
    }
}

// Implement DrawPixel so this can be used as backend for Adafruit_GFX methods
impl<I: I2cBus, S: SpiBus, G: GpioPin, D: DelayProvider> DrawPixel
    for Adafruit_SSD1306<I, S, G, D>
{
    fn draw_pixel(&mut self, x: i16, y: i16, color: u16) {
        Adafruit_SSD1306::draw_pixel(self, x, y, color);
    }
}
