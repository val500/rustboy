#![allow(dead_code)]
#![feature(or_patterns)]
#![feature(unsigned_abs)]
#![allow(non_snake_case)]
#![feature(wrapping_int_impl)]
use std::fs;
use std::{
    thread,
    sync::{Arc,  RwLock, Barrier},
    time::Duration,
};
use std::sync::mpsc::channel;

mod cpu;
mod instructions;
mod ppu;
fn main() {
    let pattern = std::env::args().nth(1);
    
    match pattern {
	None => {
	    println!("Starting boot rom!");
	    let binary = &fs::read("bootix_dmg.bin").unwrap();
	    let mut io_regs = Arc::new(RwLock::new([0; 0x0080]));
	    let mut ppu = Arc::new(RwLock::new(ppu::PPU::new(io_regs.clone())));
	    let mut cpu = cpu::CPU::new(ppu.clone(), io_regs.clone());
	    let mut i = 0;
	    println!("{}", binary.len());
	    for el in binary {
		cpu.memory.rom_bank0[i] = el.clone();
		i = i + 1;
	    }
	    let barrier = Arc::new(Barrier::new(3));
	    let b1 = barrier.clone();
	    let mut clock = cpu::Clock::new(barrier.clone(), io_regs.clone());
	    thread::spawn(move || {
		clock.start();
	    });
	    thread::spawn(move || {
		cpu.execute(b1, false);
	    });
	    ppu::ppu_execute(ppu, barrier.clone());
	}
	Some(flag) => {
	    if flag == "-d" {
		let fname = std::env::args().nth(2).expect("No binary!");
		let binary = &fs::read(fname).unwrap();
		let mut io_regs = Arc::new(RwLock::new([0; 0x0080]));
		let mut ppu = Arc::new(RwLock::new(ppu::PPU::new(io_regs.clone())));
		let mut cpu = cpu::CPU::new(ppu.clone(), io_regs.clone());
		let mut i = 0;
		for el in binary {
		    cpu.memory.rom_bank0[i] = el.clone();
		    i = i + 1;
		}

		let (clock_tx, clock_rx) = channel();
		thread::spawn(move || loop {
		    thread::sleep(Duration::from_nanos(953));
		    clock_tx.send(0).unwrap();
		});
		let barrier = Arc::new(Barrier::new(3));
		cpu.execute(barrier, true);
	    }

	}
    }
}
