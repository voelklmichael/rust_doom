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
/// MF_NOGRAVITY - no gravity.
pub const MF_NOGRAVITY: i32 = 512;
/// MF_CORPSE - dead body.
pub const MF_CORPSE: i32 = 0x100000;
/// MF_SKULLFLY - skull in flight.
pub const MF_SKULLFLY: i32 = 0x1000000;
/// FF_FRAMEMASK - frame index mask.
pub const FF_FRAMEMASK: i32 = 0x7fff;
/// FF_FULLBRIGHT - full bright sprite.
pub const FF_FULLBRIGHT: i32 = 0x8000;
/// MF_TRANSSHIFT - shift for player color translation (multiplayer).
pub const MF_TRANSSHIFT: i32 = 26;

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

/// MobjThinker - called each tic. Original: P_MobjThinker
pub unsafe extern "C" fn p_mobj_thinker(mobj: *mut ()) {
    let mo = mobj as *mut Mobj;
    if mo.is_null() {
        return;
    }
    if (*mo).momx != 0 || (*mo).momy != 0 || ((*mo).flags & MF_SKULLFLY) != 0 {
        p_xy_movement(mo);
    }
    if (*mo).z != (*mo).floorz || (*mo).momz != 0 {
        p_z_movement(mo);
    }
    // State tic countdown and advancement
    if (*mo).tics != -1 {
        (*mo).tics -= 1;
        if (*mo).tics == 0 {
            let state = (*mo).state;
            if state.is_null() {
                return;
            }
            let nextstate = (*state).nextstate;
            if !p_set_mobj_state(mo, nextstate) {
                return; // freed itself
            }
        }
    }
}

const STOPSPEED: Fixed = 0x1000;
const FRICTION: Fixed = 0xe800;

/// XY movement with collision. Original: P_XYMovement
fn p_xy_movement(mo: *mut Mobj) {
    use crate::m_fixed::fixed_mul;
    use super::p_map::{p_slide_move, p_try_move};
    use super::MAXMOVE;

    if mo.is_null() {
        return;
    }

    unsafe {
        if (*mo).momx == 0 && (*mo).momy == 0 {
            if ((*mo).flags & MF_SKULLFLY) != 0 {
                (*mo).flags &= !MF_SKULLFLY;
                (*mo).momx = 0;
                (*mo).momy = 0;
                (*mo).momz = 0;
                let spawnstate = if (*mo).info.is_null() {
                    1
                } else {
                    (*(*mo).info).spawnstate
                };
                p_set_mobj_state(mo, spawnstate);
            }
            return;
        }

        (*mo).momx = (*mo).momx.clamp(-MAXMOVE, MAXMOVE);
        (*mo).momy = (*mo).momy.clamp(-MAXMOVE, MAXMOVE);
    }

    let mut xmove = unsafe { (*mo).momx };
    let mut ymove = unsafe { (*mo).momy };

    loop {
        let (ptryx, ptryy, done) = if xmove > MAXMOVE / 2 || ymove > MAXMOVE / 2 {
            let ptryx = unsafe { (*mo).x + xmove / 2 };
            let ptryy = unsafe { (*mo).y + ymove / 2 };
            xmove >>= 1;
            ymove >>= 1;
            (ptryx, ptryy, false)
        } else {
            let ptryx = unsafe { (*mo).x + xmove };
            let ptryy = unsafe { (*mo).y + ymove };
            xmove = 0;
            ymove = 0;
            (ptryx, ptryy, true)
        };

        if !p_try_move(mo, ptryx, ptryy) {
            let player = unsafe { (*mo).player };
            if !player.is_null() {
                p_slide_move(mo);
            } else if (unsafe { (*mo).flags } & MF_MISSILE) != 0 {
                // Sky check: if missile hit sky ceiling, remove instead of explode.
                // Skip for now - always explode (SKYFLATNUM requires r_sky which is private).
                p_explode_missile(mo);
            } else {
                unsafe {
                    (*mo).momx = 0;
                    (*mo).momy = 0;
                }
            }
        }

        if done {
            break;
        }
    }

    unsafe {
        if ((*mo).flags & (MF_MISSILE | MF_SKULLFLY)) != 0 {
            return;
        }
        if (*mo).z > (*mo).floorz {
            return;
        }
        if ((*mo).flags & MF_CORPSE) != 0 {
            let momx = (*mo).momx;
            let momy = (*mo).momy;
            if momx > FRACUNIT / 4 || momx < -FRACUNIT / 4 || momy > FRACUNIT / 4 || momy < -FRACUNIT / 4 {
                let ss = (*mo).subsector.cast::<crate::rendering::defs::Subsector>();
                if !ss.is_null() {
                    let sec = (*ss).sector;
                    if !sec.is_null() && (*mo).floorz != (*sec).floorheight {
                        return;
                    }
                }
            }
        }
        if (*mo).momx > -STOPSPEED
            && (*mo).momx < STOPSPEED
            && (*mo).momy > -STOPSPEED
            && (*mo).momy < STOPSPEED
        {
            (*mo).momx = 0;
            (*mo).momy = 0;
        } else {
            (*mo).momx = fixed_mul((*mo).momx, FRICTION);
            (*mo).momy = fixed_mul((*mo).momy, FRICTION);
        }
    }
}

/// Z movement and gravity. Original: P_ZMovement
fn p_z_movement(mo: *mut Mobj) {
    use super::{FLOATSPEED, GRAVITY};

    if mo.is_null() {
        return;
    }

    unsafe {
        (*mo).z += (*mo).momz;
    }

    if (unsafe { (*mo).flags } & MF_FLOAT) != 0 && !unsafe { (*mo).target }.is_null() {
        let dist = super::p_maputl::p_aprox_distance(
            unsafe { (*mo).x - (*(*mo).target).x },
            unsafe { (*mo).y - (*(*mo).target).y },
        );
        let delta = unsafe {
            (*(*mo).target).z + ((*mo).height >> 1) - (*mo).z
        };
        if (unsafe { (*mo).flags } & MF_SKULLFLY) == 0 {
            if delta < 0 && dist < -delta * 3 {
                unsafe { (*mo).z -= FLOATSPEED };
            } else if delta > 0 && dist < delta * 3 {
                unsafe { (*mo).z += FLOATSPEED };
            }
        }
    }

    unsafe {
        if (*mo).z <= (*mo).floorz {
            if (*mo).momz < 0 {
                (*mo).momz = 0;
            }
            (*mo).z = (*mo).floorz;
            if ((*mo).flags & MF_MISSILE) != 0 && ((*mo).flags & MF_NOCLIP) == 0 {
                p_explode_missile(mo);
            }
        } else if ((*mo).flags & MF_NOGRAVITY) == 0 {
            if (*mo).momz == 0 {
                (*mo).momz = -GRAVITY * 2;
            } else {
                (*mo).momz -= GRAVITY;
            }
        }

        if (*mo).z + (*mo).height > (*mo).ceilingz {
            if (*mo).momz > 0 {
                (*mo).momz = 0;
            }
            (*mo).z = (*mo).ceilingz - (*mo).height;
            if ((*mo).flags & MF_MISSILE) != 0 && ((*mo).flags & MF_NOCLIP) == 0 {
                p_explode_missile(mo);
            }
        }
    }
}

/// Set mobj state. Returns false if removed (S_NULL). Original: P_SetMobjState
fn p_set_mobj_state(mobj: *mut Mobj, state: i32) -> bool {
    use crate::game::d_think::no_op_acp1;
    use crate::info::tables::states;
    use crate::info::S_NULL;

    if mobj.is_null() {
        return false;
    }

    let mut state_idx = state;
    let states_ref = states();

    loop {
        if state_idx == S_NULL {
            unsafe {
                (*mobj).state = std::ptr::null();
                p_remove_mobj(mobj);
            }
            return false;
        }

        let st_idx = state_idx as usize;
        if st_idx >= states_ref.len() {
            return true;
        }
        let st = &states_ref[st_idx];

        unsafe {
            (*mobj).state = st as *const _;
            (*mobj).tics = st.tics;
            (*mobj).sprite = st.sprite;
            (*mobj).frame = st.frame;
        }

        unsafe {
            if st.action.acp1 != no_op_acp1 {
                (st.action.acp1)(mobj as *mut ());
            }
        }

        state_idx = st.nextstate;
        if unsafe { (*mobj).tics } != 0 {
            break;
        }
    }

    true
}

/// Explode missile on impact. Original: P_ExplodeMissile
fn p_explode_missile(mo: *mut Mobj) {
    if mo.is_null() {
        return;
    }
    unsafe {
        (*mo).momx = 0;
        (*mo).momy = 0;
        (*mo).momz = 0;
        if !(*mo).info.is_null() {
            p_set_mobj_state(mo, (*(*mo).info).deathstate);
        }
        if (*mo).tics < 1 {
            (*mo).tics = 1;
        }
        (*mo).flags &= !MF_MISSILE;
    }
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

/// Spawn player at map thing (type 1-4). Original: P_SpawnPlayer
pub fn p_spawn_player(mthing: &MapThing) {
    use crate::doomstat::{PlayerState, PLAYERINGAME, PLAYERS};
    use crate::game::g_game::g_player_reborn;
    use crate::info::MT_PLAYER;

    let mt_type = mthing.type_ as i32;
    if mt_type == 0 {
        return;
    }
    let idx = (mt_type - 1) as usize;
    if idx >= crate::doomdef::MAXPLAYERS {
        return;
    }
    if !unsafe { PLAYERINGAME[idx] } {
        return;
    }

    let x = (mthing.x as i32) * FRACUNIT;
    let y = (mthing.y as i32) * FRACUNIT;
    let z = crate::player::ONFLOORZ;

    let mobj = p_spawn_mobj(x, y, z, MT_PLAYER);
    if mobj.is_null() {
        return;
    }

    unsafe {
        let p = &mut PLAYERS[idx];
        if p.playerstate == PlayerState::Reborn {
            g_player_reborn(idx);
        }
        let p = &mut PLAYERS[idx];

        if mt_type > 1 {
            (*mobj).flags |= ((mt_type - 1) as i32) << MF_TRANSSHIFT;
        }
        (*mobj).angle =
            (crate::geometry::ANG45 as u32).wrapping_mul((mthing.angle as i32).max(0) as u32 / 45);
        (*mobj).player = p as *mut crate::doomstat::Player as *mut std::ffi::c_void;
        (*mobj).health = p.health;

        p.mo = mobj as *mut std::ffi::c_void;
        p.playerstate = PlayerState::Live;
        p.viewheight = super::VIEWHEIGHT;
        p.viewz = (*mobj).z + super::VIEWHEIGHT;
        p.extralight = 0;
        p.fixedcolormap = 0;
    }

    super::p_pspr::p_setup_psprites(unsafe { &mut PLAYERS[idx] as *mut crate::doomstat::Player as *mut std::ffi::c_void });
}

/// Spawn a map thing (from THINGS lump). Original: P_SpawnMapThing
/// Handles player starts (1-4), deathmatch (11). Spawns monsters/items by doomednum.
pub fn p_spawn_map_thing(mthing: &MapThing) {
    use crate::doomstat::{DEATHMATCH, PLAYERSTARTS};
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
    // Player starts (1-4)
    if mt_type <= 4 {
        let idx = (mt_type - 1) as usize;
        if idx < crate::doomdef::MAXPLAYERS {
            unsafe {
                PLAYERSTARTS[idx] = *mthing;
            }
            let deathmatch = unsafe { DEATHMATCH };
            if deathmatch == 0 {
                p_spawn_player(mthing);
            }
        }
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
    // Teleport dest (MT_TELEPORTMAN) spawns regardless of skill.
    let skill_bits = mthing.options as i32 & 0xF;
    if skill_bits == 0 && mobj_type != crate::info::MT_TELEPORTMAN {
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
