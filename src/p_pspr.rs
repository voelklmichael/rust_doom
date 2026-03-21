// doomgeneric/p_pspr.h — player sprite / psprite definitions

pub use crate::info::*;
pub use crate::m_fixed::*;
pub use crate::tables::*;

// Original: #define FF_FULLBRIGHT 0x8000
pub const FF_FULLBRIGHT: i32 = 0x8000;
// Original: #define FF_FRAMEMASK 0x7fff
pub const FF_FRAMEMASK: i32 = 0x7fff;

/// Original: typedef enum { ps_weapon, ps_flash, NUMPSPRITES } psprnum_t
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PsprnumT {
    PsWeapon = 0,
    PsFlash = 1,
    NUMPSPRITES = 2,
}

pub const NUMPSPRITES: usize = PsprnumT::NUMPSPRITES as usize;

/// Original: typedef struct { state_t *state; int tics; fixed_t sx, sy; } pspdef_t
#[repr(C)]
pub struct PspdefT {
    // Original: state
    pub state: *mut StateT,
    // Original: tics
    pub tics: i32,
    // Original: sx
    pub sx: FixedT,
    // Original: sy
    pub sy: FixedT,
}

#[allow(non_camel_case_types)]
pub struct P_PsprState;

impl P_PsprState {
    pub fn new() -> Self {
        Self
    }
}
