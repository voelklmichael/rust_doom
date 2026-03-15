//! Rust translation of doomgeneric/f_wipe.h

/// C enum: wipe types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: wipe_type_t
pub enum WipeType {
    ColorXForm,
    Melt,
    Numwipes,
}

/// C function: wipe_StartScreen
pub fn wipe_start_screen(x: i32, y: i32, width: i32, height: i32) -> i32 {
    todo!("original: wipe_StartScreen")
}

/// C function: wipe_EndScreen
pub fn wipe_end_screen(x: i32, y: i32, width: i32, height: i32) -> i32 {
    todo!("original: wipe_EndScreen")
}

/// C function: wipe_ScreenWipe
pub fn wipe_screen_wipe(wipeno: i32, x: i32, y: i32, width: i32, height: i32, ticks: i32) -> i32 {
    todo!("original: wipe_ScreenWipe")
}
