//! Rust translation of doomgeneric/m_fixed.h
//! Fixed point arithmetic, implementation.

/// C #define: FRACBITS
pub const FRACBITS: i32 = 16;

/// C #define: FRACUNIT
pub const FRACUNIT: i32 = 1 << FRACBITS;

/// C typedef: fixed_t = int (32-bit fixed point 16.16)
pub type FixedT = i32;

/// C function: FixedMul
pub fn fixed_mul(a: FixedT, b: FixedT) -> FixedT {
    todo!("original: FixedMul")
}

/// C function: FixedDiv
pub fn fixed_div(a: FixedT, b: FixedT) -> FixedT {
    todo!("original: FixedDiv")
}
