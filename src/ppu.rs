use crate::register_maps::IORegisters;
use sdl2::pixels::Color;
use std::convert::TryInto;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameboyColor {
    Transparent,
    White,
    LightGray,
    DarkGray,
    Black,
}
impl Into<Color> for GameboyColor {
    fn into(self) -> Color {
        match self {
            GameboyColor::White => Color::RGB(155, 188, 15),
            GameboyColor::LightGray => Color::RGB(139, 172, 15),
            GameboyColor::DarkGray => Color::RGB(48, 98, 48),
            GameboyColor::Black => Color::RGB(15, 56, 15),
            GameboyColor::Transparent => Color::RGB(15, 56, 15),
        }
    }
}

impl From<&GameboyColor> for u32 {
    fn from(color: &GameboyColor) -> u32 {
        match color {
            GameboyColor::White => 0xFF << 24 | 155 << 16 | 188 << 8 | 15,
            GameboyColor::LightGray => 0xFF << 24 | 139 << 16 | 172 << 8 | 15,
            GameboyColor::DarkGray => 0xFF << 24 | 48 << 16 | 98 << 8 | 48,
            GameboyColor::Black => 0xFF << 24 | 15 << 16 | 56 << 8 | 15,
            GameboyColor::Transparent => 0xFF << 24 | 15 << 16 | 56 << 8 | 15,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct OamEntry {
    y_coord: u8,
    x_coord: u8,
    data_tile_num: u8,
    sprite_priority: bool,
    y_flip: bool,
    x_flip: bool,
    palette: bool,
}

impl From<[u8; 4]> for OamEntry {
    fn from(bytes: [u8; 4]) -> Self {
        OamEntry {
            y_coord: bytes[0],
            x_coord: bytes[1],
            data_tile_num: bytes[2],
            sprite_priority: !(bytes[3] & 8 == 0),
            y_flip: !(bytes[3] & 4 == 0),
            x_flip: !(bytes[3] & 2 == 0),
            palette: !(bytes[3] & 1 == 0),
        }
    }
}

impl From<&OamEntry> for [u8; 4] {
    fn from(entry: &OamEntry) -> Self {
        let mut byte_4: u8 = 0;
        if entry.sprite_priority {
            byte_4 |= 8;
        }
        if entry.y_flip {
            byte_4 |= 4;
        }
        if entry.x_flip {
            byte_4 |= 2;
        }
        if entry.palette {
            byte_4 |= 1;
        }
        [entry.y_coord, entry.x_coord, entry.data_tile_num, byte_4]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OAM {
    oam_entries: [OamEntry; 40],
}

impl OAM {
    pub fn set(&mut self, addr: usize, value: u8) {
	let mut oam_arr: [u8; 0xA0] = self.clone().into();
	oam_arr[addr - 0xFE00] = value;
	*self = oam_arr.into();
    }
    pub fn get(&self, addr: usize) -> u8 {
	let oam_arr: [u8; 0xA0] = self.clone().into();
	oam_arr[addr]
    }
}

impl From<OAM> for [u8; 0xA0] {
    fn from(oam: OAM) -> Self {
        oam.oam_entries
            .iter()
            .flat_map(|s| Into::<[u8; 4]>::into(s))
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap()
    }
}

impl From<[u8; 0xA0]> for OAM {
    fn from(oam: [u8; 0xA0]) -> Self {
	let mut oam_vec: Vec<OamEntry> = Vec::new();
        for i in 0..40 {
	    let j = 4 * i;
	    let new_entry: [u8; 4] = oam[j .. j+4].try_into().unwrap();
	    oam_vec.push(new_entry.into());
	}
	OAM {
	    oam_entries: oam_vec.as_slice().try_into().unwrap()
	}
    }
}

#[derive(Copy, Clone)]
struct Tile {
    color_map: [[GameboyColor; 8]; 8],
}

impl Tile {
    fn from(data: [u8; 16], palette: u8) -> Tile {
	let mut color_map = [[GameboyColor::White; 8]; 8];
	for i in 0..8 {
	    let ind = 2 * i;
	    let mut color_vec: Vec<GameboyColor> = Vec::new();
	    PPU::get_color_line(&mut color_vec, data[ind + 1], data[ind], palette, false);
	    color_map[i] = color_vec.try_into().unwrap();
	}
	Tile {
	    color_map,
	}
    }
}

impl Deref for Tile {
    type Target = [[GameboyColor; 8]; 8];

    fn deref(&self) -> &Self::Target {
	&self.color_map
    }
}

impl DerefMut for Tile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.color_map
    }
}

#[derive(Clone, Copy)]
pub struct PPU {
    pub vram: [u8; 0x2000],
    pub oam: OAM,
    pub io_registers: IORegisters,
    pub mode: Mode,
}

impl PPU {
    pub fn new(io_registers: IORegisters) -> PPU {
        PPU {
            vram: [0; 0x2000],
            oam: OAM { oam_entries: [OamEntry::default(); 40].into()},
            io_registers,
            mode: Mode::Off,
        }
    }

    pub fn object_search(&self) -> Vec<OamEntry> {
        let scanline = self.io_registers.ly;
        let mut oam_vec: Vec<OamEntry> = Vec::new();
        let use_16 = self.io_registers.lcdc.obj_size == 1;
        for s in 0..40 {
            let new_entry = self.oam.oam_entries[s];
            oam_vec.push(new_entry);
        }
        let offset = if use_16 { 16 } else { 8 };
        oam_vec = oam_vec
            .into_iter()
            .filter(|oam_entry| {
                scanline + 16 >= oam_entry.y_coord && scanline + 16 < oam_entry.y_coord + offset
            })
            .take(10)
            .collect::<Vec<OamEntry>>();
        oam_vec.sort_by_key(|o| o.x_coord);
        oam_vec.reverse();
        oam_vec
    }

    pub fn draw(&self, sprites: Vec<OamEntry>) -> Vec<GameboyColor> {
        let scanline = self.io_registers.ly;
        let lcdc = self.io_registers.lcdc;
        let palette = self.io_registers.bgp;
        let mut color_line: Vec<GameboyColor> = Vec::new();
	for x in 0..=20 {
	    let use_window = lcdc.window_enable == 1
		&& scanline >= self.io_registers.wy
		&& x + 7 >= self.io_registers.wx;
	    //if lcdc.window_enable == 1 { println!("window"); }
	    let tile_index;
	    let pixel_y;
	    let tile_x;
	    let tile_y;
	    if use_window {
		println!("windowww!");
		let pixel_x = x + 7 - self.io_registers.wx;
		pixel_y = scanline - self.io_registers.wy;
		tile_x = pixel_x / 8;
		tile_y = pixel_y / 8;
		let window_map_index = tile_x as usize + 32 * tile_y as usize;
		let base_pointer = if lcdc.window_tile_map_area == 0 {
		    0x1800
		} else {
		    0x1C00
		};
		tile_index = self.vram[base_pointer + window_map_index];
	    } else {
		pixel_y = self.io_registers.scy.wrapping_add(scanline);
		tile_y = pixel_y / 8;
		tile_x = (self.io_registers.scx / 8 + x) & 0x1F;
		let background_map_index = tile_x as usize + 32 * tile_y as usize;
		let base_pointer = if lcdc.bg_tile_map_area == 0 {
		    0x1800
		} else {
		    0x1C00
		};
		tile_index = self.vram[base_pointer + background_map_index];
		//assert!(tile_index == self.get_tile(x), "new: {}, old: {}", tile_index, self.get_tile(x));
	    }
	    let mem_tile_index = if lcdc.bg_window_tile_data_area == 0 {
		((0x1000 as isize)
		 + ((tile_index as i8 as isize) * 16)
		 + (pixel_y % 8) as isize * 2) as usize
	    } else {
		(tile_index as usize * 16) + ((pixel_y % 8) as usize) * 2
	    };
	    if lcdc.bg_window_tile_data_area == 0 {
		//println!("other");
		assert!(mem_tile_index >= 0x800 && mem_tile_index <= 0x17FF, "mem_tile_index: {}", mem_tile_index)
	    }
		
	    Self::get_color_line(&mut color_line, self.vram[mem_tile_index + 1], self.vram[mem_tile_index], palette, false);
	}
	color_line.drain(0..(self.io_registers.scx % 8) as usize);
	color_line.drain(160..);

        for sprite in sprites {
            let tile_height = if lcdc.obj_size == 1 { 16 } else { 8 };
            let tile_row = if sprite.y_flip {
                tile_height - (scanline - (sprite.y_coord - 16))
            } else {
                scanline - (sprite.y_coord - 16)
            };
            let tile_low =
                self.vram[(sprite.data_tile_num as usize * 16 + 2 * tile_row as usize) as usize];
            let tile_high = self.vram
                [(sprite.data_tile_num as usize * 16 + 2 * tile_row as usize + 1) as usize];
            let sprite_palette = if sprite.palette {
                self.io_registers.obp1
            } else {
                self.io_registers.obp0
            };
            let mut sprite_row: Vec<GameboyColor> = Vec::new();
            PPU::get_color_line(&mut sprite_row, tile_high, tile_low, sprite_palette, true);
            if sprite.x_flip {
                sprite_row.reverse()
            };
            for i in 0..8 {
                if sprite.x_coord + i >= 168 || sprite.x_coord + i < 8 {
                    continue;
                }
                if (!sprite.sprite_priority
                    || color_line[(sprite.x_coord + i - 8) as usize] == GameboyColor::White)
		    && sprite_row[i as usize] != GameboyColor::Transparent
                {
                    color_line[(sprite.x_coord + i - 8) as usize] = sprite_row[i as usize];
                }
            }
        }
        color_line
    }

    fn get_tile(&self, x: u8) -> u8 {
        let cur_scanline = self.io_registers.ly;
        let lcdc = self.io_registers.lcdc;
        let window_y = self.io_registers.wy;
        let window_x = self.io_registers.wx;
        let scroll_y = self.io_registers.scy;
        let scroll_x = self.io_registers.scx;

        let use_window = cur_scanline >= window_y
            && x + 7 >= window_x / 8
            && x + 7 <= ((window_x as u16 + 160) / 8) as u8
            && lcdc.window_enable == 1;

        let fetcher_y = if use_window {
            cur_scanline - window_y
        } else {
            ((cur_scanline as u16 + scroll_y as u16) & 255) as u8
        };

        let fetcher_x = if use_window {
            x * 8 - window_x + 7
        } else {
            (scroll_x / 8 + x) & 0x1F
        };

        let tilemap_base: u16 = if use_window {
            if lcdc.window_tile_map_area == 1 {
                0x1C00
            } else {
                0x1800
            }
        } else {
            if lcdc.bg_tile_map_area == 1 {
                0x1C00
            } else {
                0x1800
            }
        };
        let vram_index = tilemap_base + ((fetcher_y as u16 / 8 * 32) + fetcher_x as u16) as u16;
        self.vram[vram_index as usize]
    }

    fn get_tile_data_low(&self, tile_index: u8) -> u8 {
        let cur_scanline = self.io_registers.ly;
        let lcdc = self.io_registers.lcdc;
        let tile_row = (cur_scanline % 8) as usize;

        let index = if lcdc.bg_window_tile_data_area == 0 {
            (0x1000 as usize)
                .wrapping_add(((tile_index as i8 as isize) * 16) as usize)
                .wrapping_add(tile_row * 2)
        } else {
            (tile_index as usize) * 16 + tile_row * 2
        };

        self.vram[index]
    }

    fn get_tile_data_high(&self, tile_index: u8) -> u8 {
        let cur_scanline = self.io_registers.ly;
        let lcdc = self.io_registers.lcdc;
        let tile_row = (cur_scanline % 8) as usize;
        let index = if lcdc.bg_window_tile_data_area == 0 {
            (0x1000 as usize)
                .wrapping_add(((tile_index as i8 as isize) * 16) as usize)
                .wrapping_add(tile_row * 2)
        } else {
            let tile_index = tile_index as usize;
            (tile_index as usize) * 16 + tile_row * 2
        };
        self.vram[index + 1]
    }

    fn get_color_line(
        color_line: &mut Vec<GameboyColor>,
        high_byte: u8,
        low_byte: u8,
        palette: u8,
	is_sprite: bool,
    ) {
        color_line.push(PPU::get_color(
	    ((0x80 & high_byte) >> 6) | ((0x80 & low_byte) >> 7),
	    palette,
	    is_sprite,
        ));

        color_line.push(PPU::get_color(
	    ((0x40 & high_byte) >> 5) | ((0x40 & low_byte) >> 6),
	    palette,
	    is_sprite,
        ));

        color_line.push(PPU::get_color(
	    ((0x20 & high_byte) >> 4) | ((0x20 & low_byte) >> 5),
	    palette,
	    is_sprite,
        ));

        color_line.push(PPU::get_color(
	    ((0x10 & high_byte) >> 3) | ((0x10 & low_byte) >> 4),
	    palette,
	    is_sprite,
        ));
        color_line.push(PPU::get_color(
	    ((0x08 & high_byte) >> 2) | ((0x08 & low_byte) >> 3),
	    palette,
	    is_sprite,
        ));

        color_line.push(PPU::get_color(
	    ((0x04 & high_byte) >> 1) | ((0x04 & low_byte) >> 2),
	    palette,
	    is_sprite,
        ));

        color_line.push(PPU::get_color(
	    ((0x02 & high_byte) >> 0) | ((0x02 & low_byte) >> 1),
	    palette,
	    is_sprite,
        ));
        color_line.push(PPU::get_color(
	    ((0x01 & high_byte) << 1) | ((0x01 & low_byte) >> 0),
	    palette,
	    is_sprite,
        ));
        for color in color_line {
            if *color != GameboyColor::White {
                //println!("{:?}", color);
            }
        }
    }

    fn get_color(color_number: u8, palette: u8, is_sprite: bool) -> GameboyColor {
        let color_id = match color_number {
            0 => if is_sprite {
		return GameboyColor::Transparent;
	    } else {
		palette & 0x03
	    },
            1 => (palette & 0x0C) >> 2,
            2 => (palette & 0x30) >> 4,
            3 => (palette & 0xC0) >> 6,
            _ => panic!("invalid color number: {}", color_number),
        };
        match color_id {
            0 => GameboyColor::White,
            1 => GameboyColor::LightGray,
            2 => GameboyColor::DarkGray,
            3 => GameboyColor::Black,
            _ => panic!("invalid color id: {}", color_id),
        }
    }
}

fn check_bit(num: u8, bit_num: u8) -> bool {
    !(num & (1 << bit_num) == 0)
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
    Off,
}
