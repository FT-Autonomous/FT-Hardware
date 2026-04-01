/*
This is the core graphics library for all our displays, providing a common
set of graphics primitives (points, lines, circles, etc.).  It needs to be
paired with a hardware-specific library for each display device we carry
(to handle the lower-level functions).

Adafruit invests time and resources providing this open source code, please
support Adafruit & open-source hardware by purchasing products from Adafruit!

Copyright (c) 2013 Adafruit Industries.  All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

- Redistributions of source code must retain the above copyright notice,
  this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
 */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::cmp::min;

// ---------------------------------------------------------------------------
// Font structures (from gfxfont.h)
// ---------------------------------------------------------------------------

/// Font data stored PER GLYPH
#[derive(Debug, Clone, Copy)]
pub struct GFXglyph {
    pub bitmap_offset: u16, ///< Pointer into GFXfont->bitmap
    pub width: u8,          ///< Bitmap dimensions in pixels
    pub height: u8,         ///< Bitmap dimensions in pixels
    pub x_advance: u8,      ///< Distance to advance cursor (x axis)
    pub x_offset: i8,       ///< X dist from cursor pos to UL corner
    pub y_offset: i8,       ///< Y dist from cursor pos to UL corner
}

/// Data stored for FONT AS A WHOLE
pub struct GFXfont {
    pub bitmap: &'static [u8],    ///< Glyph bitmaps, concatenated
    pub glyph: &'static [GFXglyph], ///< Glyph array
    pub first: u16,                ///< ASCII extents (first char)
    pub last: u16,                 ///< ASCII extents (last char)
    pub y_advance: u8,             ///< Newline distance (y axis)
}

// ---------------------------------------------------------------------------
// Built-in font placeholder
// ---------------------------------------------------------------------------

/// Placeholder for the built-in 5x7 font (glcdfont.c).
/// In a real port this would be a 1275-byte static array.
pub static BUILTIN_FONT: [u8; 0] = [];

// ---------------------------------------------------------------------------
// DrawPixel trait -- the minimum a subclass must implement
// ---------------------------------------------------------------------------

/// A generic graphics superclass that can handle all sorts of drawing. At a
/// minimum you can subclass and provide draw_pixel(). At a maximum you can do a
/// ton of overriding to optimize. Used for any/all Adafruit displays!
pub trait DrawPixel {
    /**********************************************************************/
    /*!
      @brief  Draw to the screen/framebuffer/etc.
      Must be overridden in subclass.
      @param  x    X coordinate in pixels
      @param  y    Y coordinate in pixels
      @param color  16-bit pixel color.
    */
    /**********************************************************************/
    fn draw_pixel(&mut self, x: i16, y: i16, color: u16);
}

// ---------------------------------------------------------------------------
// Adafruit_GFX struct
// ---------------------------------------------------------------------------

pub struct Adafruit_GFX {
    // protected fields (mirroring C++ protected section)
    pub WIDTH: i16,          ///< This is the 'raw' display width - never changes
    pub HEIGHT: i16,         ///< This is the 'raw' display height - never changes
    pub _width: i16,         ///< Display width as modified by current rotation
    pub _height: i16,        ///< Display height as modified by current rotation
    pub cursor_x: i16,       ///< x location to start print()ing text
    pub cursor_y: i16,       ///< y location to start print()ing text
    pub textcolor: u16,      ///< 16-bit background color for print()
    pub textbgcolor: u16,    ///< 16-bit text color for print()
    pub textsize_x: u8,      ///< Desired magnification in X-axis of text to print()
    pub textsize_y: u8,      ///< Desired magnification in Y-axis of text to print()
    pub rotation: u8,        ///< Display rotation (0 thru 3)
    pub wrap: bool,          ///< If set, 'wrap' text at right edge of display
    pub _cp437: bool,        ///< If set, use correct CP437 charset (default is off)
    pub gfx_font: Option<&'static GFXfont>, ///< Pointer to special font
}

/**************************************************************************/
/*!
   @brief    Instatiate a GFX context for graphics! Can only be done by a
   superclass
   @param    w   Display width, in pixels
   @param    h   Display height, in pixels
*/
/**************************************************************************/
impl Adafruit_GFX {
    pub fn new(w: i16, h: i16) -> Self {
        Adafruit_GFX {
            WIDTH: w,
            HEIGHT: h,
            _width: w,
            _height: h,
            rotation: 0,
            cursor_x: 0,
            cursor_y: 0,
            textsize_x: 1,
            textsize_y: 1,
            textcolor: 0xFFFF,
            textbgcolor: 0xFFFF,
            wrap: true,
            _cp437: false,
            gfx_font: None,
        }
    }

    // -- TRANSACTION API / CORE DRAW API --
    // These MAY be overridden by the subclass to provide device-specific
    // optimized code.  Otherwise 'generic' versions are used.

    /**************************************************************************/
    /*!
       @brief    Start a display-writing routine, overwrite in subclasses.
    */
    /**************************************************************************/
    pub fn start_write(&mut self) {}

    /**************************************************************************/
    /*!
       @brief    Write a pixel, overwrite in subclasses if startWrite is defined!
        @param   x   x coordinate
        @param   y   y coordinate
       @param    color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn write_pixel(&mut self, x: i16, y: i16, color: u16, backend: &mut dyn DrawPixel) {
        backend.draw_pixel(x, y, color);
    }

    /**************************************************************************/
    /*!
       @brief    Write a line.  Bresenham's algorithm - thx wikpedia
        @param    x0  Start point x coordinate
        @param    y0  Start point y coordinate
        @param    x1  End point x coordinate
        @param    y1  End point y coordinate
        @param    color 16-bit 5-6-5 Color to draw with
    */
    /**************************************************************************/
    pub fn write_line(
        &mut self,
        mut x0: i16,
        mut y0: i16,
        mut x1: i16,
        mut y1: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        let steep = (y1 - y0).abs() > (x1 - x0).abs();
        if steep {
            core::mem::swap(&mut x0, &mut y0);
            core::mem::swap(&mut x1, &mut y1);
        }

        if x0 > x1 {
            core::mem::swap(&mut x0, &mut x1);
            core::mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = (y1 - y0).abs();

        let mut err = dx / 2;
        let ystep: i16 = if y0 < y1 { 1 } else { -1 };

        while x0 <= x1 {
            if steep {
                backend.draw_pixel(y0, x0, color);
            } else {
                backend.draw_pixel(x0, y0, color);
            }
            err -= dy;
            if err < 0 {
                y0 += ystep;
                err += dx;
            }
            x0 += 1;
        }
    }

    /**************************************************************************/
    /*!
       @brief    Write a perfectly vertical line, overwrite in subclasses if
       startWrite is defined!
        @param    x   Top-most x coordinate
        @param    y   Top-most y coordinate
        @param    h   Height in pixels
       @param    color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn write_fast_v_line(
        &mut self,
        x: i16,
        y: i16,
        h: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        // Overwrite in subclasses if startWrite is defined!
        // Can be just writeLine(x, y, x, y+h-1, color);
        // or writeFillRect(x, y, 1, h, color);
        self.draw_fast_v_line(x, y, h, color, backend);
    }

    /**************************************************************************/
    /*!
       @brief    Write a perfectly horizontal line, overwrite in subclasses if
       startWrite is defined!
        @param    x   Left-most x coordinate
        @param    y   Left-most y coordinate
        @param    w   Width in pixels
       @param    color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn write_fast_h_line(
        &mut self,
        x: i16,
        y: i16,
        w: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        // Overwrite in subclasses if startWrite is defined!
        // Example: writeLine(x, y, x+w-1, y, color);
        // or writeFillRect(x, y, w, 1, color);
        self.draw_fast_h_line(x, y, w, color, backend);
    }

    /**************************************************************************/
    /*!
       @brief    Write a rectangle completely with one color, overwrite in
       subclasses if startWrite is defined!
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    w   Width in pixels
        @param    h   Height in pixels
       @param    color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn write_fill_rect(
        &mut self,
        x: i16,
        y: i16,
        w: i16,
        h: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        // Overwrite in subclasses if desired!
        self.fill_rect(x, y, w, h, color, backend);
    }

    /**************************************************************************/
    /*!
       @brief    End a display-writing routine, overwrite in subclasses if
       startWrite is defined!
    */
    /**************************************************************************/
    pub fn end_write(&mut self) {}

    // -- BASIC DRAW API --
    // These MAY be overridden by the subclass to provide device-specific
    // optimized code.  Otherwise 'generic' versions are used.

    /**************************************************************************/
    /*!
       @brief    Draw a perfectly vertical line (this is often optimized in a
       subclass!)
        @param    x   Top-most x coordinate
        @param    y   Top-most y coordinate
        @param    h   Height in pixels
       @param    color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn draw_fast_v_line(
        &mut self,
        x: i16,
        y: i16,
        h: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        self.start_write();
        self.write_line(x, y, x, y + h - 1, color, backend);
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief    Draw a perfectly horizontal line (this is often optimized in a
       subclass!)
        @param    x   Left-most x coordinate
        @param    y   Left-most y coordinate
        @param    w   Width in pixels
       @param    color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn draw_fast_h_line(
        &mut self,
        x: i16,
        y: i16,
        w: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        self.start_write();
        self.write_line(x, y, x + w - 1, y, color, backend);
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief    Fill a rectangle completely with one color. Update in subclasses if
       desired!
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    w   Width in pixels
        @param    h   Height in pixels
       @param    color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn fill_rect(
        &mut self,
        x: i16,
        y: i16,
        w: i16,
        h: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        self.start_write();
        for i in x..(x + w) {
            self.write_fast_v_line(i, y, h, color, backend);
        }
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief    Fill the screen completely with one color. Update in subclasses if
       desired!
        @param    color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn fill_screen(&mut self, color: u16, backend: &mut dyn DrawPixel) {
        self.fill_rect(0, 0, self._width, self._height, color, backend);
    }

    /**************************************************************************/
    /*!
       @brief    Draw a line
        @param    x0  Start point x coordinate
        @param    y0  Start point y coordinate
        @param    x1  End point x coordinate
        @param    y1  End point y coordinate
        @param    color 16-bit 5-6-5 Color to draw with
    */
    /**************************************************************************/
    pub fn draw_line(
        &mut self,
        mut x0: i16,
        mut y0: i16,
        mut x1: i16,
        mut y1: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        // Update in subclasses if desired!
        if x0 == x1 {
            if y0 > y1 {
                core::mem::swap(&mut y0, &mut y1);
            }
            self.draw_fast_v_line(x0, y0, y1 - y0 + 1, color, backend);
        } else if y0 == y1 {
            if x0 > x1 {
                core::mem::swap(&mut x0, &mut x1);
            }
            self.draw_fast_h_line(x0, y0, x1 - x0 + 1, color, backend);
        } else {
            self.start_write();
            self.write_line(x0, y0, x1, y1, color, backend);
            self.end_write();
        }
    }

    /**************************************************************************/
    /*!
       @brief    Draw a circle outline
        @param    x0   Center-point x coordinate
        @param    y0   Center-point y coordinate
        @param    r   Radius of circle
        @param    color 16-bit 5-6-5 Color to draw with
    */
    /**************************************************************************/
    pub fn draw_circle(
        &mut self,
        x0: i16,
        y0: i16,
        r: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        let mut f: i16 = 1 - r;
        let mut dd_f_x: i16 = 1;
        let mut dd_f_y: i16 = -2 * r;
        let mut x: i16 = 0;
        let mut y: i16 = r;

        self.start_write();
        backend.draw_pixel(x0, y0 + r, color);
        backend.draw_pixel(x0, y0 - r, color);
        backend.draw_pixel(x0 + r, y0, color);
        backend.draw_pixel(x0 - r, y0, color);

        while x < y {
            if f >= 0 {
                y -= 1;
                dd_f_y += 2;
                f += dd_f_y;
            }
            x += 1;
            dd_f_x += 2;
            f += dd_f_x;

            backend.draw_pixel(x0 + x, y0 + y, color);
            backend.draw_pixel(x0 - x, y0 + y, color);
            backend.draw_pixel(x0 + x, y0 - y, color);
            backend.draw_pixel(x0 - x, y0 - y, color);
            backend.draw_pixel(x0 + y, y0 + x, color);
            backend.draw_pixel(x0 - y, y0 + x, color);
            backend.draw_pixel(x0 + y, y0 - x, color);
            backend.draw_pixel(x0 - y, y0 - x, color);
        }
        self.end_write();
    }

    /**************************************************************************/
    /*!
        @brief    Quarter-circle drawer, used to do circles and roundrects
        @param    x0   Center-point x coordinate
        @param    y0   Center-point y coordinate
        @param    r   Radius of circle
        @param    cornername  Mask bit #1 or bit #2 to indicate which quarters of
       the circle we're doing
        @param    color 16-bit 5-6-5 Color to draw with
    */
    /**************************************************************************/
    pub fn draw_circle_helper(
        &mut self,
        x0: i16,
        y0: i16,
        r: i16,
        cornername: u8,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        let mut f: i16 = 1 - r;
        let mut dd_f_x: i16 = 1;
        let mut dd_f_y: i16 = -2 * r;
        let mut x: i16 = 0;
        let mut y: i16 = r;

        while x < y {
            if f >= 0 {
                y -= 1;
                dd_f_y += 2;
                f += dd_f_y;
            }
            x += 1;
            dd_f_x += 2;
            f += dd_f_x;
            if (cornername & 0x4) != 0 {
                backend.draw_pixel(x0 + x, y0 + y, color);
                backend.draw_pixel(x0 + y, y0 + x, color);
            }
            if (cornername & 0x2) != 0 {
                backend.draw_pixel(x0 + x, y0 - y, color);
                backend.draw_pixel(x0 + y, y0 - x, color);
            }
            if (cornername & 0x8) != 0 {
                backend.draw_pixel(x0 - y, y0 + x, color);
                backend.draw_pixel(x0 - x, y0 + y, color);
            }
            if (cornername & 0x1) != 0 {
                backend.draw_pixel(x0 - y, y0 - x, color);
                backend.draw_pixel(x0 - x, y0 - y, color);
            }
        }
    }

    /**************************************************************************/
    /*!
       @brief    Draw a circle with filled color
        @param    x0   Center-point x coordinate
        @param    y0   Center-point y coordinate
        @param    r   Radius of circle
        @param    color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn fill_circle(
        &mut self,
        x0: i16,
        y0: i16,
        r: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        self.start_write();
        self.write_fast_v_line(x0, y0 - r, 2 * r + 1, color, backend);
        self.fill_circle_helper(x0, y0, r, 3, 0, color, backend);
        self.end_write();
    }

    /**************************************************************************/
    /*!
        @brief  Quarter-circle drawer with fill, used for circles and roundrects
        @param  x0       Center-point x coordinate
        @param  y0       Center-point y coordinate
        @param  r        Radius of circle
        @param  corners  Mask bits indicating which quarters we're doing
        @param  delta    Offset from center-point, used for round-rects
        @param  color    16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn fill_circle_helper(
        &mut self,
        x0: i16,
        y0: i16,
        r: i16,
        corners: u8,
        delta: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        let mut f: i16 = 1 - r;
        let mut dd_f_x: i16 = 1;
        let mut dd_f_y: i16 = -2 * r;
        let mut x: i16 = 0;
        let mut y: i16 = r;
        let mut px: i16 = x;
        let mut py: i16 = y;

        let delta = delta + 1; // Avoid some +1's in the loop

        while x < y {
            if f >= 0 {
                y -= 1;
                dd_f_y += 2;
                f += dd_f_y;
            }
            x += 1;
            dd_f_x += 2;
            f += dd_f_x;
            // These checks avoid double-drawing certain lines, important
            // for the SSD1306 library which has an INVERT drawing mode.
            if x < (y + 1) {
                if (corners & 1) != 0 {
                    self.write_fast_v_line(x0 + x, y0 - y, 2 * y + delta, color, backend);
                }
                if (corners & 2) != 0 {
                    self.write_fast_v_line(x0 - x, y0 - y, 2 * y + delta, color, backend);
                }
            }
            if y != py {
                if (corners & 1) != 0 {
                    self.write_fast_v_line(x0 + py, y0 - px, 2 * px + delta, color, backend);
                }
                if (corners & 2) != 0 {
                    self.write_fast_v_line(x0 - py, y0 - px, 2 * px + delta, color, backend);
                }
                py = y;
            }
            px = x;
        }
    }

    /**************************************************************************/
    /*!
       @brief   Draw a rectangle with no fill color
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    w   Width in pixels
        @param    h   Height in pixels
        @param    color 16-bit 5-6-5 Color to draw with
    */
    /**************************************************************************/
    pub fn draw_rect(
        &mut self,
        x: i16,
        y: i16,
        w: i16,
        h: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        self.start_write();
        self.write_fast_h_line(x, y, w, color, backend);
        self.write_fast_h_line(x, y + h - 1, w, color, backend);
        self.write_fast_v_line(x, y, h, color, backend);
        self.write_fast_v_line(x + w - 1, y, h, color, backend);
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief   Draw a rounded rectangle with no fill color
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    w   Width in pixels
        @param    h   Height in pixels
        @param    r   Radius of corner rounding
        @param    color 16-bit 5-6-5 Color to draw with
    */
    /**************************************************************************/
    pub fn draw_round_rect(
        &mut self,
        x: i16,
        y: i16,
        w: i16,
        h: i16,
        mut r: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        let max_radius = (if w < h { w } else { h }) / 2; // 1/2 minor axis
        if r > max_radius {
            r = max_radius;
        }
        // smarter version
        self.start_write();
        self.write_fast_h_line(x + r, y, w - 2 * r, color, backend);         // Top
        self.write_fast_h_line(x + r, y + h - 1, w - 2 * r, color, backend); // Bottom
        self.write_fast_v_line(x, y + r, h - 2 * r, color, backend);         // Left
        self.write_fast_v_line(x + w - 1, y + r, h - 2 * r, color, backend); // Right
        // draw four corners
        self.draw_circle_helper(x + r, y + r, r, 1, color, backend);
        self.draw_circle_helper(x + w - r - 1, y + r, r, 2, color, backend);
        self.draw_circle_helper(x + w - r - 1, y + h - r - 1, r, 4, color, backend);
        self.draw_circle_helper(x + r, y + h - r - 1, r, 8, color, backend);
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief   Draw a rounded rectangle with fill color
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    w   Width in pixels
        @param    h   Height in pixels
        @param    r   Radius of corner rounding
        @param    color 16-bit 5-6-5 Color to draw/fill with
    */
    /**************************************************************************/
    pub fn fill_round_rect(
        &mut self,
        x: i16,
        y: i16,
        w: i16,
        h: i16,
        mut r: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        let max_radius = (if w < h { w } else { h }) / 2; // 1/2 minor axis
        if r > max_radius {
            r = max_radius;
        }
        // smarter version
        self.start_write();
        self.write_fill_rect(x + r, y, w - 2 * r, h, color, backend);
        // draw four corners
        self.fill_circle_helper(x + w - r - 1, y + r, r, 1, h - 2 * r - 1, color, backend);
        self.fill_circle_helper(x + r, y + r, r, 2, h - 2 * r - 1, color, backend);
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief   Draw a triangle with no fill color
        @param    x0  Vertex #0 x coordinate
        @param    y0  Vertex #0 y coordinate
        @param    x1  Vertex #1 x coordinate
        @param    y1  Vertex #1 y coordinate
        @param    x2  Vertex #2 x coordinate
        @param    y2  Vertex #2 y coordinate
        @param    color 16-bit 5-6-5 Color to draw with
    */
    /**************************************************************************/
    pub fn draw_triangle(
        &mut self,
        x0: i16,
        y0: i16,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        self.draw_line(x0, y0, x1, y1, color, backend);
        self.draw_line(x1, y1, x2, y2, color, backend);
        self.draw_line(x2, y2, x0, y0, color, backend);
    }

    /**************************************************************************/
    /*!
       @brief     Draw a triangle with color-fill
        @param    x0  Vertex #0 x coordinate
        @param    y0  Vertex #0 y coordinate
        @param    x1  Vertex #1 x coordinate
        @param    y1  Vertex #1 y coordinate
        @param    x2  Vertex #2 x coordinate
        @param    y2  Vertex #2 y coordinate
        @param    color 16-bit 5-6-5 Color to fill/draw with
    */
    /**************************************************************************/
    pub fn fill_triangle(
        &mut self,
        mut x0: i16,
        mut y0: i16,
        mut x1: i16,
        mut y1: i16,
        mut x2: i16,
        mut y2: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        // Sort coordinates by Y order (y2 >= y1 >= y0)
        if y0 > y1 {
            core::mem::swap(&mut y0, &mut y1);
            core::mem::swap(&mut x0, &mut x1);
        }
        if y1 > y2 {
            core::mem::swap(&mut y2, &mut y1);
            core::mem::swap(&mut x2, &mut x1);
        }
        if y0 > y1 {
            core::mem::swap(&mut y0, &mut y1);
            core::mem::swap(&mut x0, &mut x1);
        }

        self.start_write();
        if y0 == y2 {
            // Handle awkward all-on-same-line case as its own thing
            let mut a = x0;
            let mut b = x0;
            if x1 < a {
                a = x1;
            } else if x1 > b {
                b = x1;
            }
            if x2 < a {
                a = x2;
            } else if x2 > b {
                b = x2;
            }
            self.write_fast_h_line(a, y0, b - a + 1, color, backend);
            self.end_write();
            return;
        }

        let dx01 = x1 - x0;
        let dy01 = y1 - y0;
        let dx02 = x2 - x0;
        let dy02 = y2 - y0;
        let dx12 = x2 - x1;
        let dy12 = y2 - y1;
        let mut sa: i32 = 0;
        let mut sb: i32 = 0;

        let last = if y1 == y2 { y1 } else { y1 - 1 };

        let mut y = y0;
        while y <= last {
            let a = x0 as i32 + sa / dy01 as i32;
            let b = x0 as i32 + sb / dy02 as i32;
            sa += dx01 as i32;
            sb += dx02 as i32;
            let (a, b) = if a > b { (b, a) } else { (a, b) };
            self.write_fast_h_line(a as i16, y, (b - a + 1) as i16, color, backend);
            y += 1;
        }

        // For lower part of triangle, find scanline crossings for segments
        // 0-2 and 1-2.  This loop is skipped if y1=y2.
        sa = dx12 as i32 * (y - y1) as i32;
        sb = dx02 as i32 * (y - y0) as i32;
        while y <= y2 {
            let a = x1 as i32 + sa / dy12 as i32;
            let b = x0 as i32 + sb / dy02 as i32;
            sa += dx12 as i32;
            sb += dx02 as i32;
            let (a, b) = if a > b { (b, a) } else { (a, b) };
            self.write_fast_h_line(a as i16, y, (b - a + 1) as i16, color, backend);
            y += 1;
        }
        self.end_write();
    }

    // BITMAP / XBITMAP / GRAYSCALE / RGB BITMAP FUNCTIONS ---------------------

    /**************************************************************************/
    /*!
       @brief      Draw a PROGMEM-resident 1-bit image at the specified (x,y)
       position, using the specified foreground color (unset bits are transparent).
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    bitmap  byte array with monochrome bitmap
        @param    w   Width of bitmap in pixels
        @param    h   Height of bitmap in pixels
        @param    color 16-bit 5-6-5 Color to draw with
    */
    /**************************************************************************/
    pub fn draw_bitmap(
        &mut self,
        x: i16,
        mut y: i16,
        bitmap: &[u8],
        w: i16,
        h: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        let byte_width = ((w + 7) / 8) as usize;
        let mut byte_val: u8 = 0;

        self.start_write();
        for j in 0..h {
            for i in 0..w {
                if (i & 7) != 0 {
                    byte_val <<= 1;
                } else {
                    byte_val = bitmap[(j as usize) * byte_width + (i as usize) / 8];
                }
                if (byte_val & 0x80) != 0 {
                    backend.draw_pixel(x + i, y, color);
                }
            }
            y += 1;
        }
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief      Draw a PROGMEM-resident 1-bit image at the specified (x,y)
       position, using the specified foreground (for set bits) and background (unset
       bits) colors.
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    bitmap  byte array with monochrome bitmap
        @param    w   Width of bitmap in pixels
        @param    h   Height of bitmap in pixels
        @param    color 16-bit 5-6-5 Color to draw pixels with
        @param    bg 16-bit 5-6-5 Color to draw background with
    */
    /**************************************************************************/
    pub fn draw_bitmap_bg(
        &mut self,
        x: i16,
        mut y: i16,
        bitmap: &[u8],
        w: i16,
        h: i16,
        color: u16,
        bg: u16,
        backend: &mut dyn DrawPixel,
    ) {
        let byte_width = ((w + 7) / 8) as usize;
        let mut byte_val: u8 = 0;

        self.start_write();
        for j in 0..h {
            for i in 0..w {
                if (i & 7) != 0 {
                    byte_val <<= 1;
                } else {
                    byte_val = bitmap[(j as usize) * byte_width + (i as usize) / 8];
                }
                backend.draw_pixel(
                    x + i,
                    y,
                    if (byte_val & 0x80) != 0 { color } else { bg },
                );
            }
            y += 1;
        }
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief      Draw PROGMEM-resident XBitMap Files (*.xbm), exported from GIMP.
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    bitmap  byte array with monochrome bitmap
        @param    w   Width of bitmap in pixels
        @param    h   Height of bitmap in pixels
        @param    color 16-bit 5-6-5 Color to draw pixels with
    */
    /**************************************************************************/
    pub fn draw_x_bitmap(
        &mut self,
        x: i16,
        mut y: i16,
        bitmap: &[u8],
        w: i16,
        h: i16,
        color: u16,
        backend: &mut dyn DrawPixel,
    ) {
        let byte_width = ((w + 7) / 8) as usize;
        let mut byte_val: u8 = 0;

        self.start_write();
        for j in 0..h {
            for i in 0..w {
                if (i & 7) != 0 {
                    byte_val >>= 1;
                } else {
                    byte_val = bitmap[(j as usize) * byte_width + (i as usize) / 8];
                }
                // Nearly identical to drawBitmap(), only the bit order
                // is reversed here (left-to-right = LSB to MSB):
                if (byte_val & 0x01) != 0 {
                    backend.draw_pixel(x + i, y, color);
                }
            }
            y += 1;
        }
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief   Draw a PROGMEM-resident 8-bit image (grayscale) at the specified
       (x,y) pos.
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    bitmap  byte array with grayscale bitmap
        @param    w   Width of bitmap in pixels
        @param    h   Height of bitmap in pixels
    */
    /**************************************************************************/
    pub fn draw_grayscale_bitmap(
        &mut self,
        x: i16,
        mut y: i16,
        bitmap: &[u8],
        w: i16,
        h: i16,
        backend: &mut dyn DrawPixel,
    ) {
        self.start_write();
        for j in 0..h {
            for i in 0..w {
                backend.draw_pixel(
                    x + i,
                    y,
                    bitmap[(j as usize) * (w as usize) + (i as usize)] as u16,
                );
            }
            y += 1;
        }
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief   Draw a PROGMEM-resident 8-bit image (grayscale) with a 1-bit mask
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    bitmap  byte array with grayscale bitmap
        @param    mask  byte array with mask bitmap
        @param    w   Width of bitmap in pixels
        @param    h   Height of bitmap in pixels
    */
    /**************************************************************************/
    pub fn draw_grayscale_bitmap_masked(
        &mut self,
        x: i16,
        mut y: i16,
        bitmap: &[u8],
        mask: &[u8],
        w: i16,
        h: i16,
        backend: &mut dyn DrawPixel,
    ) {
        let bw = ((w + 7) / 8) as usize;
        let mut byte_val: u8 = 0;
        self.start_write();
        for j in 0..h {
            for i in 0..w {
                if (i & 7) != 0 {
                    byte_val <<= 1;
                } else {
                    byte_val = mask[(j as usize) * bw + (i as usize) / 8];
                }
                if (byte_val & 0x80) != 0 {
                    backend.draw_pixel(
                        x + i,
                        y,
                        bitmap[(j as usize) * (w as usize) + (i as usize)] as u16,
                    );
                }
            }
            y += 1;
        }
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief   Draw a PROGMEM-resident 16-bit image (RGB 5/6/5) at the specified
       (x,y) position.
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    bitmap  byte array with 16-bit color bitmap
        @param    w   Width of bitmap in pixels
        @param    h   Height of bitmap in pixels
    */
    /**************************************************************************/
    pub fn draw_rgb_bitmap(
        &mut self,
        x: i16,
        mut y: i16,
        bitmap: &[u16],
        w: i16,
        h: i16,
        backend: &mut dyn DrawPixel,
    ) {
        self.start_write();
        for j in 0..h {
            for i in 0..w {
                backend.draw_pixel(
                    x + i,
                    y,
                    bitmap[(j as usize) * (w as usize) + (i as usize)],
                );
            }
            y += 1;
        }
        self.end_write();
    }

    /**************************************************************************/
    /*!
       @brief   Draw a PROGMEM-resident 16-bit image (RGB 5/6/5) with a 1-bit mask
        @param    x   Top left corner x coordinate
        @param    y   Top left corner y coordinate
        @param    bitmap  byte array with 16-bit color bitmap
        @param    mask  byte array with monochrome mask bitmap
        @param    w   Width of bitmap in pixels
        @param    h   Height of bitmap in pixels
    */
    /**************************************************************************/
    pub fn draw_rgb_bitmap_masked(
        &mut self,
        x: i16,
        mut y: i16,
        bitmap: &[u16],
        mask: &[u8],
        w: i16,
        h: i16,
        backend: &mut dyn DrawPixel,
    ) {
        let bw = ((w + 7) / 8) as usize;
        let mut byte_val: u8 = 0;
        self.start_write();
        for j in 0..h {
            for i in 0..w {
                if (i & 7) != 0 {
                    byte_val <<= 1;
                } else {
                    byte_val = mask[(j as usize) * bw + (i as usize) / 8];
                }
                if (byte_val & 0x80) != 0 {
                    backend.draw_pixel(
                        x + i,
                        y,
                        bitmap[(j as usize) * (w as usize) + (i as usize)],
                    );
                }
            }
            y += 1;
        }
        self.end_write();
    }

    // TEXT- AND CHARACTER-HANDLING FUNCTIONS ----------------------------------

    // Draw a character
    /**************************************************************************/
    /*!
       @brief   Draw a single character
        @param    x   Bottom left corner x coordinate
        @param    y   Bottom left corner y coordinate
        @param    c   The 8-bit font-indexed character (likely ascii)
        @param    color 16-bit 5-6-5 Color to draw chraracter with
        @param    bg 16-bit 5-6-5 Color to fill background with (if same as color,
       no background)
        @param    size  Font magnification level, 1 is 'original' size
    */
    /**************************************************************************/
    pub fn draw_char(
        &mut self,
        x: i16,
        y: i16,
        c: u8,
        color: u16,
        bg: u16,
        size: u8,
        backend: &mut dyn DrawPixel,
    ) {
        self.draw_char_xy(x, y, c, color, bg, size, size, backend);
    }

    // Draw a character
    /**************************************************************************/
    /*!
       @brief   Draw a single character
        @param    x   Bottom left corner x coordinate
        @param    y   Bottom left corner y coordinate
        @param    c   The 8-bit font-indexed character (likely ascii)
        @param    color 16-bit 5-6-5 Color to draw chraracter with
        @param    bg 16-bit 5-6-5 Color to fill background with (if same as color,
       no background)
        @param    size_x  Font magnification level in X-axis, 1 is 'original' size
        @param    size_y  Font magnification level in Y-axis, 1 is 'original' size
    */
    /**************************************************************************/
    pub fn draw_char_xy(
        &mut self,
        x: i16,
        y: i16,
        mut c: u8,
        color: u16,
        bg: u16,
        size_x: u8,
        size_y: u8,
        backend: &mut dyn DrawPixel,
    ) {
        if self.gfx_font.is_none() {
            // 'Classic' built-in font

            if (x >= self._width)
                || (y >= self._height)
                || ((x + 6 * size_x as i16 - 1) < 0)
                || ((y + 8 * size_y as i16 - 1) < 0)
            {
                return;
            }

            if !self._cp437 && c >= 176 {
                c = c.wrapping_add(1); // Handle 'classic' charset behavior
            }

            self.start_write();
            for i in 0i8..5 {
                // Char bitmap = 5 columns
                let line = if !BUILTIN_FONT.is_empty() {
                    BUILTIN_FONT[(c as usize) * 5 + (i as usize)]
                } else {
                    0u8
                };
                let mut line_shifted = line;
                for j in 0i8..8 {
                    if (line_shifted & 1) != 0 {
                        if size_x == 1 && size_y == 1 {
                            backend.draw_pixel(x + i as i16, y + j as i16, color);
                        } else {
                            self.write_fill_rect(
                                x + i as i16 * size_x as i16,
                                y + j as i16 * size_y as i16,
                                size_x as i16,
                                size_y as i16,
                                color,
                                backend,
                            );
                        }
                    } else if bg != color {
                        if size_x == 1 && size_y == 1 {
                            backend.draw_pixel(x + i as i16, y + j as i16, bg);
                        } else {
                            self.write_fill_rect(
                                x + i as i16 * size_x as i16,
                                y + j as i16 * size_y as i16,
                                size_x as i16,
                                size_y as i16,
                                bg,
                                backend,
                            );
                        }
                    }
                    line_shifted >>= 1;
                }
            }
            if bg != color {
                // If opaque, draw vertical line for last column
                if size_x == 1 && size_y == 1 {
                    self.write_fast_v_line(x + 5, y, 8, bg, backend);
                } else {
                    self.write_fill_rect(x + 5 * size_x as i16, y, size_x as i16, 8 * size_y as i16, bg, backend);
                }
            }
            self.end_write();
        } else {
            // Custom font
            let font = self.gfx_font.unwrap();

            let ci = c.wrapping_sub(font.first as u8) as usize;
            let glyph = &font.glyph[ci];
            let bitmap = font.bitmap;

            let mut bo = glyph.bitmap_offset as usize;
            let w = glyph.width;
            let h = glyph.height;
            let xo = glyph.x_offset;
            let yo = glyph.y_offset;
            let mut bits: u8 = 0;
            let mut bit: u8 = 0;
            let mut xo16: i16 = 0;
            let mut yo16: i16 = 0;

            if size_x > 1 || size_y > 1 {
                xo16 = xo as i16;
                yo16 = yo as i16;
            }

            self.start_write();
            for yy in 0..h {
                for xx in 0..w {
                    if (bit & 7) == 0 {
                        bits = bitmap[bo];
                        bo += 1;
                    }
                    bit += 1;
                    if (bits & 0x80) != 0 {
                        if size_x == 1 && size_y == 1 {
                            backend.draw_pixel(
                                x + xo as i16 + xx as i16,
                                y + yo as i16 + yy as i16,
                                color,
                            );
                        } else {
                            self.write_fill_rect(
                                x + (xo16 + xx as i16) * size_x as i16,
                                y + (yo16 + yy as i16) * size_y as i16,
                                size_x as i16,
                                size_y as i16,
                                color,
                                backend,
                            );
                        }
                    }
                    bits <<= 1;
                }
            }
            self.end_write();
        }
    }

    /**************************************************************************/
    /*!
        @brief  Print one byte/character of data, used to support print()
        @param  c  The 8-bit ascii character to write
    */
    /**************************************************************************/
    pub fn write(&mut self, c: u8, backend: &mut dyn DrawPixel) -> usize {
        if self.gfx_font.is_none() {
            // 'Classic' built-in font

            if c == b'\n' {
                self.cursor_x = 0;
                self.cursor_y += self.textsize_y as i16 * 8;
            } else if c != b'\r' {
                if self.wrap && ((self.cursor_x + self.textsize_x as i16 * 6) > self._width) {
                    self.cursor_x = 0;
                    self.cursor_y += self.textsize_y as i16 * 8;
                }
                self.draw_char_xy(
                    self.cursor_x,
                    self.cursor_y,
                    c,
                    self.textcolor,
                    self.textbgcolor,
                    self.textsize_x,
                    self.textsize_y,
                    backend,
                );
                self.cursor_x += self.textsize_x as i16 * 6;
            }
        } else {
            // Custom font
            let font = self.gfx_font.unwrap();
            if c == b'\n' {
                self.cursor_x = 0;
                self.cursor_y += self.textsize_y as i16 * font.y_advance as i16;
            } else if c != b'\r' {
                let first = font.first as u8;
                if c >= first && c <= font.last as u8 {
                    let glyph = &font.glyph[(c - first) as usize];
                    let w = glyph.width;
                    let h = glyph.height;
                    if w > 0 && h > 0 {
                        let xo = glyph.x_offset;
                        if self.wrap
                            && ((self.cursor_x + self.textsize_x as i16 * (xo as i16 + w as i16))
                                > self._width)
                        {
                            self.cursor_x = 0;
                            self.cursor_y +=
                                self.textsize_y as i16 * font.y_advance as i16;
                        }
                        self.draw_char_xy(
                            self.cursor_x,
                            self.cursor_y,
                            c,
                            self.textcolor,
                            self.textbgcolor,
                            self.textsize_x,
                            self.textsize_y,
                            backend,
                        );
                    }
                    self.cursor_x += glyph.x_advance as i16 * self.textsize_x as i16;
                }
            }
        }
        1
    }

    /**************************************************************************/
    /*!
       @brief   Set text 'magnification' size. Each increase in s makes 1 pixel
       that much bigger.
        @param  s  Desired text size. 1 is default 6x8, 2 is 12x16, 3 is 18x24, etc
    */
    /**************************************************************************/
    pub fn set_text_size(&mut self, s: u8) {
        self.set_text_size_xy(s, s);
    }

    /**************************************************************************/
    /*!
       @brief   Set text 'magnification' size. Each increase in s makes 1 pixel
       that much bigger.
        @param  s_x  Desired text width magnification level in X-axis. 1 is default
        @param  s_y  Desired text width magnification level in Y-axis. 1 is default
    */
    /**************************************************************************/
    pub fn set_text_size_xy(&mut self, s_x: u8, s_y: u8) {
        self.textsize_x = if s_x > 0 { s_x } else { 1 };
        self.textsize_y = if s_y > 0 { s_y } else { 1 };
    }

    /**************************************************************************/
    /*!
        @brief      Set rotation setting for display
        @param  x   0 thru 3 corresponding to 4 cardinal rotations
    */
    /**************************************************************************/
    pub fn set_rotation(&mut self, x: u8) {
        self.rotation = x & 3;
        match self.rotation {
            0 | 2 => {
                self._width = self.WIDTH;
                self._height = self.HEIGHT;
            }
            1 | 3 => {
                self._width = self.HEIGHT;
                self._height = self.WIDTH;
            }
            _ => {}
        }
    }

    /**************************************************************************/
    /*!
        @brief Set the font to display when print()ing, either custom or default
        @param  f  The GFXfont object, if None use built in 6x8 font
    */
    /**************************************************************************/
    pub fn set_font(&mut self, f: Option<&'static GFXfont>) {
        if f.is_some() {
            if self.gfx_font.is_none() {
                // Switching from classic to new font behavior.
                // Move cursor pos down 6 pixels so it's on baseline.
                self.cursor_y += 6;
            }
        } else if self.gfx_font.is_some() {
            // Switching from new to classic font behavior.
            // Move cursor pos up 6 pixels so it's at top-left of char.
            self.cursor_y -= 6;
        }
        self.gfx_font = f;
    }

    /**********************************************************************/
    /*!
      @brief  Set text cursor location
      @param  x    X coordinate in pixels
      @param  y    Y coordinate in pixels
    */
    /**********************************************************************/
    pub fn set_cursor(&mut self, x: i16, y: i16) {
        self.cursor_x = x;
        self.cursor_y = y;
    }

    /**********************************************************************/
    /*!
      @brief   Set text font color with transparant background
      @param   c   16-bit 5-6-5 Color to draw text with
      @note    For 'transparent' background, background and foreground
               are set to same color rather than using a separate flag.
    */
    /**********************************************************************/
    pub fn set_text_color(&mut self, c: u16) {
        self.textcolor = c;
        self.textbgcolor = c;
    }

    /**********************************************************************/
    /*!
      @brief   Set text font color with custom background color
      @param   c   16-bit 5-6-5 Color to draw text with
      @param   bg  16-bit 5-6-5 Color to draw background/fill with
    */
    /**********************************************************************/
    pub fn set_text_color_bg(&mut self, c: u16, bg: u16) {
        self.textcolor = c;
        self.textbgcolor = bg;
    }

    /**********************************************************************/
    /*!
    @brief  Set whether text that is too long for the screen width should
            automatically wrap around to the next line (else clip right).
    @param  w  true for wrapping, false for clipping
    */
    /**********************************************************************/
    pub fn set_text_wrap(&mut self, w: bool) {
        self.wrap = w;
    }

    /**********************************************************************/
    /*!
      @brief  Enable (or disable) Code Page 437-compatible charset.
      @param  x  true = enable (new behavior), false = disable (old behavior)
    */
    /**********************************************************************/
    pub fn cp437(&mut self, x: bool) {
        self._cp437 = x;
    }

    /************************************************************************/
    /*!
      @brief      Get width of the display, accounting for current rotation
      @returns    Width in pixels
    */
    /************************************************************************/
    pub fn width(&self) -> i16 {
        self._width
    }

    /************************************************************************/
    /*!
      @brief      Get height of the display, accounting for current rotation
      @returns    Height in pixels
    */
    /************************************************************************/
    pub fn height(&self) -> i16 {
        self._height
    }

    /************************************************************************/
    /*!
      @brief      Get rotation setting for display
      @returns    0 thru 3 corresponding to 4 cardinal rotations
    */
    /************************************************************************/
    pub fn get_rotation(&self) -> u8 {
        self.rotation
    }

    // get current cursor position (get rotation safe maximum values,
    // using: width() for x, height() for y)
    /************************************************************************/
    /*!
      @brief  Get text cursor X location
      @returns    X coordinate in pixels
    */
    /************************************************************************/
    pub fn get_cursor_x(&self) -> i16 {
        self.cursor_x
    }

    /************************************************************************/
    /*!
      @brief      Get text cursor Y location
      @returns    Y coordinate in pixels
    */
    /************************************************************************/
    pub fn get_cursor_y(&self) -> i16 {
        self.cursor_y
    }

    /**************************************************************************/
    /*!
        @brief    Helper to determine size of a character with current font/size.
        @param    c     The ascii character in question
        @param    x     Pointer to x location of character
        @param    y     Pointer to y location of character
        @param    minx  Minimum clipping value for X
        @param    miny  Minimum clipping value for Y
        @param    maxx  Maximum clipping value for X
        @param    maxy  Maximum clipping value for Y
    */
    /**************************************************************************/
    pub fn char_bounds(
        &self,
        c: u8,
        x: &mut i16,
        y: &mut i16,
        minx: &mut i16,
        miny: &mut i16,
        maxx: &mut i16,
        maxy: &mut i16,
    ) {
        if let Some(font) = self.gfx_font {
            if c == b'\n' {
                *x = 0;
                *y += self.textsize_y as i16 * font.y_advance as i16;
            } else if c != b'\r' {
                let first = font.first as u8;
                let last = font.last as u8;
                if c >= first && c <= last {
                    let glyph = &font.glyph[(c - first) as usize];
                    let gw = glyph.width;
                    let gh = glyph.height;
                    let xa = glyph.x_advance;
                    let xo = glyph.x_offset;
                    let yo = glyph.y_offset;
                    if self.wrap && ((*x + ((xo as i16 + gw as i16) * self.textsize_x as i16)) > self._width) {
                        *x = 0;
                        *y += self.textsize_y as i16 * font.y_advance as i16;
                    }
                    let tsx = self.textsize_x as i16;
                    let tsy = self.textsize_y as i16;
                    let x1 = *x + xo as i16 * tsx;
                    let y1 = *y + yo as i16 * tsy;
                    let x2 = x1 + gw as i16 * tsx - 1;
                    let y2 = y1 + gh as i16 * tsy - 1;
                    if x1 < *minx { *minx = x1; }
                    if y1 < *miny { *miny = y1; }
                    if x2 > *maxx { *maxx = x2; }
                    if y2 > *maxy { *maxy = y2; }
                    *x += xa as i16 * tsx;
                }
            }
        } else {
            // Default font
            if c == b'\n' {
                *x = 0;
                *y += self.textsize_y as i16 * 8;
            } else if c != b'\r' {
                if self.wrap && ((*x + self.textsize_x as i16 * 6) > self._width) {
                    *x = 0;
                    *y += self.textsize_y as i16 * 8;
                }
                let x2 = *x + self.textsize_x as i16 * 6 - 1;
                let y2 = *y + self.textsize_y as i16 * 8 - 1;
                if x2 > *maxx { *maxx = x2; }
                if y2 > *maxy { *maxy = y2; }
                if *x < *minx { *minx = *x; }
                if *y < *miny { *miny = *y; }
                *x += self.textsize_x as i16 * 6;
            }
        }
    }

    /**************************************************************************/
    /*!
        @brief    Helper to determine size of a string with current font/size.
        @param    str     The ascii string to measure
        @param    x       The current cursor X
        @param    y       The current cursor Y
        @returns  (x1, y1, w, h)
    */
    /**************************************************************************/
    pub fn get_text_bounds(&self, s: &str, mut x: i16, mut y: i16) -> (i16, i16, u16, u16) {
        let mut x1 = x;
        let mut y1 = y;
        let mut w: u16 = 0;
        let mut h: u16 = 0;

        let mut minx = self._width;
        let mut miny = self._height;
        let mut maxx: i16 = -1;
        let mut maxy: i16 = -1;

        for c in s.bytes() {
            self.char_bounds(c, &mut x, &mut y, &mut minx, &mut miny, &mut maxx, &mut maxy);
        }

        if maxx >= minx {
            x1 = minx;
            w = (maxx - minx + 1) as u16;
        }
        if maxy >= miny {
            y1 = miny;
            h = (maxy - miny + 1) as u16;
        }

        (x1, y1, w, h)
    }

    /**************************************************************************/
    /*!
        @brief      Invert the display (ideally using built-in hardware command)
        @param   i  True if you want to invert, false to make 'normal'
    */
    /**************************************************************************/
    pub fn invert_display(&mut self, _i: bool) {
        // Do nothing, must be subclassed if supported by hardware
    }
}

// ---------------------------------------------------------------------------
// Adafruit_GFX_Button
// ---------------------------------------------------------------------------

/// A simple drawn button UI element
pub struct Adafruit_GFX_Button {
    _x1: i16,
    _y1: i16,
    _w: u16,
    _h: u16,
    _textsize_x: u8,
    _textsize_y: u8,
    _outlinecolor: u16,
    _fillcolor: u16,
    _textcolor: u16,
    _label: [u8; 10],
    currstate: bool,
    laststate: bool,
}

impl Adafruit_GFX_Button {
    /**************************************************************************/
    /*!
       @brief    Create a simple drawn button UI element
    */
    /**************************************************************************/
    pub fn new() -> Self {
        Adafruit_GFX_Button {
            _x1: 0,
            _y1: 0,
            _w: 0,
            _h: 0,
            _textsize_x: 0,
            _textsize_y: 0,
            _outlinecolor: 0,
            _fillcolor: 0,
            _textcolor: 0,
            _label: [0u8; 10],
            currstate: false,
            laststate: false,
        }
    }

    /**************************************************************************/
    /*!
       @brief    Initialize button with our desired color/size/settings
       @param    x       The X coordinate of the center of the button
       @param    y       The Y coordinate of the center of the button
       @param    w       Width of the buttton
       @param    h       Height of the buttton
       @param    outline  Color of the outline (16-bit 5-6-5 standard)
       @param    fill  Color of the button fill (16-bit 5-6-5 standard)
       @param    textcolor  Color of the button label (16-bit 5-6-5 standard)
       @param    label  Ascii string of the text inside the button
       @param    textsize The font magnification of the label text
    */
    /**************************************************************************/
    // Classic initButton() function: pass center & size
    pub fn init_button(
        &mut self,
        x: i16,
        y: i16,
        w: u16,
        h: u16,
        outline: u16,
        fill: u16,
        textcolor: u16,
        label: &str,
        textsize: u8,
    ) {
        self.init_button_ul(
            x - (w as i16 / 2),
            y - (h as i16 / 2),
            w,
            h,
            outline,
            fill,
            textcolor,
            label,
            textsize,
            textsize,
        );
    }

    /**************************************************************************/
    /*!
       @brief    Initialize button with our desired color/size/settings, with
       upper-left coordinates
    */
    /**************************************************************************/
    pub fn init_button_ul(
        &mut self,
        x1: i16,
        y1: i16,
        w: u16,
        h: u16,
        outline: u16,
        fill: u16,
        textcolor: u16,
        label: &str,
        textsize_x: u8,
        textsize_y: u8,
    ) {
        self._x1 = x1;
        self._y1 = y1;
        self._w = w;
        self._h = h;
        self._outlinecolor = outline;
        self._fillcolor = fill;
        self._textcolor = textcolor;
        self._textsize_x = textsize_x;
        self._textsize_y = textsize_y;
        let bytes = label.as_bytes();
        let copy_len = core::cmp::min(bytes.len(), 9);
        self._label[..copy_len].copy_from_slice(&bytes[..copy_len]);
        self._label[copy_len] = 0;
    }

    /**************************************************************************/
    /*!
       @brief    Draw the button on the screen
       @param    gfx       The GFX context
       @param    inverted Whether to draw with fill/text swapped to indicate
       'pressed'
    */
    /**************************************************************************/
    pub fn draw_button(
        &self,
        gfx: &mut Adafruit_GFX,
        inverted: bool,
        backend: &mut dyn DrawPixel,
    ) {
        let (fill, outline, text) = if !inverted {
            (self._fillcolor, self._outlinecolor, self._textcolor)
        } else {
            (self._textcolor, self._outlinecolor, self._fillcolor)
        };

        let r = min(self._w, self._h) as i16 / 4;
        gfx.fill_round_rect(self._x1, self._y1, self._w as i16, self._h as i16, r, fill, backend);
        gfx.draw_round_rect(self._x1, self._y1, self._w as i16, self._h as i16, r, outline, backend);

        let label_len = self._label.iter().position(|&b| b == 0).unwrap_or(self._label.len());
        gfx.set_cursor(
            self._x1 + (self._w as i16 / 2) - (label_len as i16 * 3 * self._textsize_x as i16),
            self._y1 + (self._h as i16 / 2) - (4 * self._textsize_y as i16),
        );
        gfx.set_text_color(text);
        gfx.set_text_size_xy(self._textsize_x, self._textsize_y);
        for i in 0..label_len {
            gfx.write(self._label[i], backend);
        }
    }

    /**************************************************************************/
    /*!
        @brief    Helper to let us know if a coordinate is within the bounds of the
       button
        @param    x       The X coordinate to check
        @param    y       The Y coordinate to check
        @returns  True if within button graphics outline
    */
    /**************************************************************************/
    pub fn contains(&self, x: i16, y: i16) -> bool {
        (x >= self._x1)
            && (x < (self._x1 + self._w as i16))
            && (y >= self._y1)
            && (y < (self._y1 + self._h as i16))
    }

    /**********************************************************************/
    /*!
      @brief    Sets button state, should be done by some touch function
      @param    p  True for pressed, false for not.
    */
    /**********************************************************************/
    pub fn press(&mut self, p: bool) {
        self.laststate = self.currstate;
        self.currstate = p;
    }

    /**************************************************************************/
    /*!
       @brief    Query whether the button was pressed since we last checked state
       @returns  True if was not-pressed before, now is.
    */
    /**************************************************************************/
    pub fn just_pressed(&self) -> bool {
        self.currstate && !self.laststate
    }

    /**************************************************************************/
    /*!
       @brief    Query whether the button was released since we last checked state
       @returns  True if was pressed before, now is not.
    */
    /**************************************************************************/
    pub fn just_released(&self) -> bool {
        !self.currstate && self.laststate
    }

    /**********************************************************************/
    /*!
      @brief    Query whether the button is currently pressed
      @returns  True if pressed
    */
    /**********************************************************************/
    pub fn is_pressed(&self) -> bool {
        self.currstate
    }
}

// ---------------------------------------------------------------------------
// GFXcanvas1 -- A GFX 1-bit canvas context for graphics
// ---------------------------------------------------------------------------

/// A GFX 1-bit canvas context for graphics
pub struct GFXcanvas1 {
    pub gfx: Adafruit_GFX,
    buffer: Vec<u8>,
}

impl GFXcanvas1 {
    /**************************************************************************/
    /*!
       @brief    Instatiate a GFX 1-bit canvas context for graphics
       @param    w   Display width, in pixels
       @param    h   Display height, in pixels
    */
    /**************************************************************************/
    pub fn new(w: u16, h: u16) -> Self {
        let bytes = ((w as usize + 7) / 8) * h as usize;
        GFXcanvas1 {
            gfx: Adafruit_GFX::new(w as i16, h as i16),
            buffer: vec![0u8; bytes],
        }
    }

    /**************************************************************************/
    /*!
        @brief  Draw a pixel to the canvas framebuffer
        @param  x     x coordinate
        @param  y     y coordinate
        @param  color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn draw_pixel(&mut self, x: i16, y: i16, color: u16) {
        if self.buffer.is_empty() {
            return;
        }
        let mut x = x;
        let mut y = y;
        if x < 0 || y < 0 || x >= self.gfx._width || y >= self.gfx._height {
            return;
        }

        match self.gfx.rotation {
            1 => {
                let t = x;
                x = self.gfx.WIDTH - 1 - y;
                y = t;
            }
            2 => {
                x = self.gfx.WIDTH - 1 - x;
                y = self.gfx.HEIGHT - 1 - y;
            }
            3 => {
                let t = x;
                x = y;
                y = self.gfx.HEIGHT - 1 - t;
            }
            _ => {}
        }

        let idx = (x / 8) as usize + y as usize * ((self.gfx.WIDTH as usize + 7) / 8);
        if color != 0 {
            self.buffer[idx] |= 0x80 >> (x & 7);
        } else {
            self.buffer[idx] &= !(0x80 >> (x & 7));
        }
    }

    /**************************************************************************/
    /*!
        @brief  Fill the framebuffer completely with one color
        @param  color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn fill_screen(&mut self, color: u16) {
        if !self.buffer.is_empty() {
            let bytes = ((self.gfx.WIDTH as usize + 7) / 8) * self.gfx.HEIGHT as usize;
            let val = if color != 0 { 0xFF } else { 0x00 };
            for b in self.buffer[..bytes].iter_mut() {
                *b = val;
            }
        }
    }

    /**********************************************************************/
    /*!
      @brief    Get a pointer to the internal buffer memory
      @returns  A reference to the allocated buffer
    */
    /**********************************************************************/
    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl DrawPixel for GFXcanvas1 {
    fn draw_pixel(&mut self, x: i16, y: i16, color: u16) {
        GFXcanvas1::draw_pixel(self, x, y, color);
    }
}

// ---------------------------------------------------------------------------
// GFXcanvas8 -- A GFX 8-bit canvas context for graphics
// ---------------------------------------------------------------------------

/// A GFX 8-bit canvas context for graphics
pub struct GFXcanvas8 {
    pub gfx: Adafruit_GFX,
    buffer: Vec<u8>,
}

impl GFXcanvas8 {
    /**************************************************************************/
    /*!
       @brief    Instatiate a GFX 8-bit canvas context for graphics
       @param    w   Display width, in pixels
       @param    h   Display height, in pixels
    */
    /**************************************************************************/
    pub fn new(w: u16, h: u16) -> Self {
        let bytes = w as usize * h as usize;
        GFXcanvas8 {
            gfx: Adafruit_GFX::new(w as i16, h as i16),
            buffer: vec![0u8; bytes],
        }
    }

    /**************************************************************************/
    /*!
        @brief  Draw a pixel to the canvas framebuffer
        @param  x   x coordinate
        @param  y   y coordinate
        @param  color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn draw_pixel(&mut self, x: i16, y: i16, color: u16) {
        if self.buffer.is_empty() {
            return;
        }
        let mut x = x;
        let mut y = y;
        if x < 0 || y < 0 || x >= self.gfx._width || y >= self.gfx._height {
            return;
        }

        match self.gfx.rotation {
            1 => {
                let t = x;
                x = self.gfx.WIDTH - 1 - y;
                y = t;
            }
            2 => {
                x = self.gfx.WIDTH - 1 - x;
                y = self.gfx.HEIGHT - 1 - y;
            }
            3 => {
                let t = x;
                x = y;
                y = self.gfx.HEIGHT - 1 - t;
            }
            _ => {}
        }

        self.buffer[x as usize + y as usize * self.gfx.WIDTH as usize] = color as u8;
    }

    /**************************************************************************/
    /*!
        @brief  Fill the framebuffer completely with one color
        @param  color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn fill_screen(&mut self, color: u16) {
        if !self.buffer.is_empty() {
            let total = self.gfx.WIDTH as usize * self.gfx.HEIGHT as usize;
            for b in self.buffer[..total].iter_mut() {
                *b = color as u8;
            }
        }
    }

    pub fn write_fast_h_line(&mut self, x: i16, y: i16, mut w: i16, color: u16) {
        if (x >= self.gfx._width) || (y < 0) || (y >= self.gfx._height) {
            return;
        }
        let mut x = x;
        let x2 = x + w - 1;
        if x2 < 0 {
            return;
        }

        // Clip left/right
        if x < 0 {
            x = 0;
            w = x2 + 1;
        }
        if x2 >= self.gfx._width {
            w = self.gfx._width - x;
        }

        let mut rx = x;
        let mut ry = y;
        match self.gfx.rotation {
            1 => {
                let t = rx;
                rx = self.gfx.WIDTH - 1 - ry;
                ry = t;
            }
            2 => {
                rx = self.gfx.WIDTH - 1 - rx;
                ry = self.gfx.HEIGHT - 1 - ry;
            }
            3 => {
                let t = rx;
                rx = ry;
                ry = self.gfx.HEIGHT - 1 - t;
            }
            _ => {}
        }

        let start = ry as usize * self.gfx.WIDTH as usize + rx as usize;
        for i in 0..w as usize {
            self.buffer[start + i] = color as u8;
        }
    }

    /**********************************************************************/
    /*!
     @brief    Get a pointer to the internal buffer memory
     @returns  A reference to the allocated buffer
    */
    /**********************************************************************/
    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl DrawPixel for GFXcanvas8 {
    fn draw_pixel(&mut self, x: i16, y: i16, color: u16) {
        GFXcanvas8::draw_pixel(self, x, y, color);
    }
}

// ---------------------------------------------------------------------------
// GFXcanvas16 -- A GFX 16-bit canvas context for graphics
// ---------------------------------------------------------------------------

///  A GFX 16-bit canvas context for graphics
pub struct GFXcanvas16 {
    pub gfx: Adafruit_GFX,
    buffer: Vec<u16>,
}

impl GFXcanvas16 {
    /**************************************************************************/
    /*!
       @brief    Instatiate a GFX 16-bit canvas context for graphics
       @param    w   Display width, in pixels
       @param    h   Display height, in pixels
    */
    /**************************************************************************/
    pub fn new(w: u16, h: u16) -> Self {
        let pixels = w as usize * h as usize;
        GFXcanvas16 {
            gfx: Adafruit_GFX::new(w as i16, h as i16),
            buffer: vec![0u16; pixels],
        }
    }

    /**************************************************************************/
    /*!
        @brief  Draw a pixel to the canvas framebuffer
        @param  x   x coordinate
        @param  y   y coordinate
        @param  color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn draw_pixel(&mut self, x: i16, y: i16, color: u16) {
        if self.buffer.is_empty() {
            return;
        }
        let mut x = x;
        let mut y = y;
        if x < 0 || y < 0 || x >= self.gfx._width || y >= self.gfx._height {
            return;
        }

        match self.gfx.rotation {
            1 => {
                let t = x;
                x = self.gfx.WIDTH - 1 - y;
                y = t;
            }
            2 => {
                x = self.gfx.WIDTH - 1 - x;
                y = self.gfx.HEIGHT - 1 - y;
            }
            3 => {
                let t = x;
                x = y;
                y = self.gfx.HEIGHT - 1 - t;
            }
            _ => {}
        }

        self.buffer[x as usize + y as usize * self.gfx.WIDTH as usize] = color;
    }

    /**************************************************************************/
    /*!
        @brief  Fill the framebuffer completely with one color
        @param  color 16-bit 5-6-5 Color to fill with
    */
    /**************************************************************************/
    pub fn fill_screen(&mut self, color: u16) {
        if !self.buffer.is_empty() {
            let hi = (color >> 8) as u8;
            let lo = (color & 0xFF) as u8;
            if hi == lo {
                // When both bytes are the same, we can use a fast memset equivalent
                let total = self.gfx.WIDTH as usize * self.gfx.HEIGHT as usize;
                for b in self.buffer[..total].iter_mut() {
                    *b = color;
                }
            } else {
                let pixels = self.gfx.WIDTH as usize * self.gfx.HEIGHT as usize;
                for i in 0..pixels {
                    self.buffer[i] = color;
                }
            }
        }
    }

    /**************************************************************************/
    /*!
        @brief  Reverses the "endian-ness" of each 16-bit pixel within the
                canvas; little-endian to big-endian, or big-endian to little.
    */
    /**************************************************************************/
    pub fn byte_swap(&mut self) {
        if !self.buffer.is_empty() {
            let pixels = self.gfx.WIDTH as usize * self.gfx.HEIGHT as usize;
            for i in 0..pixels {
                self.buffer[i] = self.buffer[i].swap_bytes();
            }
        }
    }

    /**********************************************************************/
    /*!
      @brief    Get a pointer to the internal buffer memory
      @returns  A reference to the allocated buffer
    */
    /**********************************************************************/
    pub fn get_buffer(&self) -> &[u16] {
        &self.buffer
    }
}

impl DrawPixel for GFXcanvas16 {
    fn draw_pixel(&mut self, x: i16, y: i16, color: u16) {
        GFXcanvas16::draw_pixel(self, x, y, color);
    }
}
