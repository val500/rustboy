use crate::cpu::{Flags, Reg16, Reg8, RegFile, CPU};

use std::io;
#[derive(Clone, Copy, Default, Debug)]
pub struct StagePassThrough {
    data: u8,
    data16: u16,
    flags: Flags,
    pub ei: bool,
    pub di: bool,
    pub next_pc: u16,
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
    //println!("opcode: {:#x}, stage: {}", opcode, passed.instruction_stage);
    match opcode {
        0x00 => {
            let mut pass_to_next_stage = StagePassThrough::default();
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage
        } //NOOP
        n @ (0x20 | 0x30 | 0x18 | 0x28 | 0x38) => jr_cc_e(cpu, n, passed),
        n @ (0x01 | 0x11 | 0x21 | 0x31) => ld_rr_nn(cpu, n, passed),
        n @ (0x02 | 0x12 | 0x22 | 0x32) => ld_16_a(cpu, n, passed),
        n @ (0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C) => inc(cpu, passed, n),
        n @ (0x03 | 0x13 | 0x23 | 0x33) => inc_rr(reg_file, n, passed),
        n @ (0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D) => dec(cpu, passed, n),
        n @ (0x0B | 0x1B | 0x2B | 0x3B) => dec_rr(reg_file, n, passed),
        n @ (0x06 | 0x16 | 0x26 | 0x0E | 0x1E | 0x2E | 0x3E) => ld_r_n(cpu, n, passed),

        n @ (0x0A | 0x1A) => ld_a_16(cpu, n, passed),
        0x07 => rlca(reg_file, passed),
        0x17 => rla(reg_file, passed),
        0x0F => rrca(reg_file, passed),
        0x1F => rra(reg_file, passed),
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

        n @ (0x70..=0x75 | 0x77) => ld_hl_r(cpu, passed, n),
        0x76 => {
            let mut pass_to_next_stage = StagePassThrough::default();
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage
        } //HALT

        0x78 => load8(reg_file, Reg8::A, reg_file.B, passed),
        0x79 => load8(reg_file, Reg8::A, reg_file.C, passed),
        0x7A => load8(reg_file, Reg8::A, reg_file.D, passed),
        0x7B => load8(reg_file, Reg8::A, reg_file.E, passed),
        0x7C => load8(reg_file, Reg8::A, reg_file.H, passed),
        0x7D => load8(reg_file, Reg8::A, reg_file.L, passed),
        0x7F => load8(reg_file, Reg8::A, reg_file.A, passed),
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
        0xE9 => jp_hl(reg_file),
        0xEA => ld_nn_a(cpu, passed),
        0xF8 => ld_hl_sp_e(cpu, passed),
        0xF9 => ld_sp_hl(reg_file, passed),
	0xF3 => di(passed),
	0xFb => ei(passed),
        _ => panic!("Invalid instruction: {:#x}", opcode),
    }
}

fn di(passed: StagePassThrough) -> StagePassThrough {
    StagePassThrough {
	di: true,
	next_pc: passed.next_pc + 1,
	..Default::default()
    }
}

fn ei(passed: StagePassThrough) -> StagePassThrough {
    StagePassThrough {
	ei: true,
	next_pc: passed.next_pc + 1,
	..Default::default()
    }
}

fn jr_cc_e(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
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
            if reg_file.check_condition(cc) {
                pass_to_next_stage.instruction_stage = 2;
                pass_to_next_stage.data = passed.data;
            } else {
                pass_to_next_stage.instruction_stage = 0;
            }
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        2 => {
            pass_to_next_stage.next_pc = passed.next_pc + passed.data as u16;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("jr_cc_ee: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_rr_nn(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            let val = ((cpu.read() as u16) << 8) | passed.data as u16;
            let reg16 = match n {
                0x01 => Reg16::BC,
                0x11 => Reg16::DE,
                0x21 => Reg16::HL,
                0x31 => Reg16::SP,
                _ => panic!("ld_rr_nn: invalid opcode"),
            };
            reg_file.set16(reg16, val);
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ld_rr_nn: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_16_a(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
            cpu.addr_bus = reg_file.get16(reg16);
            cpu.write_data(reg_file[Reg8::A]);
            pass_to_next_stage.instruction_stage = 1;
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        1 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
            if n == 0x22 {
                cpu.reg_file.set16(Reg16::HL, cpu.reg_file.get16(Reg16::HL) + 1);
            } else if n == 0x32 {
                cpu.reg_file.set16(Reg16::HL, cpu.reg_file.get16(Reg16::HL) - 1);
            }
        }
        _ => panic!("ld BC/DE a: invalid instruction stage"),
    };
    pass_to_next_stage
}

fn ld_a_16(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            let reg16 = match n {
                0x0A => Reg16::BC,
                0x1A => Reg16::DE,
                0x2A | 0x3A => Reg16::HL,
                _ => panic!("invalid opcode"),
            };
            cpu.addr_bus = reg_file.get16(reg16);
            reg_file[Reg8::A] = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        1 => {
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;

            if n == 0x2A {
                reg_file.set16(Reg16::HL, reg_file.get16(Reg16::HL) + 1)
            } else if n == 0x3A {
                reg_file.set16(Reg16::HL, reg_file.get16(Reg16::HL) - 1)
            }
        }
        _ => panic!("ld_a_16: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_r_n(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = passed.next_pc + 1;
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
            reg_file[reg8] = passed.data;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ld_r_n: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_hl_n(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = passed.next_pc + 1;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ld_hl_n: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rlca(reg_file: &mut RegFile, passed: StagePassThrough) -> StagePassThrough {
    reg_file.flags.C = reg_file[Reg8::A] >> 7;
    reg_file[Reg8::A] = reg_file[Reg8::A].rotate_left(1);
    StagePassThrough {
        next_pc: passed.next_pc + 1,
        ..Default::default()
    }
}

fn rrca(reg_file: &mut RegFile, passed: StagePassThrough) -> StagePassThrough {
    reg_file.flags.C = reg_file[Reg8::A] & 1;
    reg_file[Reg8::A] = reg_file[Reg8::A].rotate_right(1);
    StagePassThrough {
        next_pc: passed.next_pc + 1,
        ..Default::default()
    }
}

fn rla(reg_file: &mut RegFile, passed: StagePassThrough) -> StagePassThrough {
    let carry = reg_file.flags.C;
    reg_file.flags.C = reg_file[Reg8::A] >> 7;
    reg_file[Reg8::A] = (reg_file[Reg8::A] << 1) | carry;
    StagePassThrough {
        next_pc: passed.next_pc + 1,
        ..Default::default()
    }
}

fn rra(reg_file: &mut RegFile, passed: StagePassThrough) -> StagePassThrough {
    let carry = reg_file.flags.C;
    reg_file.flags.C = reg_file[Reg8::A] & 1;
    reg_file[Reg8::A] = (reg_file[Reg8::A] >> 1) | (carry << 7);
    StagePassThrough {
        next_pc: passed.next_pc + 1,
        ..Default::default()
    }
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

fn ld_hl_r(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let reg_file = cpu.reg_file;
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
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => panic!("0x70-0x75: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn add_adc(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                    cpu.addr_bus = reg_file.get16(Reg16::HL);
                    cpu.read()
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
                load8(&mut reg_file, Reg8::A, val, passed);
                reg_file.flags = flags;
                pass_to_next_stage.next_pc = passed.next_pc + 1;
            }
        }
        1 => {
            load8(&mut reg_file, Reg8::A, passed.data, passed);
            reg_file.flags = passed.flags;
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => (),
    };
    pass_to_next_stage
}

fn sub_sbc_cp(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                    cpu.addr_bus = reg_file.get16(Reg16::HL);
                    cpu.read()
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
                    load8(&mut reg_file, Reg8::A, val as u8, passed);
                    reg_file.flags = flags;
                    pass_to_next_stage.next_pc = passed.next_pc + 1;
                }
                _ => (),
            }
        }
        1 => {
            if n != 0xBE {
                load8(&mut reg_file, Reg8::A, passed.data, passed);
            }
            reg_file.flags = passed.flags;
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn xor_or(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                    cpu.addr_bus = reg_file.get16(Reg16::HL);
                    cpu.read()
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
                load8(&mut reg_file, Reg8::A, val, passed);
                reg_file.flags = flags;
                pass_to_next_stage.next_pc = passed.next_pc + 1;
            }
        }
        1 => {
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            load8(&mut reg_file, Reg8::A, passed.data, passed);
            reg_file.flags = passed.flags;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn and(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                    cpu.addr_bus = reg_file.get16(Reg16::HL);
                    cpu.read()
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
                load8(&mut reg_file, Reg8::A, val, passed);
                reg_file.flags = flags;
                pass_to_next_stage.next_pc = passed.next_pc + 1;
            }
        }
        1 => {
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            load8(&mut reg_file, Reg8::A, passed.data, passed);
            reg_file.flags = passed.flags;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn inc(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        // INC instructions
        0 => {
            let a = match n {
                0x04 => reg_file[Reg8::B],
                0x14 => reg_file[Reg8::D],
                0x24 => reg_file[Reg8::H],
                0x34 => {
                    cpu.addr_bus = reg_file.get16(Reg16::HL);
                    cpu.read()
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
                0x04 => load8(&mut reg_file, Reg8::B, a + 1, passed),
                0x14 => load8(&mut reg_file, Reg8::D, a + 1, passed),
                0x24 => load8(&mut reg_file, Reg8::H, a + 1, passed),
                0x0C => load8(&mut reg_file, Reg8::C, a + 1, passed),
                0x1C => load8(&mut reg_file, Reg8::E, a + 1, passed),
                0x2C => load8(&mut reg_file, Reg8::L, a + 1, passed),
                0x3C => load8(&mut reg_file, Reg8::A, a + 1, passed),
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
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
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

fn inc_rr(reg_file: &mut RegFile, n: u8, passed: StagePassThrough) -> StagePassThrough {
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
            pass_to_next_stage.data16 = reg_file.get16(reg16);
            pass_to_next_stage.next_pc = passed.next_pc;
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
            reg_file.set16(reg16, passed.data16 + 1);
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inc16: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn dec(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        // DEC instructions
        0 => {
            let a = match n {
                0x05 => reg_file[Reg8::B],
                0x15 => reg_file[Reg8::D],
                0x25 => reg_file[Reg8::H],
                0x35 => {
                    cpu.addr_bus = reg_file.get16(Reg16::HL);
                    cpu.read()
                }
                0x0D => reg_file[Reg8::C],
                0x1D => reg_file[Reg8::E],
                0x2D => reg_file[Reg8::L],
                0x3D => reg_file[Reg8::A],
                _ => panic!("invalid instruction"),
            };
            let mut flags = Flags::default();
            flags.Z = if (a.wrapping_sub(1)) == 0 { 1 } else { 0 };
            flags.N = 0;
            flags.H = if (a & 0x07) == 0 { 1 } else { 0 };

            match n {
                0x05 => load8(&mut reg_file, Reg8::B, a - 1, passed),
                0x15 => load8(&mut reg_file, Reg8::D, a - 1, passed),
                0x25 => load8(&mut reg_file, Reg8::H, a - 1, passed),
                0x0D => load8(&mut reg_file, Reg8::C, a - 1, passed),
                0x1D => load8(&mut reg_file, Reg8::E, a - 1, passed),
                0x2D => load8(&mut reg_file, Reg8::L, a - 1, passed),
                0x3D => load8(&mut reg_file, Reg8::A, a - 1, passed),
                _ => panic!("invalid instruction"),
            };
            if n == 0x35 {
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.data = a.wrapping_sub(1);
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
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
        }
        2 => {
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => (),
    }
    pass_to_next_stage
}

fn dec_rr(reg_file: &mut RegFile, n: u8, passed: StagePassThrough) -> StagePassThrough {
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
            pass_to_next_stage.data16 = reg_file.get16(reg16);
            pass_to_next_stage.next_pc = passed.next_pc;
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
            reg_file.set16(reg16, passed.data16 - 1);
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inc16: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn pop(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
            reg_file[lsb_reg] = cpu.pop();
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
            reg_file[msb_reg] = cpu.pop();
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

fn push(cpu: &mut CPU, passed: StagePassThrough, n: u8) -> StagePassThrough {
    let reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
<<<<<<< Updated upstream
            cpu.addr_bus = reg_file.SP;
=======
>>>>>>> Stashed changes
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
            cpu.push(msb_reg);
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
            cpu.push(lsb_reg);
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

fn ret(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.data = cpu.pop();
            pass_to_next_stage.instruction_stage = 1;
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        1 => {
            pass_to_next_stage.data16 = (cpu.pop() << 8) as u16 | passed.data as u16;
            pass_to_next_stage.instruction_stage = 2;
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        2 => {
            pass_to_next_stage.data16 = passed.data16;
            pass_to_next_stage.instruction_stage = 3;
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.data16;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ret: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ret_cc(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    let cc = match n {
        0x0C => CC::NZ,
        0x0D => CC::NC,
        0x8C => CC::Z,
        0x8D => CC::C,
        _ => panic!("ret_cc: invalid op code"),
    };
    match passed.instruction_stage {
        0 => {
            if reg_file.check_condition(cc) {
                pass_to_next_stage.data = cpu.pop();
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.next_pc = passed.next_pc;
            } else {
                pass_to_next_stage.instruction_stage = 1;
                pass_to_next_stage.next_pc = passed.next_pc;
            }
        }
        1 => {
            if reg_file.check_condition(cc) {
                pass_to_next_stage.data16 = (cpu.pop() << 8) as u16 | passed.data as u16;
                pass_to_next_stage.instruction_stage = 2;
                pass_to_next_stage.next_pc = passed.next_pc;
            } else {
                pass_to_next_stage.instruction_stage = 0;
                pass_to_next_stage.next_pc = passed.next_pc + 1;
            }
        }
        2 => {
            pass_to_next_stage.data16 = passed.data16;
            pass_to_next_stage.instruction_stage = 3;
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.data16;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ret: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ldh_n_a(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.addr_bus = 0xFF00 | passed.data as u16;
<<<<<<< Updated upstream
            cpu.write_data(reg_file[Reg8::A]);
=======
	    println!("addr: {:#x}", cpu.addr_bus);
            cpu.write_data(cpu.reg_file[Reg8::A]);
>>>>>>> Stashed changes
            pass_to_next_stage.instruction_stage = 2;
            pass_to_next_stage.next_pc = passed.next_pc;

	    let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
        }
        2 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ldh_n_a: invalid instruction stage"),
    }

    pass_to_next_stage
}

fn ldh_a_n(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            cpu.addr_bus = 0xFF00 | passed.data as u16;
            reg_file[Reg8::A] = cpu.read();
            pass_to_next_stage.instruction_stage = 2;
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        2 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ldh_n_a: invalid instruction stage"),
    }

    pass_to_next_stage
}

fn jp_nn(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
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
            if reg_file.check_condition(cc) {
                pass_to_next_stage.data16 = passed.data16;
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 3;
            } else {
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            }
        }
        3 => {
            pass_to_next_stage.next_pc = passed.data16;
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

fn ldh_c_a(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.addr_bus = 0xFF00 | reg_file[Reg8::C] as u16;
            cpu.write_data(reg_file[Reg8::A]);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ldh_c_a: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ldh_a_c(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            cpu.addr_bus = 0xFF00 | reg_file[Reg8::C] as u16;
            reg_file[Reg8::A] = cpu.read();
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("ldh_a_c: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn call_nn(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
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

            pass_to_next_stage.next_pc = passed.next_pc + 1;
            if cpu.reg_file.check_condition(cc) {
                pass_to_next_stage.data16 = passed.data16;
                pass_to_next_stage.instruction_stage = 3;
            } else {
                pass_to_next_stage.instruction_stage = 0;
            }
        }
        3 => {
            cpu.push_val((passed.next_pc >> 8) as u8);
            pass_to_next_stage.instruction_stage = 4;
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.data16 = passed.data16;
        }
        4 => {
            cpu.push_val(passed.next_pc as u8);
            pass_to_next_stage.instruction_stage = 5;
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.data16 = passed.data16;
        }
        5 => {
            pass_to_next_stage.next_pc = passed.data16;
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
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.instruction_stage = 2;
            cpu.push_val((passed.next_pc >> 8) as u8);
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        2 => {
            pass_to_next_stage.instruction_stage = 3;
            cpu.push_val(passed.next_pc as u8);
            pass_to_next_stage.next_pc = passed.next_pc;
        }
        3 => {
            pass_to_next_stage.next_pc = match n {
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
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            let imm = passed.data;
            let tup = match n {
                0xC6 => add(reg_file[Reg8::A], imm),
                0xD6 => sub(reg_file[Reg8::A], imm),
                0xE6 => and_calc(reg_file[Reg8::A], imm),
                0xF6 => or(reg_file[Reg8::A], imm),
                0xCE => adc(reg_file[Reg8::A], imm, reg_file.flags.C),
                0xDE => subc(reg_file[Reg8::A], imm, reg_file.flags.C),
                0xEE => xor(reg_file[Reg8::A], imm),
                0xFE => sub(reg_file[Reg8::A], imm),
                _ => panic!("alu_imm: invalid opcode"),
            };
            if n != 0xFE {
                reg_file[Reg8::A] = tup.0;
            }
            reg_file.flags = tup.1;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn add_sp_e(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.data = passed.data;
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            let mut flags = Flags::default();
            let a = reg_file.get16(Reg16::SP);
            let b = passed.data as u16;
            flags.H = if (a & 0x0F) + (b & 0x0F) > 0xF { 1 } else { 0 };
            flags.C = if (a as u32) + (b as u32) > 0xFF { 1 } else { 0 };
            reg_file.flags = flags;
            let sum = a + b;
            reg_file.set16(Reg16::SP, sum);

            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_hl_sp_e(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            let mut flags = Flags::default();
            let a = reg_file.get16(Reg16::SP);
            let b = passed.data as u16;
            flags.H = if (a & 0x0F) + (b & 0x0F) > 0xF { 1 } else { 0 };
            flags.C = if (a as u32) + (b as u32) > 0xFF { 1 } else { 0 };
            reg_file.flags = flags;
            pass_to_next_stage.data16 = a + b;
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 2;
        }
        3 => {
            reg_file.set16(Reg16::HL, passed.data16);
            pass_to_next_stage.instruction_stage = 0;
            pass_to_next_stage.next_pc = passed.next_pc + 1;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_nn_a(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data16 = ((cpu.read() as u16) << 8) | (passed.data as u16);
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            cpu.addr_bus = passed.data16;
            cpu.write_data(reg_file[Reg8::A]);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn ld_a_nn(cpu: &mut CPU, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data16 = ((cpu.read() as u16) << 8) | (passed.data as u16);
            pass_to_next_stage.instruction_stage = 2;
        }
        2 => {
            cpu.addr_bus = passed.data16;
            reg_file[Reg8::A] = cpu.read();
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
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
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            cpu.addr_bus = pass_to_next_stage.next_pc;
            pass_to_next_stage.data = cpu.read();
            pass_to_next_stage.instruction_stage = 1;
        }
        1..=3 => {
            pass_to_next_stage = match passed.data {
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
        }
        _ => panic!("cb_op: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rlc(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                flags.C = reg_file[reg8] >> 7;
                reg_file[reg8] = reg_file[reg8].rotate_left(1);
                flags.Z = if reg_file[reg8] == 0 { 1 } else { 0 };
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.C = val >> 7;
                pass_to_next_stage.data = val.rotate_left(1);
                flags.Z = if pass_to_next_stage.data == 0 { 1 } else { 0 };
            }
            reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rrc(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                flags.C = reg_file[reg8] & 1;
                reg_file[reg8] = reg_file[reg8].rotate_right(1);
                flags.Z = if reg_file[reg8] == 0 { 1 } else { 0 };
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                reg_file.flags.C = val & 1;
                pass_to_next_stage.data = val.rotate_right(1);
                flags.Z = if pass_to_next_stage.data == 0 { 1 } else { 0 };
            }
            reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rl(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                let carry = reg_file.flags.C;
                flags.C = reg_file[reg8] >> 7;
                reg_file[reg8] = (reg_file[reg8] << 1) | carry;
                flags.Z = if reg_file[reg8] == 0 { 1 } else { 0 };
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
                let carry = reg_file.flags.C;
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.C = val >> 7;
                pass_to_next_stage.data = (val << 1) | carry;
                flags.Z = if pass_to_next_stage.data == 0 { 1 } else { 0 };
            }
            reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn rr(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                let carry = reg_file.flags.C;
                flags.C = reg_file[reg8] & 1;
                reg_file[reg8] = (reg_file[reg8] >> 1) | (carry << 7);
                flags.Z = if reg_file[reg8] == 0 { 1 } else { 0 };
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
                let carry = reg_file.flags.C;
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.C = val & 1;
                pass_to_next_stage.data = (val >> 1) | (carry << 7);
                flags.Z = if pass_to_next_stage.data == 0 { 1 } else { 0 };
            }
            reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn sla(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                flags.C = reg_file[reg8] >> 7;
                reg_file[reg8] = reg_file[reg8] << 1;
                flags.Z = if reg_file[reg8] == 0 { 1 } else { 0 };
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                reg_file.flags.C = val >> 7;
                pass_to_next_stage.data = val << 1;
                flags.Z = if pass_to_next_stage.data == 0 { 1 } else { 0 };
            }
            reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn sra(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                flags.C = reg_file[reg8] & 1;
                reg_file[reg8] = reg_file[reg8] >> 1;
                if reg_file[reg8] == 0 {
                    flags.Z = 1;
                }
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.C = val & 1;
                pass_to_next_stage.data = val >> 1;
                if val >> 1 == 0 {
                    flags.Z = 1;
                }
            }
            reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn swap(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
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
                reg_file[reg8] = ((reg_file[reg8] & 0x0F) << 4) | ((reg_file[reg8] & 0xF0) >> 4);
                if reg_file[reg8] == 0 {
                    flags.Z = 1;
                }
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                pass_to_next_stage.data = ((val & 0x0F) << 4) | ((val & 0xF0) >> 4);
                if (val as i8 >> 1) == 0 {
                    flags.Z = 1;
                }
            }
            reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn srl(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            if n != 0x06 {
                let reg8 = match n {
                    0x38 => Reg8::B,
                    0x39 => Reg8::C,
                    0x3A => Reg8::D,
                    0x3B => Reg8::E,
                    0x3C => Reg8::H,
                    0x3D => Reg8::L,
                    0x3F => Reg8::A,
                    _ => panic!("rlc: invalid op"),
                };
                flags.C = reg_file[reg8] & 1;
                reg_file[reg8] = (reg_file[reg8] as i8 >> 1) as u8;
                if reg_file[reg8] == 0 {
                    flags.Z = 1;
                }
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.C = val & 1;
                pass_to_next_stage.data = ((val as i8) >> 1) as u8;
                if (val as i8 >> 1) == 0 {
                    flags.Z = 1;
                }
            }
            reg_file.flags = flags;
        }
        2 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("rlc: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn bit(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let mut flags = Flags::default();
            let lower_nibble = n & 0x0F;
            let bit_num = match n {
                0x40..=0x47 => 0,
                0x48..=0x48 => 1,
                0x50..=0x57 => 2,
                0x58..=0x5F => 3,
                0x60..=0x67 => 4,
                0x68..=0x6F => 5,
                0x70..=0x77 => 6,
                0x78..=0x7F => 7,
                _ => panic!("invalid opcode bit"),
            };
            if lower_nibble != 0xE || lower_nibble != 0x6 {
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
                flags.Z = if (reg_file[reg8] & mask) == 0 { 1 } else { 0 };
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;

		println!("woah2: {:#x}, mask: {:#x}, flags: {:?}", cpu.reg_file[reg8], mask,flags);

            } else {
                let mask = 1 << bit_num;
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                flags.Z = if (val & mask) == 0 { 1 } else { 0 };
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
            }
            cpu.reg_file.flags = flags;
	    println!("flags: {:?}", cpu.reg_file.flags);
        }
        2 => {
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn res(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let lower_nibble = n & 0x0F;
            let bit_num = match n {
                0x40..=0x47 => 0,
                0x48..=0x48 => 1,
                0x50..=0x57 => 2,
                0x58..=0x5F => 3,
                0x60..=0x67 => 4,
                0x68..=0x6F => 5,
                0x70..=0x77 => 6,
                0x78..=0x7F => 7,
                _ => panic!("invalid opcode bit"),
            };
            if lower_nibble != 0xE || lower_nibble != 0x6 {
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
                reg_file[reg8] = reg_file[reg8] & mask;
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                let mask = !(1 << bit_num);
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                pass_to_next_stage.data = val & mask;
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
            }
        }
        2 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn set(cpu: &mut CPU, n: u8, passed: StagePassThrough) -> StagePassThrough {
    let mut reg_file = cpu.reg_file;
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        1 => {
            let lower_nibble = n & 0x0F;
            let bit_num = match n {
                0x40..=0x47 => 0,
                0x48..=0x48 => 1,
                0x50..=0x57 => 2,
                0x58..=0x5F => 3,
                0x60..=0x67 => 4,
                0x68..=0x6F => 5,
                0x70..=0x77 => 6,
                0x78..=0x7F => 7,
                _ => panic!("invalid opcode bit"),
            };
            if lower_nibble != 0xE || lower_nibble != 0x6 {
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
                reg_file[reg8] = reg_file[reg8] | mask;
                pass_to_next_stage.next_pc = passed.next_pc + 1;
                pass_to_next_stage.instruction_stage = 0;
            } else {
                let mask = 1 << bit_num;
                cpu.addr_bus = reg_file.get16(Reg16::HL);
                let val = cpu.read();
                pass_to_next_stage.data = val | mask;
                pass_to_next_stage.next_pc = passed.next_pc;
                pass_to_next_stage.instruction_stage = 2;
            }
        }
        2 => {
            cpu.addr_bus = reg_file.get16(Reg16::HL);
            cpu.write_data(passed.data);
            pass_to_next_stage.next_pc = passed.next_pc;
            pass_to_next_stage.instruction_stage = 3;
        }
        3 => {
            pass_to_next_stage.next_pc = passed.next_pc + 1;
            pass_to_next_stage.instruction_stage = 0;
        }
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
}

fn func_template(
    reg_file: &mut RegFile,
    cpu: &mut CPU,
    n: u8,
    passed: StagePassThrough,
) -> StagePassThrough {
    let mut pass_to_next_stage = StagePassThrough::default();
    match passed.instruction_stage {
        0 => {}
        _ => panic!("inst_name: invalid instruction stage"),
    }
    pass_to_next_stage
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
