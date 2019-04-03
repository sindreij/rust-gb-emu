use std::sync::Arc;

use parking_lot::Mutex;

use crate::{error::Error, mem::Mmu};

// screen is 20 tiles by 18 tiles (160x144pixels)
// Viewport on a 32x32 tiles map (wrapping around)

// A tile is 8x8 pixels. 4 colors (i.e. 2 bit). 16 bytes per tile. 2 bytes per line

// Palette 2bits - 2bits mapping (or something). 0xff47 (8 bits, 4 colors, 2 bits each color)
// Så det er 4 farger, og man velger hvordan de 4 fargene skal mappe til en palette.
// 00 white, 11 black. And between

// 256 tiles.

// Sprites is also called objects (OBJ). OAM entry. Sprites can pick palettes, between 2.
// 40 sprites in total. 10 sprites per pixelline

// ff40 styrer diverse display-ting.

// VRAM. 4KB Sprite tiles. 4KB Bacgrkound tiles. 1KB BG Map. 1KB WIndow Map
// All this in 4KB of videoram
// fra 0x8000 til 0x????
// Man kan flytte hva som skal være BG tiles.

// Hver linje består av 20 klokker med OAM search, 43 klokker med pixel transfer
// og 51 klokker med H-Blank (totalt 114)
// Det er 144 linjer på skjermen
// Så er det 10 linjer med V-Blank
// Totalt 114 x (144 + 10) = 17 556 klokker for å tegne en skjerm
// Gameboyen er klokket til 1 048 576 klokker pr sekund
// Så mao 1 048 576 / 17556 = 59.7 Hz

pub struct Ppu {
    display: Arc<Mutex<Vec<u8>>>,
    pub vram: Vec<u8>,
    buffer: Vec<u8>,
    current_line: u8,
    next_pixel: u8,
}

impl Ppu {
    pub fn new(display: Arc<Mutex<Vec<u8>>>) -> Self {
        Ppu {
            display,
            vram: vec![0; 0x2000],
            buffer: vec![0; 160 * 144 * 3],
            current_line: 0,
            next_pixel: 0,
        }
    }

    pub fn step(&mut self) -> Result<(), Error> {
        // copy 4 pixels each time. TODO: Implement Pixel FIFO

        for i in 0..5 {
            println!("{}", i);
        }

        Ok(())
    }
}
