// doomgeneric/f_finale.h

pub use crate::d_event::*;
pub use crate::d_main::*;
pub use crate::deh_main::*;
pub use crate::doomstat::*;
pub use crate::doomtype::*;
pub use crate::dstrings::*;
pub use crate::hu_stuff::*;
pub use crate::i_swap::*;
pub use crate::i_system::*;
pub use crate::r_local::*;
pub use crate::r_state::*;
pub use crate::s_sound::*;
pub use crate::sounds::*;
pub use crate::v_video::*;
pub use crate::w_wad::*;
pub use crate::z_zone::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct F_FinaleState {
    pub _placeholder: RefCell<()>,
}

impl F_FinaleState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: F_Responder
    pub fn f_responder(&self, _ev: *mut EventT) -> Boolean {
        todo!("F_Responder");
    }

    // Original: F_Ticker
    pub fn f_ticker(&self) {
        todo!("F_Ticker");
    }

    // Original: F_Drawer
    pub fn f_drawer(&self) {
        todo!("F_Drawer");
    }

    // Original: F_StartFinale
    pub fn f_start_finale(&self) {
        todo!("F_StartFinale");
    }
}
