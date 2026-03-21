// tables.h / tables.c — LUT data in tables_data.rs (generated from doomgeneric/tables.c)

pub use crate::doomtype::*;
pub use crate::m_fixed::*;
pub use crate::tables_data::*;

// Original: #define FINEANGLES 8192
pub const FINEANGLES: i32 = 8192;
// Original: #define FINEMASK (FINEANGLES-1)
pub const FINEMASK: i32 = FINEANGLES - 1;

// Original: #define ANGLETOFINESHIFT 19
pub const ANGLETOFINESHIFT: i32 = 19;

// Original: #define ANG45 0x20000000
pub const ANG45: u32 = 0x2000_0000;
pub const ANG90: u32 = 0x4000_0000;
pub const ANG180: u32 = 0x8000_0000;
pub const ANG270: u32 = 0xc000_0000;
pub const ANG_MAX: u32 = 0xffff_ffff;

// Original: #define ANG1 (ANG45 / 45)
pub const ANG1: u32 = ANG45 / 45;
// Original: #define ANG60 (ANG180 / 3)
pub const ANG60: u32 = ANG180 / 3;

// Original: #define ANG1_X 0x01000000
pub const ANG1_X: u32 = 0x0100_0000;

// Original: #define SLOPERANGE 2048
pub const SLOPERANGE: u32 = 2048;
// Original: #define SLOPEBITS 11
pub const SLOPEBITS: i32 = 11;
// Original: #define DBITS (FRACBITS-SLOPEBITS)
pub const DBITS: i32 = FRACBITS - SLOPEBITS;

// Original: typedef unsigned angle_t
pub type AngleT = u32;

#[allow(non_camel_case_types)]
pub struct TablesState;

impl TablesState {
    pub fn new() -> Self {
        Self
    }

    // Original: SlopeDiv
    pub fn slope_div(&self, num: u32, den: u32) -> i32 {
        if den < 512 {
            return SLOPERANGE as i32;
        }
        let ans = (num << 3) / (den >> 8);
        if ans <= SLOPERANGE {
            ans as i32
        } else {
            SLOPERANGE as i32
        }
    }
}

// Original: extern const fixed_t *finecosine (= &finesine[FINEANGLES/4])
#[inline]
pub fn finecosine() -> &'static [FixedT] {
    let o = FINE_COSINE_OFFSET;
    &FINESINE[o..o + FINEANGLES as usize]
}
