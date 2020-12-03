use crate::instructions::{instruction_decode, StagePassThrough};
use std::ops::{Index, IndexMut};

pub struct CPU {
    reg_file: RegFile,
    memory: Memory,
    pc: u16,
}

impl CPU {
    pub fn execute(&mut self) {
        let mut instruction: u8 = 0x0000;
        let mut instruction_stage = 0;
        let mut pass = StagePassThrough::default();
	pass.next_pc = self.pc + 1;
        loop {
            let tup = instruction_decode(
                instruction,
                &mut self.reg_file,
                &mut self.memory,
                instruction_stage,
                pass,
		self.pc,
            );
	    instruction_stage = tup.0;
	    pass = tup.1;
	    
	    if instruction_stage == 0 {
		self.memory.addr_bus = self.pc;
		instruction = self.memory.read();
	    }
	    self.pc = pass.next_pc;
        }
    }
}

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

pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    //PC,
    SP,
}

#[derive(Clone, Copy, Default)]
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
    boot_rom: [u8; 0x00FF],        // 0x0000 - 0x00FF
    game_rom_bank0: [u8; 0x3FFF],  // 0x0000 - 0x3FFF
    game_rom_bankn: [u8; 0x3FFF],  // 0x4000 - 0x7FFF
    tile_ram: [u8; 0x17FF],        // 0x8000 - 0x97FF
    background_map: [u8; 0x07FF],  // 0x9800 - 0x9FFF
    cartridge_ram: [u8; 0x1FFF],   // 0xA000 - 0xBFFF
    working_ram: [u8; 0x1FFF],     // 0xC000 - 0xDFFF
    echo_ram: [u8; 0x1DFF],        // 0xE000 - 0xFDFF
    oam: [u8; 0x009F],             // 0xFE00 - 0xFE9F
    io_registers: [u8; 0x007F],    // 0xFF00 - 0xFF7F
    high_ram: [u8; 0x007E],        // 0xFF80 - 0xFFFE
    interrupt_enable_register: u8, // 0xFFFF
}

impl Memory {
    pub fn read(&self) -> u8 {
        let addr = self.addr_bus as usize;
        match addr {
            0x0000..=0x3FFF => self.boot_rom[addr],
            0x4000..=0x7FFF => self.game_rom_bank0[addr - 0x4000],
            0x8000..=0x97FF => self.game_rom_bankn[addr - 0x8000],
            0x9800..=0x9FFF => self.tile_ram[addr - 0x9800],
            0xA000..=0xBFFF => self.cartridge_ram[addr - 0xA000],
            0xC000..=0xDFFF => self.working_ram[addr - 0xC000],
            0xE000..=0xFDFF => self.echo_ram[addr - 0xE000],
            0xFE00..=0xFE9F => self.oam[addr - 0xFE00],
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF7F => self.io_registers[addr - 0xFF00],
            0xFF80..=0xFFFE => self.high_ram[addr - 0xFF80],
            0xFFFF => self.interrupt_enable_register,
            _ => 0,
        }
    }
    pub fn write_data(&mut self, data: u8) {
        let addr = self.addr_bus as usize;
        match addr {
            0x0000..=0x3FFF => self.boot_rom[addr] = data,
            0x4000..=0x7FFF => self.game_rom_bank0[addr - 0x4000] = data,
            0x8000..=0x97FF => self.game_rom_bankn[addr - 0x8000] = data,
            0x9800..=0x9FFF => self.tile_ram[addr - 0x9800] = data,
            0xA000..=0xBFFF => self.cartridge_ram[addr - 0xA000] = data,
            0xC000..=0xDFFF => self.working_ram[addr - 0xC000] = data,
            0xE000..=0xFDFF => self.echo_ram[addr - 0xE000] = data,
            0xFE00..=0xFE9F => self.oam[addr - 0xFE00] = data,
            0xFEA0..=0xFEFF => (),
            0xFF00..=0xFF7F => self.io_registers[addr - 0xFF00] = data,
            0xFF80..=0xFFFE => self.high_ram[addr - 0xFF80] = data,
            0xFFFF => self.interrupt_enable_register = data,
            _ => (),
        };
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
