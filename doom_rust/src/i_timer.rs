//! Rust translation of doomgeneric/i_timer.h
//! System-specific timer interface.

/// C #define: TICRATE
pub const TICRATE: i32 = 35;

/// C function: I_GetTime - returns current time in tics
pub fn i_get_time() -> i32 {
    todo!("original: I_GetTime")
}

/// C function: I_GetTimeMS - returns current time in ms
pub fn i_get_time_ms() -> i32 {
    todo!("original: I_GetTimeMS")
}

/// C function: I_Sleep - pause for a specified number of ms
pub fn i_sleep(ms: i32) {
    todo!("original: I_Sleep")
}

/// C function: I_InitTimer - initialize timer
pub fn i_init_timer() {
    todo!("original: I_InitTimer")
}

/// C function: I_WaitVBL - wait for vertical retrace or pause a bit
pub fn i_wait_vbl(count: i32) {
    todo!("original: I_WaitVBL")
}
