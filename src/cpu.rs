use crate::instructions::{instruction_decode, StagePassThrough, CC};
use crate::ppu::{PPU, ppu_execute};
use std::ops::{Index, IndexMut};
use std::sync::mpsc::channel;
use std::thread;
use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

type Tile = [u16; 8];

pub struct CPU {
    pub reg_file: RegFile,
    pub addr_bus: u16,
    memory: Memory,
    pc: u16,
    ppu: Arc<RwLock<PPU>>,
    io_registers: [u8; 0x007F],    // 0xFF00 - 0xFF7F
    ime: bool,
    interrupt_enable_register: u8, // 0xFFFF
}

impl CPU {
    pub fn execute(&mut self) {
        let mut instruction: u8 = 0x0000;
        let mut pass = StagePassThrough::default();
        pass.next_pc = self.pc + 1;
        let (clock_tx, clock_rx) = channel();
	let (clock_tx_2, clock_rx_2) = channel();
	let mut ei = false;
	let mut di = false;
        // Tick every 953 nanoseconds (every 4 machine cycles)
        thread::spawn(move || loop {
            thread::sleep(Duration::from_nanos(953));
            clock_tx.send(0).unwrap();
	    clock_tx_2.send(0).unwrap();
        });
	
	let ppu_clone = self.ppu.clone();
	thread::spawn(move || {
	    ppu_execute(ppu_clone, clock_rx_2);
	});
	
        loop {
            clock_rx.recv();
            let pass = instruction_decode(instruction, self, pass);
            if pass.instruction_stage == 0 {
                self.memory.addr_bus = self.pc;
                instruction = self.read();
		if ei {
		    self.ime = true;
		    ei = false;
		} else if di {
		    self.ime = false;
		    di = false;
		}
            }
	    ei = pass.ei;
	    di = pass.di;
            self.pc = pass.next_pc;
        }
    }

    pub fn read(&self) -> u8 {
        let addr = self.addr_bus as usize;
        match addr {
            0x0000..=0x3FFF => self.memory.rom_bank0[addr],
            0x4000..=0x7FFF => self.memory.rom_bank1[addr - 0x4000],
            0x8000..=0x9FFF => self.ppu.try_read().map_or_else(|_| 0xFF, |ppu| ppu.vram[addr - 0x8000]),
            0xA000..=0xBFFF => self.memory.cartridge_ram[addr - 0xA000],
            0xC000..=0xDFFF => self.memory.working_ram[addr - 0xC000],
            0xE000..=0xFDFF => self.memory.echo_ram[addr - 0xE000],
            0xFE00..=0xFE9F => self.ppu.try_read().map_or_else(|_| 0xFF, |ppu| ppu.oam[addr - 0xFE00]),
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF7F => self.io_registers[addr - 0xFF00],
            0xFF80..=0xFFFE => self.memory.high_ram[addr - 0xFF80],
            0xFFFF => self.interrupt_enable_register,
            _ => 0,
        }
    }
    pub fn write_data(&mut self, data: u8) {
        let addr = self.memory.addr_bus as usize;
        match addr {
            0x0000..=0x3FFF => self.memory.rom_bank0[addr] = data,
            0x4000..=0x7FFF => self.memory.rom_bank1[addr - 0x4000] = data,
            0x8000..=0x9FFF => {
		let ppu_result = self.ppu.try_write();
		match ppu_result {
		    Ok(mut ppu) => ppu.vram[addr - 0x8000] = data,
		    _ => ()
		};
	    },
            0xA000..=0xBFFF => self.memory.cartridge_ram[addr - 0xA000] = data,
            0xC000..=0xDFFF => self.memory.working_ram[addr - 0xC000] = data,
            0xE000..=0xFDFF => self.memory.echo_ram[addr - 0xE000] = data,
            0xFE00..=0xFE9F => {
		let ppu_result = self.ppu.try_write();
		match ppu_result {
		    Ok(mut ppu) => ppu.oam[addr - 0xFE00] = data,
		    _ => (),
		};
	    },
            0xFEA0..=0xFEFF => (),
            0xFF00..=0xFF7F => self.io_registers[addr - 0xFF00] = data,
            0xFF80..=0xFFFE => self.memory.high_ram[addr - 0xFF80] = data,
            0xFFFF => self.interrupt_enable_register = data,
            _ => (),
        };
    }
    pub fn pop(&mut self) -> u8 {
	let mut reg_file = self.reg_file;
        self.memory.addr_bus = reg_file.SP;
        let stack_val = self.read();
        reg_file.SP = reg_file.SP + 1;
        stack_val
    }

    pub fn push(&mut self, push_reg: Reg8) {
	let mut reg_file = self.reg_file;
        reg_file.SP = reg_file.SP - 1;
        self.memory.addr_bus = reg_file.SP;
        self.write_data(reg_file[push_reg]);
    }

    pub fn push_val(&mut self, val: u8) {
	let mut reg_file = self.reg_file;
        reg_file.SP = reg_file.SP - 1;
        self.memory.addr_bus = reg_file.SP;
        self.write_data(val);
    }
}



#[derive(Copy, Clone)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    //PC,
    SP,
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Flags {
    pub Z: u8,
    pub N: u8,
    pub H: u8,
    pub C: u8,
}

#[derive(Clone, Copy)]
pub struct RegFile {
    pub A: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub F: u8,
    pub H: u8,
    pub L: u8,
    pub SP: u16,
    // pub PC: u16,
    pub flags: Flags,
}

pub struct Memory {
    pub addr_bus: u16,
    rom_bank0: [u8; 0x00FF],       // 0x0000 - 0x00FF
    rom_bank1: [u8; 0x3FFF],       // 0x0000 - 0x3FFF
    cartridge_ram: [u8; 0x1FFF],   // 0xA000 - 0xBFFF
    working_ram: [u8; 0x1FFF],     // 0xC000 - 0xDFFF
    echo_ram: [u8; 0x1DFF],        // 0xE000 - 0xFDFF
    high_ram: [u8; 0x007E],        // 0xFF80 - 0xFFFE
    
}


impl RegFile {
    pub fn set16(&mut self, reg: Reg16, value: u16) {
        match reg {
            Reg16::AF => {
                self.A = (value >> 8) as u8;
                self.F = value as u8;
            }
            Reg16::BC => {
                self.B = (value >> 8) as u8;
                self.C = value as u8;
            }
            Reg16::DE => {
                self.D = (value >> 8) as u8;
                self.E = value as u8;
            }
            Reg16::HL => {
                self.H = (value >> 8) as u8;
                self.L = value as u8;
            }
            Reg16::SP => {
                self.SP = value;
            }
        }
    }
    pub fn get16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => (self.A as u16) << 8 | (self.F as u16),
            Reg16::BC => (self.B as u16) << 8 | (self.C as u16),
            Reg16::DE => (self.D as u16) << 8 | (self.E as u16),
            Reg16::HL => (self.H as u16) << 8 | (self.L as u16),
            Reg16::SP => self.SP,
        }
    }

    pub fn check_condition(&self, cc: CC) -> bool {
        match cc {
            CC::NZ => self.flags.Z == 0,
            CC::Z => self.flags.Z == 1,
            CC::C => self.flags.C == 1,
            CC::NC => self.flags.C == 0,
            CC::UC => true,
        }
    }
}

impl Index<Reg8> for RegFile {
    type Output = u8;
    fn index(&self, reg: Reg8) -> &u8 {
        match reg {
            Reg8::A => &self.A,
            Reg8::B => &self.B,
            Reg8::C => &self.C,
            Reg8::D => &self.D,
            Reg8::E => &self.E,
            Reg8::F => &self.F,
            Reg8::H => &self.H,
            Reg8::L => &self.L,
        }
    }
}

impl IndexMut<Reg8> for RegFile {
    fn index_mut(&mut self, reg: Reg8) -> &mut u8 {
        match reg {
            Reg8::A => &mut self.A,
            Reg8::B => &mut self.B,
            Reg8::C => &mut self.C,
            Reg8::D => &mut self.D,
            Reg8::E => &mut self.E,
            Reg8::F => &mut self.F,
            Reg8::H => &mut self.H,
            Reg8::L => &mut self.L,
        }
    }
}
