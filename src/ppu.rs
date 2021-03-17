use queue::Queue;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::thread;
use std::{
    sync::{Arc, Mutex, RwLock, Barrier},
    time::Duration,
};

type Tile = [u16; 8];

#[derive(Clone, Copy, PartialEq)]
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

pub fn ppu_execute(ppu_lock: Arc<RwLock<PPU>>, barrier: Arc<Barrier>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Gameboy Window", 160, 144).build().unwrap();
    let mut canvas : Canvas<Window> = window.into_canvas()
	.present_vsync() 
	.build().unwrap();
    
    loop {
	let lcdc = {
	    ppu_lock.read().unwrap().io_registers.read().unwrap()[0x40]
	};

	if lcdc >> 7 != 1 {
	    PPU::clear_canvas(&mut canvas);
	} else {
	    for scanline in 0..144 {
		let pixel_row;
		{
		    let ppu = ppu_lock.read().unwrap();
		    let sprites = ppu.object_search(scanline);
		    barrier.wait();
		    pixel_row = ppu.draw(scanline, sprites);
		}
		barrier.wait();
		PPU::update_canvas(&mut canvas, pixel_row, scanline);
		barrier.wait();
	    }
	}
	barrier.wait();
    }
}

#[derive(Copy, Clone)]
struct OamEntry {
    y_coord: u8,
    x_coord: u8,
    data_tile_num: u8,
    sprite_priority: bool,
    y_flip: bool,
    x_flip: bool,
    palette: bool,
}

pub struct PPU {
    pub vram: [u8; 0x2000],
    pub oam: [u8; 0x00A0],
    pub io_registers: Arc<RwLock<[u8; 0x0080]>>,
    pub mode: Mode
}

impl PPU {
    pub fn new(io_registers: Arc<RwLock<[u8; 0x0080]>>) -> PPU {
	PPU {
	    vram: [0; 0x2000],
	    oam: [0; 0x00A0],
	    io_registers,
	    mode: Mode::Off,
	}
    }
    
    fn object_search(&self, scanline: u8) -> Vec<OamEntry> {
        let mut oam_vec: Vec<OamEntry> = Vec::new();
	let use_16;
	{
	    use_16 = check_bit(self.io_registers.read().unwrap()[0x40], 2);
	}
        for s in 0..40 {
	    let i = 4 * s;
            oam_vec.push(OamEntry {
                y_coord: self.oam[i],
                x_coord: self.oam[i + 1],
                data_tile_num: self.oam[i + 2],
                sprite_priority: !(self.oam[i + 3] & 8 == 0),
                y_flip: !(self.oam[i + 3] & 4 == 0),
                x_flip: !(self.oam[i + 3] & 2 == 0),
                palette: !(self.oam[i + 3] & 1 == 0),
            });
        }
	let offset = if use_16 { 16 } else { 8 };
        oam_vec = oam_vec
            .into_iter()
            .filter(|oam_entry| {
		(oam_entry.y_coord as u16) >= scanline as u16 + 16
		    && (oam_entry.y_coord as u16) < scanline as u16 + 16 + offset
            })
            .collect::<Vec<OamEntry>>();
        oam_vec.sort_by_key(|o| o.x_coord);
	oam_vec
    }

    fn draw(&self, scanline: u8, sprites: Vec<OamEntry>) -> Vec<GameboyColor> {
	let lcdc = self.io_registers.read().unwrap()[0x40];
	let palette = self.vram[0x47];
	let mut color_line: Vec<GameboyColor> = Vec::new();
	for tile_x in 0..20 {
            let tile_index = self.get_tile(tile_x, scanline);
            let tile_low = self.get_tile_data_low(tile_index, scanline);
            let tile_high = self.get_tile_data_high(tile_index, scanline);

	    color_line.push(PPU::get_color(tile_high >> 6, palette));
	    color_line.push(PPU::get_color((tile_high & 0x30) >> 6, palette));
	    color_line.push(PPU::get_color((tile_high & 0x0C) >> 6, palette));
	    color_line.push(PPU::get_color((tile_high & 0x03) >> 6, palette));
	    color_line.push(PPU::get_color(tile_low >> 6, palette));
	    color_line.push(PPU::get_color((tile_low & 0x30) >> 6, palette));
	    color_line.push(PPU::get_color((tile_low & 0x0C) >> 6, palette));
	    color_line.push(PPU::get_color((tile_low & 0x03) >> 6, palette));
	}
	
	for sprite in sprites {
	    let tile_height = if check_bit(lcdc, 2) { 16 } else { 8 };
	    let tile_row = if sprite.y_flip { tile_height - (scanline - sprite.y_coord) } else { scanline - sprite.y_coord };
	    let tile_low = self.vram[(sprite.data_tile_num * 16 + 2 * tile_row) as usize];
	    let tile_high = self.vram[(sprite.data_tile_num * 16 + 2 * tile_row) as usize];
	    let sprite_palette = if sprite.palette { self.io_registers.read().unwrap()[0x49] } else { self.io_registers.read().unwrap()[0x48] };
	    let mut sprite_row: Vec<GameboyColor> = Vec::new();

	    sprite_row.push(PPU::get_color(tile_high >> 6, sprite_palette));
	    sprite_row.push(PPU::get_color((tile_high & 0x30) >> 6, sprite_palette));
	    sprite_row.push(PPU::get_color((tile_high & 0x0C) >> 6, sprite_palette));
	    sprite_row.push(PPU::get_color((tile_high & 0x03) >> 6, sprite_palette));
	    sprite_row.push(PPU::get_color(tile_low >> 6, sprite_palette));
	    sprite_row.push(PPU::get_color((tile_low & 0x30) >> 6, sprite_palette));
	    sprite_row.push(PPU::get_color((tile_low & 0x0C) >> 6, sprite_palette));
	    sprite_row.push(PPU::get_color((tile_low & 0x03) >> 6, sprite_palette));

	    if sprite.x_flip { sprite_row.reverse() };

	    for i in 0..8 {
		if sprite.x_coord as i16 - 8 + i as i16 >= 160 || (sprite.x_coord as i16 - 8 + i as i16) < 0 {
		    continue;
		}
		if sprite.sprite_priority || color_line[(sprite.x_coord + i) as usize] == GameboyColor::White {
		    color_line[(sprite.x_coord + i) as usize] = sprite_row[i as usize];
		}
	    }
	}
	color_line
    }

    fn update_canvas(canvas: &mut Canvas<Window>, color_row: Vec<GameboyColor>, scanline: u8) {
        for x in 0..160 {
            canvas.set_draw_color(color_row[x]);
            canvas.draw_point((x as i32, scanline as i32)).unwrap();
        }
        canvas.present();
    }

    fn clear_canvas(canvas: &mut Canvas<Window>) {
	canvas.set_draw_color(GameboyColor::White);
        canvas.clear();
    }

    fn get_tile(&self, tile_x: u8, cur_scanline: u8) -> u8 {
	let lcdc: u8;
	let window_y: i16;
	let window_x: i16;
	let scroll_y: i16;
	let scroll_x: i16;
	{
            lcdc = self.io_registers.read().unwrap()[0x40];
            window_y = self.io_registers.read().unwrap()[0x4A] as i16;
            window_x = self.io_registers.read().unwrap()[0x4B] as i16;
            scroll_y = self.io_registers.read().unwrap()[0x42] as i16;
            scroll_x = self.io_registers.read().unwrap()[0x43] as i16;
	}
        let use_window = cur_scanline as i16 >= window_y && tile_x as i16 * 8 >= window_x - 7;

        let fetcher_y = if use_window {
            cur_scanline as i16 - window_y
        } else {
            (cur_scanline as i16 + scroll_y) & 0xFF
        };

        let fetcher_x = if use_window {
            (tile_x as i16 * 8) - (window_x - 7)
        } else {
            ((scroll_x / 8) + tile_x as i16) & 0x1F
        };

        let tilemap_base: u16 = if use_window {
            if lcdc & 0x40 == 0 {
                0x1800
            } else {
                0x1C00
            }
        } else {
            if lcdc & 0x08 == 0 {
                0x1800
            } else {
                0x1C00
            }
        };
        let vram_index = tilemap_base + (((fetcher_y / 8) * 32) + fetcher_x) as u16;
        self.vram[vram_index as usize]
    }

    fn get_tile_data_low(
        &self,
        tile_index: u8,
        cur_scanline: u8,
    ) -> u8 {
	let lcdc;
        {
	    lcdc = self.io_registers.read().unwrap()[0x40];
	}
        let tile_row = (cur_scanline % 8) as u16;
        if lcdc & 0x10 == 0 {
            let base = 0x1000 + (tile_index as i8 * 16) as i16;

            self.vram[(base.unsigned_abs() + 2 * tile_row) as usize]
        } else {
            let base = (tile_index * 16) as usize;
            self.vram[base + 2 * tile_row as usize]
        }
    }

    fn get_tile_data_high(
        &self,
        tile_index: u8,
        cur_scanline: u8,
    ) -> u8 {
        let tile_row = (cur_scanline % 8) as u16;
        let lcdc;
	{
	    lcdc = self.io_registers.read().unwrap()[0x40];
	}
        if lcdc & 0x10 == 0 {
            let base = 0x1000 + (tile_index as i8 * 16) as i16;
            self.vram[(base.unsigned_abs() + 1 + 2 * tile_row) as usize]
        } else {
            let base = (tile_index * 16) as usize;
            self.vram[base + 1 + 2 * tile_row as usize]
        }
    }

    fn get_color(color_number: u8, palette: u8) -> GameboyColor {
        let color_id = match color_number {
            0 => palette & 0x03,
            1 => palette & 0x0C,
            2 => palette & 0x30,
            3 => palette & 0xC0,
            _ => panic!("invalid color id"),
        };
        match color_id {
            0 => GameboyColor::White,
            1 => GameboyColor::LightGray,
            2 => GameboyColor::DarkGray,
            3 => GameboyColor::Black,
            _ => panic!("invalid color id"),
        }
    }

	
}

fn check_bit(num: u8, bit_num: u8) -> bool {
    !(num & (1 << bit_num) == 0)
}

pub enum Mode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
    Off,
}
