// doomgeneric/am_map.h

pub use crate::deh_main::*;
pub use crate::doomdef::*;
pub use crate::doomkeys::*;
pub use crate::doomstat::*;
pub use crate::dstrings::*;
pub use crate::i_system::*;
pub use crate::m_cheat::*;
pub use crate::m_controls::*;
pub use crate::m_misc::*;
pub use crate::p_local::*;
pub use crate::r_state::*;
// st_stuff omitted (circular: st_stuff → am_map)
pub use crate::v_video::*;
pub use crate::w_wad::*;
pub use crate::z_zone::*;

use crate::d_event::EventT;

use std::cell::RefCell;

pub const AM_MSGHEADER: i32 = (b'a' as i32) << 24 | (b'm' as i32) << 16;
pub const AM_MSGENTERED: i32 = AM_MSGHEADER | (b'e' as i32) << 8;
pub const AM_MSGEXITED: i32 = AM_MSGHEADER | (b'x' as i32) << 8;

#[allow(non_camel_case_types)]
pub struct Am_MapState {
    // Original: cheat_amap
    pub cheat_amap: RefCell<CheatseqT>,
}

impl Am_MapState {
    pub fn new() -> Self {
        Self {
            cheat_amap: RefCell::new(crate::m_cheat::CheatseqT {
                sequence: [0; crate::m_cheat::MAX_CHEAT_LEN],
                sequence_len: 0,
                parameter_chars: 0,
                chars_read: 0,
                param_chars_read: 0,
                parameter_buf: [0; crate::m_cheat::MAX_CHEAT_PARAMS],
            }),
        }
    }

    // Original: AM_Responder
    pub fn am_responder(&self, _ev: *mut EventT) -> crate::doomtype::Boolean {
        todo!("AM_Responder");
    }

    // Original: AM_Ticker
    pub fn am_ticker(&self) {
        todo!("AM_Ticker");
    }

    // Original: AM_Drawer
    pub fn am_drawer(&self) {
        todo!("AM_Drawer");
    }

    // Original: AM_Stop
    pub fn am_stop(&self) {
        todo!("AM_Stop");
    }
}
