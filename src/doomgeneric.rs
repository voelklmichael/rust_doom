//! Doom Generic platform interface (doomgeneric.h, doomgeneric.c)
//! Original: doomgeneric.h, doomgeneric.c

use std::sync::Arc;
use std::sync::Mutex;

// DOOMGENERIC_RESX 640
pub const DOOMGENERIC_RESX: u32 = 640;
// DOOMGENERIC_RESY 400
pub const DOOMGENERIC_RESY: u32 = 400;

// typedef uint32_t pixel_t (CMAP256 unused, use 32-bit)
pub type PixelT = u32;

pub struct DoomgenericState {
    // extern pixel_t* DG_ScreenBuffer
    pub dg_screen_buffer: Arc<Mutex<Option<Vec<PixelT>>>>,
}

impl DoomgenericState {
    /// Original: void doomgeneric_Create(int argc, char **argv)
    pub fn doomgeneric_create(&self, _argc: i32, _argv: *mut *mut i8) {
        // C body:
        // myargc = argc; myargv = argv;
        // M_FindResponseFile();
        // DG_ScreenBuffer = malloc(DOOMGENERIC_RESX * DOOMGENERIC_RESY * 4);
        // DG_Init();
        // D_DoomMain();
        todo!("Basic stage-0 stub")
    }

    /// Original: void doomgeneric_Tick()
    pub fn doomgeneric_tick(&self) {
        todo!("Basic stage-0 stub")
    }
}
