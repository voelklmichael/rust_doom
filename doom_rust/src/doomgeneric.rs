//! Rust translation of doomgeneric/doomgeneric.h

// C #define DOOMGENERIC_RESX
/// C #define: DOOMGENERIC_RESX
pub const DOOMGENERIC_RESX: u32 = 640;

// C #define DOOMGENERIC_RESY
/// C #define: DOOMGENERIC_RESY
pub const DOOMGENERIC_RESY: u32 = 400;

// C typedef: pixel_t (uint8_t for CMAP256, else uint32_t)
/// C typedef: pixel_t
pub type PixelT = u32;

/// C extern: DG_ScreenBuffer
pub static mut DG_SCREEN_BUFFER: *mut PixelT = std::ptr::null_mut();

/// C function: doomgeneric_Create
pub fn doomgeneric_create(argc: i32, argv: &mut [&str]) {
    todo!("original: doomgeneric_Create")
}

/// C function: doomgeneric_Tick
pub fn doomgeneric_tick() {
    todo!("original: doomgeneric_Tick")
}

/// C function: DG_Init
pub fn dg_init() {
    todo!("original: DG_Init")
}

/// C function: DG_DrawFrame
pub fn dg_draw_frame() {
    todo!("original: DG_DrawFrame")
}

/// C function: DG_SleepMs
pub fn dg_sleep_ms(ms: u32) {
    todo!("original: DG_SleepMs")
}

/// C function: DG_GetTicksMs
pub fn dg_get_ticks_ms() -> u32 {
    todo!("original: DG_GetTicksMs")
}

/// C function: DG_GetKey
pub fn dg_get_key(pressed: &mut i32, key: &mut u8) -> i32 {
    todo!("original: DG_GetKey")
}

/// C function: DG_SetWindowTitle
pub fn dg_set_window_title(title: *const i8) {
    todo!("original: DG_SetWindowTitle")
}
