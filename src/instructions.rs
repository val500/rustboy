use crate::cpu::{Flags, Memory, Reg16, Reg8, RegFile};

pub fn instruction_decode(
    opcode: u8,
    reg_file: &mut RegFile,
    mem: &mut Memory,
    instruction_stage: u8,
    passed: StagePassThrough,
    pc: u16,
) -> (u8, StagePassThrough) {
    let mut new_instruction_stage = instruction_stage;
    let mut pass_to_next_stage = StagePassThrough::default();
    pass_to_next_stage.next_pc = pc + 1;
    match opcode {
        0x00 => (), //NOOP
        0x2F => {
            load8(reg_file, Reg8::A, !reg_file.A);
            reg_file.flags.N = 1;
            reg_file.flags.H = 1;
        }
        0x3f => {
            // CCF
            reg_file.flags.C = if reg_file.flags.C == 1 { 0 } else { 1 };
            reg_file.flags.N = 0;
            reg_file.flags.H = 0;
        }
        0x37 => {
            // SCF
            reg_file.flags.C = 1;
            reg_file.flags.N = 0;
            reg_file.flags.H = 0;
        }
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

        n @ (0x70..=0x75 | 0x77) => match instruction_stage {
            0 => {
                new_instruction_stage = 1;
                pass_to_next_stage.data = match n {
                    0x70 => reg_file[Reg8::B],
                    0x71 => reg_file[Reg8::C],
                    0x72 => reg_file[Reg8::D],
                    0x73 => reg_file[Reg8::E],
                    0x74 => reg_file[Reg8::H],
                    0x75 => reg_file[Reg8::L],
                    0x77 => reg_file[Reg8::A],
                    _ => panic!("invalid op"),
                }
            }
            1 => {
                mem.addr_bus = reg_file.get16(Reg16::HL);
                mem.write_data(passed.data);
                new_instruction_stage = 0;
            }
            _ => panic!("0x70-0x75: invalid instruction stage"),
        },
        0x76 => (), //HALT

        0x78 => load8(reg_file, Reg8::A, reg_file.B),
        0x79 => load8(reg_file, Reg8::A, reg_file.C),
        0x7A => load8(reg_file, Reg8::A, reg_file.D),
        0x7B => load8(reg_file, Reg8::A, reg_file.E),
        0x7C => load8(reg_file, Reg8::A, reg_file.H),
        0x7D => load8(reg_file, Reg8::A, reg_file.L),
        0x7F => load8(reg_file, Reg8::A, reg_file.A),
        // ADD/ADC
        n @ (0x80..=0x8F) => {
            let a = reg_file[Reg8::A];
            match instruction_stage {
                0 => {
                    let b = match n {
                        0x80 | 0x88 => reg_file[Reg8::B],
                        0x81 | 0x89 => reg_file[Reg8::C],
                        0x82 | 0x8A => reg_file[Reg8::D],
                        0x83 | 0x8B => reg_file[Reg8::E],
                        0x84 | 0x8C => reg_file[Reg8::H],
                        0x85 | 0x8D => reg_file[Reg8::L],
                        0x86 | 0x8E => {
                            mem.addr_bus = reg_file.get16(Reg16::HL);
                            mem.read()
                        }
                        0x87 | 0x8F => reg_file[Reg8::A],
                        _ => panic!("invalid op"),
                    };
                    let (val, flags) = add(a, b);
                    if n == 0x86 || n == 0x8E {
                        new_instruction_stage = 1;
                        pass_to_next_stage.data = val;
                        pass_to_next_stage.flags = flags;
                        pass_to_next_stage.next_pc = pc;
                    } else {
                        load8(reg_file, Reg8::A, val);
                        reg_file.flags = flags;
                    }
                }
                1 => {
                    load8(reg_file, Reg8::A, passed.data);
                    reg_file.flags = passed.flags;
                    new_instruction_stage = 0;
                }
                _ => (),
            };
        }
        // SUB/SBC/CP
        n @ ((0x90..=0x9F) | (0xB8..=0xBF)) => {
            let a = reg_file[Reg8::A];
            match new_instruction_stage {
                0 => {
                    let b = match n {
                        0x90 | 0x98 | 0xB8 => reg_file[Reg8::B],
                        0x91 | 0x99 | 0xB9 => reg_file[Reg8::C],
                        0x92 | 0x9A | 0xBA => reg_file[Reg8::D],
                        0x93 | 0x9B | 0xBB => reg_file[Reg8::E],
                        0x94 | 0x9C | 0xBC => reg_file[Reg8::H],
                        0x95 | 0x9D | 0xBD => reg_file[Reg8::L],
                        0x96 | 0x9E | 0xBE => {
                            mem.addr_bus = reg_file.get16(Reg16::HL);
                            mem.read()
                        }
                        0x97 | 0x9F | 0xBF => reg_file[Reg8::A],
                        _ => panic!("Invalid instruction!"),
                    };
                    //let mut flags: u8 = 0x40; // set the subtraction flag
                    let (val, flags) = match n {
                        0x90..=0x97 => sub(a, b),
                        _ => subc(a, b, reg_file.flags.C),
                    };
                    match n {
                        0x96 | 0x9E | 0xBE => {
                            new_instruction_stage = 1;
                            pass_to_next_stage.data = val;
                            pass_to_next_stage.flags = flags;
                            pass_to_next_stage.next_pc = pc;
                        }
                        (0x90..=0x9F) => {
                            load8(reg_file, Reg8::A, val as u8);
                            reg_file.flags = flags;
                        }
                        _ => (),
                    }
                }
                1 => {
                    if n != 0xBE {
                        load8(reg_file, Reg8::A, passed.data);
                    }
                    reg_file.flags = passed.flags;
                    new_instruction_stage = 0;
                }
                _ => (),
            }
        }
        // AND instructions
        n @ (0xA0..=0xA7) => match instruction_stage {
            0 => {
                let a = reg_file[Reg8::A];
                let b = match n {
                    0xA0 => reg_file[Reg8::B],
                    0xA1 => reg_file[Reg8::C],
                    0xA2 => reg_file[Reg8::D],
                    0xA3 => reg_file[Reg8::E],
                    0xA4 => reg_file[Reg8::H],
                    0xA5 => reg_file[Reg8::L],
                    0xA6 => {
                        mem.addr_bus = reg_file.get16(Reg16::HL);
                        mem.read()
                    }
                    0xA7 => reg_file[Reg8::A],
                    _ => panic!("invalid instruction!"),
                };
                let (val, flags) = and(a, b);
                if n == 0xA6 {
                    new_instruction_stage = 1;
                    pass_to_next_stage.data = val;
                    pass_to_next_stage.flags = flags;
                    pass_to_next_stage.next_pc = pc;
                } else {
                    load8(reg_file, Reg8::A, val);
                    reg_file.flags = flags;
                }
            }
            1 => {
                new_instruction_stage = 0;
                load8(reg_file, Reg8::A, passed.data);
                reg_file.flags = passed.flags;
            }
            _ => (),
        },
        // XOR/OR instructions
        n @ (0xA8..=0xB7) => match instruction_stage {
            0 => {
                let a = reg_file[Reg8::A];
                let b = match n {
                    0xB0 | 0xA8 => reg_file[Reg8::B],
                    0xB1 | 0xA9 => reg_file[Reg8::C],
                    0xB2 | 0xAA => reg_file[Reg8::D],
                    0xB3 | 0xAB => reg_file[Reg8::E],
                    0xB4 | 0xAC => reg_file[Reg8::H],
                    0xB5 | 0xAD => reg_file[Reg8::L],
                    0xB6 | 0xAE => {
                        mem.addr_bus = reg_file.get16(Reg16::HL);
                        mem.read()
                    }
                    0xB7 | 0xAF => reg_file[Reg8::A],
                    _ => panic!("invalid instruction!"),
                };
                let (val, flags) = match n {
                    0xA8..=0xAF => xor(a, b),
                    0xB0..=0xB7 => or(a, b),
                    _ => panic!("invalid instruction!"),
                };

                if n == 0xB6 || n == 0xAE {
                    new_instruction_stage = 1;
                    pass_to_next_stage.data = val;
                    pass_to_next_stage.flags = flags;
                    pass_to_next_stage.next_pc = pc;
                } else {
                    load8(reg_file, Reg8::A, val);
                    reg_file.flags = flags;
                }
            }
            1 => {
                new_instruction_stage = 0;
                load8(reg_file, Reg8::A, passed.data);
                reg_file.flags = passed.flags;
            }
            _ => (),
        },
        n @ (0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C) => match instruction_stage {
            // INC instructions
            0 => {
                let a = match n {
                    0x04 => reg_file[Reg8::B],
                    0x14 => reg_file[Reg8::D],
                    0x24 => reg_file[Reg8::H],
                    0x34 => {
                        mem.addr_bus = reg_file.get16(Reg16::HL);
                        mem.read()
                    }
                    0x0C => reg_file[Reg8::C],
                    0x1C => reg_file[Reg8::E],
                    0x2C => reg_file[Reg8::L],
                    0x3C => reg_file[Reg8::A],
                    _ => panic!("invalid instruction"),
                };
                let mut flags = Flags::default();
                flags.Z = if (a + 1) == 0 { 1 } else { 0 };
                flags.N = 0;
                flags.H = if (a & 0x07) == 7 { 1 } else { 0 };
                match n {
                    0x04 => load8(reg_file, Reg8::B, a + 1),
                    0x14 => load8(reg_file, Reg8::D, a + 1),
                    0x24 => load8(reg_file, Reg8::H, a + 1),
                    0x0C => load8(reg_file, Reg8::C, a + 1),
                    0x1C => load8(reg_file, Reg8::E, a + 1),
                    0x2C => load8(reg_file, Reg8::L, a + 1),
                    0x3C => load8(reg_file, Reg8::A, a + 1),
                    _ => panic!("invalid instruction"),
                };
                if n == 0x34 {
                    new_instruction_stage = 1;
                    pass_to_next_stage.data = a + 1;
                    pass_to_next_stage.flags = flags;
                    pass_to_next_stage.next_pc = pc;
                } else {
                    reg_file.flags = flags;
                }
            }
            1 => {
                mem.addr_bus = reg_file.get16(Reg16::HL);
                mem.write_data(passed.data);
                new_instruction_stage = 2;
                pass_to_next_stage.next_pc = pc;
            }
            2 => {
                new_instruction_stage = 0;
            }
            _ => (),
        },
        n @ (0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D) => match instruction_stage {
            // DEC instructions
            0 => {
                let a = match n {
                    0x05 => reg_file[Reg8::B],
                    0x15 => reg_file[Reg8::D],
                    0x25 => reg_file[Reg8::H],
                    0x35 => {
                        mem.addr_bus = reg_file.get16(Reg16::HL);
                        mem.read()
                    }
                    0x0D => reg_file[Reg8::C],
                    0x1D => reg_file[Reg8::E],
                    0x2D => reg_file[Reg8::L],
                    0x3D => reg_file[Reg8::A],
                    _ => panic!("invalid instruction"),
                };
                let mut flags = Flags::default();
                flags.Z = if (a as i8 - 1) == 0 { 1 } else { 0 };
                flags.N = 0;
                flags.H = if (a & 0x07) == 0 { 1 } else { 0 };

                match n {
                    0x05 => load8(reg_file, Reg8::B, a - 1),
                    0x15 => load8(reg_file, Reg8::D, a - 1),
                    0x25 => load8(reg_file, Reg8::H, a - 1),
                    0x0D => load8(reg_file, Reg8::C, a - 1),
                    0x1D => load8(reg_file, Reg8::E, a - 1),
                    0x2D => load8(reg_file, Reg8::L, a - 1),
                    0x3D => load8(reg_file, Reg8::A, a - 1),
                    _ => panic!("invalid instruction"),
                };
                if n == 0x35 {
                    new_instruction_stage = 1;
                    pass_to_next_stage.data = a - 1;
                    pass_to_next_stage.flags = flags;
                    pass_to_next_stage.next_pc = pc;
                } else {
                    reg_file.flags = flags;
                }
            }
            1 => {
                new_instruction_stage = 2;
                pass_to_next_stage.next_pc = pc;
                mem.addr_bus = reg_file.get16(Reg16::HL);
                mem.write_data(passed.data);
            }
            2 => {
                new_instruction_stage = 0;
            }
            _ => (),
        },
        0xE9 => pass_to_next_stage.next_pc = reg_file.get16(Reg16::HL),
        _ => panic!("Not an 8bit instruction!"),
    }
    (new_instruction_stage, pass_to_next_stage)
}

fn add(a: u8, b: u8) -> (u8, Flags) {
    let val: u16 = a as u16 + b as u16;
    let mut flags = Flags::default();
    flags.H = if (((a & 0x0F) + (b & 0x0F)) & 0x10) == 0x10 {
        1
    } else {
        0
    };
    flags.Z = if val as u8 == 0 { 1 } else { 0 };
    flags.C = if val > 255 { 1 } else { 0 };
    (val as u8, flags)
}

fn adc(a: u8, b: u8, carry: u8) -> (u8, Flags) {
    if carry > 1 {
        panic!("ADC: carry is greater than one");
    }
    let val = a as u16 + b as u16 + carry as u16 >> 4 as u16;
    let mut flags = Flags::default();
    flags.H = if (((a & 0x0F) + (b & 0x0F) + carry) & 0x10) == 0x10 {
        1
    } else {
        0
    };
    flags.Z = if val as u8 == 0 { 1 } else { 0 };
    flags.C = if val > 255 { 1 } else { 0 };

    (val as u8, flags)
}

fn sub(a: u8, b: u8) -> (u8, Flags) {
    let val = (a - b) as i8;
    let mut flags = Flags::default();
    flags.N = 1;
    flags.H = if (a & 0x0F) < (b & 0x0F) { 1 } else { 0 };
    flags.Z = if val == 0 { 1 } else { 0 };
    flags.C = if val < 0 { 1 } else { 0 };
    (val as u8, flags)
}

fn subc(a: u8, b: u8, carry: u8) -> (u8, Flags) {
    if carry > 1 {
        panic!("SUBC: carry is greater than one");
    }
    let val = (a - (b + carry)) as i8;
    let mut flags = Flags::default();
    flags.N = 1;
    flags.H = if (((a & 0x0F) - ((b * 0x0F) + carry)) as i8) < 0 {
        1
    } else {
        0
    };
    flags.C = if val < 0 { 1 } else { 0 };
    flags.Z = if val == 0 { 1 } else { 0 };
    (val as u8, flags)
}

fn and(a: u8, b: u8) -> (u8, Flags) {
    let val = a & b;
    let mut flags = Flags::default();
    flags.Z = if val == 0 { 1 } else { 0 };
    flags.N = 0;
    flags.H = 1;
    flags.C = 0;
    (val, flags)
}

fn xor(a: u8, b: u8) -> (u8, Flags) {
    let val = a ^ b;
    let mut flags = Flags::default();
    flags.Z = if val == 0 { 1 } else { 0 };
    flags.N = 0;
    flags.H = 0;
    flags.C = 0;
    (val, flags)
}

fn or(a: u8, b: u8) -> (u8, Flags) {
    let val = a | b;
    let mut flags = Flags::default();
    flags.Z = if val == 0 { 1 } else { 0 };
    flags.N = 0;
    flags.H = 0;
    flags.C = 0;
    (val, flags)
}

#[derive(Clone, Copy, Default)]
pub struct StagePassThrough {
    data: u8,
    flags: Flags,
    pub next_pc: u16,
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
        // Reg16::PC => reg_file.PC = value,
        Reg16::SP => reg_file.SP = value,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut flags = Flags::default();
        let mut test_tup = add(1, 1);
        assert_eq!(test_tup.0, 2);
        assert_eq!(test_tup.1, flags);

        flags = Flags::default();
        flags.H = 1;
        flags.Z = 1;
        flags.C = 1;
        test_tup = add(255, 1);
        assert_eq!(test_tup.0, 0);
        assert_eq!(test_tup.1, flags);

        flags = Flags::default();
        flags.H = 1;
        test_tup = add(63, 1);
        assert_eq!(test_tup.0, 64);
        assert_eq!(test_tup.1, flags);
    }
}
