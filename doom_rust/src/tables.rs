//! Rust translation of doomgeneric/tables.h

use crate::doomtype::*;
use crate::m_fixed::*;

/// C #define: FINEANGLES
pub const FINEANGLES: i32 = 8192;

/// C #define: FINEMASK
pub const FINEMASK: i32 = FINEANGLES - 1;

/// C #define: ANGLETOFINESHIFT
pub const ANGLETOFINESHIFT: i32 = 19;

/// C extern: finesine
pub static finesine: [FixedT; 10240] = [0; 10240]; // 5*FINEANGLES/4

/// C extern: finecosine (use .as_ptr() for *const FixedT)
pub static finecosine: &[FixedT] = &[];

/// C extern: finetangent
pub static finetangent: [FixedT; 4096] = [0; 4096]; // FINEANGLES/2

/// C extern: gammatable
pub static gammatable: [[byte; 256]; 5] = [[0; 256]; 5];

/// C #define: ANG45
pub const ANG45: u32 = 0x2000_0000;
/// C #define: ANG90
pub const ANG90: u32 = 0x4000_0000;
/// C #define: ANG180
pub const ANG180: u32 = 0x8000_0000;
/// C #define: ANG270
pub const ANG270: u32 = 0xc000_0000;
/// C #define: ANG_MAX
pub const ANG_MAX: u32 = 0xffff_ffff;
/// C #define: ANG1
pub const ANG1: u32 = ANG45 / 45;
/// C #define: ANG60
pub const ANG60: u32 = ANG180 / 3;
/// C #define: ANG1_X
pub const ANG1_X: u32 = 0x0100_0000;

/// C #define: SLOPERANGE
pub const SLOPERANGE: i32 = 2048;
/// C #define: SLOPEBITS
pub const SLOPEBITS: i32 = 11;
/// C #define: DBITS
pub const DBITS: i32 = FRACBITS - SLOPEBITS;

/// C typedef: angle_t
pub type AngleT = u32;

/// C extern: tantoangle
pub static tantoangle: [AngleT; 2049] = [0; 2049]; // SLOPERANGE+1

/// C function: SlopeDiv
pub fn slope_div(num: u32, den: u32) -> i32 {
    todo!("original: SlopeDiv")
}
