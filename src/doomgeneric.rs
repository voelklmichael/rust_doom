//! Doom Generic platform (doomgeneric.h, doomgeneric.c)
//! Original: doomgeneric.h, doomgeneric.c
//! Plan §4.2: DOOMGENERIC_RESX 640, DOOMGENERIC_RESY 400. CMAP256 unused -> pixel_t = u32.

use std::sync::Arc;
use std::sync::Mutex;

// #define DOOMGENERIC_RESX 640
pub const DOOMGENERIC_RESX: u32 = 640;
// #define DOOMGENERIC_RESY 400
pub const DOOMGENERIC_RESY: u32 = 400;

// typedef uint32_t pixel_t (CMAP256 unused)
pub type PixelT = u32;

pub struct DoomgenericState {
    // extern pixel_t* DG_ScreenBuffer
    pub dg_screen_buffer: Arc<Mutex<Option<Vec<PixelT>>>>,
}

impl DoomgenericState {
    /// Original: void doomgeneric_Create(int argc, char **argv)
    pub fn doomgeneric_create(&self, _argc: i32, _argv: &[String]) {
        // C body: (from doomgeneric.c)
        todo!("Basic stage-0 stub")
    }

    /// Original: void doomgeneric_Tick(void)
    pub fn doomgeneric_tick(&self) {
        // C body: (from doomgeneric.c)
        todo!("Basic stage-0 stub")
    }

    /// Original: void DG_Init(void)
    pub fn dg_init(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void DG_DrawFrame(void)
    pub fn dg_draw_frame(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void DG_SleepMs(uint32_t ms)
    pub fn dg_sleep_ms(&self, _ms: u32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: uint32_t DG_GetTicksMs(void)
    pub fn dg_get_ticks_ms(&self) -> u32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int DG_GetKey(int* pressed, unsigned char* key)
    pub fn dg_get_key(&self, _pressed: &mut i32, _key: &mut u8) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void DG_SetWindowTitle(const char* title)
    pub fn dg_set_window_title(&self, _title: &str) {
        todo!("Basic stage-0 stub")
    }
}
