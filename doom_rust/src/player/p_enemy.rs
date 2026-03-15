//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Enemy AI, monster thinking, action pointers.
//
// Original: p_enemy.c

use crate::player::mobjs::mobj_index_from_ptr;
use crate::rendering::state;

use super::p_mobj::Mobj;
use super::p_spec::get_next_sector;

/// Alert nearby monsters to target's presence. Original: P_NoiseAlert
/// Sets soundtarget on the target's sector and adjacent sectors so monsters
/// can react to the noise (e.g. when player fires a weapon).
pub fn p_noise_alert(target: *mut Mobj, emitter: *mut Mobj) {
    if target.is_null() || emitter.is_null() {
        return;
    }
    let emitter_idx = match mobj_index_from_ptr(emitter) {
        Some(idx) => idx,
        None => return,
    };
    let sec_idx = {
        let sub_idx = unsafe { (*target).subsector as usize };
        state::with_state(|s| {
            s.subsectors
                .get(sub_idx)
                .map(|sub| sub.sector_idx)
        })
    };
    let sec_idx = match sec_idx {
        Some(idx) => idx,
        None => return,
    };
    // Mark sector and adjacent sectors with sound source
    set_sector_soundtarget(sec_idx, emitter_idx);
    state::with_state(|s| {
        let sec = match s.sectors.get(sec_idx) {
            Some(sec) => sec,
            None => return,
        };
        for &line_idx in &sec.lines {
            if let Some(other_idx) = get_next_sector(line_idx, sec_idx) {
                set_sector_soundtarget(other_idx, emitter_idx);
            }
        }
    });
}

fn set_sector_soundtarget(sec_idx: usize, emitter_idx: crate::player::mobjs::MobjIndex) {
    state::with_state_mut(|s| {
        if let Some(sec) = s.sectors.get_mut(sec_idx) {
            sec.soundtarget = Some(emitter_idx);
        }
    });
}

// TODO: dirtype_t, A_Fall, A_Look, A_Chase, A_FaceTarget, A_* action functions
// NOTE: P_SetMobjState is in p_mobj
