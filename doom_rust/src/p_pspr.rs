//! Rust translation of doomgeneric/p_pspr.h
//! Sprite animation.

use crate::info::*;
use crate::m_fixed::*;
use crate::tables::*;

/// C #define: FF_FULLBRIGHT
pub const FF_FULLBRIGHT: i32 = 0x8000;
/// C #define: FF_FRAMEMASK
pub const FF_FRAMEMASK: i32 = 0x7fff;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: psprnum_t
pub enum PsprnumT {
    PsWeapon,
    PsFlash,
    Numpsprites,
}

/// C #define: NUMPSPRITES
pub const NUMPSPRITES: usize = 2;

/// pspdef_t
#[repr(C)]
#[derive(Clone, Copy)]
/// C typedef: pspdef_t
pub struct PspdefT {
    pub state: *mut StateT,
    pub tics: i32,
    pub sx: FixedT,
    pub sy: FixedT,
}
