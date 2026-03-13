//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Map Objects - things in the world (monsters, items, etc.).
//
// Original: p_mobj.h / p_mobj.c

use crate::doomdata::MapThing;
use crate::game::d_think::Thinker;
use crate::info::types::{Mobjinfo, Mobjtype, State};
use crate::m_fixed::{Fixed, FRACUNIT};
use crate::z_zone::{z_malloc, PU_LEVEL};

use std::ptr;

// =============================================================================
// Mobj flags
// =============================================================================

/// MF_SHADOW - render as shadow (invisible player).
pub const MF_SHADOW: i32 = 32;
/// MF_NOSECTOR - don't use sector links (invisible but touchable).
pub const MF_NOSECTOR: i32 = 8;
/// MF_NOBLOCKMAP - don't use blocklinks (inert but displayable).
pub const MF_NOBLOCKMAP: i32 = 16;
/// MF_SOLID - blocks movement.
pub const MF_SOLID: i32 = 2;
/// MF_SPECIAL - touch activates (pickups).
pub const MF_SPECIAL: i32 = 1;
/// MF_SHOOTABLE - can be damaged.
pub const MF_SHOOTABLE: i32 = 4;
/// MF_PICKUP - player can pick up items.
pub const MF_PICKUP: i32 = 0x800;
/// MF_NOCLIP - walk through walls.
pub const MF_NOCLIP: i32 = 0x1000;
/// MF_TELEPORT - don't cross special lines.
pub const MF_TELEPORT: i32 = 0x8000;
/// MF_DROPOFF - allow dropoffs.
pub const MF_DROPOFF: i32 = 0x400;
/// MF_FLOAT - floating (no gravity).
pub const MF_FLOAT: i32 = 0x4000;
/// MF_MISSILE - projectile.
pub const MF_MISSILE: i32 = 0x10000;
/// MF_SPAWNCEILING - spawn on ceiling.
pub const MF_SPAWNCEILING: i32 = 256;
/// FF_FRAMEMASK - frame index mask.
pub const FF_FRAMEMASK: i32 = 0x7fff;
/// FF_FULLBRIGHT - full bright sprite.
pub const FF_FULLBRIGHT: i32 = 0x8000;

// =============================================================================
// Mobj struct - Thinker must be first for P_AddThinker cast
// =============================================================================

/// Map object - thing in the world (monster, item, projectile, etc.).
/// Layout matches C mobj_t; Thinker is first so mobj* casts to thinker*.
#[repr(C)]
#[derive(Debug)]
pub struct Mobj {
    pub thinker: Thinker,
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
    pub snext: *mut Mobj,
    pub sprev: *mut Mobj,
    pub angle: u32,
    pub sprite: i32,
    pub frame: i32,
    pub bnext: *mut Mobj,
    pub bprev: *mut Mobj,
    pub subsector: *mut std::ffi::c_void,
    pub floorz: Fixed,
    pub ceilingz: Fixed,
    pub radius: Fixed,
    pub height: Fixed,
    pub momx: Fixed,
    pub momy: Fixed,
    pub momz: Fixed,
    pub validcount: i32,
    pub type_: Mobjtype,
    pub info: *const Mobjinfo,
    pub tics: i32,
    pub state: *const State,
    pub flags: i32,
    pub health: i32,
    pub movedir: i32,
    pub movecount: i32,
    pub target: *mut Mobj,
    pub reactiontime: i32,
    pub threshold: i32,
    pub player: *mut std::ffi::c_void,
    pub lastlook: i32,
    pub spawnpoint: MapThing,
    pub tracer: *mut Mobj,
}

/// MobjThinker - called each tic. Stub for now (no P_XYMovement, P_ZMovement).
pub unsafe extern "C" fn p_mobj_thinker(mobj: *mut ()) {
    let _ = mobj;
    // TODO: P_XYMovement, P_ZMovement, state tics
}

/// Spawn a map object at (x,y,z) of given type. Original: P_SpawnMobj
pub fn p_spawn_mobj(x: Fixed, y: Fixed, z: Fixed, type_: Mobjtype) -> *mut Mobj {
    use crate::info::{states, MOBJINFO, NUMMOBJTYPES};
    use super::p_maputl::p_set_thing_position;
    use super::p_tick::p_add_thinker;

    if (type_ as usize) >= NUMMOBJTYPES {
        return ptr::null_mut();
    }

    let info = &MOBJINFO[type_ as usize];
    let ptr = z_malloc(std::mem::size_of::<Mobj>(), PU_LEVEL, ptr::null_mut()) as *mut Mobj;
    if ptr.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        ptr::write_bytes(ptr as *mut u8, 0, std::mem::size_of::<Mobj>());
    }

    let states_ref = states();
    let spawnstate = info.spawnstate as usize;
    if spawnstate >= states_ref.len() {
        unsafe { return ptr };
    }
    let st = &states_ref[spawnstate];

    let radius_fixed = (info.radius as i32) * FRACUNIT;
    let height_fixed = (info.height as i32) * FRACUNIT;

    unsafe {
        (*ptr).type_ = type_;
        (*ptr).info = info as *const Mobjinfo;
        (*ptr).x = x;
        (*ptr).y = y;
        (*ptr).radius = radius_fixed;
        (*ptr).height = height_fixed;
        (*ptr).flags = info.flags;
        (*ptr).health = info.spawnhealth;
        (*ptr).reactiontime = info.reactiontime;
        (*ptr).state = st as *const State;
        (*ptr).tics = st.tics;
        (*ptr).sprite = st.sprite;
        (*ptr).frame = st.frame;
        (*ptr).thinker.function.acp1 = p_mobj_thinker;
    }

    p_set_thing_position(ptr);

    unsafe {
        (*ptr).floorz = {
            let ss = (*ptr).subsector.cast::<crate::rendering::defs::Subsector>();
            if ss.is_null() {
                0
            } else {
                let sec = (*ss).sector;
                if sec.is_null() {
                    0
                } else {
                    (*sec).floorheight
                }
            }
        };
        (*ptr).ceilingz = {
            let ss = (*ptr).subsector.cast::<crate::rendering::defs::Subsector>();
            if ss.is_null() {
                0
            } else {
                let sec = (*ss).sector;
                if sec.is_null() {
                    0
                } else {
                    (*sec).ceilingheight
                }
            }
        };
    }

    unsafe {
        if z == crate::player::ONFLOORZ {
            (*ptr).z = (*ptr).floorz;
        } else if z == crate::player::ONCEILINGZ {
            (*ptr).z = (*ptr).ceilingz - height_fixed;
        } else {
            (*ptr).z = z;
        }
    }

    p_add_thinker(unsafe { &mut (*ptr).thinker as *mut Thinker });

    ptr
}

/// Remove mobj from world and thinker list. Original: P_RemoveMobj
pub fn p_remove_mobj(mobj: *mut Mobj) {
    if mobj.is_null() {
        return;
    }
    super::p_maputl::p_unset_thing_position(mobj);
    super::p_tick::p_remove_thinker(unsafe { &mut (*mobj).thinker as *mut Thinker });
}

/// Spawn a map thing (from THINGS lump). Original: P_SpawnMapThing
/// Skips player starts (1-4), deathmatch (11), unknown types. Spawns monsters/items by doomednum.
pub fn p_spawn_map_thing(mthing: &MapThing) {
    use crate::geometry::ANG45;
    use crate::info::{MOBJINFO, NUMMOBJTYPES};

    let mt_type = mthing.type_ as i32;

    // Deathmatch start (type 11)
    if mt_type == 11 {
        return;
    }
    // Thing type 0 = "player -1" - ignore
    if mt_type <= 0 {
        return;
    }
    // Player starts (1-4) - skip for now (no P_SpawnPlayer)
    if mt_type <= 4 {
        return;
    }

    // Find mobj type by doomednum
    let mut mobj_type: Option<Mobjtype> = None;
    for i in 0..NUMMOBJTYPES {
        if mt_type == MOBJINFO[i].doomednum {
            mobj_type = Some(i as Mobjtype);
            break;
        }
    }

    let mobj_type = match mobj_type {
        Some(t) => t,
        None => return, // Unknown type - skip
    };

    let info = &MOBJINFO[mobj_type as usize];

    // options & 16 = don't spawn in single player
    if (mthing.options as i32 & 16) != 0 {
        return;
    }
    // Skill: bits 1=easy, 2=normal, 4=hard, 8=nightmare. Spawn if any skill bit set.
    let skill_bits = mthing.options as i32 & 0xF;
    if skill_bits == 0 {
        return;
    }

    let x = (mthing.x as i32) * FRACUNIT;
    let y = (mthing.y as i32) * FRACUNIT;
    let z = if (info.flags & MF_SPAWNCEILING) != 0 {
        crate::player::ONCEILINGZ
    } else {
        crate::player::ONFLOORZ
    };

    let mobj = p_spawn_mobj(x, y, z, mobj_type);
    if mobj.is_null() {
        return;
    }

    unsafe {
        (*mobj).spawnpoint = *mthing;
        (*mobj).angle = (ANG45 as u32).wrapping_mul((mthing.angle as i32).max(0) as u32 / 45);
    }
}
