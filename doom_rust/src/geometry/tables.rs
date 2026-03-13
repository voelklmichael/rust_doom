//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Lookup tables - minimal stub for sound.
//
// Original: tables.h

use crate::m_fixed::{Fixed, FRACUNIT};

pub const FINEANGLES: usize = 8192;
pub const ANGLETOFINESHIFT: i32 = 19;

/// Sine for angle index (0..FINEANGLES-1). Returns fixed_t.
#[inline]
pub fn finesine(index: usize) -> Fixed {
    let idx = index % FINEANGLES;
    let angle_rad = (idx as f64) * 2.0 * std::f64::consts::PI / (FINEANGLES as f64);
    (angle_rad.sin() * (FRACUNIT as f64)) as Fixed
}
