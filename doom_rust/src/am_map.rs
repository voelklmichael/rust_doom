//! Rust translation of doomgeneric/am_map.h

use crate::d_event::*;
use crate::m_cheat::*;

/// C #define: AM_MSGHEADER
pub const AM_MSGHEADER: i32 = (b'a' as i32) << 24 | (b'm' as i32) << 16;
/// C #define: AM_MSGENTERED
pub const AM_MSGENTERED: i32 = AM_MSGHEADER | (b'e' as i32) << 8;
/// C #define: AM_MSGEXITED
pub const AM_MSGEXITED: i32 = AM_MSGHEADER | (b'x' as i32) << 8;

/// C function: AM_Responder
pub fn am_responder(ev: &mut EventT) -> crate::doomtype::boolean {
    todo!("original: AM_Responder")
}

/// C function: AM_Ticker
pub fn am_ticker() {
    todo!("original: AM_Ticker")
}

/// C function: AM_Drawer
pub fn am_drawer() {
    todo!("original: AM_Drawer")
}

/// C function: AM_Stop
pub fn am_stop() {
    todo!("original: AM_Stop")
}

/// C extern: cheat_amap
pub static mut cheat_amap: CheatseqT = CheatseqT {
    sequence: [0; crate::m_cheat::MAX_CHEAT_LEN],
    sequence_len: 0,
    parameter_chars: 0,
    chars_read: 0,
    param_chars_read: 0,
    parameter_buf: [0; crate::m_cheat::MAX_CHEAT_PARAMS],
};
