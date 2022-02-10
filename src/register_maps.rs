#[derive(Copy, Clone, Default)]
pub struct InterruptEnable {
    pub vblank: u8,
    pub lcd_stat: u8,
    pub timer: u8,
    pub serial: u8,
    pub joypad: u8,
}

impl From<InterruptEnable> for u8 {
    fn from(item: InterruptEnable) -> Self {
        (item.joypad << 4)
            | (item.serial << 3)
            | (item.timer << 2)
            | (item.lcd_stat << 1)
            | (item.vblank)
    }
}

impl From<u8> for InterruptEnable {
    fn from(item: u8) -> Self {
        InterruptEnable {
            vblank: item & 1,
            lcd_stat: (item >> 1) & 1,
            timer: (item >> 2) & 1,
            serial: (item >> 3) & 1,
            joypad: (item >> 4) & 1,
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct InterruptFlag {
    pub vblank: u8,
    pub lcd_stat: u8,
    pub timer: u8,
    pub serial: u8,
    pub joypad: u8,
}

impl From<InterruptFlag> for u8 {
    fn from(item: InterruptFlag) -> Self {
        (item.joypad << 4)
            | (item.serial << 3)
            | (item.timer << 2)
            | (item.lcd_stat << 1)
            | (item.vblank)
    }
}

impl From<u8> for InterruptFlag {
    fn from(item: u8) -> Self {
        InterruptFlag {
            vblank: item & 1,
            lcd_stat: (item >> 1) & 1,
            timer: (item >> 2) & 1,
            serial: (item >> 3) & 1,
            joypad: (item >> 4) & 1,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct LCDC {
    pub lcd_ppu_enable: u8,
    pub window_tile_map_area: u8,
    pub window_enable: u8,
    pub bg_window_tile_data_area: u8,
    pub bg_tile_map_area: u8,
    pub obj_size: u8,
    pub obj_enable: u8,
    pub bg_window_enable_priority: u8,
}

impl From<LCDC> for u8 {
    fn from(item: LCDC) -> Self {
        (item.lcd_ppu_enable << 7)
            | (item.window_tile_map_area << 6)
            | (item.window_enable << 5)
            | (item.bg_window_tile_data_area << 4)
            | (item.bg_tile_map_area << 3)
            | (item.obj_size << 2)
            | (item.obj_enable << 1)
            | (item.bg_window_enable_priority)
    }
}

impl From<u8> for LCDC {
    fn from(item: u8) -> Self {
        LCDC {
            lcd_ppu_enable: item >> 7,
            window_tile_map_area: (item >> 6) & 1,
            window_enable: (item >> 5) & 1,
            bg_window_tile_data_area: (item >> 4) & 1,
            bg_tile_map_area: (item >> 3) & 1,
            obj_size: (item >> 2) & 1,
            obj_enable: (item >> 1) & 1,
            bg_window_enable_priority: item & 1,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct LCDStatus {
    pub lyc_ly_stat_interrupt: u8,
    pub mode_two_stat_interrupt: u8,
    pub mode_one_stat_interrupt: u8,
    pub mode_zero_stat_interrupt: u8,
    pub lyc_eq_ly: u8,
    pub mode: u8,
}

impl From<LCDStatus> for u8 {
    fn from(item: LCDStatus) -> Self {
        (item.lyc_ly_stat_interrupt << 6)
            | (item.mode_two_stat_interrupt << 5)
            | (item.mode_one_stat_interrupt << 4)
            | (item.mode_zero_stat_interrupt << 3)
            | (item.lyc_eq_ly << 2)
            | (item.mode)
    }
}

impl From<u8> for LCDStatus {
    fn from(item: u8) -> Self {
        LCDStatus {
            lyc_ly_stat_interrupt: (item >> 6) & 1,
            mode_two_stat_interrupt: (item >> 5) & 1,
            mode_one_stat_interrupt: (item >> 4) & 1,
            mode_zero_stat_interrupt: (item >> 3) & 1,
            lyc_eq_ly: (item >> 2) & 1,
            mode: item & 3,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Joypad {
    pub select_action: u8,
    pub select_direction: u8,
    pub down_or_start: u8,
    pub up_or_select: u8,
    pub left_or_b: u8,
    pub right_or_a: u8,
}

impl From<Joypad> for u8 {
    fn from(item: Joypad) -> Self {
        (item.select_action << 5)
            | (item.select_direction << 4)
            | (item.down_or_start << 3)
            | (item.up_or_select << 2)
            | (item.left_or_b << 1)
            | (item.right_or_a)
    }
}

impl From<u8> for Joypad {
    fn from(item: u8) -> Self {
        Joypad {
            select_action: (item >> 5) & 1,
            select_direction: (item >> 4) & 1,
            down_or_start: (item >> 3) & 1,
            up_or_select: (item >> 2) & 1,
            left_or_b: (item >> 1) & 1,
            right_or_a: item & 1,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub enum InputClockSelect {
    #[default]
    Mode1024 = 0,
    Mode16 = 1,
    Mode64 = 2,
    Mode256 = 3,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct TAC {
    pub timer_enable: bool,
    pub input_clock_select: InputClockSelect,
}

impl From<u8> for TAC {
    fn from(val: u8) -> TAC {
        TAC {
            timer_enable: (val & 0x04) >> 2 == 1,
            input_clock_select: {
                let clock_select = val & 0x03;
                if clock_select == 0 {
                    InputClockSelect::Mode1024
                } else if clock_select == 1 {
                    InputClockSelect::Mode16
                } else if clock_select == 2 {
                    InputClockSelect::Mode64
                } else {
                    InputClockSelect::Mode256
                }
            },
        }
    }
}

impl From<TAC> for u8 {
    fn from(val: TAC) -> u8 {
        ((val.timer_enable as u8) << 2) & (val.input_clock_select as u8)
    }
}

#[derive(Copy, Clone, Default)]
pub struct IORegisters {
    pub dma_in_progress: bool,
    pub clock_count_for_dma: u8,
    pub joypad: Joypad,
    pub communication: [u8; 2],        //0xFF01 - 0xFF02
    pub timer: Timer,                  //0xFF04 - 0xFF07
    pub interrupt_flag: InterruptFlag, // 0xFF0F
    pub sound: [u8; 0x17],             // 0xFF10 - 0xFF26
    pub waveform_ram: [u8; 0x10],      // 0xFF30 - 0xFF3F
    pub lcdc: LCDC,                    // 0xFF40
    pub lcd_status: LCDStatus,         // 0xFF41
    pub scy: u8,                       // 0xFF42
    pub scx: u8,                       // 0xFF43
    pub ly: u8,                        // 0xFF44
    pub lyc: u8,                       // 0xFF45
    pub dma: u8,                       // 0xFF46
    pub bgp: u8,                       // 0xFF47
    pub obp0: u8,                      // 0xFF48
    pub obp1: u8,                      // 0xFF49
    pub wy: u8,                        // 0xFF4A
    pub wx: u8,                        // 0xFF4B
    pub vbk: u8,                       // 0xFF4F, not used
    pub use_boot_rom: u8,              //0xFF50
    pub other1: [u8; 0x20],            // 0xFF51 - 0xFF70
    pub other2: [u8; 0x10],            //0xFF70-0xFF7F
}

impl IORegisters {
    pub fn get(&self, index: usize) -> u8 {
        match index {
            0xFF00 => self.joypad.into(),
            0xFF01 | 0xFF02 => self.communication[index - 0xFF01],
            0xFF04 => (self.timer.timer_counter >> 8) as u8,
            0xFF05 => self.timer.tima,
            0xFF06 => self.timer.tma,
            0xFF07 => self.timer.tac.into(),
            0xFF0F => self.interrupt_flag.into(),
            0xFF10..=0xFF26 => self.sound[index - 0xFF10],
            0xFF30..=0xFF3F => self.waveform_ram[index - 0xFF30],
            0xFF40 => self.lcdc.into(),
            0xFF41 => self.lcd_status.into(),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF4D => 0xFF,
            0xFF4F => self.vbk,
            0xFF50 => self.use_boot_rom,
            0xFF51..=0xFF70 => self.other1[index - 0xFF51],
            0xFF71..=0xFF7F => self.other2[index - 0xFF71],
            _ => panic!("invalid index for io registers: {:#x}", index),
        }
    }

    pub fn set(&mut self, index: usize, value: u8) {
        match index {
            0xFF00 => self.joypad = value.into(),
            0xFF01 | 0xFF02 => self.communication[index - 0xFF01] = value,
            0xFF04 => self.timer.timer_counter = 0,
            0xFF05 => self.timer.tima = value,
            0xFF06 => self.timer.tma = value,
            0xFF07 => self.timer.tac = value.into(),
            0xFF0F => self.interrupt_flag = value.into(),
            0xFF10..=0xFF26 => self.sound[index - 0xFF10] = value,
            0xFF30..=0xFF3F => self.waveform_ram[index - 0xFF30] = value,
            0xFF40 => self.lcdc = value.into(),
            0xFF41 => self.lcd_status = value.into(),
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => self.ly = value,
            0xFF45 => self.lyc = value,
            0xFF46 => {
		self.dma = value;
		if value <= 0xDF {
		    self.clock_count_for_dma = 0;
		    self.dma_in_progress = true;
		}
	    }
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            0xFF4F => self.vbk = value,
            0xFF50 => {
                self.use_boot_rom = value;
                self.joypad = 0xFF.into(); // 0 means pressed which resets Tetris
            }
            0xFF51..=0xFF70 => self.other1[index - 0xFF51] = value,
            0xFF71..=0xFF7F => self.other2[index - 0xFF71] = value,
            _ => panic!("invalid index for io registers: 0x{:x}", index),
        }
    }

    pub fn tick_timer(&mut self) {
        if self.timer.tick_timer() {
            self.interrupt_flag.timer = 1;
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Timer {
    pub tima: u8,       // 0xFF05
    pub tma: u8,        // 0xFF06
    pub tac: TAC,       // 0xFF07
    timer_counter: u16, // High bits are div -> 0xFF04
}

impl Timer {
    pub fn tick_timer(&mut self) -> bool {
        let old_timer = self.timer_counter;
        self.timer_counter = self.timer_counter.wrapping_add(4);
	if self.tac.timer_enable {
            let max_clock_count = match self.tac.input_clock_select {
		InputClockSelect::Mode1024 => 1024,
		InputClockSelect::Mode16 => 16,
		InputClockSelect::Mode64 => 64,
		InputClockSelect::Mode256 => 256,
            };

            if old_timer & (max_clock_count - 1) >= max_clock_count - 4 {
		if self.tima == 0xFF {
                    self.tima = self.tma;
                    true
		} else {
                    self.tima += 1;
                    false
		}
            } else {
		false
            }
	} else {
	    false
	}
    }
}
