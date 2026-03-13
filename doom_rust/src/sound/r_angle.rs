//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Angle utilities - R_PointToAngle2 for sound stereo positioning.
//
// Original: r_main.c (R_PointToAngle2, R_PointToAngle)

use crate::m_fixed::Fixed;

pub type Angle = u32;

pub const ANG45: Angle = 0x2000_0000;
pub const ANG90: Angle = 0x4000_0000;
pub const ANG180: Angle = 0x8000_0000;
pub const ANG270: Angle = 0xC000_0000;
pub const ANG_MAX: Angle = 0xFFFF_FFFF;

const SLOPERANGE: i32 = 2048;

/// SlopeDiv - from tables.c. Maps num/den to slope index.
fn slope_div(num: u32, den: u32) -> i32 {
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

/// ArcTan lookup - linear interpolation for simplicity.
/// tantoangle[slope] maps tan to angle in first octant.
fn tantoangle(slope: i32) -> Angle {
    if slope <= 0 {
        return 0;
    }
    if slope >= SLOPERANGE {
        return (ANG90 / 4) as Angle; // ~22.5 degrees in first octant
    }
    // Linear: angle = slope * (ANG90/4) / SLOPERANGE
    ((slope as u64 * (ANG90 as u64 / 4) / SLOPERANGE as u64) & 0xFFFF_FFFF) as Angle
}

/// Angle from (x1,y1) to (x2,y2). Returns angle_t (0 = east, ANG90 = south).
pub fn r_point_to_angle2(x1: Fixed, y1: Fixed, x2: Fixed, y2: Fixed) -> Angle {
    let dx = (x2 - x1) as i64;
    let dy = (y2 - y1) as i64;

    if dx == 0 && dy == 0 {
        return 0;
    }

    let x = dx.abs() as u32;
    let y = dy.abs() as u32;

    if dx >= 0 {
        if dy >= 0 {
            if x > y {
                tantoangle(slope_div(y, x) as i32)
            } else {
                ANG90 - 1 - tantoangle(slope_div(x, y) as i32)
            }
        } else {
            if x > y {
                0u32.wrapping_sub(tantoangle(slope_div(y, x) as i32))
            } else {
                ANG270 + tantoangle(slope_div(x, y) as i32)
            }
        }
    } else {
        if dy >= 0 {
            if x > y {
                ANG180 - 1 - tantoangle(slope_div(y, x) as i32)
            } else {
                ANG90 + tantoangle(slope_div(x, y) as i32)
            }
        } else {
            if x > y {
                ANG180 + tantoangle(slope_div(y, x) as i32)
            } else {
                ANG270 - 1 - tantoangle(slope_div(x, y) as i32)
            }
        }
    }
}
