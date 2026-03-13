//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 1993-2008 Raven Software
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Lookup tables.
//  Do not try to look them up :-).
//  In the order of appearance:
//  - finetangent[4096]  - Tangens LUT
//  - finesine[10240]    - Sine lookup (serves as cosine too)
//  - tantoangle[2049]   - ArcTan LUT, maps tan(angle) to angle fast
//
// Original: tables.h / tables.c

use crate::m_fixed::{Fixed, FRACBITS};

use super::tables_data::{FINESINE, FINETANGENT, GAMMATABLE, TANTOANGLE};

// =============================================================================
// Public API (tables.h)
// =============================================================================

/// Fine angle count.
pub const FINEANGLES: usize = 8192;
/// Mask for fine angle index (FINEANGLES - 1).
pub const FINEMASK: usize = FINEANGLES - 1;

/// Shift to convert angle to fine angle index. 0x100000000 to 0x2000.
pub const ANGLETOFINESHIFT: i32 = 19;

/// Sine lookup table. Effective size 5*FINEANGLES/4 = 10240.
/// Index with 0..FINEANGLES-1 for sine; add FINEANGLES/4 for cosine.
pub fn finesine(index: usize) -> Fixed {
    FINESINE[index % (5 * FINEANGLES / 4)]
}

/// Cosine as slice into finesine (PI/2 phase shift). Re-use data.
pub fn finecosine(index: usize) -> Fixed {
    FINESINE[(index + FINEANGLES / 4) % (5 * FINEANGLES / 4)]
}

/// Tangent lookup. Effective size FINEANGLES/2 = 4096.
pub fn finetangent(index: usize) -> Fixed {
    FINETANGENT[index % (FINEANGLES / 2)]
}

/// Gamma correction tables. Index [gamma_level][pixel_value].
pub const fn gammatable(gamma: usize, index: usize) -> u8 {
    GAMMATABLE[gamma % 5][index % 256]
}

/// Binary Angle Measurement.
pub type Angle = u32;

pub const ANG45: Angle = 0x2000_0000;
pub const ANG90: Angle = 0x4000_0000;
pub const ANG180: Angle = 0x8000_0000;
pub const ANG270: Angle = 0xC000_0000;
pub const ANG_MAX: Angle = 0xFFFF_FFFF;

/// One degree (ANG45 / 45).
pub const ANG1: Angle = ANG45 / 45;
/// 60 degrees (ANG180 / 3).
pub const ANG60: Angle = ANG180 / 3;

/// Heretic: ~1.40 degrees (not exactly one degree).
pub const ANG1_X: Angle = 0x0100_0000;

pub const SLOPERANGE: i32 = 2048;
pub const SLOPEBITS: i32 = 11;
pub const DBITS: i32 = FRACBITS - SLOPEBITS;

/// ArcTan lookup. Maps slope (0..SLOPERANGE) to angle. Size SLOPERANGE+1 for x==y case.
pub fn tantoangle(slope: usize) -> Angle {
    let idx = slope.min(SLOPERANGE as usize);
    TANTOANGLE[idx]
}

/// SlopeDiv - maps num/den to slope index. Called by R_PointToAngle.
pub fn slope_div(num: u32, den: u32) -> i32 {
    if den < 512 {
        SLOPERANGE
    } else {
        let ans = (num << 3) / (den >> 8);
        if ans <= SLOPERANGE as u32 {
            ans as i32
        } else {
            SLOPERANGE
        }
    }
}
