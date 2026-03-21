// doomgeneric/wi_stuff.h

pub use crate::deh_main::*;
pub use crate::doomdef::*;
pub use crate::doomstat::*;
// g_game omitted (circular with g_game → wi_stuff)
pub use crate::i_swap::*;
pub use crate::i_system::*;
pub use crate::m_misc::*;
pub use crate::m_random::*;
pub use crate::r_local::*;
pub use crate::s_sound::*;
pub use crate::sounds::*;
pub use crate::v_video::*;
pub use crate::w_wad::*;

use crate::d_player::WbstartstructT;

use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WiStateEnumT {
    NoState = -1,
    StatCount = 0,
    ShowNextLoc = 1,
}

#[allow(non_camel_case_types)]
pub struct Wi_StuffState {
    pub _placeholder: RefCell<()>,
}

impl Wi_StuffState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: WI_Ticker
    pub fn wi_ticker(&self) {
        todo!("WI_Ticker");
    }

    // Original: WI_Drawer
    pub fn wi_drawer(&self) {
        todo!("WI_Drawer");
    }

    // Original: WI_Start
    pub fn wi_start(&self, _wbstartstruct: *mut WbstartstructT) {
        todo!("WI_Start");
    }

    // Original: WI_End
    pub fn wi_end(&self) {
        todo!("WI_End");
    }
}
