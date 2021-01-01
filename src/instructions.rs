use crate::cpu::{Flags, Memory, Reg16, Reg8, RegFile};

pub fn instruction_decode(
    opcode: u8,
    reg_file: &mut RegFile,
    mem: &mut Memory,
    passed: StagePassThrough,
) -> StagePassThrough {
    match opcode {
        0x00 => {
            let mut pass_to_next_stage = StagePassThrough::default();
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage
        } //NOOP
	n @ (0x02 | 0x12) => ld_16_a(reg_file, mem, n, passed),
        0x2F => cpl(reg_file, passed),
        0x3f => ccf(reg_file, passed),
        0x37 => scf(reg_file, passed),
        0x40 => load8(reg_file, Reg8::B, reg_file.B, passed),
        0x41 => load8(reg_file, Reg8::B, reg_file.C, passed),
        0x42 => load8(reg_file, Reg8::B, reg_file.D, passed),
        0x43 => load8(reg_file, Reg8::B, reg_file.E, passed),
        0x44 => load8(reg_file, Reg8::B, reg_file.H, passed),
        0x45 => load8(reg_file, Reg8::B, reg_file.L, passed),
        0x47 => load8(reg_file, Reg8::B, reg_file.A, passed),

        0x48 => load8(reg_file, Reg8::C, reg_file.B, passed),
        0x49 => load8(reg_file, Reg8::C, reg_file.C, passed),
        0x4A => load8(reg_file, Reg8::C, reg_file.D, passed),
        0x4B => load8(reg_file, Reg8::C, reg_file.E, passed),
        0x4C => load8(reg_file, Reg8::C, reg_file.H, passed),
        0x4D => load8(reg_file, Reg8::C, reg_file.L, passed),
        0x4F => load8(reg_file, Reg8::C, reg_file.A, passed),

        0x50 => load8(reg_file, Reg8::D, reg_file.B, passed),
        0x51 => load8(reg_file, Reg8::D, reg_file.C, passed),
        0x52 => load8(reg_file, Reg8::D, reg_file.D, passed),
        0x53 => load8(reg_file, Reg8::D, reg_file.E, passed),
        0x54 => load8(reg_file, Reg8::D, reg_file.H, passed),
        0x55 => load8(reg_file, Reg8::D, reg_file.L, passed),
        0x57 => load8(reg_file, Reg8::D, reg_file.A, passed),

        0x58 => load8(reg_file, Reg8::E, reg_file.B, passed),
        0x59 => load8(reg_file, Reg8::E, reg_file.C, passed),
        0x5A => load8(reg_file, Reg8::E, reg_file.D, passed),
        0x5B => load8(reg_file, Reg8::E, reg_file.E, passed),
        0x5C => load8(reg_file, Reg8::E, reg_file.H, passed),
        0x5D => load8(reg_file, Reg8::E, reg_file.L, passed),
        0x5F => load8(reg_file, Reg8::E, reg_file.A, passed),

        0x60 => load8(reg_file, Reg8::H, reg_file.B, passed),
        0x61 => load8(reg_file, Reg8::H, reg_file.C, passed),
        0x62 => load8(reg_file, Reg8::H, reg_file.D, passed),
        0x63 => load8(reg_file, Reg8::H, reg_file.E, passed),
        0x64 => load8(reg_file, Reg8::H, reg_file.H, passed),
        0x65 => load8(reg_file, Reg8::H, reg_file.L, passed),
        0x67 => load8(reg_file, Reg8::H, reg_file.A, passed),

        0x68 => load8(reg_file, Reg8::L, reg_file.B, passed),
        0x69 => load8(reg_file, Reg8::L, reg_file.C, passed),
        0x6A => load8(reg_file, Reg8::L, reg_file.D, passed),
        0x6B => load8(reg_file, Reg8::L, reg_file.E, passed),
        0x6C => load8(reg_file, Reg8::L, reg_file.H, passed),
        0x6D => load8(reg_file, Reg8::L, reg_file.L, passed),
        0x6F => load8(reg_file, Reg8::L, reg_file.A, passed),

        n @ (0x70..=0x75 | 0x77) => ld_hl_r(reg_file, mem, passed, n),
        0x76 => {
            let mut pass_to_next_stage = StagePassThrough::default();
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage
        }, //HALT

        0x78 => load8(reg_file, Reg8::A, reg_file.B, passed),
        0x79 => load8(reg_file, Reg8::A, reg_file.C, passed),
        0x7A => load8(reg_file, Reg8::A, reg_file.D, passed),
        0x7B => load8(reg_file, Reg8::A, reg_file.E, passed),
        0x7C => load8(reg_file, Reg8::A, reg_file.H, passed),
        0x7D => load8(reg_file, Reg8::A, reg_file.L, passed),
        0x7F => load8(reg_file, Reg8::A, reg_file.A, passed),
        // ADD/ADC
        n @ (0x80..=0x8F) => add_adc(reg_file, mem, passed, n),
        // SUB/SBC/CP
        n @ ((0x90..=0x9F) | (0xB8..=0xBF)) => sub_sbc_cp(reg_file, mem, passed, n),
        // AND instructions
        n @ (0xA0..=0xA7) => and(reg_file, mem, passed, n),
        // XOR/OR instructions
        n @ (0xA8..=0xB7) => xor_or(reg_file, mem, passed, n),
        n @ (0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C) => {
            inc(reg_file, mem, passed, n)
        }
        n @ (0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D) => dec(reg_file, mem, passed, n),
        n @ (0xC1 | 0xD1 | 0xE1 | 0xF1) => pop(reg_file, mem, passed, n),
        n @ (0xC5 | 0xD5 | 0xE5 | 0xF5) => push(reg_file, mem, passed, n),
        0xE9 => jp_hl(reg_file),
        0xF9 => ld_sp_hl(reg_file, passed),
        _ => panic!("Invalid instruction"),
    }
}

fn ld_16_a(reg_file: &RegFile, mem: &mut Memory, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
	0 => {
	    let reg16 = match n {
		0x02 => Reg16::BC,
		0x12 => Reg16::DE,
		_ => panic!("invalid op"),
	    };
	    mem.addr_bus = reg_file.get16(reg16);
	    mem.write_data(reg_file[Reg8::A]);
	    pass_to_next_stage.instruction_stage = 1;
	    pass_to_next_stage.next_pc = passed.next_pc;
	}
	1 => {
	    pass_to_next_stage.next_pc = passed.next_pc + 1;
	    pass_to_next_stage.instruction_stage = 0;
	}
	_ => panic!("ld BC/DE a: invalid instruction stage"),
    };
    pass_to_next_stage
}
fn cpl(reg_file: &mut RegFile, passed: StagePassThrough) -> StagePassThrough {
    let pass_to_next_stage = load8(reg_file, Reg8::A, !reg_file.A, passed);
    reg_file.flags.N = 1;
    reg_file.flags.H = 1;
    pass_to_next_stage
}

fn ccf(reg_file: &mut RegFile, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    pass_to_next_stage.next_pc = passed.next_pc + 1;
    reg_file.flags.C = if reg_file.flags.C == 1 { 0 } else { 1 };
    reg_file.flags.N = 0;
    reg_file.flags.H = 0;
    pass_to_next_stage
}

fn scf(reg_file: &mut RegFile, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    pass_to_next_stage.next_pc = passed.next_pc + 1;
    reg_file.flags.C = 1;
    reg_file.flags.N = 0;
    reg_file.flags.H = 0;
    pass_to_next_stage
}

fn ld_hl_r(
    reg_file: &mut RegFile,
    mem: &mut Memory,
    passed: StagePassThrough,
    n: u8,
) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.instruction_stage = 1;
            pass_to_next_stage.data = match n {
                0x70 => reg_file[Reg8::B],
                0x71 => reg_file[Reg8::C],
                0x72 => reg_file[Reg8::D],
                0x73 => reg_file[Reg8::E],
                0x74 => reg_file[Reg8::H],
                0x75 => reg_file[Reg8::L],
                0x77 => reg_file[Reg8::A],
                _ => panic!("invalid op"),
            };
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        1 => {
            mem.addr_bus = reg_file.get16(Reg16::HL);
            mem.write_data(passed.data);
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => panic!("0x70-0x75: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn add_adc(
    reg_file: &mut RegFile,
    mem: &mut Memory,
    passed: StagePassThrough,
    n: u8,
) -> StagePassThrough {
    let a = reg_file[Reg8::A];
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
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
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.data = val;
                pass_to_next_stage.flags = flags;
                pass_to_next_stage.next_pc = passed.next_pc;
            } else {
                load8(reg_file, Reg8::A, val, passed);
                reg_file.flags = flags;
                pass_to_next_stage.next_pc + passed.next_pc + 1;
            }
        }
        1 => {
            load8(reg_file, Reg8::A, passed.data, passed);
            reg_file.flags = passed.flags;
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => (),
    };
    pass_to_next_stage
}

fn sub_sbc_cp(
    reg_file: &mut RegFile,
    mem: &mut Memory,
    passed: StagePassThrough,
    n: u8,
) -> StagePassThrough {
    let a = reg_file[Reg8::A];
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
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
                    pass_to_next_stage.instruction_stage = 1;
                    pass_to_next_stage.data = val;
                    pass_to_next_stage.flags = flags;
                    pass_to_next_stage.next_pc = passed.next_pc;
                }
                (0x90..=0x9F) => {
                    load8(reg_file, Reg8::A, val as u8, passed);
                    reg_file.flags = flags;
                    pass_to_next_stage.next_pc + passed.next_pc + 1;
                }
                _ => (),
            }
        }
        1 => {
            if n != 0xBE {
                load8(reg_file, Reg8::A, passed.data, passed);
            }
            reg_file.flags = passed.flags;
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn xor_or(
    reg_file: &mut RegFile,
    mem: &mut Memory,
    passed: StagePassThrough,
    n: u8,
) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
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
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.data = val;
                pass_to_next_stage.flags = flags;
                pass_to_next_stage.next_pc = passed.next_pc;
            } else {
                load8(reg_file, Reg8::A, val, passed);
                reg_file.flags = flags;
                pass_to_next_stage.next_pc + passed.next_pc + 1;
            }
        }
        1 => {
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            load8(reg_file, Reg8::A, passed.data, passed);
            reg_file.flags = passed.flags;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn and(
    reg_file: &mut RegFile,
    mem: &mut Memory,
    passed: StagePassThrough,
    n: u8,
) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
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
            let (val, flags) = and_calc(a, b);
            if n == 0xA6 {
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.data = val;
                pass_to_next_stage.flags = flags;
                pass_to_next_stage.next_pc = passed.next_pc;
            } else {
                load8(reg_file, Reg8::A, val, passed);
                reg_file.flags = flags;
                pass_to_next_stage.next_pc + passed.next_pc + 1;
            }
        }
        1 => {
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            load8(reg_file, Reg8::A, passed.data, passed);
            reg_file.flags = passed.flags;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn inc(
    reg_file: &mut RegFile,
    mem: &mut Memory,
    passed: StagePassThrough,
    n: u8,
) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
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
                0x04 => load8(reg_file, Reg8::B, a + 1, passed),
                0x14 => load8(reg_file, Reg8::D, a + 1, passed),
                0x24 => load8(reg_file, Reg8::H, a + 1, passed),
                0x0C => load8(reg_file, Reg8::C, a + 1, passed),
                0x1C => load8(reg_file, Reg8::E, a + 1, passed),
                0x2C => load8(reg_file, Reg8::L, a + 1, passed),
                0x3C => load8(reg_file, Reg8::A, a + 1, passed),
                _ => panic!("invalid instruction"),
            };
            if n == 0x34 {
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.data = a + 1;
                pass_to_next_stage.flags = flags;
                pass_to_next_stage.next_pc = passed.next_pc;
            } else {
                reg_file.flags = flags;
                pass_to_next_stage.next_pc = passed.next_pc + 1;
            }
        }
        1 => {
            mem.addr_bus = reg_file.get16(Reg16::HL);
            mem.write_data(passed.data);
            pass_to_next_stage.instruction_stage = 2;
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        2 => {
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn dec(
    reg_file: &mut RegFile,
    mem: &mut Memory,
    passed: StagePassThrough,
    n: u8,
) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
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
                0x05 => load8(reg_file, Reg8::B, a - 1, passed),
                0x15 => load8(reg_file, Reg8::D, a - 1, passed),
                0x25 => load8(reg_file, Reg8::H, a - 1, passed),
                0x0D => load8(reg_file, Reg8::C, a - 1, passed),
                0x1D => load8(reg_file, Reg8::E, a - 1, passed),
                0x2D => load8(reg_file, Reg8::L, a - 1, passed),
                0x3D => load8(reg_file, Reg8::A, a - 1, passed),
                _ => panic!("invalid instruction"),
            };
            if n == 0x35 {
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.data = a - 1;
                pass_to_next_stage.flags = flags;
                pass_to_next_stage.next_pc = passed.next_pc;
            } else {
                reg_file.flags = flags;
                pass_to_next_stage.next_pc = passed.next_pc + 1;
            }
        }
        1 => {
            pass_to_next_stage.instruction_stage = 2;
            pass_to_next_stage.next_pc = passed.next_pc;
            mem.addr_bus = reg_file.get16(Reg16::HL);
            mem.write_data(passed.data);
        }
        2 => {
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn pop(
    reg_file: &mut RegFile,
    mem: &mut Memory,
    passed: StagePassThrough,
    n: u8,
) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            let lsb_reg = match n {
                0xC1 => Reg8::C,
                0xD1 => Reg8::E,
                0xE1 => Reg8::L,
                0xF1 => Reg8::F,
                _ => panic!("POP: invalid op code"),
            };
            reg_file[lsb_reg] = mem.pop(reg_file);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            let msb_reg = match n {
                0xC1 => Reg8::B,
                0xD1 => Reg8::D,
                0xE1 => Reg8::H,
                0xF1 => Reg8::A,
                _ => panic!("POP: invalid op code"),
            };
            reg_file[msb_reg] = mem.pop(reg_file);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => panic!("POP: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn push(
    reg_file: &mut RegFile,
    mem: &mut Memory,
    passed: StagePassThrough,
    n: u8,
) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            mem.addr_bus = reg_file.SP;
            pass_to_next_stage.instruction_stage = 1;
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        1 => {
            let msb_reg = match n {
                0xC1 => Reg8::B,
                0xD1 => Reg8::D,
                0xE1 => Reg8::H,
                0xF1 => Reg8::A,
                _ => panic!("POP: invalid op code"),
            };
            mem.push(reg_file, msb_reg);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            let lsb_reg = match n {
                0xC5 => Reg8::C,
                0xD5 => Reg8::E,
                0xE5 => Reg8::L,
                0xF5 => Reg8::F,
                _ => panic!("POP: invalid op code"),
            };
            mem.push(reg_file, lsb_reg);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => panic!("PUSH: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn jp_hl(reg_file: &RegFile) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    pass_to_next_stage.next_pc = reg_file.get16(Reg16::HL);
    pass_to_next_stage
}

fn ld_sp_hl(reg_file: &mut RegFile, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.data16 = reg_file.get16(Reg16::HL);
            pass_to_next_stage.instruction_stage = 1;
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        1 => {
            reg_file.set16(Reg16::SP, passed.data16);
            pass_to_next_stage.instruction_stage = 0;
	    pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => panic!("LD SP, HL: invalid instruction stage"),
    }
    pass_to_next_stage
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

fn and_calc(a: u8, b: u8) -> (u8, Flags) {
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
    data16: u16,
    flags: Flags,
    pub next_pc: u16,
    pub instruction_stage: u8,
}

fn load8(
    reg_file: &mut RegFile,
    reg: Reg8,
    value: u8,
    passed: StagePassThrough,
) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    pass_to_next_stage.next_pc = passed.next_pc + 1;
    reg_file[reg] = value;
    pass_to_next_stage
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
