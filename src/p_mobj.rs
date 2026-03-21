// doomgeneric/p_mobj.h — map object (partial: mobj_t + flags; full logic in p_mobj.c)

pub use crate::d_think::*;
pub use crate::doomdata::*;
pub use crate::info::*;
pub use crate::m_fixed::*;
pub use crate::tables::*;

// Original: typedef enum { MF_SPECIAL = 1, ... } mobjflag_t (values used as bitmasks on mobj_t.flags)
pub mod mobjflag {
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
    pub const MF_DROPOFF: i32 = 0x400;
    pub const MF_PICKUP: i32 = 0x800;
    pub const MF_NOCLIP: i32 = 0x1000;
    pub const MF_SLIDE: i32 = 0x2000;
    pub const MF_FLOAT: i32 = 0x4000;
    pub const MF_TELEPORT: i32 = 0x8000;
    pub const MF_MISSILE: i32 = 0x10000;
    pub const MF_DROPPED: i32 = 0x20000;
    pub const MF_SHADOW: i32 = 0x40000;
    pub const MF_NOBLOOD: i32 = 0x80000;
    pub const MF_CORPSE: i32 = 0x100000;
    pub const MF_INFLOAT: i32 = 0x200000;
    pub const MF_COUNTKILL: i32 = 0x400000;
    pub const MF_COUNTITEM: i32 = 0x800000;
    pub const MF_SKULLFLY: i32 = 0x1000000;
    pub const MF_NOTDMATCH: i32 = 0x2000000;
    pub const MF_TRANSLATION: i32 = 0xc000000;
    pub const MF_TRANSSHIFT: i32 = 26;
}

/// Original: typedef struct mobj_s { ... } mobj_t
#[repr(C)]
pub struct MobjT {
    // Original: thinker
    pub thinker: ThinkerT,
    // Original: x
    pub x: FixedT,
    // Original: y
    pub y: FixedT,
    // Original: z
    pub z: FixedT,
    // Original: snext
    pub snext: *mut MobjT,
    // Original: sprev
    pub sprev: *mut MobjT,
    // Original: angle
    pub angle: AngleT,
    // Original: sprite
    pub sprite: SpritenumT,
    // Original: frame
    pub frame: i32,
    // Original: bnext
    pub bnext: *mut MobjT,
    // Original: bprev
    pub bprev: *mut MobjT,
    /// Original: subsector_s* — stub opaque until r_ / p_ modules
    pub subsector: *mut std::ffi::c_void,
    // Original: floorz
    pub floorz: FixedT,
    // Original: ceilingz
    pub ceilingz: FixedT,
    // Original: radius
    pub radius: FixedT,
    // Original: height
    pub height: FixedT,
    // Original: momx
    pub momx: FixedT,
    // Original: momy
    pub momy: FixedT,
    // Original: momz
    pub momz: FixedT,
    // Original: validcount
    pub validcount: i32,
    // Original: type
    pub type_: MobjtypeT,
    // Original: info
    pub info: *mut MobjinfoT,
    // Original: tics
    pub tics: i32,
    // Original: state
    pub state: *mut StateT,
    // Original: flags
    pub flags: i32,
    // Original: health
    pub health: i32,
    // Original: movedir
    pub movedir: i32,
    // Original: movecount
    pub movecount: i32,
    // Original: target
    pub target: *mut MobjT,
    // Original: reactiontime
    pub reactiontime: i32,
    // Original: threshold
    pub threshold: i32,
    /// Original: player_s* — avoid cycle with d_player
    pub player: *mut std::ffi::c_void,
    // Original: lastlook
    pub lastlook: i32,
    // Original: spawnpoint
    pub spawnpoint: MapthingT,
    // Original: tracer
    pub tracer: *mut MobjT,
}

#[allow(non_camel_case_types)]
pub struct P_MobjState;

impl P_MobjState {
    pub fn new() -> Self {
        Self
    }
}
