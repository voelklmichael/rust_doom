//! Map objects (p_mobj.h, p_mobj.c)
//! Original: p_mobj.h, p_mobj.c

use crate::m_fixed::FixedT;
use crate::doomtype::Boolean;

pub const MF_SPECIAL: i32 = 1;
pub const MF_SOLID: i32 = 2;
pub const MF_SHOOTABLE: i32 = 4;
pub const MF_NOSECTOR: i32 = 8;
pub const MF_NOBLOCKMAP: i32 = 16;
pub const MF_AMBUSH: i32 = 32;
pub const MF_JUSTHIT: i32 = 64;
pub const MF_JUSTATTACKED: i32 = 128;
pub const MF_SPAWNCEILING: i32 = 256;
pub const MF_NOGRAVITY: i32 = 512;
pub const MF_DROPOFF: i32 = 1024;
pub const MF_PICKUP: i32 = 2048;
pub const MF_NOCLIP: i32 = 4096;
pub const MF_SLIDE: i32 = 8192;
pub const MF_FLOAT: i32 = 16384;
pub const MF_TELEPORT: i32 = 32768;
pub const MF_MISSILE: i32 = 65536;

pub struct MobjT {
    pub x: FixedT,
    pub y: FixedT,
    pub z: FixedT,
    pub momx: FixedT,
    pub momy: FixedT,
    pub momz: FixedT,
    pub angle: u32,
    pub sprite: i32,
    pub frame: i32,
    pub flags: i32,
}

pub struct P_MobjState;

impl P_MobjState {
    /// Original: mobj_t *P_SpawnMobj(fixed_t x, fixed_t y, fixed_t z, mobjtype_t type)
    pub fn p_spawn_mobj(&self, _x: FixedT, _y: FixedT, _z: FixedT, _mobj_type: i32) -> Option<MobjT> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_RemoveMobj(mobj_t *th)
    pub fn p_remove_mobj(&self, _th: &MobjT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_SetMobjState(mobj_t *mobj, statenum_t state)
    pub fn p_set_mobj_state(&self, _mobj: &mut MobjT, _state: i32) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_MobjThinker(mobj_t *mobj)
    pub fn p_mobj_thinker(&self, _mobj: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_TeleportMove(mobj_t *thing, fixed_t x, fixed_t y)
    pub fn p_teleport_move(&self, _thing: &(), _x: FixedT, _y: FixedT) -> Boolean {
        todo!("Basic stage-0 stub")
    }
}
