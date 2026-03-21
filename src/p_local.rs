// doomgeneric/p_local.h — play functions aggregate (stub)

pub use crate::d_player::*;
pub use crate::d_think::*;
pub use crate::doomdata::*;
pub use crate::doomdef::*;
pub use crate::doomtype::*;
pub use crate::info::*;
pub use crate::m_fixed::*;
pub use crate::p_mobj::*;
pub use crate::p_spec::*;
pub use crate::r_defs::*;
pub use crate::r_local::*;
pub use crate::tables::AngleT;

use std::cell::RefCell;

// Original: #define FLOATSPEED (FRACUNIT*4)
pub const FLOATSPEED: FixedT = FRACUNIT * 4;
// Original: #define MAXHEALTH 100
pub const MAXHEALTH: i32 = 100;
// Original: #define VIEWHEIGHT (41*FRACUNIT)
pub const VIEWHEIGHT: FixedT = 41 * FRACUNIT;

pub const MAPBLOCKUNITS: i32 = 128;
pub const MAPBLOCKSIZE: FixedT = MAPBLOCKUNITS * FRACUNIT;
pub const MAPBLOCKSHIFT: i32 = FRACBITS + 7;
pub const MAPBMASK: FixedT = MAPBLOCKSIZE - 1;
pub const MAPBTOFRAC: i32 = MAPBLOCKSHIFT - FRACBITS;
pub const PLAYERRADIUS: FixedT = 16 * FRACUNIT;
pub const MAXRADIUS: FixedT = 32 * FRACUNIT;
pub const GRAVITY: FixedT = FRACUNIT;
pub const MAXMOVE: FixedT = 30 * FRACUNIT;
pub const USERANGE: FixedT = 64 * FRACUNIT;
pub const MELEERANGE: FixedT = 64 * FRACUNIT;
pub const MISSILERANGE: FixedT = 32 * 64 * FRACUNIT;
pub const BASETHRESHOLD: i32 = 100;

pub const ONFLOORZ: i32 = i32::MIN;
pub const ONCEILINGZ: i32 = i32::MAX;

pub const ITEMQUESIZE: usize = 128;
pub const MAXINTERCEPTS_ORIGINAL: usize = 128;
pub const MAXINTERCEPTS: usize = MAXINTERCEPTS_ORIGINAL + 61;

pub const PT_ADDLINES: i32 = 1;
pub const PT_ADDTHINGS: i32 = 2;
pub const PT_EARLYOUT: i32 = 4;

pub const MAXSPECIALCROSS: usize = 20;
pub const MAXSPECIALCROSS_ORIGINAL: usize = 8;

/// Original: divline_t
#[repr(C)]
pub struct DivlineT {
    pub x: FixedT,
    pub y: FixedT,
    pub dx: FixedT,
    pub dy: FixedT,
}

/// Original: intercept_t.d union
#[repr(C)]
pub union InterceptD {
    pub thing: *mut MobjT,
    pub line: *mut LineT,
}

/// Original: intercept_t
#[repr(C)]
pub struct InterceptT {
    pub frac: FixedT,
    pub isaline: Boolean,
    pub d: InterceptD,
}

pub type TraverserT = unsafe extern "C" fn(*mut InterceptT) -> Boolean;

#[allow(non_camel_case_types)]
pub struct P_LocalState {
    /// Original: thinkercap, intercept_p, trace, tm* , floatok, etc. (wired in later)
    pub _placeholder: RefCell<i32>,
}

impl P_LocalState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(0),
        }
    }

    pub fn p_init_thinkers(&self) {
        todo!("P_InitThinkers");
    }

    pub fn p_add_thinker(&self, _thinker: *mut ThinkerT) {
        todo!("P_AddThinker");
    }

    pub fn p_remove_thinker(&self, _thinker: *mut ThinkerT) {
        todo!("P_RemoveThinker");
    }

    pub fn p_setup_psprites(&self, _curplayer: *mut PlayerT) {
        todo!("P_SetupPsprites");
    }

    pub fn p_move_psprites(&self, _curplayer: *mut PlayerT) {
        todo!("P_MovePsprites");
    }

    pub fn p_drop_weapon(&self, _player: *mut PlayerT) {
        todo!("P_DropWeapon");
    }

    pub fn p_player_think(&self, _player: *mut PlayerT) {
        todo!("P_PlayerThink");
    }

    pub fn p_respawn_specials(&self) {
        todo!("P_RespawnSpecials");
    }

    pub fn p_spawn_mobj(
        &self,
        _x: FixedT,
        _y: FixedT,
        _z: FixedT,
        _type_: MobjtypeT,
    ) -> *mut MobjT {
        todo!("P_SpawnMobj");
    }

    pub fn p_remove_mobj(&self, _th: *mut MobjT) {
        todo!("P_RemoveMobj");
    }

    pub fn p_subst_null_mobj(&self, _th: *mut MobjT) -> *mut MobjT {
        todo!("P_SubstNullMobj");
    }

    pub fn p_set_mobj_state(&self, _mobj: *mut MobjT, _state: StatenumT) -> Boolean {
        todo!("P_SetMobjState");
    }

    pub fn p_mobj_thinker(&self, _mobj: *mut MobjT) {
        todo!("P_MobjThinker");
    }

    pub fn p_spawn_puff(&self, _x: FixedT, _y: FixedT, _z: FixedT) {
        todo!("P_SpawnPuff");
    }

    pub fn p_spawn_blood(&self, _x: FixedT, _y: FixedT, _z: FixedT, _damage: i32) {
        todo!("P_SpawnBlood");
    }

    pub fn p_spawn_missile(
        &self,
        _source: *mut MobjT,
        _dest: *mut MobjT,
        _type_: MobjtypeT,
    ) -> *mut MobjT {
        todo!("P_SpawnMissile");
    }

    pub fn p_spawn_player_missile(&self, _source: *mut MobjT, _type_: MobjtypeT) {
        todo!("P_SpawnPlayerMissile");
    }

    pub fn p_noise_alert(&self, _target: *mut MobjT, _emitter: *mut MobjT) {
        todo!("P_NoiseAlert");
    }

    pub fn p_aprox_distance(&self, _dx: FixedT, _dy: FixedT) -> FixedT {
        todo!("P_AproxDistance");
    }

    pub fn p_point_on_line_side(&self, _x: FixedT, _y: FixedT, _line: *mut LineT) -> i32 {
        todo!("P_PointOnLineSide");
    }

    pub fn p_point_on_divline_side(&self, _x: FixedT, _y: FixedT, _line: *mut DivlineT) -> i32 {
        todo!("P_PointOnDivlineSide");
    }

    pub fn p_make_divline(&self, _li: *mut LineT, _dl: *mut DivlineT) {
        todo!("P_MakeDivline");
    }

    pub fn p_intercept_vector(&self, _v2: *mut DivlineT, _v1: *mut DivlineT) -> FixedT {
        todo!("P_InterceptVector");
    }

    pub fn p_box_on_line_side(&self, _tmbox: *mut FixedT, _ld: *mut LineT) -> i32 {
        todo!("P_BoxOnLineSide");
    }

    pub fn p_line_opening(&self, _linedef: *mut LineT) {
        todo!("P_LineOpening");
    }

    pub fn p_block_lines_iterator(
        &self,
        _x: i32,
        _y: i32,
        _func: Option<unsafe extern "C" fn(*mut LineT) -> Boolean>,
    ) -> Boolean {
        todo!("P_BlockLinesIterator");
    }

    pub fn p_block_things_iterator(
        &self,
        _x: i32,
        _y: i32,
        _func: Option<unsafe extern "C" fn(*mut MobjT) -> Boolean>,
    ) -> Boolean {
        todo!("P_BlockThingsIterator");
    }

    pub fn p_path_traverse(
        &self,
        _x1: FixedT,
        _y1: FixedT,
        _x2: FixedT,
        _y2: FixedT,
        _flags: i32,
        _trav: TraverserT,
    ) -> Boolean {
        todo!("P_PathTraverse");
    }

    pub fn p_unset_thing_position(&self, _thing: *mut MobjT) {
        todo!("P_UnsetThingPosition");
    }

    pub fn p_set_thing_position(&self, _thing: *mut MobjT) {
        todo!("P_SetThingPosition");
    }

    pub fn p_check_position(&self, _thing: *mut MobjT, _x: FixedT, _y: FixedT) -> Boolean {
        todo!("P_CheckPosition");
    }

    pub fn p_try_move(&self, _thing: *mut MobjT, _x: FixedT, _y: FixedT) -> Boolean {
        todo!("P_TryMove");
    }

    pub fn p_teleport_move(&self, _thing: *mut MobjT, _x: FixedT, _y: FixedT) -> Boolean {
        todo!("P_TeleportMove");
    }

    pub fn p_slide_move(&self, _mo: *mut MobjT) {
        todo!("P_SlideMove");
    }

    pub fn p_check_sight(&self, _t1: *mut MobjT, _t2: *mut MobjT) -> Boolean {
        todo!("P_CheckSight");
    }

    pub fn p_use_lines(&self, _player: *mut PlayerT) {
        todo!("P_UseLines");
    }

    pub fn p_change_sector(&self, _sector: *mut SectorT, _crunch: Boolean) -> Boolean {
        todo!("P_ChangeSector");
    }

    pub fn p_aim_line_attack(
        &self,
        _t1: *mut MobjT,
        _angle: AngleT,
        _distance: FixedT,
    ) -> FixedT {
        todo!("P_AimLineAttack");
    }

    pub fn p_line_attack(
        &self,
        _t1: *mut MobjT,
        _angle: AngleT,
        _distance: FixedT,
        _slope: FixedT,
        _damage: i32,
    ) {
        todo!("P_LineAttack");
    }

    pub fn p_radius_attack(&self, _spot: *mut MobjT, _source: *mut MobjT, _damage: i32) {
        todo!("P_RadiusAttack");
    }

    pub fn p_touch_special_thing(&self, _special: *mut MobjT, _toucher: *mut MobjT) {
        todo!("P_TouchSpecialThing");
    }

    pub fn p_damage_mobj(
        &self,
        _target: *mut MobjT,
        _inflictor: *mut MobjT,
        _source: *mut MobjT,
        _damage: i32,
    ) {
        todo!("P_DamageMobj");
    }
}
