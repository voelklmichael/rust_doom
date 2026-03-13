//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Fixed point arithmetic.
//
// Original: m_fixed.h

pub const FRACBITS: i32 = 16;
pub const FRACUNIT: i32 = 1 << FRACBITS;

pub type Fixed = i32;

/// Fixed-point multiply. Original: FixedMul
#[inline]
pub fn fixed_mul(a: Fixed, b: Fixed) -> Fixed {
    ((a as i64 * b as i64) >> FRACBITS) as Fixed
}

/// Fixed-point divide. Original: FixedDiv
#[inline]
pub fn fixed_div(a: Fixed, b: Fixed) -> Fixed {
    if b == 0 {
        return if (a ^ b) < 0 {
            i32::MIN
        } else {
            i32::MAX
        };
    }
    if (a.abs() >> 14) >= b.abs() {
        return if (a ^ b) < 0 {
            i32::MIN
        } else {
            i32::MAX
        };
    }
    (((a as i64) << 16) / (b as i64)) as Fixed
}
