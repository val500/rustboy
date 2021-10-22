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

#[derive(Copy, Clone, Default)]
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
    select_action: u8,
    select_direction: u8,
    down_or_start: u8,
    up_or_select: u8,
    left_or_b: u8,
    right_or_a: u8,
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




