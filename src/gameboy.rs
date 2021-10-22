use crate::cpu::CPU;
use crate::ppu::{GameboyColor, Mode, PPU};
use byteorder::{ByteOrder, NativeEndian};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::Sdl;
use sdl2::{
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};
use spin_sleep::LoopHelper;
use std::fs;

const CPU_CYCLES_PER_FRAME: u32 = 69833;
const MODE2_CYCLES: u16 = 80 / 4;
const MODE3_CYCLES: u16 = 172 / 4;
const MODE0_CYCLES: u16 = 204 / 4;
const MODE1_CYCLES: u16 = 456 / 4;
const MODE_OFF_CYCLES: u16 = 456 / 4;

const MODE2_RPS: f64 = 4190000 as f64 / MODE2_CYCLES as f64;
const MODE3_RPS: f64 = 4190000 as f64 / MODE3_CYCLES as f64;
const MODE0_RPS: f64 = 4190000 as f64 / MODE0_CYCLES as f64;
const MODE1_RPS: f64 = 4190000 as f64 / MODE1_CYCLES as f64;
const MODE_OFF_RPS: f64 = 4190000 as f64 / MODE_OFF_CYCLES as f64;

pub struct Gameboy {
    pub cpu: CPU,
    mode: Mode,
}

impl<'a> Gameboy {
    pub fn new() -> Gameboy {
        let ppu = PPU::new(Default::default());
        let cpu = CPU::new(ppu);
        Gameboy {
            cpu,
            mode: Mode::Off,
        }
    }
    pub fn init_gameboy(&mut self) {
        let logo: Vec<u8> = vec![
            0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c,
            0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6,
            0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc,
            0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
        ];
        println!("Starting up!");
        let binary = &fs::read("bootix_dmg.bin").unwrap();
        println!("Boot Rom Length: {}", binary.len());
        let mut i = 0;
        for el in binary {
            self.cpu.memory.rom_bank0[i] = el.clone();
            i = i + 1;
        }
        let mut i = 0;
        for el in logo {
            self.cpu.memory.rom_bank0[0x104 + i] = el;
            i += 1;
        }
    }
    pub fn run_emulator(
        &mut self,
        sdl_context: &Sdl,
        texture_creator: &'a TextureCreator<WindowContext>,
        canvas: &mut Canvas<Window>,
    ) {
        let mut loop_helper = LoopHelper::builder().build_with_target_rate(60.0);
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut texture: Texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, 160, 144)
            .unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            while self.cpu.ppu.io_registers.lcdc.lcd_ppu_enable == 0 {
                self.cpu.step(false);
            }
            //println!("LCD on!");

            loop_helper.loop_start();
            canvas.clear();
            for scanline in 0..154 {
                self.cpu.ppu.io_registers.ly = scanline;
                self.cpu.ppu.io_registers.lcd_status.lyc_eq_ly =
                    if self.cpu.ppu.io_registers.ly == self.cpu.ppu.io_registers.lyc {
                        1
                    } else {
                        0
                    };

                if scanline < 144 {
                    let oam_entries = self.cpu.ppu.object_search();
                    for _i in 0..MODE2_CYCLES {
                        self.cpu.step(false);
                    }

                    let color_line = self.cpu.ppu.draw(oam_entries);
                    for _i in 0..MODE3_CYCLES {
                        self.cpu.step(false);
                    }
                    //println!("vram: {:?}", self.cpu.ppu.vram);
                    Gameboy::write_line_to_texture(&mut texture, color_line, scanline);
                    for _i in 0..MODE0_CYCLES {
                        self.cpu.step(false);
                    }
                } else {
                    for _i in 0..MODE1_CYCLES {
                        self.cpu.step(false);
                    }
                }
            }
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
            loop_helper.loop_sleep();
        }
    }

    pub fn write_line_to_texture(
        texture: &mut Texture,
        color_line: Vec<GameboyColor>,
        scanline: u8,
    ) {
        let color_line: Vec<u32> = color_line.iter().map(|color| color.into()).collect();
        for (index, color) in color_line.iter().enumerate() {
            let mut colors: [u8; 4] = [0; 4];
            NativeEndian::write_u32(&mut colors, *color);
            texture
                .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    for i in 0..4 {
                        buffer[scanline as usize * pitch + 4 * index + i] = colors[i];
                    }
                })
                .unwrap();
        }
    }
}
