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
const SCALE: u32 = 4;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Gameboy Window", X_DIM * SCALE, Y_DIM * SCALE)
        .opengl()
        .build()
        .unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let mut gameboy = gameboy::Gameboy::new();
    gameboy.init_gameboy();
    gameboy.run_emulator(&sdl_context, &texture_creator, &mut canvas);
}
