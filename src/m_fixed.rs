//! Fixed-point arithmetic (m_fixed.h, m_fixed.c)
//! Original: m_fixed.h, m_fixed.c

// #define FRACBITS 16
pub const FRACBITS: i32 = 16;
// #define FRACUNIT (1<<FRACBITS)
pub const FRACUNIT: i32 = 1 << FRACBITS;

// typedef int fixed_t
pub type FixedT = i32;

pub struct M_FixedState;

impl M_FixedState {
    /// Original: fixed_t FixedMul(fixed_t a, fixed_t b)
    pub fn fixed_mul(&self, _a: FixedT, _b: FixedT) -> FixedT {
        // C body:
        // return ((int64_t) a * (int64_t) b) >> FRACBITS;
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t FixedDiv(fixed_t a, fixed_t b)
    pub fn fixed_div(&self, _a: FixedT, _b: FixedT) -> FixedT {
        // C body:
        // if ((abs(a) >> 14) >= abs(b))
        //     return (a^b) < 0 ? INT_MIN : INT_MAX;
        // else
        //     return (fixed_t)(((int64_t) a << 16) / b);
        todo!("Basic stage-0 stub")
    }
}
