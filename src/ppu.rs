use queue::Queue;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

type Tile = [u16; 8];

#[derive(Clone, Copy)]
enum GameboyColor {
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

pub fn ppu_execute(ppu_lock: Arc<RwLock<PPU>>, clock_rx: Receiver<u8>) {
    let (ppu_clk_send, ppu_clk_recv) = channel();
    thread::spawn(move || loop {
        for _ in 0..80 {
            clock_rx.recv().unwrap();
        }
        ppu_clk_send.send(0).unwrap();
        for _ in 0..172 {
            clock_rx.recv().unwrap();
        }
        ppu_clk_send.send(0).unwrap();
        for _ in 0..204 {
            clock_rx.recv().unwrap();
        }
        ppu_clk_send.send(0).unwrap();
    });

    
    
}

pub struct PPU {
    pub vram: [u8; 0x1FFF],
    pub oam: [u8; 0x009F],
    pixel_fifo: Queue<GameboyColor>,
    screen: [[GameboyColor; 144]; 160],
}

impl PPU {
    fn update_canvas(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(GameboyColor::White);
        canvas.clear();
        for x in 0..160 {
            for y in 0..144 {
                canvas.set_draw_color(self.screen[x][y]);
                canvas.draw_point((x as i32, y as i32)).unwrap();
            }
        }
        canvas.present();
    }

    

    fn pixel_fetcher(&mut self, tile_x: u8, cur_scanline: u8, io_registers: &[u8; 0x007F]) {
        let palette = self.vram[0x47];
        let tile_index = self.get_tile(tile_x, cur_scanline, io_registers);
        let tile_low = self.get_tile_data_low(tile_index, cur_scanline, io_registers);
        let tile_high = self.get_tile_data_high(tile_index, cur_scanline, io_registers);

        self.pixel_fifo
            .queue(PPU::get_color(tile_high >> 6, palette));
        self.pixel_fifo
            .queue(PPU::get_color((tile_high & 0x30) >> 6, palette));
        self.pixel_fifo
            .queue(PPU::get_color((tile_high & 0x0C) >> 6, palette));
        self.pixel_fifo
            .queue(PPU::get_color((tile_high & 0x03) >> 6, palette));

        self.pixel_fifo
            .queue(PPU::get_color(tile_low >> 6, palette));
        self.pixel_fifo
            .queue(PPU::get_color((tile_low & 0x30) >> 6, palette));
        self.pixel_fifo
            .queue(PPU::get_color((tile_low & 0x0C) >> 6, palette));
        self.pixel_fifo
            .queue(PPU::get_color((tile_low & 0x03) >> 6, palette));
    }

    fn get_tile(&self, tile_x: u8, cur_scanline: u8, io_registers: &[u8; 0x007F]) -> u8 {
        let lcdc = io_registers[0x41];
        let window_y = io_registers[0x4A];
        let window_x = io_registers[0x4B];
        let scroll_y = io_registers[0x42];
        let scroll_x = io_registers[0x43];
        let use_window = cur_scanline >= window_y && tile_x * 8 >= window_x - 7;

        let fetcher_y = if use_window {
            cur_scanline - window_y
        } else {
            (cur_scanline + scroll_y) & 0xFF
        };

        let fetcher_x = if use_window {
            (tile_x * 8) - (window_x - 7)
        } else {
            ((scroll_x / 8) + tile_x) & 0x1F
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
        io_registers: &[u8; 0x007F],
    ) -> u8 {
        let lcdc = io_registers[0x41];
        let tile_row = (cur_scanline % 8) as u16;
        if lcdc & 0x10 == 0 {
            let base = 0x1000 + (tile_index as i8 * 2) as i16;

            self.vram[(base.unsigned_abs() + 2 * tile_row) as usize]
        } else {
            let base = (tile_index * 2) as usize;
            self.vram[base + 2 * tile_row as usize]
        }
    }

    fn get_tile_data_high(
        &self,
        tile_index: u8,
        cur_scanline: u8,
        io_registers: &[u8; 0x007F],
    ) -> u8 {
        let tile_row = (cur_scanline % 8) as u16;
        let lcdc = io_registers[0x41];
        if lcdc & 0x10 == 0 {
            let base = 0x1000 + (tile_index as i8 * 2) as i16;
            self.vram[(base.unsigned_abs() + 1 + 2 * tile_row) as usize]
        } else {
            let base = (tile_index * 2) as usize;
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
