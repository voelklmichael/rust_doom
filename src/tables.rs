//! Lookup tables (tables.h, tables.c)
//! Original: tables.h, tables.c

use crate::m_fixed::FixedT;
use crate::doomtype::Byte;

pub const FINEANGLES: usize = 8192;
pub const FINEMASK: usize = FINEANGLES - 1;
pub const ANGLETOFINESHIFT: i32 = 19;

pub const ANG45: u32 = 0x20000000;
pub const ANG90: u32 = 0x40000000;
pub const ANG180: u32 = 0x80000000;
pub const ANG270: u32 = 0xc0000000;
pub const ANG_MAX: u32 = 0xffffffff;
pub const ANG1: u32 = ANG45 / 45;
pub const ANG60: u32 = ANG180 / 3;
pub const ANG1_X: u32 = 0x01000000;

pub const SLOPERANGE: i32 = 2048;
pub const SLOPEBITS: i32 = 11;

pub struct TablesState;

impl TablesState {
    /// Original: int SlopeDiv(unsigned int num, unsigned int den)
    pub fn slope_div(&self, _num: u32, _den: u32) -> i32 {
        todo!("Basic stage-0 stub")
    }
}
