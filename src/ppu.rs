use crate::register_maps::{InterruptEnable, InterruptFlag, Joypad, LCDC, LCDStatus};
use std::convert::TryInto;
use sdl2::pixels::Color;


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
	    GameboyColor::White =>       0xFF << 24 | 155 << 16 | 188 << 8 | 15,
            GameboyColor::LightGray =>   0xFF << 24 | 139 << 16 | 172 << 8 | 15,
            GameboyColor::DarkGray =>    0xFF << 24 | 48 << 16  | 98 << 8  | 48,
            GameboyColor::Black =>       0xFF << 24 | 15 << 16  | 56 << 8  | 15,
            GameboyColor::Transparent => 0xFF << 24 | 15 << 16  | 56 << 8  | 15,
	}
    }
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Clone, Copy)]
pub struct PPU {
    pub vram: [u8; 0x2000],
    pub oam: [u8; 0x00A0],
    pub io_registers: IORegisters,
    pub mode: Mode,
}

impl PPU {
    pub fn new(io_registers: IORegisters) -> PPU {
	PPU {
	    vram: [0; 0x2000],
	    oam: [0; 0x00A0],
	    io_registers,
	    mode: Mode::Off,
	}
    }
    
    pub fn object_search(&self) -> Vec<OamEntry> {
	let scanline = self.io_registers.ly;
        let mut oam_vec: Vec<OamEntry> = Vec::new();
	let use_16 = self.io_registers.lcdc.obj_size == 1;
        for s in 0..40 {
	    let i = 4 * s;
	    let new_entry: [u8; 4] = self.oam[i .. i+4].try_into().unwrap();
            oam_vec.push(new_entry.into());
        }
	let offset = if use_16 { 16 } else { 8 };
        oam_vec = oam_vec
            .into_iter()
            .filter(|oam_entry| {
		scanline + 16 >= oam_entry.y_coord
		    && scanline + 16 < oam_entry.y_coord + offset
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
	for tile_x in 0..20 {
            let tile_index = self.get_tile(tile_x);
            let tile_low = self.get_tile_data_low(tile_index);
            let tile_high = self.get_tile_data_high(tile_index);
	    PPU::get_color_line(&mut color_line, tile_high, tile_low, palette);
	}
	
	for sprite in sprites {
	    let tile_height = if lcdc.obj_size == 1 { 16 } else { 8 };
	    let tile_row = if sprite.y_flip { tile_height - (scanline - sprite.y_coord) } else { scanline - sprite.y_coord };
	    let tile_low = self.vram[(sprite.data_tile_num * 16 + 2 * tile_row) as usize];
	    let tile_high = self.vram[(sprite.data_tile_num * 16 + 2 * tile_row) as usize];
	    let sprite_palette = if sprite.palette { self.io_registers.obp1 } else { self.io_registers.obp0 };
	    let mut sprite_row: Vec<GameboyColor> = Vec::new();
	    PPU::get_color_line(&mut sprite_row, tile_high, tile_low, sprite_palette);
	    if sprite.x_flip { sprite_row.reverse() };
	    for i in 0..8 {
		if sprite.x_coord + i >= 168 || sprite.x_coord + i < 8 {
		    continue;
		}
		if sprite.sprite_priority || color_line[(sprite.x_coord + i) as usize] == GameboyColor::White {
		    color_line[(sprite.x_coord + i) as usize] = sprite_row[i as usize];
		}
	    }
	}
	color_line
    }

    fn get_tile(&self, x: u8) -> u8 {
	let cur_scanline = self.io_registers.ly;
	let lcdc  = self.io_registers.lcdc;
	let window_y = self.io_registers.wy;
	let window_x = self.io_registers.wx;
	let scroll_y = self.io_registers.scy;
	let scroll_x = self.io_registers.scx;

	let use_window = cur_scanline >= window_y && x + 7 >= window_x / 8 && x + 7 <= ((window_x as u16 + 160) / 8) as u8 && lcdc.window_enable == 1;

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

    fn get_tile_data_low(
        &self,
        tile_index: u8,
    ) -> u8 {
	let cur_scanline = self.io_registers.ly;
	let lcdc = self.io_registers.lcdc;
        let tile_row = (cur_scanline % 8) as usize;
	let tile_index = tile_index as usize;
        let index = if lcdc.bg_window_tile_data_area == 0 {
	    if tile_index < 128 {
		0x1000 + tile_index * 16 + tile_row * 2
	    } else {
		0x0800 + tile_index * 16 + tile_row * 2
	    }
        } else {
	    tile_index * 16 + tile_row * 2
        };
	
	self.vram[index]
    }

    fn get_tile_data_high(
        &self,
        tile_index: u8,
    ) -> u8 {
	let cur_scanline = self.io_registers.ly;
	let lcdc = self.io_registers.lcdc;
        let tile_row = (cur_scanline % 8) as usize;
	let tile_index = tile_index as usize;
        let index = if lcdc.bg_window_tile_data_area == 0 {
	    if tile_index < 128 {
		0x1000 + tile_index * 16 + tile_row * 2
	    } else {
		0x0800 + tile_index * 16 + tile_row * 2
	    }
        } else {
	    tile_index * 16 + tile_row * 2
        };
	self.vram[index + 1]
    }

    fn get_color_line(color_line: &mut Vec<GameboyColor>, high_byte: u8, low_byte: u8, palette: u8) {
	color_line.push(PPU::get_color(((0x80 & high_byte) >> 6) | ((0x80 & low_byte) >> 7), palette));
	color_line.push(PPU::get_color(((0x40 & high_byte) >> 5) | ((0x40 & low_byte) >> 6), palette));
	color_line.push(PPU::get_color(((0x20 & high_byte) >> 4) | ((0x20 & low_byte) >> 5), palette));
	color_line.push(PPU::get_color(((0x10 & high_byte) >> 3) | ((0x20 & low_byte) >> 4), palette));
	color_line.push(PPU::get_color(((0x08 & high_byte) >> 2) | ((0x08 & low_byte) >> 3), palette));
	color_line.push(PPU::get_color(((0x04 & high_byte) >> 1) | ((0x04 & low_byte) >> 2), palette));
	color_line.push(PPU::get_color(((0x02 & high_byte) >> 0) | ((0x04 & low_byte) >> 1), palette));
	color_line.push(PPU::get_color(((0x01 & high_byte) << 1) | ((0x01 & low_byte) >> 0), palette));
	for color in color_line {
	    if *color != GameboyColor::White {
		//println!("{:?}", color);
	    }
	}
	    
    }
    
    fn get_color(color_number: u8, palette: u8) -> GameboyColor {
        let color_id = match color_number {
            0 => palette & 0x03,
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

#[derive(Copy, Clone, Default)]    
pub struct IORegisters {
    pub joypad: Joypad,
    pub communication: [u8; 2], //0xFF01 - 0xFF02
    pub divider_and_timer: [u8; 4], // 0xFF04 - 0xFF07
    pub interrupt_flag: InterruptFlag, // 0xFF0F
    pub sound: [u8; 0x17], // 0xFF10 - 0xFF26
    pub waveform_ram: [u8; 0x10], // 0xFF30 - 0xFF3F
    pub lcdc: LCDC, // 0xFF40
    pub lcd_status: LCDStatus, // 0xFF41
    pub scy: u8, // 0xFF42
    pub scx: u8, // 0xFF43
    pub ly: u8, // 0xFF44
    pub lyc: u8, // 0xFF45
    pub dma: u8, // 0xFF46
    pub bgp: u8, // 0xFF47
    pub obp0: u8, // 0xFF48
    pub obp1: u8, // 0xFF49
    pub wy: u8, // 0xFF4A
    pub wx: u8, // 0xFF4B
    pub ie: InterruptEnable, // 0xFFFF
}

impl IORegisters {
    pub fn get(&self, index: usize) -> u8 {
	match index {
	    0xFF00 => self.joypad.into(),
	    0xFF01 | 0xFF02 => self.communication[index - 0xFF00],
	    0xFF04..=0xFF07 => self.divider_and_timer[index - 0xFF00],
	    0xFF10..=0xFF26 => self.sound[index - 0xFF00],
	    0xFF30..=0xFF3F => self.waveform_ram[index - 0xFF00],
	    0xFF40 => self.lcdc.into(),
	    0xFF41 => self.lcd_status.into(),
	    0xFF42 => self.scy,
	    0xFF43 => self.scx,
	    0xFF44 => self.ly,
	    0xFF45 => self.lyc,
	    0xFF46 => self.dma,
	    0xFF47 => self.bgp,
	    0xFF48 => self.obp0,
	    0xFF49 => self.obp1,
	    0xFF4A => self.wy,
	    0xFF4B => self.wx,
	    _ => panic!("invalid index for io registers"),
	}
    }
    
    pub fn set(&mut self, index: usize, value: u8) {
	match index {
	    0xFF00 => self.joypad = value.into(),
	    0xFF01 | 0xFF02 => self.communication[index - 0xFF01] = value,
	    0xFF04..=0xFF07 => self.divider_and_timer[index - 0xFF04] = value,
	    0xFF10..=0xFF26 => self.sound[index - 0xFF10] = value,
	    0xFF30..=0xFF3F => self.waveform_ram[index - 0xFF30] = value,
	    0xFF40 => self.lcdc = value.into(),
	    0xFF41 => self.lcd_status = value.into(),
	    0xFF42 => self.scy = value,
	    0xFF43 => self.scx = value,
	    0xFF44 => self.ly = value,
	    0xFF45 => self.lyc = value,
	    0xFF46 => self.dma = value,
	    0xFF47 => self.bgp = value,
	    0xFF48 => self.obp0 = value,
	    0xFF49 => self.obp1 = value,
	    0xFF4A => self.wy = value,
	    0xFF4B => self.wx = value,
	    0xFF50 => panic!("done"),
	    _ => panic!("invalid index for io registers: 0x{:x}", index),
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
