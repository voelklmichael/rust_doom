// doomgeneric/hu_stuff.h

pub use crate::deh_main::*;
pub use crate::doomdef::*;
pub use crate::doomkeys::*;
pub use crate::doomstat::*;
pub use crate::dstrings::*;
pub use crate::hu_lib::*;
pub use crate::i_swap::*;
pub use crate::i_video::*;
pub use crate::m_controls::*;
pub use crate::m_misc::*;
pub use crate::s_sound::*;
pub use crate::sounds::*;
pub use crate::w_wad::*;
pub use crate::z_zone::*;

use crate::d_event::EventT;

use std::cell::RefCell;
use std::ffi::c_char;

// Original: HU_FONTSTART, HU_FONTEND, HU_FONTSIZE
pub const HU_FONTSTART: i32 = b'!' as i32;
pub const HU_FONTEND: i32 = b'_' as i32;
pub const HU_FONTSIZE: i32 = HU_FONTEND - HU_FONTSTART + 1;

pub const HU_BROADCAST: i32 = 5;
pub const HU_MSGX: i32 = 0;
pub const HU_MSGY: i32 = 0;
pub const HU_MSGWIDTH: i32 = 64;
pub const HU_MSGHEIGHT: i32 = 1;

pub const HU_MSGTIMEOUT: i32 = 4 * crate::i_timer::TICRATE;

#[allow(non_camel_case_types)]
pub struct Hu_StuffState {
    // Original: chat_macros
    pub chat_macros: RefCell<[*mut c_char; 10]>,
}

impl Hu_StuffState {
    pub fn new() -> Self {
        Self {
            chat_macros: RefCell::new([std::ptr::null_mut(); 10]),
        }
    }

    // Original: HU_Init
    pub fn hu_init(&self) {
        todo!("HU_Init");
    }

    // Original: HU_Start
    pub fn hu_start(&self) {
        todo!("HU_Start");
    }

    // Original: HU_Responder
    pub fn hu_responder(&self, _ev: *mut EventT) -> crate::doomtype::Boolean {
        todo!("HU_Responder");
    }

    // Original: HU_Ticker
    pub fn hu_ticker(&self) {
        todo!("HU_Ticker");
    }

    // Original: HU_Drawer
    pub fn hu_drawer(&self) {
        todo!("HU_Drawer");
    }

    // Original: HU_dequeueChatChar
    pub fn hu_dequeue_chat_char(&self) -> c_char {
        todo!("HU_dequeueChatChar");
    }

    // Original: HU_Erase
    pub fn hu_erase(&self) {
        todo!("HU_Erase");
    }
}
