//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Enemy AI, monster thinking, action pointers.
//
// Original: p_enemy.c

use crate::rendering::defs::{Sector, Subsector};

use super::p_mobj::Mobj;
use super::p_spec::get_next_sector;

/// Alert nearby monsters to target's presence. Original: P_NoiseAlert
/// Sets soundtarget on the target's sector and adjacent sectors so monsters
/// can react to the noise (e.g. when player fires a weapon).
pub fn p_noise_alert(target: *mut Mobj, emitter: *mut Mobj) {
    if target.is_null() || emitter.is_null() {
        return;
    }
    let ss = unsafe { (*target).subsector };
    if ss.is_null() {
        return;
    }
    let sec = unsafe { (*(ss as *const Subsector)).sector };
    if sec.is_null() {
        return;
    }
    // Mark sector and adjacent sectors with sound source
    set_sector_soundtarget(sec, emitter);
    let linecount = unsafe { (*sec).linecount } as usize;
    let lines = unsafe { (*sec).lines };
    if lines.is_null() {
        return;
    }
    for i in 0..linecount {
        let line = unsafe { *lines.add(i) };
        if line.is_null() {
            continue;
        }
        let other = get_next_sector(line, sec);
        if !other.is_null() {
            set_sector_soundtarget(other, emitter);
        }
    }
}

fn set_sector_soundtarget(sec: *mut Sector, emitter: *mut Mobj) {
    if sec.is_null() {
        return;
    }
    unsafe {
        (*sec).soundtarget = emitter;
    }
}

// TODO: dirtype_t, A_Fall, A_Look, A_Chase, A_FaceTarget, A_* action functions
// NOTE: P_SetMobjState is in p_mobj
