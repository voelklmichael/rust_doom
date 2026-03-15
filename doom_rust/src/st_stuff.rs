//! Rust translation of doomgeneric/st_stuff.h

use crate::d_event::*;
use crate::doomtype::*;
use crate::i_video::{SCREENHEIGHT, SCREENWIDTH};
use crate::m_cheat::*;

/// C #define: ST_HEIGHT
pub const ST_HEIGHT: i32 = 32;
/// C #define: ST_WIDTH
pub const ST_WIDTH: i32 = SCREENWIDTH;
/// C #define: ST_Y
pub const ST_Y: i32 = SCREENHEIGHT - ST_HEIGHT;

/// C function: ST_Responder
pub fn st_responder(ev: *mut EventT) -> boolean {
    todo!("original: ST_Responder")
}

/// C function: ST_Ticker
pub fn st_ticker() {
    todo!("original: ST_Ticker")
}

/// C function: ST_Drawer
pub fn st_drawer(fullscreen: boolean, refresh: boolean) {
    todo!("original: ST_Drawer")
}

/// C function: ST_Start
pub fn st_start() {
    todo!("original: ST_Start")
}

/// C function: ST_Init
pub fn st_init() {
    todo!("original: ST_Init")
}

/// C typedef: st_stateenum_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StStateenumT {
    AutomapState,
    FirstPersonState,
}

/// C typedef: st_chatstateenum_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StChatstateenumT {
    StartChatState,
    WaitDestState,
    GetChatState,
}

/// C extern
pub static mut st_backing_screen: *mut byte = std::ptr::null_mut();
pub static mut cheat_mus: CheatseqT = CheatseqT {
    sequence: [0; MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; MAX_CHEAT_PARAMS],
};
pub static mut cheat_god: CheatseqT = CheatseqT {
    sequence: [0; MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; MAX_CHEAT_PARAMS],
};
pub static mut cheat_ammo: CheatseqT = CheatseqT {
    sequence: [0; MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; MAX_CHEAT_PARAMS],
};
pub static mut cheat_ammonokey: CheatseqT = CheatseqT {
    sequence: [0; MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; MAX_CHEAT_PARAMS],
};
pub static mut cheat_noclip: CheatseqT = CheatseqT {
    sequence: [0; MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; MAX_CHEAT_PARAMS],
};
pub static mut cheat_commercial_noclip: CheatseqT = CheatseqT {
    sequence: [0; MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; MAX_CHEAT_PARAMS],
};
pub static mut cheat_choppers: CheatseqT = CheatseqT {
    sequence: [0; MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; MAX_CHEAT_PARAMS],
};
pub static mut cheat_clev: CheatseqT = CheatseqT {
    sequence: [0; MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; MAX_CHEAT_PARAMS],
};
pub static mut cheat_mypos: CheatseqT = CheatseqT {
    sequence: [0; MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; MAX_CHEAT_PARAMS],
};
pub static mut cheat_powerup: [CheatseqT; 7] = [CheatseqT {
    sequence: [0; MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; MAX_CHEAT_PARAMS],
}; 7];
