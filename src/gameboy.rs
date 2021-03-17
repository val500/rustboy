use crate::cpu::CPU;
use crate::instructions::StagePassThrough;
use crate::ppu::{GameboyColor, Mode, PPU};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use spin_sleep::LoopHelper;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

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
        let mode2_cycles = 80;
        let mode3_cycles = 172;
        let mode0_cycles = 204;
        let mode1_cycles = 4560;
        let mode_off_cycles = 456;
        let mode2_rps: f64 = 4190000 as f64 / mode2_cycles as f64;
        let mode3_rps: f64 = 4190000 as f64 / mode3_cycles as f64;
        let mode0_rps: f64 = 4190000 as f64 / mode0_cycles as f64;
        let mode1_rps: f64 = 4190000 as f64 / mode1_cycles as f64;
        let mode_off_rps: f64 = 4190000 as f64 / mode_off_cycles as f64;
	self.cpu.addr_bus = self.cpu.pc;
        let mut pass = (self.cpu.read(), StagePassThrough::default());
        let mut scanline = 0;
        let mut oam_vec = Vec::new();
        let mut color_line = Vec::new();
        let mut loop_helper = LoopHelper::builder().build_with_target_rate(mode1_rps);
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
                    pass = self.cpu.execute_n_cycles(false, mode_off_cycles, pass);
                    loop_helper.loop_sleep();
		    println!("yello: {}", self.cpu.pc);
		    
                    if self.cpu.ppu.io_registers[0x40] >> 7 == 1 {
			println!("new mode!");
                        loop_helper.set_target_rate(mode2_rps);
                        self.cpu.mode = Mode::Mode2;
                    }
                }
                Mode::Mode2 => {
                    loop_helper.loop_start();
                    pass = self.cpu.execute_n_cycles(false, mode2_cycles, pass);
                    oam_vec = self.cpu.ppu.object_search(scanline);
                    loop_helper.loop_sleep();
                    if self.cpu.ppu.io_registers[0x40] >> 7 == 0 {
                        self.cpu.mode = Mode::Off;
                        loop_helper.set_target_rate(mode_off_rps);
			PPU::clear_canvas(&mut canvas);
                    } else {
                        self.cpu.mode = Mode::Mode3;
                        loop_helper.set_target_rate(mode3_rps);
                    }
                }
                Mode::Mode3 => {
                    loop_helper.loop_start();
                    pass = self.cpu.execute_n_cycles(false, mode3_cycles, pass);
                    color_line = self.cpu.ppu.draw(scanline, oam_vec.clone());
                    loop_helper.loop_sleep();
                    if self.cpu.ppu.io_registers[0x40] >> 7 == 0 {
                        self.cpu.mode = Mode::Off;
                        loop_helper.set_target_rate(mode_off_rps);
			PPU::clear_canvas(&mut canvas);
                    } else {
                        self.cpu.mode = Mode::Mode0;
                        loop_helper.set_target_rate(mode0_rps);
                    }
                }
                Mode::Mode0 => {
                    loop_helper.loop_start();
                    pass = self.cpu.execute_n_cycles(false, mode0_cycles, pass);
		    PPU::update_canvas(&mut canvas, color_line.clone(), scanline);
                    loop_helper.loop_sleep();
                    if self.cpu.ppu.io_registers[0x40] >> 7 == 0 {
			scanline = 0;
                        self.cpu.mode = Mode::Off;
                        loop_helper.set_target_rate(mode_off_rps);
			PPU::clear_canvas(&mut canvas);
                    } else {
			if scanline == 143 {
			    scanline = 0;
			    self.cpu.mode = Mode::Mode1;
			    loop_helper.set_target_rate(mode1_rps);
			} else {
			    scanline += 1;
			}
                    }
                }
		Mode::Mode1 => {
		    loop_helper.loop_start();
                    pass = self.cpu.execute_n_cycles(false, mode1_cycles, pass);
                    loop_helper.loop_sleep();
                    if self.cpu.ppu.io_registers[0x40] >> 7 == 1 {
                        loop_helper.set_target_rate(mode2_rps);
                        self.cpu.mode = Mode::Mode2;
                    } else {
			loop_helper.set_target_rate(mode_off_rps);
                        self.cpu.mode = Mode::Off;
			PPU::clear_canvas(&mut canvas);
		    }
		}
            }
        }
    }
}
