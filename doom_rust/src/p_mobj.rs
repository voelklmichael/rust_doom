//! Rust translation of doomgeneric/p_mobj.h
//! Map Objects, MObj, definition and handling.

use crate::doomdata::*;
use crate::d_think::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::r_defs::*;
use crate::tables::*;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: mobjflag_t
pub enum MobjflagT {
    MfSpecial = 1,
    MfSolid = 2,
    MfShootable = 4,
    MfNosector = 8,
    MfNoblockmap = 16,
    MfAmbush = 32,
    MfJusthit = 64,
    MfJustattacked = 128,
    MfSpawnceiling = 256,
    MfNogravity = 512,
    MfDropoff = 0x400,
    MfPickup = 0x800,
    MfNoclip = 0x1000,
    MfSlide = 0x2000,
    MfFloat = 0x4000,
    MfTeleport = 0x8000,
    MfMissile = 0x10000,
    MfDropped = 0x20000,
    MfShadow = 0x40000,
    MfNoblood = 0x80000,
    MfCorpse = 0x100000,
    MfInfloat = 0x200000,
    MfCountkill = 0x400000,
    MfCountitem = 0x800000,
    MfSkullfly = 0x1000000,
    MfNotdmatch = 0x2000000,
    MfTranslation = 0xc000000,
    MfTransshift = 26,
}

/// mobj_t
#[repr(C)]
/// C typedef: mobj_t
pub struct MobjT {
    pub thinker: ThinkerT,
    pub x: FixedT,
    pub y: FixedT,
    pub z: FixedT,
    pub snext: Option<Arc<Mutex<MobjT>>>,
    pub sprev: Option<Arc<Mutex<MobjT>>>,
    pub angle: AngleT,
    pub sprite: SpritenumT,
    pub frame: i32,
    pub bnext: Option<Arc<Mutex<MobjT>>>,
    pub bprev: Option<Arc<Mutex<MobjT>>>,
    pub subsector: Option<Arc<Mutex<SubsectorT>>>,
    pub floorz: FixedT,
    pub ceilingz: FixedT,
    pub radius: FixedT,
    pub height: FixedT,
    pub momx: FixedT,
    pub momy: FixedT,
    pub momz: FixedT,
    pub validcount: i32,
    pub type_: MobjtypeT,
    pub info: Option<Arc<Mutex<MobjinfoT>>>,
    pub tics: i32,
    pub state: Option<Arc<Mutex<StateT>>>,
    pub flags: i32,
    pub health: i32,
    pub movedir: i32,
    pub movecount: i32,
    pub target: Option<Arc<Mutex<MobjT>>>,
    pub reactiontime: i32,
    pub threshold: i32,
    pub player: Option<Arc<Mutex<Vec<u8>>>>, // *mut PlayerT
    pub lastlook: i32,
    pub spawnpoint: MapthingT,
    pub tracer: Option<Arc<Mutex<MobjT>>>,
}
