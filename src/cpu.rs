use crate::instructions::{instruction_decode, StagePassThrough, CC};
use std::ops::{Index, IndexMut};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub struct CPU {
    reg_file: RegFile,
    memory: Memory,
    pc: u16,
}

impl CPU {
    pub fn execute(&mut self) {
        let mut instruction: u8 = 0x0000;
        let mut pass = StagePassThrough::default();
        pass.next_pc = self.pc + 1;
        let (clock_tx, clock_rx) = channel();
        // Tick every 953 nanoseconds (every 4 machine cycles)
        thread::spawn(move || loop {
            thread::sleep(Duration::from_nanos(953));
            clock_tx.send(0).unwrap();
        });
        loop {
            clock_rx.recv();
            let pass = instruction_decode(instruction, &mut self.reg_file, &mut self.memory, pass);
            if pass.instruction_stage == 0 {
                self.memory.addr_bus = self.pc;
                instruction = self.memory.read();
            }
            self.pc = pass.next_pc;
        }
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
    vram: [u8; 0x1FFF],            // 0x8000 - 0x9FFF
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
            0x0000..=0x3FFF => self.rom_bank0[addr],
            0x4000..=0x7FFF => self.rom_bank1[addr - 0x4000],
            0x8000..=0x9FFF => self.vram[addr - 0x8000],
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
            0x0000..=0x3FFF => self.rom_bank0[addr] = data,
            0x4000..=0x7FFF => self.rom_bank1[addr - 0x4000] = data,
            0x8000..=0x9FFF => self.vram[addr - 0x8000] = data,
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
    pub fn pop(&mut self, reg_file: &mut RegFile) -> u8 {
        self.addr_bus = reg_file.SP;
        let stack_val = self.read();
        reg_file.SP = reg_file.SP + 1;
        stack_val
    }

    pub fn push(&mut self, reg_file: &mut RegFile, push_reg: Reg8) {
        reg_file.SP = reg_file.SP - 1;
        self.addr_bus = reg_file.SP;
        self.write_data(reg_file[push_reg]);
    }

    pub fn push_val(&mut self, reg_file: &mut RegFile, val: u8) {
        reg_file.SP = reg_file.SP - 1;
        self.addr_bus = reg_file.SP;
        self.write_data(val);
    }

    pub fn read_lcdc(&self) -> u8 {
        self.io_registers[0xFF41 - 0xFF00]
    }

    pub fn get_tile_bg(&self, tile_index: u8) -> u16 {
        let lcdc = self.read_lcdc();
        let offset: u16 = if lcdc & 8 == 0 { 0x1800 } else { 0x1C00 };
        let base = (tile_index as u16 + offset) as usize;
	(self.vram[base] << 8) as u16 | (self.vram[base + 1]) as u16
    }

    pub fn get_tile_window(&self, tile_index: u8) -> u16 {
        let lcdc = self.read_lcdc();
        let offset: u16 = if lcdc & 0x40 == 0 { 0x1800 } else { 0x1C00 };
        let base = (tile_index as u16 + offset) as usize;
        (self.vram[base] << 8) as u16 | (self.vram[base + 1]) as u16
    }

    fn get_tile_8000(&self, tile_index: u8) -> u16 {
	let base = (tile_index * 2) as usize;
	((self.vram[base] << 8) as u16) | (self.vram[base + 1] as u16)
    }

    fn get_tile_8800(&self, tile_index: i8) -> u16 {
	let base = (0x1000 + (tile_index * 2) as i16) as usize;
	((self.vram[base] << 8) as u16) | (self.vram[base + 1] as u16)
    }

    pub fn get_tile(&self, tile_index: u8) -> u16 {
	let lcdc = self.read_lcdc();
        if lcdc & 0x10 == 0 {
	    self.get_tile_8800(tile_index as i8)
	} else {
	    self.get_tile_8000(tile_index)
	}
    }
    
    pub fn get_background(&self) -> Vec<u16> {
	let scroll_y = self.io_registers[0x42];
	let scroll_x = self.io_registers[0x43];
	let window_y = self.io_registers[0x4A];
	let window_x = self.io_registers[0x4B];

	let lcdc = self.read_lcdc();
        let offset: u16 = if lcdc & 8 == 0 { 0x1800 } else { 0x1C00 };
	let start_index = ((scroll_y * 20) + scroll_x) * 2;
	let mut background: Vec<u16> = Vec::with_capacity(360);
	for y in 0..18 {
	    for x in 0..20 {
		let index = start_index as u16 + offset + (y * 20 * 2) + (x * 2) as u16;
		let tile_index = self.vram[index as usize];
		background.push(self.get_tile(tile_index));
	    };
	};
	background
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
