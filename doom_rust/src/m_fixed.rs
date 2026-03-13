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
