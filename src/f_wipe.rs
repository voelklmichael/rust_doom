// doomgeneric/f_wipe.h

pub use crate::doomtype::*;
pub use crate::i_video::*;
pub use crate::m_random::*;
pub use crate::v_video::*;
pub use crate::z_zone::*;

use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WipeNum {
    ColorXForm = 0,
    Melt = 1,
}

// Original: wipe_NUMWIPES (count of wipe types)
pub const WIPE_NUM_WIPES: i32 = 2;

#[allow(non_camel_case_types)]
pub struct F_WipeState {
    pub _placeholder: RefCell<()>,
}

impl F_WipeState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    pub fn wipe_start_screen(&self, _x: i32, _y: i32, _width: i32, _height: i32) -> i32 {
        todo!("wipe_StartScreen");
    }

    pub fn wipe_end_screen(&self, _x: i32, _y: i32, _width: i32, _height: i32) -> i32 {
        todo!("wipe_EndScreen");
    }

    pub fn wipe_screen_wipe(
        &self,
        _wipeno: i32,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
        _ticks: i32,
    ) -> i32 {
        todo!("wipe_ScreenWipe");
    }
}
