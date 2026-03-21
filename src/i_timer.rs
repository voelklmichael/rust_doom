// i_timer.h / i_timer.c

pub use crate::doomgeneric::*;
pub use crate::doomtype::*;

use std::cell::RefCell;

// Original: #define TICRATE 35
pub const TICRATE: i32 = 35;

#[allow(non_camel_case_types)]
pub struct I_TimerState {
    // Original: static basetime (from i_timer.c)
    pub basetime: RefCell<u32>,
}

impl I_TimerState {
    pub fn new() -> Self {
        Self {
            basetime: RefCell::new(0),
        }
    }

    // Original: I_GetTicks (doomgeneric.c exposes DG_GetTicksMs)
    pub fn i_get_ticks(&self, dg: &DoomgenericState) -> u32 {
        dg.dg_get_ticks_ms()
    }

    // Original: I_GetTime
    pub fn i_get_time(&self, dg: &DoomgenericState) -> i32 {
        let ticks = self.i_get_ticks(dg);
        let mut base = *self.basetime.borrow();
        if base == 0 {
            *self.basetime.borrow_mut() = ticks;
            base = ticks;
        }
        let elapsed = ticks - base;
        ((elapsed as i64) * (TICRATE as i64) / 1000) as i32
    }

    // Original: I_GetTimeMS
    pub fn i_get_time_ms(&self, dg: &DoomgenericState) -> i32 {
        let ticks = self.i_get_ticks(dg);
        let mut base = *self.basetime.borrow();
        if base == 0 {
            *self.basetime.borrow_mut() = ticks;
            base = ticks;
        }
        (ticks - base) as i32
    }

    // Original: I_Sleep
    pub fn i_sleep(&self, dg: &DoomgenericState, ms: i32) {
        dg.dg_sleep_ms(ms as u32);
    }

    // Original: I_WaitVBL
    pub fn i_wait_vbl(&self, _count: i32) {}

    // Original: I_InitTimer
    pub fn i_init_timer(&self) {}
}
