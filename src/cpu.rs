use crate::instructions::{instruction_decode, StagePassThrough, CC};
use crate::ppu::{ppu_execute, GameboyColor, Mode, PPU};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::{
    io,
    ops::{Index, IndexMut},
    sync::{Arc, Barrier, RwLock},
    time::Duration,
};

type Tile = [u16; 8];

pub struct Clock {
    barrier: Arc<Barrier>,
    io_registers: Arc<RwLock<[u8; 0x0080]>>,
    mode: Mode,
}

impl Clock {
    pub fn new(barrier: Arc<Barrier>, io_registers: Arc<RwLock<[u8; 0x0080]>>) -> Clock {
        Clock {
            barrier,
            io_registers,
            mode: Mode::Off,
        }
    }
    pub fn start(&mut self) {
        let mut scanline = 0;
        loop {
            match self.mode {
                Mode::Mode2 => {
                    // Object Search
                    spin_sleep::sleep(Duration::from_micros(19));
                    self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                        Mode::Mode3
                    } else {
                        Mode::Off
                    };
                    self.barrier.wait();
                }
                Mode::Mode3 => {
                    // Drawing
                    spin_sleep::sleep(Duration::from_micros(41));
                    self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                        Mode::Mode0
                    } else {
                        Mode::Off
                    };
                    self.barrier.wait();
                }
                Mode::Mode0 => {
                    // HBlank
                    spin_sleep::sleep(Duration::from_micros(49));
                    if scanline == 145 {
                        self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                            Mode::Mode1
                        } else {
                            Mode::Off
                        };
                    } else {
                        scanline += 1;
                        self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                            Mode::Mode2
                        } else {
                            Mode::Off
                        };
                        self.barrier.wait();
                    }
                }
                Mode::Mode1 => {
                    // VBlank
                    spin_sleep::sleep(Duration::from_micros(1088));
                    scanline = 0;
                    self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                        Mode::Mode2
                    } else {
                        Mode::Off
                    };
                    self.barrier.wait();
                }
                Mode::Off => {
                    spin_sleep::sleep(Duration::from_micros(41));
                    self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                        Mode::Mode2
                    } else {
                        Mode::Off
                    };
                    self.barrier.wait();
                }
            }
        }
    }
}

pub struct CPU {
    pub reg_file: RegFile,
    pub addr_bus: u16,
    pub memory: Memory,
    pc: u16,
    ppu: Arc<RwLock<PPU>>,
    io_registers: Arc<RwLock<[u8; 0x0080]>>,
    ime: bool,
    interrupt_enable_register: u8, // 0xFFFF
    mode: Mode,
}

impl CPU {
    pub fn new(ppu: Arc<RwLock<PPU>>, io_registers: Arc<RwLock<[u8; 0x0080]>>) -> CPU {
        CPU {
            reg_file: RegFile::default(),
            addr_bus: 0,
            memory: Memory::default(),
            pc: 0,
            ppu,
            io_registers,
            ime: true,
            interrupt_enable_register: 0,
            mode: Mode::Off,
        }
    }

    pub fn execute(&mut self, barrier: Arc<Barrier>, debug: bool) {
        let mut clock = 0;
        self.addr_bus = self.pc;
        let mut instruction = self.read();
        let mut pass = StagePassThrough::default();
        let mut ei = false;
        let mut di = false;
        let mut break_loop;
        let mut continue_loop = false;
        let mut break_int = 0;
        let mut mode_count = 0;
        let mut scanline = 0;
        loop {
            break_loop = self.pc == break_int || !continue_loop;
            if !debug {
                match self.mode {
                    Mode::Mode2 => {
                        if mode_count == 80 {
                            // Object Search
                            mode_count = 0;
                            self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                                Mode::Mode3
                            } else {
                                Mode::Off
                            };
                            barrier.wait();
                        }
                    }
                    Mode::Mode3 => {
                        if mode_count == 172 {
                            // Drawing
                            mode_count = 0;
                            self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                                Mode::Mode0
                            } else {
                                Mode::Off
                            };
                            barrier.wait();
                        }
                    }
                    Mode::Mode0 => {
                        if mode_count == 204 {
                            // HBlank
                            mode_count = 0;
                            if scanline == 145 {
                                self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                                    Mode::Mode1
                                } else {
                                    Mode::Off
                                };
                            } else {
                                scanline += 1;
                                self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                                    Mode::Mode2
                                } else {
                                    Mode::Off
                                };
                            }
                            barrier.wait();
                        }
                    }
                    Mode::Mode1 => {
                        if mode_count == 4560 {
                            // VBlank
                            mode_count = 0;
                            scanline = 0;
                            self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                                Mode::Mode2
                            } else {
                                Mode::Off
                            };
                            barrier.wait();
                        }
                    }
                    Mode::Off => {
                        if mode_count == 80 {
                            mode_count = 0;
                            self.mode = if self.io_registers.read().unwrap()[0x40] >> 7 == 1 {
                                Mode::Mode2
                            } else {
                                Mode::Off
                            };
                            barrier.wait();
                        }
                    }
                }
            } else if break_loop {
                continue_loop = false;
                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line");
                let input_list: Vec<&str> = user_input.split(" ").collect();
                if input_list.len() > 0 && input_list[0] == "br" {
                    let mut num = input_list[1].to_string();
                    num.pop();
                    break_int = num.parse::<u16>().unwrap();
                    continue_loop = true;
                }
                println!(
                    "Clock: {}\nInstruction: {:#x}\nRegs: {:?}\nPass: {:?}",
                    clock, instruction, self.reg_file, pass
                );
            }

            pass = instruction_decode(instruction, self, pass);
            self.pc = pass.next_pc;
            if pass.instruction_stage == 0 {
                self.addr_bus = self.pc;
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
            clock = clock + 1;

            mode_count += 1;
        }
    }

    fn update_canvas(canvas: &mut Canvas<Window>, color_row: Vec<GameboyColor>, scanline: u8) {
        canvas.set_draw_color(GameboyColor::White);
        canvas.clear();
        for x in 0..160 {
            canvas.set_draw_color(color_row[x]);
            canvas.draw_point((x as i32, scanline as i32)).unwrap();
        }
        canvas.present();
    }

    pub fn read(&self) -> u8 {
        let addr = self.addr_bus as usize;
        match addr {
            0x0000..=0x3FFF => self.memory.rom_bank0[addr],
            0x4000..=0x7FFF => self.memory.rom_bank1[addr - 0x4000],
            0x8000..=0x9FFF => self
                .ppu
                .try_read()
                .map_or_else(|_| 0xFF, |ppu| ppu.vram[addr - 0x8000]),
            0xA000..=0xBFFF => self.memory.cartridge_ram[addr - 0xA000],
            0xC000..=0xDFFF => self.memory.working_ram[addr - 0xC000],
            0xE000..=0xFDFF => self.memory.echo_ram[addr - 0xE000],
            0xFE00..=0xFE9F => self
                .ppu
                .try_read()
                .map_or_else(|_| 0xFF, |ppu| ppu.oam[addr - 0xFE00]),
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF7F => self
                .io_registers
                .try_read()
                .map_or_else(|_| 0xFF, |arr| arr[addr - 0xFF00]),
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
            0x8000..=0x9FFF => {
                let ppu_result = self.ppu.try_write();
                match ppu_result {
                    Ok(mut ppu) => ppu.vram[addr - 0x8000] = data,
                    _ => (),
                };
            }
            0xA000..=0xBFFF => self.memory.cartridge_ram[addr - 0xA000] = data,
            0xC000..=0xDFFF => self.memory.working_ram[addr - 0xC000] = data,
            0xE000..=0xFDFF => self.memory.echo_ram[addr - 0xE000] = data,
            0xFE00..=0xFE9F => {
                let ppu_result = self.ppu.try_write();
                match ppu_result {
                    Ok(mut ppu) => ppu.oam[addr - 0xFE00] = data,
                    _ => (),
                };
            }
            0xFEA0..=0xFEFF => (),
            0xFF00..=0xFF7F => {
                let ppu_result = self.io_registers.try_write();
                match ppu_result {
                    Ok(mut io_regs) => io_regs[addr - 0xFF00] = data,
                    _ => (),
                };
            }
            0xFF80..=0xFFFE => self.memory.high_ram[addr - 0xFF80] = data,
            0xFFFF => self.interrupt_enable_register = data,
            _ => (),
        };
    }
    pub fn pop(&mut self) -> u8 {
        let mut reg_file = self.reg_file;
        self.addr_bus = reg_file.SP;
        let stack_val = self.read();
        reg_file.SP = reg_file.SP + 1;
        stack_val
    }

    pub fn push(&mut self, push_reg: Reg8) {
        let mut reg_file = self.reg_file;
        reg_file.SP = reg_file.SP - 1;
        self.addr_bus = reg_file.SP;
        self.write_data(reg_file[push_reg]);
    }

    pub fn push_val(&mut self, val: u8) {
        let mut reg_file = self.reg_file;
        reg_file.SP = reg_file.SP - 1;
        self.addr_bus = reg_file.SP;
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

#[derive(Clone, Copy, Default, Debug)]
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
    pub rom_bank0: [u8; 0x4000], // 0x0000 - 0x3FFF
    rom_bank1: [u8; 0x4000],     // 0x0000 - 0x3FFF
    cartridge_ram: [u8; 0x2000], // 0xA000 - 0xBFFF
    working_ram: [u8; 0x2000],   // 0xC000 - 0xDFFF
    echo_ram: [u8; 0x1E00],      // 0xE000 - 0xFDFF
    high_ram: [u8; 0x007F],      // 0xFF80 - 0xFFFE
}

impl Default for Memory {
    fn default() -> Memory {
        Memory {
            rom_bank0: [0; 0x4000],     // 0x0000 - 0x00FF
            rom_bank1: [0; 0x4000],     // 0x0000 - 0x3FFF
            cartridge_ram: [0; 0x2000], // 0xA000 - 0xBFFF
            working_ram: [0; 0x2000],   // 0xC000 - 0xDFFF
            echo_ram: [0; 0x1E00],      // 0xE000 - 0xFDFF
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
