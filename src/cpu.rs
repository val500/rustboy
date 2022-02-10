#![allow(non_snake_case)]
use crate::{instructions::{instruction_decode, StagePassThrough, CC}, ppu::{Mode, PPU}, register_maps::{InterruptEnable, InterruptFlag}};
use std::ops::{Index, IndexMut};

pub struct CPU {
    pub reg_file: RegFile,
    pub addr_bus: u16,
    pub memory: Memory,
    pub pc: u16,
    pub ppu: PPU,
    pub ime: bool,
    pub ie: InterruptEnable, // 0xFFFF
    pub mode: Mode,
    pub pass_in: (u8, StagePassThrough),
}

impl CPU {
    pub fn new(ppu: PPU) -> CPU {
        CPU {
            reg_file: RegFile::default(),
            addr_bus: 0,
            memory: Memory::default(),
            pc: 0,
            ppu,
            ime: false,
            ie: 0.into(),
            mode: Mode::Off,
	    pass_in: (0, StagePassThrough::default()),
        }
    }

    pub fn handle_interrupts(&mut self) {
	let enabled: InterruptEnable = self.ie;
	let mut flags: InterruptFlag = self.ppu.io_registers.interrupt_flag;
	self.ime = false;
	if enabled.vblank == 1 && flags.vblank == 1 {
	    self.push_val((self.pc >> 8) as u8);
	    self.push_val(self.pc as u8);
	    self.pc = 0x40;
	    flags.vblank = 0;
	    self.ppu.io_registers.interrupt_flag = flags;
	} else if enabled.lcd_stat == 1 && flags.lcd_stat == 1 {
	    self.push_val((self.pc >> 8) as u8);
	    self.push_val(self.pc as u8);
	    self.pc = 0x48;
	    flags.lcd_stat = 0;
	    self.ppu.io_registers.interrupt_flag = flags;
	} else if enabled.timer == 1 && flags.timer == 1 {
	    self.push_val((self.pc >> 8) as u8);
	    self.push_val(self.pc as u8);
	    self.pc = 0x50;
	    flags.timer = 0;
	    self.ppu.io_registers.interrupt_flag = flags;
	} else if enabled.serial == 1 && flags.serial == 1 {
	    self.push_val((self.pc >> 8) as u8);
	    self.push_val(self.pc as u8);
	    self.pc = 0x58;
	    flags.serial = 0;
	    self.ppu.io_registers.interrupt_flag = flags;
	} else if enabled.joypad == 1 && flags.joypad == 1 {
	    self.push_val((self.pc >> 8) as u8);
	    self.push_val(self.pc as u8);
	    self.pc = 0x60;
	    flags.joypad = 0;
	    self.ppu.io_registers.interrupt_flag = flags;
	} else {
	    self.ime = true;
	}
    }

    pub fn step_cpu(&mut self, _debug: bool) {
        let mut instruction = self.pass_in.0;
	let mut pass = self.pass_in.1;
	self.ppu.io_registers.tick_timer();
	if instruction == 0x76 { // HALT
	    //println!("halt!");
	    if u8::from(self.ppu.io_registers.interrupt_flag) & u8::from(self.ie) != 0 {
		//println!("im here: {:#x}, {:#x}", u8::from(self.ime), u8::from(self.ppu.io_registers.interrupt_flag));
		self.pc = self.pc + 1;
	    } else {
		return;
	    }
	}
	if pass.instruction_stage == 0 {
	    if self.ime {
		self.handle_interrupts();
	    }
            self.addr_bus = self.pc;
            instruction = self.read();
            if pass.ei {
                self.ime = true;
                pass.ei = false;
            }
        }
	pass = instruction_decode(instruction, self, pass);
	if pass.di {
            self.ime = false;
            pass.di = false;
        }
	self.pass_in = (instruction, pass);
    }

    pub fn step(&mut self, debug: bool) {
	self.step_cpu(debug);
	
    }

    pub fn read(&self) -> u8 {
        let addr = self.addr_bus as usize;
        match addr {
	    0x0000..=0x00FF => {
		if self.memory.use_boot {
		    self.memory.boot_rom[addr]
		} else {
		    self.memory.rom_bank0[addr]
		}
	    }
	    0x0100..=0x3FFF => self.memory.rom_bank0[addr],
            0x4000..=0x7FFF => self.memory.rom_bank1[addr - 0x4000],
            0x8000..=0x9FFF => match self.mode {
		Mode::Mode3 => 0xFF,
		_ => self.ppu.vram[addr - 0x8000],
	    },
            0xA000..=0xBFFF => self.memory.external_ram[addr - 0xA000],
            0xC000..=0xDFFF => self.memory.working_ram[addr - 0xC000],
            0xE000..=0xFDFF => self.memory.echo_ram[addr - 0xE000],
            0xFE00..=0xFE9F => match self.mode {
		Mode::Off | Mode::Mode0 | Mode::Mode1 => self.ppu.oam.get(addr),
		_ => 0xFF,
	    }
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF7F => self.ppu.io_registers.get(addr),
            0xFF80..=0xFFFE => self.memory.high_ram[addr - 0xFF80],
            0xFFFF => self.ie.into(),
            _ => 0,
        }
    }
    fn get_data_from_addr(&mut self, addr: u16) -> u8 {
	let addr = addr as usize;
	match addr {
	    0x0000..=0x00FF => {
		if self.memory.use_boot {
		    self.memory.boot_rom[addr]
		} else {
		    self.memory.rom_bank0[addr]
		}
	    }
	    0x0100..=0x3FFF => self.memory.rom_bank0[addr],
            0x4000..=0x7FFF => self.memory.rom_bank1[addr - 0x4000],
            0x8000..=0x9FFF => match self.mode {
		Mode::Mode3 => 0xFF,
		_ => self.ppu.vram[addr - 0x8000],
	    },
            0xA000..=0xBFFF => self.memory.external_ram[addr - 0xA000],
            0xC000..=0xDFFF => self.memory.working_ram[addr - 0xC000],
	    _ => panic!("shouldnt use this function"),
	}
    }
    pub fn write_data(&mut self, data: u8) {
        let addr = self.addr_bus as usize;
        match addr {
            0x0000..=0x3FFF => (),
            0x4000..=0x7FFF => (),
            0x8000..=0x9FFF => match self.mode {
		Mode::Mode3 => (),
		_ => self.ppu.vram[addr - 0x8000] = data,
	    },
            0xA000..=0xBFFF => self.memory.external_ram[addr - 0xA000] = data,
            0xC000..=0xDFFF => self.memory.working_ram[addr - 0xC000] = data,
            0xE000..=0xFDFF => self.memory.echo_ram[addr - 0xE000] = data,
            0xFE00..=0xFE9F => match self.mode {
		Mode::Off | Mode::Mode0 | Mode::Mode1 => self.ppu.oam.set(addr, data),
		_ => (),
	    },
            0xFEA0..=0xFEFF => (),
            0xFF00..=0xFF7F => {
		if addr == 0xFF50 && data != 0 {
		    self.memory.use_boot = false;
		}
		if addr == 0xFF46 {
		    if data <= 0xDF {
			let mut oam_arr: [u8; 0xA0] = self.ppu.oam.into();
			for i in 0..0xA0 {
			    let addr = ((data as u16) << 8) | (i);
			    oam_arr[i as usize] = self.get_data_from_addr(addr);
			}
			self.ppu.oam = oam_arr.into();
		    }
		}
		self.ppu.io_registers.set(addr, data);
	    }
            0xFF80..=0xFFFE => self.memory.high_ram[addr - 0xFF80] = data,
            0xFFFF => self.ie = data.into(),
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
	match push_reg {
	    Reg8::F => self.write_data(self.reg_file.flags.into()),
            _ => self.write_data(self.reg_file[push_reg]),
	}
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

impl From<Flags> for u8 {
    fn from(flags: Flags) -> u8 {
	(flags.Z << 7) | (flags.N << 6) | (flags.H << 5) | (flags.C << 4)
    }
}

impl From<u8> for Flags {
    fn from(val: u8) -> Flags {
	Flags {
	    Z: val >> 7,
	    N: (0x40 & val) >> 6,
	    H: (0x20 & val) >> 5,
	    C: (0x10 & val) >> 4,
	}
    }
}

#[derive(Clone, Copy, Default)]
pub struct RegFile {
    pub A: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,
    pub SP: u16,
    pub flags: Flags,
}

pub struct Memory {
    pub boot_rom: [u8; 0x4000],
    pub rom_bank0: [u8; 0x4000],       // 0x0000 - 0x3FFF
    pub rom_bank1: [u8; 0x4000],       // 0x4000 - 0x7FFF
    external_ram: [u8; 0x2000],   // 0xA000 - 0xBFFF
    working_ram: [u8; 0x2000],     // 0xC000 - 0xDFFF
    echo_ram: [u8; 0x1E00],        // 0xE000 - 0xFDFF
    high_ram: [u8; 0x007F],        // 0xFF80 - 0xFFFE
    use_boot: bool,
}

impl Default for Memory {
    fn default() -> Self {
	Memory {
	    boot_rom: [0; 0x4000],
	    rom_bank0: [0; 0x4000],
	    rom_bank1: [0; 0x4000],
	    external_ram: [0; 0x2000],
	    working_ram: [0; 0x2000],
	    echo_ram: [0; 0x1E00],
	    high_ram: [0; 0x007F],
	    use_boot: true,
	}
    }
}


impl RegFile {
    pub fn set16(&mut self, reg: Reg16, value: u16) {
        match reg {
            Reg16::AF => {
                self.A = (value >> 8) as u8;
                self.flags = (value as u8).into();
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
            Reg16::AF => (self.A as u16) << 8 | (u8::from(self.flags) as u16),
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
            Reg8::H => &self.H,
            Reg8::L => &self.L,
	    Reg8::F => panic!("should not be indexing the F register"),
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
            Reg8::H => &mut self.H,
            Reg8::L => &mut self.L,
	    Reg8::F => panic!("should not be indexing the F register"),
        }
    }
}
