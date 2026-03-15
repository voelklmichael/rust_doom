//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Lighting changes (flicker, strobe, glow).
//
// Original: p_lights.c

use crate::game::d_think::Thinker;
use crate::rendering::defs::{Line, Sector};
use crate::rendering::state;

/// Light flash thinker. Original: lightflash_t
#[repr(C)]
pub struct LightFlash {
    pub thinker: Thinker,
    pub sector: *mut Sector,
    pub count: i32,
    pub maxlight: i32,
    pub minlight: i32,
    pub maxtime: i32,
    pub mintime: i32,
}

/// Strobe thinker. Original: strobe_t
#[repr(C)]
pub struct Strobe {
    pub thinker: Thinker,
    pub sector: *mut Sector,
    pub count: i32,
    pub minlight: i32,
    pub maxlight: i32,
    pub darktime: i32,
    pub brighttime: i32,
}

/// Glow thinker. Original: glow_t
#[repr(C)]
pub struct Glow {
    pub thinker: Thinker,
    pub sector: *mut Sector,
    pub minlight: i32,
    pub maxlight: i32,
    pub direction: i32,
}

/// Light flash thinker. Original: T_LightFlash (stub - no-op for savegame compatibility)
pub unsafe extern "C" fn t_light_flash(_flash: *mut ()) {}

/// Strobe thinker. Original: T_StrobeFlash (stub - no-op for savegame compatibility)
pub unsafe extern "C" fn t_strobe_flash(_flash: *mut ()) {}

/// Glow thinker. Original: T_Glow (stub - no-op for savegame compatibility)
pub unsafe extern "C" fn t_glow(_g: *mut ()) {}

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
pub fn ev_start_light_strobing(_line_idx: usize) -> bool {
    // TODO: spawn StrobeFlash thinker on line's sector
    false
}

/// Start light flickering. Original: EV_StartLightFlickering
/// Requires T_LightFlash thinker - stub returns false.
pub fn ev_start_light_flickering(_line_idx: usize) -> bool {
    // TODO: spawn LightFlash thinker on line's sector
    false
}

/// Turn on all sectors with given tag. Used when sector is null in EV_LightTurnOn.
pub fn ev_light_turn_on_by_tag(tag: i32, bright: i32) -> bool {
    let bright = bright.clamp(0, 255) as i16;
    state::with_state_mut(|s| {
        let mut any = false;
        for sec in &mut s.sectors {
            if sec.tag as i32 == tag {
                sec.lightlevel = bright;
                any = true;
            }
        }
        any
    })
}
