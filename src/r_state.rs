// doomgeneric/r_state.h — refresh internal state (stub; no r_data import to break cycle)

pub use crate::d_player::*;
pub use crate::doomtype::*;
pub use crate::m_fixed::*;
pub use crate::r_defs::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct R_StateState {
    /// Original: viewwidth
    pub viewwidth: RefCell<i32>,
    /// Original: viewheight
    pub viewheight: RefCell<i32>,
    /// Original: scaledviewwidth
    pub scaledviewwidth: RefCell<i32>,
    /// Original: numsprites
    pub numsprites: RefCell<i32>,
    /// Original: numvertexes
    pub numvertexes: RefCell<i32>,
    /// Original: numsegs
    pub numsegs: RefCell<i32>,
    /// Original: numsectors
    pub numsectors: RefCell<i32>,
}

impl R_StateState {
    pub fn new() -> Self {
        Self {
            viewwidth: RefCell::new(0),
            viewheight: RefCell::new(0),
            scaledviewwidth: RefCell::new(0),
            numsprites: RefCell::new(0),
            numvertexes: RefCell::new(0),
            numsegs: RefCell::new(0),
            numsectors: RefCell::new(0),
        }
    }
}
