#![feature(derive_default_enum)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![feature(wrapping_int_impl)]
mod cpu;
mod gameboy;
mod instructions;
mod ppu;
mod register_maps;
use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

const X_DIM: u32 = 160;
const Y_DIM: u32 = 144;
const SCALE: u32 = 8;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Gameboy Window", X_DIM * SCALE, Y_DIM * SCALE)
        .opengl()
        .build()
        .unwrap();

    /*
    let debug_window = video_subsystem
        .window("Gameboy Window", 256 * SCALE, 256 * SCALE)
        .opengl()
        .build()
        .unwrap();
     */
    
    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

    //let mut debug_canvas = debug_window.into_canvas().present_vsync().build().unwrap();
//    debug_canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();
    
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
//    let debug_texture_creator = debug_canvas.texture_creator();
    
    let mut gameboy = gameboy::Gameboy::new();
    gameboy.init_gameboy();
    gameboy.run_emulator(&sdl_context, &texture_creator, &mut canvas);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    
    #[test]
    fn test_timer() {
        let mut gameboy = gameboy::Gameboy::new();
	let bin = &fs::read("./baz.gb").expect("Must provide binary!");
	let mut i = 0;
	gameboy.cpu.reg_file.SP = 0x8000;
	for el in bin {
	    gameboy.cpu.memory.boot_rom[i] = el.clone();
	    i = i + 1;
	}
	let mut timer = &mut gameboy.cpu.ppu.io_registers.timer;
	timer.tma = 0.into();
	timer.tac = 5.into();

	assert!(timer.tac.timer_enable);
	assert_eq!(timer.tac.input_clock_select, register_maps::InputClockSelect::Mode16);
	gameboy.run_without_graphics();

	
    }
}
