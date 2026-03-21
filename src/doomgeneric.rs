// doomgeneric.h / doomgeneric.c
// Platform abstraction layer for Doom

use std::cell::RefCell;
use std::os::raw::c_char;

pub use crate::m_argv::*;
pub use crate::d_main::*;

// Original: #ifndef DOOMGENERIC_RESX / #define DOOMGENERIC_RESX 640
pub const DOOMGENERIC_RESX: u32 = 640;

// Original: #ifndef DOOMGENERIC_RESY / #define DOOMGENERIC_RESY 400
pub const DOOMGENERIC_RESY: u32 = 400;

// Original: typedef uint32_t pixel_t (non-CMAP256 path)
pub type PixelT = u32;

// Original: extern pixel_t* DG_ScreenBuffer
// Defined in doomgeneric.c - becomes state field
#[allow(non_camel_case_types)]
pub struct DoomgenericState {
    // Original: DG_ScreenBuffer
    pub dg_screen_buffer: RefCell<Option<Box<[PixelT]>>>,
}

impl DoomgenericState {
    pub fn new() -> Self {
        Self {
            dg_screen_buffer: RefCell::new(None),
        }
    }

    // Original: doomgeneric_Create
    pub fn doomgeneric_create(
        &self,
        argc: i32,
        argv: *mut *mut c_char,
        m_argv_state: &M_ArgvState,
        d_main_state: &D_MainState,
    ) {
        // Original: myargc = argc; myargv = argv;
        *m_argv_state.myargc.borrow_mut() = argc;
        *m_argv_state.myargv.borrow_mut() = argv;

        // Original: M_FindResponseFile()
        m_argv_state.m_find_response_file();

        // Original: DG_ScreenBuffer = malloc(DOOMGENERIC_RESX * DOOMGENERIC_RESY * 4)
        let size = (DOOMGENERIC_RESX * DOOMGENERIC_RESY) as usize;
        let buffer = vec![0u32; size].into_boxed_slice();
        *self.dg_screen_buffer.borrow_mut() = Some(buffer);

        // Original: DG_Init()
        self.dg_init();

        // Original: D_DoomMain()
        d_main_state.d_doom_main();
    }

    // Original: DG_Init
    // Platform hook - implement for your platform
    pub fn dg_init(&self) {
        todo!("DG_Init - implement for your platform")
    }

    // Original: DG_DrawFrame
    // Platform hook - implement for your platform
    pub fn dg_draw_frame(&self) {
        todo!("DG_DrawFrame - implement for your platform")
    }

    // Original: DG_SleepMs
    // Platform hook - implement for your platform
    pub fn dg_sleep_ms(&self, ms: u32) {
        let _ = ms;
        todo!("DG_SleepMs - implement for your platform")
    }

    // Original: DG_GetTicksMs
    // Platform hook - implement for your platform
    pub fn dg_get_ticks_ms(&self) -> u32 {
        todo!("DG_GetTicksMs - implement for your platform")
    }

    // Original: DG_GetKey
    // Platform hook - implement for your platform
    // Returns (pressed, key)
    pub fn dg_get_key(&self) -> (i32, u8) {
        todo!("DG_GetKey - implement for your platform")
    }

    // Original: DG_SetWindowTitle
    // Platform hook - implement for your platform
    pub fn dg_set_window_title(&self, title: &str) {
        let _ = title;
        todo!("DG_SetWindowTitle - implement for your platform")
    }
}

