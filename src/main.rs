#![allow(dead_code)]
#![feature(or_patterns)]
#![feature(unsigned_abs)]
#![allow(non_snake_case)]
<<<<<<< Updated upstream
=======
#![feature(wrapping_int_impl)]
use std::fs;
>>>>>>> Stashed changes
mod cpu;
mod instructions;
mod ppu;
mod gameboy;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
fn main() {
    let mut gameboy = gameboy::Gameboy::new();
    println!("Starting up!");
    let binary = &fs::read("bootix_dmg.bin").unwrap();
    let mut i = 0;
    for el in binary {
	gameboy.cpu.memory.rom_bank0[i] = el.clone();
	i = i + 1;
    }
    gameboy.run_emulator();
}

fn run_emulator() {
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
    canvas.set_draw_color(Color::RGB(255, 0, 0));
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
    }
}
