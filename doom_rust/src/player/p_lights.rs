//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Lighting changes (flicker, strobe, glow).
//
// Original: p_lights.c

use crate::rendering::defs::{Line, Sector};
use crate::rendering::state;

/// Turn on sector lights. Original: EV_LightTurnOn
/// If sector is null, finds all sectors with tag from line and sets their lightlevel.
/// Returns true if any sector was modified.
pub fn ev_light_turn_on(sector: *mut Sector, bright: i32) -> bool {
    if !sector.is_null() {
        unsafe {
            (*sector).lightlevel = bright.clamp(0, 255) as i16;
        }
        return true;
    }
    false
}

/// Start light strobing. Original: EV_StartLightStrobing
/// Requires T_StrobeFlash thinker - stub returns false.
pub fn ev_start_light_strobing(_line: *const Line) -> bool {
    // TODO: spawn StrobeFlash thinker on line's sector
    false
}

/// Start light flickering. Original: EV_StartLightFlickering
/// Requires T_LightFlash thinker - stub returns false.
pub fn ev_start_light_flickering(_line: *const Line) -> bool {
    // TODO: spawn LightFlash thinker on line's sector
    false
}

/// Turn on all sectors with given tag. Used when sector is null in EV_LightTurnOn.
pub fn ev_light_turn_on_by_tag(tag: i32, bright: i32) -> bool {
    let sectors = unsafe { state::SECTORS };
    let numsectors = unsafe { state::NUMSECTORS };
    if sectors.is_null() || numsectors <= 0 {
        return false;
    }
    let bright = bright.clamp(0, 255) as i16;
    let mut any = false;
    for i in 0..(numsectors as usize) {
        let sec = unsafe { sectors.add(i) };
        if unsafe { (*sec).tag as i32 } == tag {
            unsafe {
                (*sec).lightlevel = bright;
            }
            any = true;
        }
    }
    any
}
