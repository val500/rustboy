use crate::{instructions::{instruction_decode, StagePassThrough, CC}, ppu::{Mode, PPU}};
use std::ops::{Index, IndexMut};

pub struct CPU {
    pub reg_file: RegFile,
    pub addr_bus: u16,
    pub memory: Memory,
    pub pc: u16,
    pub ppu: PPU,
    ime: bool,
    interrupt_enable_register: u8, // 0xFFFF
    pub mode: Mode,
}

impl CPU {
    pub fn new(ppu: PPU) -> CPU {
        CPU {
            reg_file: RegFile::default(),
            addr_bus: 0,
            memory: Memory::default(),
            pc: 0,
            ppu,
            ime: true,
            interrupt_enable_register: 0,
            mode: Mode::Off,
        }
    }

    pub fn execute_n_cycles(&mut self, debug: bool, n: u16, pass_in: (u8, StagePassThrough)) -> (u8, StagePassThrough) {
	self.addr_bus = self.pc;
        let mut instruction = pass_in.0;
	let mut pass = pass_in.1;
	for _ in 0..n {
	    if pass.instruction_stage == 0 {
                self.addr_bus = self.pc;
                instruction = self.read();
                if pass.ei {
                    self.ime = true;
                    pass.ei = false;
                } else if pass.di {
                    self.ime = false;
                    pass.di = false;
                }
            }
	    println!("inst: 0x{:x}", instruction);
	    pass = instruction_decode(instruction, self, pass);
	    self.pc = pass.next_pc;
	    println!("pc: 0x{:x}", self.pc);
	    if self.mode == Mode::Off && self.ppu.io_registers[0x40] >> 7 == 1 {
		return (instruction, pass)
	    }
	}
	(instruction, pass)
    }
    pub fn read(&self) -> u8 {
        let addr = self.addr_bus as usize;
        match addr {
            0x0000..=0x3FFF => self.memory.rom_bank0[addr],
            0x4000..=0x7FFF => self.memory.rom_bank1[addr - 0x4000],
            0x8000..=0x9FFF => match self.mode {
		Mode::Mode3 => 0xFF,
		_ => self.ppu.vram[addr - 0x8000],
	    },
            0xA000..=0xBFFF => self.memory.external_ram[addr - 0xA000],
            0xC000..=0xDFFF => self.memory.working_ram[addr - 0xC000],
            0xE000..=0xFDFF => self.memory.echo_ram[addr - 0xE000],
            0xFE00..=0xFE9F => match self.mode {
		Mode::Off | Mode::Mode0 | Mode::Mode1 => self.ppu.oam[addr - 0xFE00],
		_ => 0xFF,
	    }
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF7F => self.ppu.io_registers[addr - 0xFF00],
            0xFF80..=0xFFFE => self.memory.high_ram[addr - 0xFF80],
            0xFFFF => self.interrupt_enable_register,
            _ => 0,
        }
    }
    pub fn write_data(&mut self, data: u8) {
        let addr = self.addr_bus as usize;
        match addr {
            0x0000..=0x3FFF => self.memory.rom_bank0[addr] = data,
            0x4000..=0x7FFF => self.memory.rom_bank1[addr - 0x4000] = data,
            0x8000..=0x9FFF => match self.mode {
		Mode::Mode3 => (),
		_ => self.ppu.vram[addr - 0x8000] = data,
	    },
            0xA000..=0xBFFF => self.memory.external_ram[addr - 0xA000] = data,
            0xC000..=0xDFFF => self.memory.working_ram[addr - 0xC000] = data,
            0xE000..=0xFDFF => self.memory.echo_ram[addr - 0xE000] = data,
            0xFE00..=0xFE9F => match self.mode {
		Mode::Off | Mode::Mode0 | Mode::Mode1 => self.ppu.oam[addr - 0xFE00] = data,
		_ => (),
	    },
            0xFEA0..=0xFEFF => (),
            0xFF00..=0xFF7F => self.ppu.io_registers[addr - 0xFF00] = data,
            0xFF80..=0xFFFE => self.memory.high_ram[addr - 0xFF80] = data,
            0xFFFF => self.interrupt_enable_register = data,
            _ => (),
        };
    }
    pub fn pop(&mut self) -> u8 {
        self.addr_bus = self.reg_file.SP;
        let stack_val = self.read();
        self.reg_file.SP = self.reg_file.SP + 1;
        stack_val
    }

    pub fn push(&mut self, push_reg: Reg8) {
        self.reg_file.SP = self.reg_file.SP - 1;
        self.addr_bus = self.reg_file.SP;
        self.write_data(self.reg_file[push_reg]);
    }

    pub fn push_val(&mut self, val: u8) {
        self.reg_file.SP = self.reg_file.SP - 1;
        self.addr_bus = self.reg_file.SP;
        self.write_data(val);
    }
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Clone, Copy, Default)]
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
    pub rom_bank0: [u8; 0x4000],       // 0x0000 - 0x3FFF
    rom_bank1: [u8; 0x4000],       // 0x4000 - 0x7FFF
    external_ram: [u8; 0x2000],   // 0xA000 - 0xBFFF
    working_ram: [u8; 0x2000],     // 0xC000 - 0xDFFF
    echo_ram: [u8; 0x1E00],        // 0xE000 - 0xFDFF
    high_ram: [u8; 0x007F],        // 0xFF80 - 0xFFFE
}

impl Default for Memory {
    fn default() -> Self {
	Memory {
	    rom_bank0: [0; 0x4000],
	    rom_bank1: [0; 0x4000],
	    external_ram: [0; 0x2000],
	    working_ram: [0; 0x2000],
	    echo_ram: [0; 0x1E00],
	    high_ram: [0; 0x007F],
	}
    }
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
