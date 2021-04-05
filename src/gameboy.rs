use crate::cpu::CPU;
use crate::instructions::StagePassThrough;
use crate::ppu::{GameboyColor, Mode, PPU};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use spin_sleep::LoopHelper;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

const MODE2_CYCLES: u16 = 80;
const MODE3_CYCLES: u16 = 172;
const MODE0_CYCLES: u16 = 204;
const MODE1_CYCLES: u16 = 4560;
const MODE_OFF_CYCLES: u16 = 456;
const MODE2_RPS: f64 = 4190000 as f64 / MODE2_CYCLES as f64;
const MODE3_RPS: f64 = 4190000 as f64 / MODE3_CYCLES as f64;
const MODE0_RPS: f64 = 4190000 as f64 / MODE0_CYCLES as f64;
const MODE1_RPS: f64 = 4190000 as f64 / MODE1_CYCLES as f64;
const MODE_OFF_RPS: f64 = 4190000 as f64 / MODE_OFF_CYCLES as f64;


pub struct Gameboy {
    pub cpu: CPU,
    mode: Mode,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        let io_registers = [0; 0x80];
        let ppu = PPU::new(io_registers);
        let cpu = CPU::new(ppu);
        Gameboy {
            cpu,
            mode: Mode::Off,
        }
    }

    pub fn run_emulator(&mut self) {
	self.cpu.addr_bus = self.cpu.pc;
        let mut pass = (self.cpu.read(), StagePassThrough::default());
        let mut scanline = 0;
        let mut oam_vec = Vec::new();
        let mut color_line = Vec::new();
        let mut loop_helper = LoopHelper::builder().build_with_target_rate(MODE_OFF_RPS);
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Gameboy Window", 160, 144)
            .build()
            .unwrap();
        let mut canvas: Canvas<Window> = window
	    .into_canvas()
	    .present_vsync()
	    .build()
	    .unwrap();
	canvas.set_draw_color(GameboyColor::White);
	canvas.clear();
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
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
            match self.cpu.mode {
                Mode::Off => {
                    loop_helper.loop_start();
                    pass = self.cpu.execute_n_cycles(false, MODE_OFF_CYCLES, pass);
                    loop_helper.loop_sleep();
		    println!("yello: {}", self.cpu.pc);
		    
                    if self.cpu.ppu.io_registers[0x40] >> 7 == 1 {
			println!("new mode!");
                        loop_helper.set_target_rate(MODE2_RPS);
                        self.cpu.mode = Mode::Mode2;
                    }
                }
                Mode::Mode2 => {
                    loop_helper.loop_start();
                    pass = self.cpu.execute_n_cycles(false, MODE2_CYCLES, pass);
                    oam_vec = self.cpu.ppu.object_search(scanline);
                    loop_helper.loop_sleep();
                    if self.cpu.ppu.io_registers[0x40] >> 7 == 0 {
                        self.cpu.mode = Mode::Off;
                        loop_helper.set_target_rate(MODE_OFF_RPS);
			PPU::clear_canvas(&mut canvas);
                    } else {
                        self.cpu.mode = Mode::Mode3;
                        loop_helper.set_target_rate(MODE3_RPS);
                    }
                }
                Mode::Mode3 => {
                    loop_helper.loop_start();
                    pass = self.cpu.execute_n_cycles(false, MODE3_CYCLES, pass);
                    color_line = self.cpu.ppu.draw(scanline, oam_vec.clone());
                    loop_helper.loop_sleep();
                    if self.cpu.ppu.io_registers[0x40] >> 7 == 0 {
                        self.cpu.mode = Mode::Off;
                        loop_helper.set_target_rate(MODE_OFF_RPS);
			PPU::clear_canvas(&mut canvas);
                    } else {
                        self.cpu.mode = Mode::Mode0;
                        loop_helper.set_target_rate(MODE0_RPS);
                    }
                }
                Mode::Mode0 => {
                    loop_helper.loop_start();
                    pass = self.cpu.execute_n_cycles(false, MODE0_CYCLES, pass);
		    PPU::update_canvas(&mut canvas, color_line.clone(), scanline);
                    loop_helper.loop_sleep();
                    if self.cpu.ppu.io_registers[0x40] >> 7 == 0 {
			scanline = 0;
                        self.cpu.mode = Mode::Off;
                        loop_helper.set_target_rate(MODE_OFF_RPS);
			PPU::clear_canvas(&mut canvas);
                    } else {
			if scanline == 143 {
			    scanline = 0;
			    self.cpu.mode = Mode::Mode1;
			    loop_helper.set_target_rate(MODE1_RPS);
			} else {
			    scanline += 1;
			    
			}
                    }
                }
		Mode::Mode1 => {
		    loop_helper.loop_start();
                    pass = self.cpu.execute_n_cycles(false, MODE1_CYCLES, pass);
                    loop_helper.loop_sleep();
                    if self.cpu.ppu.io_registers[0x40] >> 7 == 1 {
                        loop_helper.set_target_rate(MODE2_RPS);
                        self.cpu.mode = Mode::Mode2;
                    } else {
			loop_helper.set_target_rate(MODE_OFF_RPS);
                        self.cpu.mode = Mode::Off;
			PPU::clear_canvas(&mut canvas);
		    }
		}
            }
	    self.cpu.ppu.io_registers[0x44] = scanline;
	    if self.cpu.ppu.io_registers[0x44] == self.cpu.ppu.io_registers[0x45] {
		self.cpu.ppu.io_registers[0x41] = 4 | self.cpu.ppu.io_registers[0x41]; // Set LY=LYC flag
	    } else {
		self.cpu.ppu.io_registers[0x41] = !4 | self.cpu.ppu.io_registers[0x41]; // Unset LY=LYC flag
	    }
        }
    }
}
