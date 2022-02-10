use crate::cpu::{Flags, Reg16, Reg8, RegFile, CPU};

#[derive(Clone, Copy, Default, Debug)]
pub struct StagePassThrough {
    data: u8,
    data16: u16,
    cb_op: u8,
    flags: Flags,
    pub ei: bool,
    pub di: bool,
    pub instruction_stage: u8,
}

pub enum CC {
    UC, //not a real condition code, but stands for unconditional
    NZ,
    NC,
    Z,
    C,
}

pub fn instruction_decode(opcode: u8, cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let reg_file = &mut cpu.reg_file;
    match opcode {
        0x00 => {
            let pass_to_next_stage = StagePassThrough::default();
	    cpu.pc += 1;
            pass_to_next_stage
        } //NOOP
        0x08 => ld_nn_sp(cpu, passed),
        n @ (0x20 | 0x30 | 0x18 | 0x28 | 0x38) => jr_cc_e(cpu, n, passed),
        n @ (0x01 | 0x11 | 0x21 | 0x31) => ld_rr_nn(cpu, n, passed),
        n @ (0x02 | 0x12 | 0x22 | 0x32) => ld_16_a(cpu, n, passed),
        n @ (0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C) => inc(cpu, passed, n),
        n @ (0x03 | 0x13 | 0x23 | 0x33) => inc_rr(cpu, n, passed),
        n @ (0x05 | 0x15 | 0x25 | 0x35 | 0x0D | 0x1D | 0x2D | 0x3D) => dec(cpu, passed, n),
        n @ (0x0B | 0x1B | 0x2B | 0x3B) => dec_rr(cpu, n, passed),
        n @ (0x06 | 0x16 | 0x26 | 0x0E | 0x1E | 0x2E | 0x3E) => ld_r_n(cpu, n, passed),
        n @ (0x46 | 0x56 | 0x66 | 0x4E | 0x5E | 0x6E | 0x7E) => ld_r_hl(cpu, n, passed),
        n @ (0x0A | 0x1A | 0x2A | 0x3A) => ld_a_16(cpu, n, passed),
        0x07 => rlca(cpu, passed),
        0x17 => rla(cpu, passed),
	0x27 => daa(cpu, passed),
        0x0F => rrca(cpu, passed),
        0x1F => rra(cpu, passed),
        0x2F => cpl(cpu, passed),
        0x3f => ccf(cpu, passed),
        0x36 => ld_hl_n(cpu, passed),
        0x37 => scf(cpu, passed),
	n @ (0x40..=0x45 | 0x47..=0x4D | 0x4F..=0x55 | 0x57..=0x5D | 0x5F..=0x65 | 0x67..=0x6D | 0x6F | 0x78..=0x7D | 0x7F) => {
	    cpu.pc += 1;
	    match n {
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

		0x78 => load8(reg_file, Reg8::A, reg_file.B, passed),
		0x79 => load8(reg_file, Reg8::A, reg_file.C, passed),
		0x7A => load8(reg_file, Reg8::A, reg_file.D, passed),
		0x7B => load8(reg_file, Reg8::A, reg_file.E, passed),
		0x7C => load8(reg_file, Reg8::A, reg_file.H, passed),
		0x7D => load8(reg_file, Reg8::A, reg_file.L, passed),
		0x7F => load8(reg_file, Reg8::A, reg_file.A, passed),
		_ => panic!("invalid load opcode"),
	    }
	}
	0x76 => StagePassThrough::default(), //HALT
        n @ (0x70..=0x75 | 0x77) => ld_hl_r(cpu, passed, n),
        
        n @ (0x09 | 0x19 | 0x29 | 0x39) => add_hl_nn(cpu, passed, n),
        n @ (0x80..=0x8F) => add_adc(cpu, passed, n),
        n @ ((0x90..=0x9F) | (0xB8..=0xBF)) => sub_sbc_cp(cpu, passed, n),
        n @ (0xA0..=0xA7) => and(cpu, passed, n),
        n @ (0xA8..=0xB7) => xor_or(cpu, passed, n),
        n @ (0xC0 | 0xD0 | 0xC8 | 0xD8) => ret_cc(cpu, n, passed),
        0xC9 => ret(cpu, passed),
        n @ (0xC1 | 0xD1 | 0xE1 | 0xF1) => pop(cpu, passed, n),
        n @ (0xC5 | 0xD5 | 0xE5 | 0xF5) => push(cpu, passed, n),
        n @ (0xC6 | 0xD6 | 0xE6 | 0xF6 | 0xCE | 0xDE | 0xEE | 0xFE) => alu_imm(cpu, n, passed),
        n @ (0xC7 | 0xD7 | 0xE7 | 0xF7 | 0xCF | 0xDF | 0xEF | 0xFF) => rst(cpu, n, passed),
        0xE8 => add_sp_e(cpu, passed),
        n @ (0xC2 | 0xD2 | 0xC3 | 0xCA | 0xDA) => jp_nn(cpu, n, passed),
        n @ (0xC4 | 0xD4 | 0xCC | 0xDC | 0xCD) => call_nn(cpu, n, passed),
        0xCB => cb_op(cpu, passed),
        0xE0 => ldh_n_a(cpu, passed),
        0xF0 => ldh_a_n(cpu, passed),
        0xE2 => ldh_c_a(cpu, passed),
        0xF2 => ldh_a_c(cpu, passed),
        0xE9 => jp_hl(cpu),
        0xEA => ld_nn_a(cpu, passed),
        0xF8 => ld_hl_sp_e(cpu, passed),
        0xF9 => ld_sp_hl(cpu, passed),
        0xFA => ld_a_nn(cpu, passed),
        0xF3 => di(cpu, passed),
        0xFb => ei(cpu, passed),
        0xD9 => reti(cpu, passed),
        _ => panic!("Invalid instruction: {:#x}, pc: {:#x}", opcode, cpu.pc),
    }
}

//Adapted from Gekkio's mooneye emulator
fn daa(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
	0 => {
	    if cpu.reg_file.flags.N == 0 {
		if cpu.reg_file.flags.C == 1 || cpu.reg_file[Reg8::A] > 0x99 {
		    cpu.reg_file[Reg8::A] = cpu.reg_file[Reg8::A].wrapping_add(0x60);
		    cpu.reg_file.flags.C = 1;
		}

		if cpu.reg_file.flags.H == 1 || cpu.reg_file[Reg8::A] & 0x0F > 0x09 {
		    cpu.reg_file[Reg8::A] = cpu.reg_file[Reg8::A].wrapping_add(0x06);
		}
	    } else if cpu.reg_file.flags.C == 1 {
		cpu.reg_file.flags.C = 1;
		cpu.reg_file[Reg8::A] = cpu.reg_file[Reg8::A].wrapping_add(if cpu.reg_file.flags.H == 1 { 0x9A } else { 0xA0 });
	    } else if cpu.reg_file.flags.H == 1 {
		cpu.reg_file[Reg8::A] = cpu.reg_file[Reg8::A].wrapping_add(0xFA);
	    }
	    cpu.reg_file.flags.H = 0;
	    cpu.reg_file.flags.Z = if cpu.reg_file[Reg8::A] == 0 { 1 } else { 0 };
	    cpu.pc += 1;
	}
	_ => panic!("daa: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn di(cpu: &mut CPU, _passed: StagePassThrough) -> StagePassThrough {
    cpu.pc += 1;
    StagePassThrough {
        di: true,
        ..Default::default()
    }
}

fn ei(cpu: &mut CPU, _passed: StagePassThrough) -> StagePassThrough {
    cpu.pc += 1;
    StagePassThrough {
        ei: true,
        ..Default::default()
    }
}

fn jr_cc_e(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
	    cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            let cc = match n {
                0x20 => CC::NZ,
                0x30 => CC::NC,
                0x18 => CC::UC,
                0x28 => CC::Z,
                0x38 => CC::C,
                _ => panic!("jr_cc_e: invalid opcode"),
            };
            if cpu.reg_file.check_condition(cc) {
                pass_to_next_stage.instruction_stage = 2;
                pass_to_next_stage.data = passed.data;
            } else {
                pass_to_next_stage.instruction_stage = 0;
            }
	    cpu.pc += 1;
        }
        2 => {
            cpu.pc =
                (cpu.pc as i16 + (passed.data as i8 as i16)) as u16;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("jr_cc_ee: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_r_hl(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            let reg8 = match n {
                0x46 => Reg8::B,
                0x56 => Reg8::D,
                0x66 => Reg8::H,
                0x4E => Reg8::C,
                0x5E => Reg8::E,
                0x6E => Reg8::L,
                0x7E => Reg8::A,
                _ => panic!("invalid n"),
            };
            cpu.reg_file[reg8] = passed.data;
	    cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ld_r_hl: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_rr_nn(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
	    cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
	    cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            let val = ((cpu.read() as u16) << 8) | passed.data as u16;
            let reg16 = match n {
                0x01 => Reg16::BC,
                0x11 => Reg16::DE,
                0x21 => Reg16::HL,
                0x31 => Reg16::SP,
                _ => panic!("ld_rr_nn: invalid opcode"),
            };
            cpu.reg_file.set16(reg16, val);
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
	    cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ld_rr_nn: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_16_a(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            let reg16 = match n {
                0x02 => Reg16::BC,
                0x12 => Reg16::DE,
                0x22 => Reg16::HL,
                0x32 => Reg16::HL,
                _ => panic!("invalid op"),
            };
            cpu.addr_bus = cpu.reg_file.get16(reg16);
            cpu.write_data(cpu.reg_file[Reg8::A]);
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
	    cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
            if n == 0x22 {
                cpu.reg_file
                    .set16(Reg16::HL, cpu.reg_file.get16(Reg16::HL).wrapping_add(1));
            } else if n == 0x32 {
                cpu.reg_file
                    .set16(Reg16::HL, cpu.reg_file.get16(Reg16::HL).wrapping_sub(1));
            }
        }
        _ => panic!("ld BC/DE a: invalid instruction stage"),
    };
    pass_to_next_stage
}

fn ld_a_16(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            let reg16 = match n {
                0x0A => Reg16::BC,
                0x1A => Reg16::DE,
                0x2A | 0x3A => Reg16::HL,
                _ => panic!("invalid opcode"),
            };
            cpu.addr_bus = cpu.reg_file.get16(reg16);
            cpu.reg_file[Reg8::A] = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.instruction_stage = 0;
	    cpu.pc += 1;
            if n == 0x2A {
                cpu.reg_file
                    .set16(Reg16::HL, cpu.reg_file.get16(Reg16::HL) + 1)
            } else if n == 0x3A {
                cpu.reg_file
                    .set16(Reg16::HL, cpu.reg_file.get16(Reg16::HL) - 1)
            }
        }
        _ => panic!("ld_a_16: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_r_n(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
	    cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            let reg8 = match n {
                0x06 => Reg8::B,
                0x16 => Reg8::D,
                0x26 => Reg8::H,
                0x0E => Reg8::C,
                0x1E => Reg8::E,
                0x2E => Reg8::L,
                0x3E => Reg8::A,
                _ => panic!("ld_r_n: invalid opcode"),
            };
            cpu.reg_file[reg8] = passed.data;
	    cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ld_r_n: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_hl_n(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
	    cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
	    cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ld_hl_n: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rlca(cpu: &mut CPU, _passed: StagePassThrough) -> StagePassThrough {
    let val = cpu.reg_file[Reg8::A].rotate_left(1);
    cpu.reg_file.flags.C = cpu.reg_file[Reg8::A] >> 7;
    cpu.reg_file.flags.N = 0;
    cpu.reg_file.flags.H = 0;
    cpu.reg_file.flags.Z = 0;
    cpu.reg_file[Reg8::A] = val;
    cpu.pc += 1;
    StagePassThrough {
        ..Default::default()
    }
}

fn rrca(cpu: &mut CPU, _passed: StagePassThrough) -> StagePassThrough {
    let val = cpu.reg_file[Reg8::A].rotate_right(1);
    cpu.reg_file.flags.C = cpu.reg_file[Reg8::A] & 1;
    cpu.reg_file.flags.N = 0;
    cpu.reg_file.flags.H = 0;
    cpu.reg_file.flags.Z = 0;
    cpu.reg_file[Reg8::A] = val;
    cpu.pc += 1;
    StagePassThrough {
        ..Default::default()
    }
}

fn rla(cpu: &mut CPU, _passed: StagePassThrough) -> StagePassThrough {
    let carry = cpu.reg_file.flags.C;
    let val = (cpu.reg_file[Reg8::A] << 1) | carry;
    cpu.reg_file.flags.C = cpu.reg_file[Reg8::A] >> 7;
    cpu.reg_file.flags.N = 0;
    cpu.reg_file.flags.H = 0;
    cpu.reg_file.flags.Z = 0;
    cpu.reg_file[Reg8::A] = val;
    cpu.pc += 1;
    StagePassThrough {
        ..Default::default()
    }
}

fn rra(cpu: &mut CPU, _passed: StagePassThrough) -> StagePassThrough {
    let carry = cpu.reg_file.flags.C;
    let val = (cpu.reg_file[Reg8::A] >> 1) | (carry << 7);
    cpu.reg_file.flags.C = cpu.reg_file[Reg8::A] & 1;
    cpu.reg_file.flags.N = 0;
    cpu.reg_file.flags.H = 0;
    cpu.reg_file.flags.Z = 0;
    cpu.reg_file[Reg8::A] = val;
    cpu.pc += 1;
    StagePassThrough {
        ..Default::default()
    }
}

fn cpl(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let reg_file = &mut cpu.reg_file;
    let pass_to_next_stage = load8(reg_file, Reg8::A, !reg_file.A, passed);
    cpu.pc += 1;
    cpu.reg_file.flags.N = 1;
    cpu.reg_file.flags.H = 1;
    pass_to_next_stage
}

fn ccf(cpu: &mut CPU, _passed: StagePassThrough) -> StagePassThrough {
    let pass_to_next_stage = StagePassThrough::default();
    cpu.pc += 1;
    cpu.reg_file.flags.C = if cpu.reg_file.flags.C == 1 { 0 } else { 1 };
    cpu.reg_file.flags.N = 0;
    cpu.reg_file.flags.H = 0;
    pass_to_next_stage
}

fn scf(cpu: &mut CPU, _passed: StagePassThrough) -> StagePassThrough {
    let pass_to_next_stage = StagePassThrough::default();
    cpu.pc += 1;
    cpu.reg_file.flags.C = 1;
    cpu.reg_file.flags.N = 0;
    cpu.reg_file.flags.H = 0;
    pass_to_next_stage
}

fn ld_hl_r(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.instruction_stage = 1;
            pass_to_next_stage.data = match n {
                0x70 => cpu.reg_file[Reg8::B],
                0x71 => cpu.reg_file[Reg8::C],
                0x72 => cpu.reg_file[Reg8::D],
                0x73 => cpu.reg_file[Reg8::E],
                0x74 => cpu.reg_file[Reg8::H],
                0x75 => cpu.reg_file[Reg8::L],
                0x77 => cpu.reg_file[Reg8::A],
                _ => panic!("invalid op"),
            };
        }
        1 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.instruction_stage = 0;
	    cpu.pc += 1;
        }
        _ => panic!("0x70-0x75: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn add_hl_nn(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    let nn = match n {
        0x09 => Reg16::BC,
        0x19 => Reg16::DE,
        0x29 => Reg16::HL,
        0x39 => Reg16::SP,
        _ => panic!("add_hl_nn: invalid opcode"),
    };
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            let (val, mut flags) = add_16(cpu.reg_file.get16(Reg16::HL), cpu.reg_file.get16(nn));
	    flags.Z = cpu.reg_file.flags.Z;
	    cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
            cpu.reg_file.flags = flags;
            cpu.reg_file.set16(Reg16::HL, val);
        }
        _ => panic!("add_hl_nn: invalid opcode"),
    }
    pass_to_next_stage
}

fn add_adc(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let a = cpu.reg_file[Reg8::A];
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            let b = match n {
                0x80 | 0x88 => cpu.reg_file[Reg8::B],
                0x81 | 0x89 => cpu.reg_file[Reg8::C],
                0x82 | 0x8A => cpu.reg_file[Reg8::D],
                0x83 | 0x8B => cpu.reg_file[Reg8::E],
                0x84 | 0x8C => cpu.reg_file[Reg8::H],
                0x85 | 0x8D => cpu.reg_file[Reg8::L],
                0x86 | 0x8E => {
                    cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                    cpu.read()
                }
                0x87 | 0x8F => cpu.reg_file[Reg8::A],
                _ => panic!("invalid op"),
            };
            let (val, flags) = match n {
                0x80..=0x87 => add(a, b),
                0x88..=0x8F => adc(a, b, cpu.reg_file.flags.C),
                _ => panic!("invalid op"),
            };
            if n == 0x86 || n == 0x8E {
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.data = val;
                pass_to_next_stage.flags = flags;
            } else {
                load8(&mut cpu.reg_file, Reg8::A, val, passed);
                cpu.reg_file.flags = flags;
		cpu.pc += 1;
            }
        }
        1 => {
            load8(&mut cpu.reg_file, Reg8::A, passed.data, passed);
            cpu.reg_file.flags = passed.flags;
            pass_to_next_stage.instruction_stage = 0;
	    cpu.pc += 1;
        }
        _ => (),
    };
    pass_to_next_stage
}

fn sub_sbc_cp(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let a = cpu.reg_file[Reg8::A];
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            let b = match n {
                0x90 | 0x98 | 0xB8 => cpu.reg_file[Reg8::B],
                0x91 | 0x99 | 0xB9 => cpu.reg_file[Reg8::C],
                0x92 | 0x9A | 0xBA => cpu.reg_file[Reg8::D],
                0x93 | 0x9B | 0xBB => cpu.reg_file[Reg8::E],
                0x94 | 0x9C | 0xBC => cpu.reg_file[Reg8::H],
                0x95 | 0x9D | 0xBD => cpu.reg_file[Reg8::L],
                0x96 | 0x9E | 0xBE => {
                    cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                    cpu.read()
                }
                0x97 | 0x9F | 0xBF => cpu.reg_file[Reg8::A],
                _ => panic!("Invalid instruction!"),
            };
            let (val, flags) = match n {
                (0x90..=0x97) | (0xB8..=0xBF) => sub(a, b),
                _ => subc(a, b, cpu.reg_file.flags.C),
            };
            match n {
                0x96 | 0x9E | 0xBE => {
                    pass_to_next_stage.instruction_stage = 1;
                    pass_to_next_stage.data = val;
                    pass_to_next_stage.flags = flags;
                }
                (0x90..=0x9F) => {
                    cpu.reg_file[Reg8::A] = val;
                    cpu.reg_file.flags = flags;
		    cpu.pc += 1;
                }
		(0xB8..=0xBF) => {
		    cpu.reg_file.flags = flags;
		    cpu.pc += 1;
		}
                _ => panic!("missed an op"),
            }
        }
        1 => {
            if n != 0xBE {
                load8(&mut cpu.reg_file, Reg8::A, passed.data, passed);
            }
            cpu.reg_file.flags = passed.flags;
            pass_to_next_stage.instruction_stage = 0;
	    cpu.pc += 1;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn xor_or(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            let a = cpu.reg_file[Reg8::A];
            let b = match n {
                0xB0 | 0xA8 => cpu.reg_file[Reg8::B],
                0xB1 | 0xA9 => cpu.reg_file[Reg8::C],
                0xB2 | 0xAA => cpu.reg_file[Reg8::D],
                0xB3 | 0xAB => cpu.reg_file[Reg8::E],
                0xB4 | 0xAC => cpu.reg_file[Reg8::H],
                0xB5 | 0xAD => cpu.reg_file[Reg8::L],
                0xB6 | 0xAE => {
                    cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                    cpu.read()
                }
                0xB7 | 0xAF => cpu.reg_file[Reg8::A],
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
            } else {
                load8(&mut cpu.reg_file, Reg8::A, val, passed);
                cpu.reg_file.flags = flags;
		cpu.pc += 1;
            }
        }
        1 => {
            pass_to_next_stage.instruction_stage = 0;
            cpu.pc += 1;
            load8(&mut cpu.reg_file, Reg8::A, passed.data, passed);
            cpu.reg_file.flags = passed.flags;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn and(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            let a = cpu.reg_file[Reg8::A];
            let b = match n {
                0xA0 => cpu.reg_file[Reg8::B],
                0xA1 => cpu.reg_file[Reg8::C],
                0xA2 => cpu.reg_file[Reg8::D],
                0xA3 => cpu.reg_file[Reg8::E],
                0xA4 => cpu.reg_file[Reg8::H],
                0xA5 => cpu.reg_file[Reg8::L],
                0xA6 => {
                    cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                    cpu.read()
                }
                0xA7 => cpu.reg_file[Reg8::A],
                _ => panic!("invalid instruction!"),
            };
            let (val, flags) = and_calc(a, b);
            if n == 0xA6 {
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.data = val;
                pass_to_next_stage.flags = flags;
                
            } else {
                load8(&mut cpu.reg_file, Reg8::A, val, passed);
                cpu.reg_file.flags = flags;
                cpu.pc += 1;
            }
        }
        1 => {
            pass_to_next_stage.instruction_stage = 0;
            cpu.pc += 1;
            load8(&mut cpu.reg_file, Reg8::A, passed.data, passed);
            cpu.reg_file.flags = passed.flags;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn inc(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        // INC instructions
        0 => {
            let a = match n {
                0x04 => cpu.reg_file[Reg8::B],
                0x14 => cpu.reg_file[Reg8::D],
                0x24 => cpu.reg_file[Reg8::H],
                0x34 => {
                    cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                    cpu.read()
                }
                0x0C => cpu.reg_file[Reg8::C],
                0x1C => cpu.reg_file[Reg8::E],
                0x2C => cpu.reg_file[Reg8::L],
                0x3C => cpu.reg_file[Reg8::A],
                _ => panic!("invalid instruction"),
            };
	    let val = a.wrapping_add(1);
            let mut flags = Flags::default();
            flags.Z = if val == 0 { 1 } else { 0 };
            flags.N = 0;
            flags.H = if a & 0x0F == 0x0F { 1 } else { 0 };
	    flags.C = cpu.reg_file.flags.C;
            match n {
                0x04 => cpu.reg_file[Reg8::B] = val,
                0x14 => cpu.reg_file[Reg8::D] = val,
                0x24 => cpu.reg_file[Reg8::H] = val,
                0x34 => (),
                0x0C => cpu.reg_file[Reg8::C] = val,
                0x1C => cpu.reg_file[Reg8::E] = val,
                0x2C => cpu.reg_file[Reg8::L] = val,
                0x3C => cpu.reg_file[Reg8::A] = val,
                _ => panic!("invalid instruction"),
            };
            if n == 0x34 {
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.data = a.wrapping_add(1);
                pass_to_next_stage.flags = flags;
                
            } else {
                cpu.reg_file.flags = flags;
                cpu.pc += 1;
            }
        }
        1 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.instruction_stage = 2;
	    cpu.reg_file.flags = passed.flags;
            
        }
        2 => {
            pass_to_next_stage.instruction_stage = 0;
            cpu.pc += 1;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn inc_rr(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            let reg16 = match n {
                0x03 => Reg16::BC,
                0x13 => Reg16::DE,
                0x23 => Reg16::HL,
                0x33 => Reg16::SP,
                _ => panic!("inc16: invalid opcode"),
            };
            pass_to_next_stage.data16 = cpu.reg_file.get16(reg16);
            
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            let reg16 = match n {
                0x03 => Reg16::BC,
                0x13 => Reg16::DE,
                0x23 => Reg16::HL,
                0x33 => Reg16::SP,
                _ => panic!("inc16: invalid opcode"),
            };
            cpu.reg_file.set16(reg16, passed.data16.wrapping_add(1));
	    cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inc16: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn dec(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        // DEC instructions
        0 => {
            let a = match n {
                0x05 => cpu.reg_file[Reg8::B],
                0x15 => cpu.reg_file[Reg8::D],
                0x25 => cpu.reg_file[Reg8::H],
                0x35 => {
                    cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                    cpu.read()
                }
                0x0D => cpu.reg_file[Reg8::C],
                0x1D => cpu.reg_file[Reg8::E],
                0x2D => cpu.reg_file[Reg8::L],
                0x3D => cpu.reg_file[Reg8::A],
                _ => panic!("invalid instruction"),
            };
	    let val = a.wrapping_sub(1);
            let mut flags = Flags::default();
            flags.Z = if val == 0 { 1 } else { 0 };
            flags.N = 1;
            flags.H = if (a & 0x0F) == 0 { 1 } else { 0 };
	    flags.C = cpu.reg_file.flags.C;
            match n {
                0x05 => cpu.reg_file[Reg8::B] = val,
                0x15 => cpu.reg_file[Reg8::D] = val,
                0x25 => cpu.reg_file[Reg8::H] = val,
                0x35 => (),
                0x0D => cpu.reg_file[Reg8::C] = val,
                0x1D => cpu.reg_file[Reg8::E] = val,
                0x2D => cpu.reg_file[Reg8::L] = val,
                0x3D => cpu.reg_file[Reg8::A] = val,
                _ => panic!("invalid instruction: {:#x}", n),
            };
            if n == 0x35 {
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.data = val;
                pass_to_next_stage.flags = flags;
                
            } else {
                cpu.reg_file.flags = flags;
                cpu.pc += 1;
            }
        }
        1 => {
            pass_to_next_stage.instruction_stage = 2;
            
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
	    cpu.reg_file.flags = passed.flags;
        }
        2 => {
            pass_to_next_stage.instruction_stage = 0;
            cpu.pc += 1;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn dec_rr(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            let reg16 = match n {
                0x0B => Reg16::BC,
                0x1B => Reg16::DE,
                0x2B => Reg16::HL,
                0x3B => Reg16::SP,
                _ => panic!("inc16: invalid opcode"),
            };
            pass_to_next_stage.data16 = cpu.reg_file.get16(reg16);
            
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            let reg16 = match n {
                0x0B => Reg16::BC,
                0x1B => Reg16::DE,
                0x2B => Reg16::HL,
                0x3B => Reg16::SP,
                _ => panic!("inc16: invalid opcode"),
            };
            cpu.reg_file.set16(reg16, passed.data16.wrapping_sub(1));
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inc16: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn pop(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
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
            match lsb_reg {
                Reg8::F => cpu.reg_file.flags = cpu.pop().into(),
                _ => cpu.reg_file[lsb_reg] = cpu.pop(),
            }
            
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
            cpu.reg_file[msb_reg] = cpu.pop();
            
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            pass_to_next_stage.instruction_stage = 0;
            cpu.pc += 1;
        }
        _ => panic!("POP: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn push(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.addr_bus = cpu.reg_file.SP;
            pass_to_next_stage.instruction_stage = 1;
            
        }
        1 => {
            let msb_reg = match n {
                0xC5 => Reg8::B,
                0xD5 => Reg8::D,
                0xE5 => Reg8::H,
                0xF5 => Reg8::A,
                _ => panic!("PUSH: invalid op code"),
            };
            cpu.push(msb_reg);
            
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            let lsb_reg = match n {
                0xC5 => Reg8::C,
                0xD5 => Reg8::E,
                0xE5 => Reg8::L,
                0xF5 => Reg8::F,
                _ => panic!("PUSH: invalid op code"),
            };
            cpu.push(lsb_reg);
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.instruction_stage = 0;
            cpu.pc += 1;
        }
        _ => panic!("PUSH: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn jp_hl(cpu: &mut CPU) -> StagePassThrough {
    let pass_to_next_stage = StagePassThrough::default();
    cpu.pc = cpu.reg_file.get16(Reg16::HL);
    pass_to_next_stage
}

fn ld_sp_hl(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.data16 = cpu.reg_file.get16(Reg16::HL);
            pass_to_next_stage.instruction_stage = 1;
            
        }
        1 => {
            cpu.reg_file.set16(Reg16::SP, passed.data16);
            pass_to_next_stage.instruction_stage = 0;
            cpu.pc += 1;
        }
        _ => panic!("LD SP, HL: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn add(a: u8, b: u8) -> (u8, Flags) {
    let val: u16 = (a as u16).wrapping_add(b as u16);
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

fn add_16(a: u16, b: u16) -> (u16, Flags) {
    let val: u32 = (a as u32).wrapping_add(b as u32);
    let mut flags = Flags::default();
    flags.H = if (((a & 0x0FFF) + (b & 0x0FFF)) & 0x1000) == 0x1000 {
        1
    } else {
        0
    };
    flags.C = if val > 65535 { 1 } else { 0 };
    (val as u16, flags)
}

fn adc(a: u8, b: u8, carry: u8) -> (u8, Flags) {
    if carry > 1 {
        panic!("ADC: carry is greater than one");
    }
    let val = (a as u16) + (b as u16) + (carry as u16);
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
    let val = a.wrapping_sub(b);
    let mut flags = Flags::default();
    flags.N = 1;
    flags.H = if (a & 0x0F).wrapping_sub(b & 0x0F) & 0x10 != 0 {
        1
    } else {
        0
    };
    flags.Z = if val == 0 { 1 } else { 0 };
    flags.C = if a < b { 1 } else { 0 };
    (val, flags)
}

fn subc(a: u8, b: u8, carry: u8) -> (u8, Flags) {
    if carry > 1 {
        panic!("SUBC: carry is greater than one");
    }
    let val = a.wrapping_sub(b).wrapping_sub(carry);
    let mut flags = Flags::default();
    flags.N = 1;
    flags.H = if (a & 0x0F).wrapping_sub(b & 0x0F).wrapping_sub(carry) & 0x10 != 0 {
        1
    } else {
        0
    };
    flags.C = if (a as u16) < (b as u16) + (carry as u16) {
        1
    } else {
        0
    };
    flags.Z = if val == 0 { 1 } else { 0 };
    (val, flags)
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

fn ret(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.data = cpu.pop();
            pass_to_next_stage.instruction_stage = 1;
            
        }
        1 => {
            pass_to_next_stage.data16 = ((cpu.pop() as u16) << 8) | passed.data as u16;
            pass_to_next_stage.instruction_stage = 2;
            
        }
        2 => {
            pass_to_next_stage.data16 = passed.data16;
            pass_to_next_stage.instruction_stage = 3;
            
        }
        3 => {
            cpu.pc = passed.data16;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ret: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn reti(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.data = cpu.pop();
            pass_to_next_stage.instruction_stage = 1;
            
        }
        1 => {
            pass_to_next_stage.data16 = ((cpu.pop() as u16) << 8) | passed.data as u16;
            pass_to_next_stage.instruction_stage = 2;
            
        }
        2 => {
            pass_to_next_stage.data16 = passed.data16;
            pass_to_next_stage.instruction_stage = 3;
            
        }
        3 => {
            cpu.pc = passed.data16;
            pass_to_next_stage.instruction_stage = 0;
            cpu.ime = true;
        }
        _ => panic!("ret: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ret_cc(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    let cc = match n {
        0xC0 => CC::NZ,
        0xD0 => CC::NC,
        0xC8 => CC::Z,
        0xD8 => CC::C,
        _ => panic!("ret_cc: invalid op code: {:#x}", n),
    };
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.instruction_stage = 1;
        }
	1 => {
	    if cpu.reg_file.check_condition(cc) {
                pass_to_next_stage.instruction_stage = 2;
		pass_to_next_stage.data = cpu.pop();
            } else {
                pass_to_next_stage.instruction_stage = 0;
		cpu.pc += 1;
            }
	}
        2 => {
            pass_to_next_stage.data16 = ((cpu.pop() as u16) << 8) | passed.data as u16;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.data16 = passed.data16;
            pass_to_next_stage.instruction_stage = 4;
            
        }
        4 => {
            cpu.pc = passed.data16;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ret: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ldh_n_a(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.addr_bus = 0xFF00 | passed.data as u16;
            cpu.write_data(cpu.reg_file[Reg8::A]);
            pass_to_next_stage.instruction_stage = 2;
            
        }
        2 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ldh_n_a: invalid instruction stage"),
    }

    pass_to_next_stage
}

fn ldh_a_n(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.addr_bus = 0xFF00 | passed.data as u16;
            cpu.reg_file[Reg8::A] = cpu.read();
            pass_to_next_stage.instruction_stage = 2;
            
        }
        2 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ldh_n_a: invalid instruction stage"),
    }

    pass_to_next_stage
}

fn jp_nn(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data16 = ((cpu.read() as u16) << 8) | (passed.data as u16);
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            let cc = match n {
                0xC2 => CC::NZ,
                0xD2 => CC::NC,
                0xC3 => CC::UC,
                0xCA => CC::Z,
                0xDA => CC::C,
                _ => panic!("jp_nn: invalid opcode"),
            };
            if cpu.reg_file.check_condition(cc) {
                pass_to_next_stage.data16 = passed.data16;
                
                pass_to_next_stage.instruction_stage = 3;
            } else {
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            }
        }
        3 => {
            cpu.pc = passed.data16;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("jp_nn: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn load8(
    reg_file: &mut RegFile,
    reg: Reg8,
    value: u8,
    _passed: StagePassThrough,
) -> StagePassThrough {
    let pass_to_next_stage = StagePassThrough::default();
    reg_file[reg] = value;
    pass_to_next_stage
}

fn load16(cpu: &mut CPU, reg: Reg16, value: u16) {
    let hi = (value >> 8) as u8;
    let lo = value as u8;
    match reg {
        Reg16::AF => {
            cpu.reg_file.A = hi;
            cpu.reg_file.flags = lo.into();
        }
        Reg16::BC => {
            cpu.reg_file.B = hi;
            cpu.reg_file.C = lo;
        }
        Reg16::DE => {
            cpu.reg_file.D = hi;
            cpu.reg_file.E = lo;
        }
        Reg16::HL => {
            cpu.reg_file.H = hi;
            cpu.reg_file.L = lo;
        }
        // Reg16::PC => cpu.reg_file.PC = value,
        Reg16::SP => cpu.reg_file.SP = value,
    }
}

fn ldh_c_a(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.addr_bus = 0xFF00 | cpu.reg_file[Reg8::C] as u16;
            cpu.write_data(cpu.reg_file[Reg8::A]);
            
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ldh_c_a: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ldh_a_c(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.addr_bus = 0xFF00 | cpu.reg_file[Reg8::C] as u16;
            cpu.reg_file[Reg8::A] = cpu.read();
            
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ldh_a_c: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn call_nn(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data16 = ((cpu.read() as u16) << 8) | (passed.data as u16);
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            let cc = match n {
                0xC4 => CC::NZ,
                0xD4 => CC::NC,
                0xCC => CC::Z,
                0xDC => CC::C,
                0xCD => CC::UC,
                _ => panic!("call_nn: invalid opcode"),
            };

            cpu.pc += 1;
            if cpu.reg_file.check_condition(cc) {
                pass_to_next_stage.data16 = passed.data16;
                pass_to_next_stage.instruction_stage = 3;
            } else {
                pass_to_next_stage.instruction_stage = 0;
            }
        }
        3 => {
            cpu.push_val((cpu.pc >> 8) as u8);
            pass_to_next_stage.instruction_stage = 4;
            
            pass_to_next_stage.data16 = passed.data16;
        }
        4 => {
            cpu.push_val(cpu.pc as u8);
            pass_to_next_stage.instruction_stage = 5;
            
            pass_to_next_stage.data16 = passed.data16;
        }
        5 => {
            cpu.pc = passed.data16;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rst(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.instruction_stage = 2;
            cpu.push_val((cpu.pc >> 8) as u8);
	}
        2 => {
            pass_to_next_stage.instruction_stage = 3;
            cpu.push_val(cpu.pc as u8);
        }
        3 => {
            cpu.pc = match n {
                0xC7 => 0x00,
                0xD7 => 0x10,
                0xE7 => 0x20,
                0xF7 => 0x30,
                0xCF => 0x08,
                0xDF => 0x18,
                0xEF => 0x28,
                0xFF => 0x38,
                _ => panic!("rst: invalid opcode"),
            };
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn alu_imm(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            let imm = passed.data;
            let tup = match n {
                0xC6 => add(cpu.reg_file[Reg8::A], imm),
                0xD6 => sub(cpu.reg_file[Reg8::A], imm),
                0xE6 => and_calc(cpu.reg_file[Reg8::A], imm),
                0xF6 => or(cpu.reg_file[Reg8::A], imm),
                0xCE => adc(cpu.reg_file[Reg8::A], imm, cpu.reg_file.flags.C),
                0xDE => subc(cpu.reg_file[Reg8::A], imm, cpu.reg_file.flags.C),
                0xEE => xor(cpu.reg_file[Reg8::A], imm),
                0xFE => sub(cpu.reg_file[Reg8::A], imm),
                _ => panic!("alu_imm: invalid opcode"),
            };
            if n != 0xFE {
                cpu.reg_file[Reg8::A] = tup.0;
            }
            cpu.reg_file.flags = tup.1;
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn add_sp_e(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.data = passed.data;
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            let a = cpu.reg_file.get16(Reg16::SP);
            let b = passed.data as i8 as i16 as u16;
	    let val = a.wrapping_add(b);
	    let mut flags = Flags::default();
	    flags.H = if (((a & 0x0F) + (b & 0x0F)) & 0x10) == 0x10 {
		1
	    } else {
		0
	    };
	    flags.C = if (((a & 0xFF) + (b & 0xFF)) & 0x100) == 0x100 {
		1
	    } else {
		0
	    };
	    flags.Z = 0;
	    flags.N = 0;
            cpu.reg_file.flags = flags;
            cpu.reg_file.set16(Reg16::SP, val);
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_hl_sp_e(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
	    let a = cpu.reg_file.get16(Reg16::SP);
            let b = passed.data as i8 as i16 as u16;
	    let val = a.wrapping_add(b);
	    let mut flags = Flags::default();
	    flags.H = if (((a & 0x0F) + (b & 0x0F)) & 0x10) == 0x10 {
		1
	    } else {
		0
	    };
	    flags.C = if (((a & 0xFF) + (b & 0xFF)) & 0x100) == 0x100 {
		1
	    } else {
		0
	    };
	    flags.Z = 0;
	    flags.N = 0;
            cpu.reg_file.flags = flags;
            pass_to_next_stage.data16 = val;
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            cpu.reg_file.set16(Reg16::HL, passed.data16);
            pass_to_next_stage.instruction_stage = 0;
            cpu.pc += 1;
        }
        _ => panic!("ld_hl_sp_e: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_nn_sp(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
	0 => {
	    pass_to_next_stage.instruction_stage = 1;
	}
        1 => {
	    cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data16 = cpu.read() as u16;
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
	    cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data16 = ((cpu.read() as u16) << 8) | passed.data16;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.addr_bus = passed.data16;
            cpu.write_data(cpu.reg_file.get16(Reg16::SP) as u8);
            pass_to_next_stage.data16 = passed.data16 + 1;
            
            pass_to_next_stage.instruction_stage = 4;
        }
        4 => {
            cpu.addr_bus = passed.data16;
            cpu.write_data((cpu.reg_file.get16(Reg16::SP) >> 8) as u8);
            pass_to_next_stage.instruction_stage = 0;
            cpu.pc += 1;
        }
        _ => panic!("ld_nn_sp: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_nn_a(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data16 = ((cpu.read() as u16) << 8) | (passed.data as u16);
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            cpu.addr_bus = passed.data16;
            cpu.write_data(cpu.reg_file[Reg8::A]);
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_a_nn(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.data16 = ((cpu.read() as u16) << 8) | (passed.data as u16);
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            cpu.addr_bus = passed.data16;
            cpu.reg_file[Reg8::A] = cpu.read();
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ld_a_nn: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn cb_op(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.pc += 1;
            cpu.addr_bus = cpu.pc;
            pass_to_next_stage.cb_op = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1..=3 => {
            pass_to_next_stage = match passed.cb_op {
                n @ (0x00..=0x07) => rlc(cpu, n, passed),
                n @ (0x08..=0x0F) => rrc(cpu, n, passed),
                n @ (0x10..=0x17) => rl(cpu, n, passed),
                n @ (0x18..=0x1F) => rr(cpu, n, passed),
                n @ (0x20..=0x27) => sla(cpu, n, passed),
                n @ (0x28..=0x2F) => sra(cpu, n, passed),
                n @ (0x30..=0x37) => swap(cpu, n, passed),
                n @ (0x38..=0x3F) => srl(cpu, n, passed),
                n @ (0x40..=0x7F) => bit(cpu, n, passed),
                n @ (0x80..=0xBF) => res(cpu, n, passed),
                n @ (0xC0..=0xFF) => set(cpu, n, passed),
            };
	    pass_to_next_stage.cb_op = passed.cb_op;
        }
        _ => panic!("cb_op: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rlc(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            if n != 0x06 {
                let reg8 = match n {
                    0x00 => Reg8::B,
                    0x01 => Reg8::C,
                    0x02 => Reg8::D,
                    0x03 => Reg8::E,
                    0x04 => Reg8::H,
                    0x05 => Reg8::L,
                    0x07 => Reg8::A,
                    _ => panic!("rlc: invalid op"),
                };
                flags.C = cpu.reg_file[reg8] >> 7;
                cpu.reg_file[reg8] = cpu.reg_file[reg8].rotate_left(1);
                flags.Z = if cpu.reg_file[reg8] == 0 { 1 } else { 0 };
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.C = val >> 7;
                pass_to_next_stage.data = val.rotate_left(1);
                flags.Z = if pass_to_next_stage.data == 0 { 1 } else { 0 };
            }
            cpu.reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);

            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rrc(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            if n != 0x0E {
                let reg8 = match n {
                    0x08 => Reg8::B,
                    0x09 => Reg8::C,
                    0x0A => Reg8::D,
                    0x0B => Reg8::E,
                    0x0C => Reg8::H,
                    0x0D => Reg8::L,
                    0x0F => Reg8::A,
                    _ => panic!("rlc: invalid op"),
                };
                flags.C = cpu.reg_file[reg8] & 1;
                cpu.reg_file[reg8] = cpu.reg_file[reg8].rotate_right(1);
                flags.Z = if cpu.reg_file[reg8] == 0 { 1 } else { 0 };
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                cpu.reg_file.flags.C = val & 1;
                pass_to_next_stage.data = val.rotate_right(1);
                flags.Z = if pass_to_next_stage.data == 0 { 1 } else { 0 };
            }
            cpu.reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rl(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            if n != 0x16 {
                let reg8 = match n {
                    0x10 => Reg8::B,
                    0x11 => Reg8::C,
                    0x12 => Reg8::D,
                    0x13 => Reg8::E,
                    0x14 => Reg8::H,
                    0x15 => Reg8::L,
                    0x17 => Reg8::A,
                    _ => panic!("rlc: invalid op"),
                };
                let carry = cpu.reg_file.flags.C;
                flags.C = cpu.reg_file[reg8] >> 7;
                cpu.reg_file[reg8] = (cpu.reg_file[reg8] << 1) | carry;
                flags.Z = if cpu.reg_file[reg8] == 0 { 1 } else { 0 };
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                
                pass_to_next_stage.instruction_stage = 2;
                let carry = cpu.reg_file.flags.C;
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.C = val >> 7;
                pass_to_next_stage.data = (val << 1) | carry;
                flags.Z = if pass_to_next_stage.data == 0 { 1 } else { 0 };
            }
            cpu.reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rr(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            if n != 0x1E {
                let reg8 = match n {
                    0x18 => Reg8::B,
                    0x19 => Reg8::C,
                    0x1A => Reg8::D,
                    0x1B => Reg8::E,
                    0x1C => Reg8::H,
                    0x1D => Reg8::L,
                    0x1F => Reg8::A,
                    _ => panic!("rlc: invalid op"),
                };
                let carry = cpu.reg_file.flags.C;
                flags.C = cpu.reg_file[reg8] & 1;
                cpu.reg_file[reg8] = (cpu.reg_file[reg8] >> 1) | (carry << 7);
                flags.Z = if cpu.reg_file[reg8] == 0 { 1 } else { 0 };
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                
                pass_to_next_stage.instruction_stage = 2;
                let carry = cpu.reg_file.flags.C;
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.C = val & 1;
                pass_to_next_stage.data = (val >> 1) | (carry << 7);
                flags.Z = if pass_to_next_stage.data == 0 { 1 } else { 0 };
            }
            cpu.reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn sla(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            if n != 0x26 {
                let reg8 = match n {
                    0x20 => Reg8::B,
                    0x21 => Reg8::C,
                    0x22 => Reg8::D,
                    0x23 => Reg8::E,
                    0x24 => Reg8::H,
                    0x25 => Reg8::L,
                    0x27 => Reg8::A,
                    _ => panic!("rlc: invalid op"),
                };
                flags.C = cpu.reg_file[reg8] >> 7;
                cpu.reg_file[reg8] = cpu.reg_file[reg8] << 1;
                flags.Z = if cpu.reg_file[reg8] == 0 { 1 } else { 0 };
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                cpu.reg_file.flags.C = val >> 7;
                pass_to_next_stage.data = val << 1;
                flags.Z = if pass_to_next_stage.data == 0 { 1 } else { 0 };
            }
            cpu.reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn sra(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            if n != 0x2E {
                let reg8 = match n {
                    0x28 => Reg8::B,
                    0x29 => Reg8::C,
                    0x2A => Reg8::D,
                    0x2B => Reg8::E,
                    0x2C => Reg8::H,
                    0x2D => Reg8::L,
                    0x2F => Reg8::A,
                    _ => panic!("rlc: invalid op"),
                };
                flags.C = cpu.reg_file[reg8] & 1;
		let val = (cpu.reg_file[reg8] & 0x80) | (cpu.reg_file[reg8] >> 1);
                cpu.reg_file[reg8] = val;
                if val == 0 {
                    flags.Z = 1;
                }
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.C = val & 1;
		let new_val = (val & 0x80) | (val >> 1);
                pass_to_next_stage.data = new_val;
                if new_val == 0 {
                    flags.Z = 1;
                }
            }
	    flags.H = 0;
	    flags.N = 0;
            cpu.reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
	    cpu.reg_file.flags = passed.flags;
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn swap(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            if n != 0x36 {
                let reg8 = match n {
                    0x30 => Reg8::B,
                    0x31 => Reg8::C,
                    0x32 => Reg8::D,
                    0x33 => Reg8::E,
                    0x34 => Reg8::H,
                    0x35 => Reg8::L,
                    0x37 => Reg8::A,
                    _ => panic!("rlc: invalid op"),
                };
                cpu.reg_file[reg8] =
                    ((cpu.reg_file[reg8] & 0x0F) << 4) | ((cpu.reg_file[reg8] & 0xF0) >> 4);
                if cpu.reg_file[reg8] == 0 {
                    flags.Z = 1;
                }
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                pass_to_next_stage.data = ((val & 0x0F) << 4) | ((val & 0xF0) >> 4);
                if (val as i8 >> 1) == 0 {
                    flags.Z = 1;
                }
            }
            cpu.reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn srl(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            if n != 0x3E {
                let reg8 = match n {
                    0x38 => Reg8::B,
                    0x39 => Reg8::C,
                    0x3A => Reg8::D,
                    0x3B => Reg8::E,
                    0x3C => Reg8::H,
                    0x3D => Reg8::L,
                    0x3F => Reg8::A,
                    _ => panic!("srl invalid op"),
                };
                flags.C = cpu.reg_file[reg8] & 1;
		let val = cpu.reg_file[reg8] >> 1;
                cpu.reg_file[reg8] = val;
                if val == 0 {
                    flags.Z = 1;
                }
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.C = val & 1;
		let new_val = val >> 1;
                pass_to_next_stage.data = val;
                if new_val == 0 {
                    flags.Z = 1;
                }
            }
	    flags.H = 0;
	    flags.N = 0;
            cpu.reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn bit(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            let lower_nibble = n & 0x0F;
            let bit_num = match n {
                0x40..=0x47 => 0,
                0x48..=0x4F => 1,
                0x50..=0x57 => 2,
                0x58..=0x5F => 3,
                0x60..=0x67 => 4,
                0x68..=0x6F => 5,
                0x70..=0x77 => 6,
                0x78..=0x7F => 7,
                _ => panic!("invalid opcode bit"),
            };
            if lower_nibble != 0xE && lower_nibble != 0x6 {
                let reg8 = match lower_nibble {
                    0x0 | 0x8 => Reg8::B,
                    0x1 | 0x9 => Reg8::C,
                    0x2 | 0xA => Reg8::D,
                    0x3 | 0xB => Reg8::E,
                    0x4 | 0xC => Reg8::H,
                    0x5 | 0xD => Reg8::L,
                    0x7 | 0xF => Reg8::A,
                    _ => panic!("invalid lower nibble"),
                };
                let mask = 1 << bit_num;
                flags.Z = if (cpu.reg_file[reg8] & mask) == 0 {
                    1
                } else {
                    0
                };
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                let mask = 1 << bit_num;
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.Z = if (val & mask) == 0 { 1 } else { 0 };
                
                pass_to_next_stage.instruction_stage = 2;
            }
	    flags.N = 0;
	    flags.H = 1;
	    flags.C = cpu.reg_file.flags.C;
            cpu.reg_file.flags = flags;
        }
        2 => {
	    cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn res(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let lower_nibble = n & 0x0F;
            let bit_num = match n {
                0x80..=0x87 => 0,
                0x88..=0x8F => 1,
                0x90..=0x97 => 2,
                0x98..=0x9F => 3,
                0xA0..=0xA7 => 4,
                0xA8..=0xAF => 5,
                0xB0..=0xB7 => 6,
                0xB8..=0xBF => 7,
                _ => panic!("invalid opcode bit: {:#x}", n),
            };
            if lower_nibble != 0xE && lower_nibble != 0x6 {
                let reg8 = match lower_nibble {
                    0x0 | 0x8 => Reg8::B,
                    0x1 | 0x9 => Reg8::C,
                    0x2 | 0xA => Reg8::D,
                    0x3 | 0xB => Reg8::E,
                    0x4 | 0xC => Reg8::H,
                    0x5 | 0xD => Reg8::L,
                    0x7 | 0xF => Reg8::A,
                    _ => panic!("invalid lower nibble"),
                };
                let mask = !(1 << bit_num);
                cpu.reg_file[reg8] = cpu.reg_file[reg8] & mask;
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                let mask = !(1 << bit_num);
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                pass_to_next_stage.data = val & mask;
                
                pass_to_next_stage.instruction_stage = 2;
            }
        }
        2 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn set(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let lower_nibble = n & 0x0F;
            let bit_num = match n {
                0xC0..=0xC7 => 0,
                0xC8..=0xCF => 1,
                0xD0..=0xD7 => 2,
                0xD8..=0xDF => 3,
                0xE0..=0xE7 => 4,
                0xE8..=0xEF => 5,
                0xF0..=0xF7 => 6,
                0xF8..=0xFF => 7,
                _ => panic!("invalid opcode bit"),
            };
            if lower_nibble != 0xE && lower_nibble != 0x6 {
                let reg8 = match lower_nibble {
                    0x0 | 0x8 => Reg8::B,
                    0x1 | 0x9 => Reg8::C,
                    0x2 | 0xA => Reg8::D,
                    0x3 | 0xB => Reg8::E,
                    0x4 | 0xC => Reg8::H,
                    0x5 | 0xD => Reg8::L,
                    0x7 | 0xF => Reg8::A,
                    _ => panic!("invalid lower nibble"),
                };
                let mask = 1 << bit_num;
                cpu.reg_file[reg8] = cpu.reg_file[reg8] | mask;
                cpu.pc += 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                let mask = 1 << bit_num;
                cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
                let val = cpu.read();
                pass_to_next_stage.data = val | mask;
                
                pass_to_next_stage.instruction_stage = 2;
            }
        }
        2 => {
            cpu.addr_bus = cpu.reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            cpu.pc += 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn func_template(
    _reg_file: &mut RegFile,
    _cpu: &mut CPU,
    _n: u8,
    passed: StagePassThrough,
) -> StagePassThrough {
    let pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {}
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}
