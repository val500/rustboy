#![allow(dead_code)]
#![allow(non_snake_case)]
#![feature(wrapping_int_impl)]
use std::fs;
mod cpu;
mod gameboy;
mod instructions;
mod ppu;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};

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

#[cfg(test)]
mod test {
    use instructions::StagePassThrough;

    use super::*;

    #[test]
    fn test_cpu() {
	let mut gameboy = gameboy::Gameboy::new();
	let binary = &fs::read("bootix_dmg.bin").unwrap();
	println!("{}", binary.len());
	let mut i = 0;
	for el in binary {
            gameboy.cpu.memory.rom_bank0[i] = el.clone();
            i = i + 1;
	}
	let mut passed = gameboy.cpu.execute_n_cycles(false, 10, (0, StagePassThrough::default()));
	println!("{:x?}", passed);
	while passed.1.next_pc <= 0xc {
	    passed = gameboy.cpu.execute_n_cycles(false, 100, passed);
	}
	println!("{:x?}", passed);
    }
}
