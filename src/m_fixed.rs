// m_fixed.h / m_fixed.c

pub use crate::doomtype::*;

// Original: #define FRACBITS 16
pub const FRACBITS: i32 = 16;
// Original: #define FRACUNIT (1<<FRACBITS)
pub const FRACUNIT: i32 = 1 << FRACBITS;

// Original: typedef int fixed_t
pub type FixedT = i32;

#[allow(non_camel_case_types)]
pub struct M_FixedState;

impl M_FixedState {
    pub fn new() -> Self {
        Self
    }

    // Original: FixedMul
    pub fn fixed_mul(&self, a: FixedT, b: FixedT) -> FixedT {
        ((a as i64 * b as i64) >> FRACBITS) as FixedT
    }

    // Original: FixedDiv — C body >10 lines
    pub fn fixed_div(&self, _a: FixedT, _b: FixedT) -> FixedT {
        todo!("FixedDiv")
    }
}
