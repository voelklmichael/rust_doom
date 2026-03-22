//! Timer interface (i_timer.h, i_timer.c)
//! Original: i_timer.h, i_timer.c

use std::sync::Arc;
use std::sync::Mutex;

// #define TICRATE 35
pub const TICRATE: i32 = 35;

pub struct I_TimerState {
    // static uint32_t basetime
    basetime: Arc<Mutex<u32>>,
}

impl I_TimerState {
    /// Original: int I_GetTime(void)
    pub fn i_get_time(&self) -> i32 {
        // C body:
        // ticks = I_GetTicks(); if (basetime == 0) basetime = ticks;
        // return (ticks * TICRATE) / 1000;
        todo!("Basic stage-0 stub")
    }

    /// Original: int I_GetTimeMS(void)
    pub fn i_get_time_ms(&self) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_Sleep(int ms)
    pub fn i_sleep(&self, _ms: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_InitTimer(void)
    pub fn i_init_timer(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_WaitVBL(int count)
    pub fn i_wait_vbl(&self, _count: i32) {
        todo!("Basic stage-0 stub")
    }
}
