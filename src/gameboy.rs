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
        println!("Starting up!");
        let binary = &fs::read("bootix_dmg.bin").unwrap();
        println!("Boot Rom Length: {}", binary.len());
	let path = std::env::args().nth(1).expect("First argument must be binary");
	let cartridge_binary = &fs::read(path).expect("Must provide binary!");
        let mut i = 0;
        for el in binary {
            self.cpu.memory.boot_rom[i] = el.clone();
            i = i + 1;
        }
	println!("Cartridge Rom Length: 0x{:x}", cartridge_binary.len());
	for (i, el) in cartridge_binary.iter().enumerate() {
	    if i < 0x4000 {
		self.cpu.memory.rom_bank0[i] = el.clone();
	    } else {
		self.cpu.memory.rom_bank1[i - 0x4000] = el.clone();
	    }
	}
    }

    pub fn step_gameboy(&mut self) {
	self.cpu.step(false);
    }
    
    pub fn run_without_graphics(&mut self) {
	loop {
	    println!("{}", self.cpu.pc);
            self.cpu.step(false);
        }
    }

    pub fn run_emulator(
        &mut self,
        sdl_context: &Sdl,
        texture_creator: &'a TextureCreator<WindowContext>,
        canvas: &mut Canvas<Window>,
    ) {
        let mut loop_helper = LoopHelper::builder().report_interval_s(0.5).build_with_target_rate(59.7);
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut texture: Texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, 160, 144)
            .unwrap();
	
	let mut frame_buffer: [u8; 92160] = [0; 92160];
	let mut current_fps;
        'running: loop {
	    for event in event_pump.poll_iter() {
		if self.handle_input(event) {
		    break 'running;
		}
	    }
	    
	    if let Some(fps) = loop_helper.report_rate() {
		current_fps = Some(fps.round());
		println!("current fps: {}", current_fps.unwrap());
	    }

            while self.cpu.ppu.io_registers.lcdc.lcd_ppu_enable == 0 {
                self.cpu.step(false);
            }
            //println!("LCD on!");

            loop_helper.loop_start();
            canvas.clear();
            for scanline in 0..154 {
                self.cpu.ppu.io_registers.ly = scanline;
		if self.cpu.ppu.io_registers.ly == self.cpu.ppu.io_registers.lyc {
                    self.cpu.ppu.io_registers.lcd_status.lyc_eq_ly = 1;
		    if self.cpu.ppu.io_registers.lcd_status.lyc_ly_stat_interrupt == 1 {
			self.cpu.ppu.io_registers.interrupt_flag.lcd_stat = 1;
		    }
                } else {
                    self.cpu.ppu.io_registers.lcd_status.lyc_eq_ly = 0;
                };
                
                if scanline < 144 {
                    let oam_entries = self.cpu.ppu.object_search();
		    self.cpu.ppu.io_registers.lcd_status.mode = 2;
		    if self.cpu.ppu.io_registers.lcd_status.mode_two_stat_interrupt == 1 {
			self.cpu.ppu.io_registers.interrupt_flag.lcd_stat = 1;
		    }
                    for _i in 0..MODE2_CYCLES {
                        self.cpu.step(false);
                    }

                    let color_line = self.cpu.ppu.draw(oam_entries);
		    self.cpu.ppu.io_registers.lcd_status.mode = 3;
                    for _i in 0..MODE3_CYCLES {
                        self.cpu.step(false);
                    }
                    Gameboy::write_line_to_frame_buffer(&mut frame_buffer, color_line, scanline);
		    self.cpu.ppu.io_registers.lcd_status.mode = 0;
		    if self.cpu.ppu.io_registers.lcd_status.mode_zero_stat_interrupt == 1 {
			self.cpu.ppu.io_registers.interrupt_flag.lcd_stat = 1;
		    }
                    for _i in 0..MODE0_CYCLES {
                        self.cpu.step(false);
                    }
                } else {
		    if scanline == 144 {
			self.cpu.ppu.io_registers.interrupt_flag.vblank = 1;
			if self.cpu.ppu.io_registers.lcd_status.mode_one_stat_interrupt == 1 {
			    self.cpu.ppu.io_registers.interrupt_flag.lcd_stat = 1;
			}
		    }
		    self.cpu.ppu.io_registers.lcd_status.mode = 1;
                    for _i in 0..MODE1_CYCLES {
                        self.cpu.step(false);
			if self.cpu.ppu.io_registers.lcdc.lcd_ppu_enable == 0 {
			    loop_helper.loop_sleep();
			    canvas.set_draw_color(GameboyColor::White);
			    canvas.clear();
			    self.cpu.ppu.io_registers.ly = 0;
			    self.cpu.ppu.io_registers.lcd_status.mode = 0;
			    continue 'running;
			}
                    }
                }
            }
	    texture
                .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                    for (i, el) in frame_buffer.iter().enumerate() {
                        buffer[i] = *el;
                    }
                })
                .unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
            loop_helper.loop_sleep();
        }
    }

    pub fn write_line_to_frame_buffer(
        frame_buffer: &mut [u8; 92160],
        color_line: Vec<GameboyColor>,
        scanline: u8,
    ) {
        let color_line: Vec<u32> = color_line.iter().map(|color| color.into()).collect();
        for (index, color) in color_line.iter().enumerate() {
	    let begin_index = scanline as usize * 160 * 4 + 4 * index;
            NativeEndian::write_u32(&mut frame_buffer[begin_index .. begin_index + 4], *color);
        }
    }

    fn handle_input(&mut self, event: Event) -> bool {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return true,
	    Event::KeyDown {
		keycode: Some(Keycode::Down),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_direction == 0 {
		    self.cpu.ppu.io_registers.joypad.down_or_start = 1;
		}
	    }
	    Event::KeyDown {
		keycode: Some(Keycode::Up),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_direction == 0 {
		    self.cpu.ppu.io_registers.joypad.up_or_select = 1;
		}
	    }
	    Event::KeyDown {
		keycode: Some(Keycode::Left),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_direction == 0 {
		    self.cpu.ppu.io_registers.joypad.left_or_b = 1;
		}
	    }
	    Event::KeyDown {
		keycode: Some(Keycode::Right),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_direction == 0 {
		    self.cpu.ppu.io_registers.joypad.right_or_a = 0;
		}
	    }
	    Event::KeyUp {
		keycode: Some(Keycode::Down),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_direction == 0 {
		    self.cpu.ppu.io_registers.joypad.down_or_start = 0;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		    
		}
	    }
	    Event::KeyUp {
		keycode: Some(Keycode::Up),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_direction == 0 {
		    self.cpu.ppu.io_registers.joypad.up_or_select = 0;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }
	    Event::KeyUp {
		keycode: Some(Keycode::Left),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_direction == 0 {
		    self.cpu.ppu.io_registers.joypad.left_or_b = 0;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }
	    Event::KeyUp {
		keycode: Some(Keycode::Right),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_direction == 0 {
		    self.cpu.ppu.io_registers.joypad.right_or_a = 0;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }

	    
	    Event::KeyDown {
		keycode: Some(Keycode::RShift),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_action == 0 {
		    self.cpu.ppu.io_registers.joypad.down_or_start = 1;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }
	    Event::KeyDown {
		keycode: Some(Keycode::Return),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_action == 0 {
		    self.cpu.ppu.io_registers.joypad.up_or_select = 1;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }
	    Event::KeyDown {
		keycode: Some(Keycode::A),
                ..
	    } => {
		println!("a pressed!");
		if self.cpu.ppu.io_registers.joypad.select_action == 0 {
		    self.cpu.ppu.io_registers.joypad.left_or_b = 1;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }
	    Event::KeyDown {
		keycode: Some(Keycode::S),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_action == 0 {
		    self.cpu.ppu.io_registers.joypad.right_or_a = 0;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }
	    Event::KeyUp {
		keycode: Some(Keycode::RShift),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_action == 0 {
		    self.cpu.ppu.io_registers.joypad.down_or_start = 0;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }
	    Event::KeyUp {
		keycode: Some(Keycode::Return),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_action == 0 {
		    self.cpu.ppu.io_registers.joypad.up_or_select = 0;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }
	    Event::KeyUp {
		keycode: Some(Keycode::A),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_action == 0 {
		    self.cpu.ppu.io_registers.joypad.left_or_b = 0;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }
	    Event::KeyUp {
		keycode: Some(Keycode::S),
                ..
	    } => {
		if self.cpu.ppu.io_registers.joypad.select_action == 0 {
		    self.cpu.ppu.io_registers.joypad.right_or_a = 0;
		    self.cpu.ppu.io_registers.interrupt_flag.joypad = 1;
		}
	    }
            _ => {},
        }
	return false;
    }
}
