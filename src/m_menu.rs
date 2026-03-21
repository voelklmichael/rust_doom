// doomgeneric/m_menu.h

pub use crate::d_event::*;
pub use crate::d_main::*;
pub use crate::deh_main::*;
pub use crate::doomdef::*;
pub use crate::doomkeys::*;
pub use crate::doomstat::*;
pub use crate::dstrings::*;
// g_game omitted here (circular with g_game → m_menu)
pub use crate::hu_stuff::*;
pub use crate::i_swap::*;
pub use crate::i_system::*;
pub use crate::i_timer::*;
pub use crate::i_video::*;
pub use crate::m_argv::*;
pub use crate::m_controls::*;
pub use crate::m_misc::*;
pub use crate::p_saveg::*;
pub use crate::r_local::*;
pub use crate::s_sound::*;
pub use crate::sounds::*;
pub use crate::v_video::*;
pub use crate::w_wad::*;
pub use crate::z_zone::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct M_MenuState {
    pub detail_level: RefCell<i32>,
    pub screenblocks: RefCell<i32>,
}

impl M_MenuState {
    pub fn new() -> Self {
        Self {
            detail_level: RefCell::new(0),
            screenblocks: RefCell::new(0),
        }
    }

    pub fn m_responder(&self, _ev: *mut EventT) -> Boolean {
        todo!("M_Responder");
    }

    pub fn m_ticker(&self) {
        todo!("M_Ticker");
    }

    pub fn m_drawer(&self) {
        todo!("M_Drawer");
    }

    pub fn m_init(&self) {
        todo!("M_Init");
    }

    pub fn m_start_control_panel(&self) {
        todo!("M_StartControlPanel");
    }
}
