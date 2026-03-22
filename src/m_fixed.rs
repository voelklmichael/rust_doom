//! Fixed-point arithmetic (m_fixed.h, m_fixed.c)
//! Original: m_fixed.h, m_fixed.c

use std::sync::Arc;
use std::sync::Mutex;

// #define FRACBITS 16
pub const FRACBITS: i32 = 16;
// #define FRACUNIT (1<<FRACBITS)
pub const FRACUNIT: i32 = 1 << FRACBITS;

// typedef int fixed_t
pub type FixedT = i32;

pub struct M_FixedState;

impl M_FixedState {
    /// Original: fixed_t FixedMul(fixed_t a, fixed_t b)
    pub fn fixed_mul(&self, a: FixedT, b: FixedT) -> FixedT {
        // C body: (from m_fixed.c)
        // return ((int64_t) a * (int64_t) b) >> FRACBITS;
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t FixedDiv(fixed_t a, fixed_t b)
    pub fn fixed_div(&self, a: FixedT, b: FixedT) -> FixedT {
        // C body: (from m_fixed.c)
        // return ((int64_t) a << FRACBITS) / b;
        todo!("Basic stage-0 stub")
    }
}
