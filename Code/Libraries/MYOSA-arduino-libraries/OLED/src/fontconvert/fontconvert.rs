/*
TrueType to Adafruit_GFX font converter.  Derived from Peter Jakobs'
Adafruit_ftGFX fork & makefont tool, and Paul Kourany's Adafruit_mfGFX.

NOT AN ARDUINO SKETCH.  This is a command-line tool for preprocessing
fonts to be used with the Adafruit_GFX Arduino library.

For UNIX-like systems.  Outputs to stdout; redirect to header file, e.g.:
  ./fontconvert ~/Library/Fonts/FreeSans.ttf 18 > FreeSans18pt7b.h

REQUIRES FREETYPE LIBRARY.  www.freetype.org

Currently this only extracts the printable 7-bit ASCII chars of a font.
Will eventually extend with some int'l chars a la ftGFX, not there yet.
Keep 7-bit fonts around as an option in that case, more compact.

See notes at end for glyph nomenclature & other tidbits.
*/

use std::env;
use std::process;

const DPI: u32 = 141; // Approximate res. of Adafruit 2.8" TFT

struct GFXGlyph {
    bitmap_offset: u32,
    width: u8,
    height: u8,
    x_advance: u8,
    x_offset: i8,
    y_offset: i8,
}

// Accumulate bits for output, with periodic hexadecimal byte write
struct BitAccumulator {
    row: u32,
    sum: u8,
    bit: u8,
    first_call: bool,
}

impl BitAccumulator {
    fn new() -> Self {
        BitAccumulator {
            row: 0,
            sum: 0,
            bit: 0x80,
            first_call: true,
        }
    }

    fn enbit(&mut self, value: bool) {
        if value {
            self.sum |= self.bit; // Set bit if needed
        }
        self.bit >>= 1;
        if self.bit == 0 { // Advance to next bit, end of byte reached?
            if !self.first_call { // Format output table nicely
                self.row += 1;
                if self.row >= 12 { // Last entry on line?
                    print!(",\n  "); //   Newline format output
                    self.row = 0;    //   Reset row counter
                } else {             // Not end of line
                    print!(", ");    //   Simple comma delim
                }
            }
            print!("0x{:02X}", self.sum); // Write byte value
            self.sum = 0;                  // Clear for next byte
            self.bit = 0x80;               // Reset bit counter
            self.first_call = false;       // Formatting flag
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command line.  Valid syntaxes are:
    //   fontconvert [filename] [size]
    //   fontconvert [filename] [size] [last char]
    //   fontconvert [filename] [size] [first char] [last char]
    // Unless overridden, default first and last chars are
    // ' ' (space) and '~', respectively

    if args.len() < 3 {
        eprintln!("Usage: {} fontfile size [first] [last]", args[0]);
        process::exit(1);
    }

    let size: u32 = args[2].parse().unwrap_or(0);
    let mut first: u32 = ' ' as u32;
    let mut last: u32 = '~' as u32;

    if args.len() == 4 {
        last = args[3].parse().unwrap_or(last);
    } else if args.len() == 5 {
        first = args[3].parse().unwrap_or(first);
        last = args[4].parse().unwrap_or(last);
    }

    if last < first {
        std::mem::swap(&mut first, &mut last);
    }

    // Derive font table names from filename
    let filename = &args[1];
    let base = filename.rsplit('/').next().unwrap_or(filename);
    let stem = base.split('.').next().unwrap_or(base);
    let bit_label = if last > 127 { 8 } else { 7 };
    let font_name: String = format!("{}{}pt{}b", stem, size, bit_label)
        .chars()
        .map(|c| if c.is_whitespace() || c.is_ascii_punctuation() { '_' } else { c })
        .collect();

    // NOTE: This Rust port requires the `freetype` crate for actual font rendering.
    // The logic below mirrors the C original but font loading/rendering calls
    // are left as stubs since they depend on the FreeType C library.

    eprintln!("Font conversion requires FreeType library bindings.");
    eprintln!("Install the `freetype` Rust crate and link against libfreetype.");
    eprintln!("Font name would be: {}", font_name);
    eprintln!("Range: {} to {} at size {}", first, last, size);

    // Init FreeType lib, load font
    // In the Rust port, you would use:
    //   let lib = freetype::Library::init().unwrap();
    //   let face = lib.new_face(filename, 0).unwrap();
    //   face.set_char_size(size as isize * 64, 0, DPI, 0).unwrap();

    let mut acc = BitAccumulator::new();
    let mut table: Vec<GFXGlyph> = Vec::with_capacity((last - first + 1) as usize);
    let mut bitmap_offset: u32 = 0;

    println!("const uint8_t {}Bitmaps[] PROGMEM = {{", font_name);
    print!("  ");

    // Process glyphs and output huge bitmap data array
    // (In the full port, this would iterate through FreeType glyphs)
    for i in first..=last {
        // FT_Load_Char, FT_Render_Glyph, FT_Get_Glyph would go here
        // For each glyph, store metrics and output bitmap bits via acc.enbit()
        table.push(GFXGlyph {
            bitmap_offset,
            width: 0,
            height: 0,
            x_advance: 0,
            x_offset: 0,
            y_offset: 0,
        });
    }

    println!(" }};");
    println!();

    // Output glyph attributes table (one per character)
    println!("const GFXglyph {}Glyphs[] PROGMEM = {{", font_name);
    for (j, i) in (first..=last).enumerate() {
        let g = &table[j];
        print!(
            "  {{ {:5}, {:3}, {:3}, {:3}, {:4}, {:4} }}",
            g.bitmap_offset, g.width, g.height, g.x_advance, g.x_offset, g.y_offset
        );
        if i < last {
            print!(",   // 0x{:02X}", i);
            if i >= ' ' as u32 && i <= '~' as u32 {
                print!(" '{}'", i as u8 as char);
            }
            println!();
        }
    }
    println!(" }}; // 0x{:02X}", last);
    if last >= ' ' as u32 && last <= '~' as u32 {
        print!(" '{}'", last as u8 as char);
    }
    println!();
    println!();

    // Output font structure
    println!("const GFXfont {} PROGMEM = {{", font_name);
    println!("  (uint8_t  *){}Bitmaps,", font_name);
    println!("  (GFXglyph *){}Glyphs,", font_name);
    println!("  0x{:02X}, 0x{:02X}, 0 }};", first, last);
    println!();
    println!(
        "// Approx. {} bytes",
        bitmap_offset + (last - first + 1) * 7 + 7
    );
}

/* -------------------------------------------------------------------------

Character metrics are slightly different from classic GFX & ftGFX.
In classic GFX: cursor position is the upper-left pixel of each 5x7
character; lower extent of most glyphs (except those w/descenders)
is +6 pixels in Y direction.
W/new GFX fonts: cursor position is on baseline, where baseline is
'inclusive' (containing the bottom-most row of pixels in most symbols,
except those with descenders; ftGFX is one pixel lower).

Cursor Y will be moved automatically when switching between classic
and new fonts.  If you switch fonts, any print() calls will continue
along the same baseline.

glyph->xOffset and yOffset are pixel offsets, in GFX coordinate space
(+Y is down), from the cursor position to the top-left pixel of the
glyph bitmap.  i.e. yOffset is typically negative, xOffset is typically
zero but a few glyphs will have other values (even negative xOffsets
sometimes, totally normal).  glyph->xAdvance is the distance to move
the cursor on the X axis after drawing the corresponding symbol.

There's also some changes with regard to 'background' color and new GFX
fonts (classic fonts unchanged).  See Adafruit_GFX.cpp for explanation.
*/
