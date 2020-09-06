use crate::{Memory, Reg16, Reg8, RegFile};

// 1 cycle 8 bit instructions
pub fn instruction81(opcode: u8, reg_file: &mut RegFile, memory: &mut Memory) {
    match opcode {
        0x40 => load8(reg_file, Reg8::B, reg_file.B),
        0x41 => load8(reg_file, Reg8::B, reg_file.C),
        0x42 => load8(reg_file, Reg8::B, reg_file.D),
        0x43 => load8(reg_file, Reg8::B, reg_file.E),
        0x44 => load8(reg_file, Reg8::B, reg_file.H),
        0x45 => load8(reg_file, Reg8::B, reg_file.L),
        0x47 => load8(reg_file, Reg8::B, reg_file.A),

        0x48 => load8(reg_file, Reg8::C, reg_file.B),
        0x49 => load8(reg_file, Reg8::C, reg_file.C),
        0x4A => load8(reg_file, Reg8::C, reg_file.D),
        0x4B => load8(reg_file, Reg8::C, reg_file.E),
        0x4C => load8(reg_file, Reg8::C, reg_file.H),
        0x4D => load8(reg_file, Reg8::C, reg_file.L),
        0x4F => load8(reg_file, Reg8::C, reg_file.A),

        0x50 => load8(reg_file, Reg8::D, reg_file.B),
        0x51 => load8(reg_file, Reg8::D, reg_file.C),
        0x52 => load8(reg_file, Reg8::D, reg_file.D),
        0x53 => load8(reg_file, Reg8::D, reg_file.E),
        0x54 => load8(reg_file, Reg8::D, reg_file.H),
        0x55 => load8(reg_file, Reg8::D, reg_file.L),
        0x57 => load8(reg_file, Reg8::D, reg_file.A),

        0x58 => load8(reg_file, Reg8::E, reg_file.B),
        0x59 => load8(reg_file, Reg8::E, reg_file.C),
        0x5A => load8(reg_file, Reg8::E, reg_file.D),
        0x5B => load8(reg_file, Reg8::E, reg_file.E),
        0x5C => load8(reg_file, Reg8::E, reg_file.H),
        0x5D => load8(reg_file, Reg8::E, reg_file.L),
        0x5F => load8(reg_file, Reg8::E, reg_file.A),

        0x60 => load8(reg_file, Reg8::H, reg_file.B),
        0x61 => load8(reg_file, Reg8::H, reg_file.C),
        0x62 => load8(reg_file, Reg8::H, reg_file.D),
        0x63 => load8(reg_file, Reg8::H, reg_file.E),
        0x64 => load8(reg_file, Reg8::H, reg_file.H),
        0x65 => load8(reg_file, Reg8::H, reg_file.L),
        0x67 => load8(reg_file, Reg8::H, reg_file.A),

        0x68 => load8(reg_file, Reg8::L, reg_file.B),
        0x69 => load8(reg_file, Reg8::L, reg_file.C),
        0x6A => load8(reg_file, Reg8::L, reg_file.D),
        0x6B => load8(reg_file, Reg8::L, reg_file.E),
        0x6C => load8(reg_file, Reg8::L, reg_file.H),
        0x6D => load8(reg_file, Reg8::L, reg_file.L),
        0x6F => load8(reg_file, Reg8::L, reg_file.A),

        0x76 => (), //HAlT

        0x78 => load8(reg_file, Reg8::A, reg_file.B),
        0x79 => load8(reg_file, Reg8::A, reg_file.C),
        0x7A => load8(reg_file, Reg8::A, reg_file.D),
        0x7B => load8(reg_file, Reg8::A, reg_file.E),
        0x7C => load8(reg_file, Reg8::A, reg_file.H),
        0x7D => load8(reg_file, Reg8::A, reg_file.L),
        0x7F => load8(reg_file, Reg8::A, reg_file.A),

        0xE9 => load16(reg_file, Reg16::PC, reg_file.get16(Reg16::HL)),
        _ => panic!("Not an 8bit instruction!"),
    }
}

// Two cycle 8 bit instructions
pub fn instruction82(opcode: u8, reg_file: &mut RegFile, memory: &mut Memory) {
    match opcode {
        0x02 => memory[reg_file.get16(Reg16::BC)] = reg_file.A,
        0x12 => memory[reg_file.get16(Reg16::DE)] = reg_file.A,
        0x22 => {
            memory[reg_file.get16(Reg16::HL)] = reg_file.A;
            reg_file.set16(Reg16::HL, reg_file.get16(Reg16::HL) + 1);
        }
        0x32 => {
            memory[reg_file.get16(Reg16::HL)] = reg_file.A;
            reg_file.set16(Reg16::HL, reg_file.get16(Reg16::HL) - 1);
        }
        0x0A => load8(reg_file, Reg8::A, memory[reg_file.get16(Reg16::BC)]),
        0x1A => load8(reg_file, Reg8::A, memory[reg_file.get16(Reg16::DE)]),
        0x2A => {
            load8(reg_file, Reg8::A, memory[reg_file.get16(Reg16::HL)]);
            reg_file.set16(Reg16::HL, reg_file.get16(Reg16::HL) + 1);
        }
        0x3A => {
            load8(reg_file, Reg8::A, memory[reg_file.get16(Reg16::HL)]);
            reg_file.set16(Reg16::HL, reg_file.get16(Reg16::HL) - 1);
        }
        0x46 => load8(reg_file, Reg8::B, memory[reg_file.get16(Reg16::HL)]),
        0x4E => load8(reg_file, Reg8::C, memory[reg_file.get16(Reg16::HL)]),
        0x56 => load8(reg_file, Reg8::D, memory[reg_file.get16(Reg16::HL)]),
        0x5E => load8(reg_file, Reg8::E, memory[reg_file.get16(Reg16::HL)]),
        0x66 => load8(reg_file, Reg8::H, memory[reg_file.get16(Reg16::HL)]),
        0x6E => load8(reg_file, Reg8::L, memory[reg_file.get16(Reg16::HL)]),
        0x7E => load8(reg_file, Reg8::A, memory[reg_file.get16(Reg16::HL)]),
        0x70 => memory[reg_file.get16(Reg16::HL)] = reg_file.B,
        0x71 => memory[reg_file.get16(Reg16::HL)] = reg_file.C,
        0x72 => memory[reg_file.get16(Reg16::HL)] = reg_file.D,
        0x73 => memory[reg_file.get16(Reg16::HL)] = reg_file.E,
        0x74 => memory[reg_file.get16(Reg16::HL)] = reg_file.H,
        0x75 => memory[reg_file.get16(Reg16::HL)] = reg_file.L,
        0x77 => memory[reg_file.get16(Reg16::HL)] = reg_file.A,
        0xE2 => memory[(0xFF as u16) << 8 | reg_file[Reg8::C] as u16] = reg_file.A,
        0xF2 => reg_file[Reg8::A] = memory[(0xFF as u16) << 8 | reg_file[Reg8::C] as u16],
        _ => panic!("Not an 8bit instruction with 2 cycles"),
    }
}

fn load8(reg_file: &mut RegFile, reg: Reg8, value: u8) {
    reg_file[reg] = value;
}

fn load16(reg_file: &mut RegFile, reg: Reg16, value: u16) {
    let hi = (value >> 8) as u8;
    let lo = value as u8;
    match reg {
        Reg16::AF => {
            reg_file.A = hi;
            reg_file.F = lo;
        }
        Reg16::BC => {
            reg_file.B = hi;
            reg_file.C = lo;
        }
        Reg16::DE => {
            reg_file.D = hi;
            reg_file.E = lo;
        }
        Reg16::HL => {
            reg_file.H = hi;
            reg_file.L = lo;
        }
        Reg16::PC => reg_file.PC = value,
        Reg16::SP => reg_file.SP = value,
    }
}
