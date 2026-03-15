//! Rust translation of doomgeneric/hu_stuff.h

use crate::d_event::*;
use crate::i_timer::TICRATE;

/// C #define: HU_FONTSTART
pub const HU_FONTSTART: i32 = b'!' as i32;
/// C #define: HU_FONTEND
pub const HU_FONTEND: i32 = b'_' as i32;
/// C #define: HU_FONTSIZE
pub const HU_FONTSIZE: i32 = HU_FONTEND - HU_FONTSTART + 1;
/// C #define: HU_BROADCAST
pub const HU_BROADCAST: i32 = 5;
/// C #define: HU_MSGX
pub const HU_MSGX: i32 = 0;
/// C #define: HU_MSGY
pub const HU_MSGY: i32 = 0;
/// C #define: HU_MSGWIDTH
pub const HU_MSGWIDTH: i32 = 64;
/// C #define: HU_MSGHEIGHT
pub const HU_MSGHEIGHT: i32 = 1;
/// C #define: HU_MSGTIMEOUT
pub const HU_MSGTIMEOUT: i32 = 4 * TICRATE;

/// C function: HU_Init
pub fn hu_init() {
    todo!("original: HU_Init")
}

/// C function: HU_Start
pub fn hu_start() {
    todo!("original: HU_Start")
}

/// C function: HU_Responder
pub fn hu_responder(ev: &mut EventT) -> crate::doomtype::boolean {
    todo!("original: HU_Responder")
}

/// C function: HU_Ticker
pub fn hu_ticker() {
    todo!("original: HU_Ticker")
}

/// C function: HU_Drawer
pub fn hu_drawer() {
    todo!("original: HU_Drawer")
}

/// C function: HU_dequeueChatChar
pub fn hu_dequeue_chat_char() -> i8 {
    todo!("original: HU_dequeueChatChar")
}

/// C function: HU_Erase
pub fn hu_erase() {
    todo!("original: HU_Erase")
}

/// C extern: chat_macros
pub static mut chat_macros: [*mut i8; 10] = [std::ptr::null_mut(); 10];
