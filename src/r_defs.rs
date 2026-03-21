// doomgeneric/r_defs.h — minimal shared render/map defs (stub)

pub use crate::d_think::*;
pub use crate::doomdef::*;
pub use crate::doomtype::*;
pub use crate::i_video::*;
pub use crate::m_fixed::*;
pub use crate::p_mobj::*;
pub use crate::v_patch::*;

// Original: #define SIL_NONE ...
pub const SIL_NONE: i32 = 0;
pub const SIL_BOTTOM: i32 = 1;
pub const SIL_TOP: i32 = 2;
pub const SIL_BOTH: i32 = 3;

// Original: #define MAXDRAWSEGS 256
pub const MAXDRAWSEGS: usize = 256;

/// Original: lighttable_t (opaque byte LUT)
pub type LighttableT = Byte;

/// Original: typedef unsigned angle_t
pub use crate::tables::AngleT;

// Opaque / stub structs (full layout deferred)
#[repr(C)]
pub struct NodeT {
    _opaque: u8,
}

#[repr(C)]
pub struct SegT {
    _opaque: u8,
}

#[repr(C)]
pub struct SectorT {
    _opaque: u8,
}

#[repr(C)]
pub struct LineT {
    _opaque: u8,
}

#[repr(C)]
pub struct SideT {
    _opaque: u8,
}

#[repr(C)]
pub struct SubsectorT {
    _opaque: u8,
}

#[repr(C)]
pub struct DrawsegT {
    _opaque: u8,
}

#[repr(C)]
pub struct SpritedefT {
    _opaque: u8,
}

/// Original: vertex_t
#[repr(C)]
pub struct VertexT {
    pub x: FixedT,
    pub y: FixedT,
}

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct R_DefsState {
    pub _placeholder: RefCell<()>,
}

impl R_DefsState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }
}
