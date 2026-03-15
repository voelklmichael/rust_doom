//! Rust translation of doomgeneric/p_pspr.h
//! Sprite animation.

use crate::info::*;
use crate::m_fixed::*;
use crate::tables::*;

pub const FF_FULLBRIGHT: i32 = 0x8000;
pub const FF_FRAMEMASK: i32 = 0x7fff;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PsprnumT {
    PsWeapon,
    PsFlash,
    Numpsprites,
}

pub const NUMPSPRITES: usize = 2;

/// pspdef_t
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PspdefT {
    pub state: *mut StateT,
    pub tics: i32,
    pub sx: FixedT,
    pub sy: FixedT,
}
