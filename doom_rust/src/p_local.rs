//! Rust translation of doomgeneric/p_local.h
//! Play functions, animation, global header.

use crate::d_player::*;
use crate::d_think::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::info::{MobjtypeT, StatenumT};
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_spec::*;
use crate::r_defs::*;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

/// C #define: FLOATSPEED
pub const FLOATSPEED: FixedT = FRACUNIT * 4;
/// C #define: MAXHEALTH
pub const MAXHEALTH: i32 = 100;
/// C #define: VIEWHEIGHT
pub const VIEWHEIGHT: FixedT = 41 * FRACUNIT;
/// C #define: MAPBLOCKUNITS
pub const MAPBLOCKUNITS: i32 = 128;
/// C #define: MAPBLOCKSIZE
pub const MAPBLOCKSIZE: FixedT = MAPBLOCKUNITS * FRACUNIT;
/// C #define: MAPBLOCKSHIFT
pub const MAPBLOCKSHIFT: i32 = FRACBITS + 7;
/// C #define: MAPBMASK
pub const MAPBMASK: FixedT = MAPBLOCKSIZE - 1;
/// C #define: MAPBTOFRAC
pub const MAPBTOFRAC: i32 = MAPBLOCKSHIFT - FRACBITS;
/// C #define: PLAYERRADIUS
pub const PLAYERRADIUS: FixedT = 16 * FRACUNIT;
/// C #define: MAXRADIUS
pub const MAXRADIUS: FixedT = 32 * FRACUNIT;
/// C #define: GRAVITY
pub const GRAVITY: FixedT = FRACUNIT;
/// C #define: MAXMOVE
pub const MAXMOVE: FixedT = 30 * FRACUNIT;
/// C #define: USERANGE
pub const USERANGE: FixedT = 64 * FRACUNIT;
/// C #define: MELEERANGE
pub const MELEERANGE: FixedT = 64 * FRACUNIT;
/// C #define: MISSILERANGE
pub const MISSILERANGE: FixedT = 32 * 64 * FRACUNIT;
/// C #define: BASETHRESHOLD
pub const BASETHRESHOLD: i32 = 100;

/// C #define: ONFLOORZ
pub const ONFLOORZ: i32 = i32::MIN;
/// C #define: ONCEILINGZ
pub const ONCEILINGZ: i32 = i32::MAX;
/// C #define: ITEMQUESIZE
pub const ITEMQUESIZE: usize = 128;

pub static mut thinkercap: ThinkerT = ThinkerT::new();

pub static mut itemrespawnque: [MapthingT; ITEMQUESIZE] = [MapthingT {
    x: 0,
    y: 0,
    angle: 0,
    type_: 0,
    options: 0,
}; ITEMQUESIZE];
pub static mut itemrespawntime: [i32; ITEMQUESIZE] = [0; ITEMQUESIZE];
pub static mut iquehead: i32 = 0;
pub static mut iquetail: i32 = 0;

pub static mut rejectmatrix: *mut byte = std::ptr::null_mut();
pub static mut blockmaplump: *mut i16 = std::ptr::null_mut();
pub static mut blockmap: *mut i16 = std::ptr::null_mut();
pub static mut bmapwidth: i32 = 0;
pub static mut bmapheight: i32 = 0;
pub static mut bmaporgx: FixedT = 0;
pub static mut bmaporgy: FixedT = 0;
pub static mut blocklinks: *mut *mut MobjT = std::ptr::null_mut();

pub static mut maxammo: [i32; crate::doomdef::NUMAMMO] = [0; crate::doomdef::NUMAMMO];
pub static mut clipammo: [i32; crate::doomdef::NUMAMMO] = [0; crate::doomdef::NUMAMMO];

pub static mut floatok: boolean = Boolean::False;
pub static mut tmfloorz: FixedT = 0;
pub static mut tmceilingz: FixedT = 0;
pub static mut ceilingline: *mut LineT = std::ptr::null_mut();

/// C #define: MAXSPECIALCROSS
pub const MAXSPECIALCROSS: usize = 20;
/// C #define: MAXSPECIALCROSS_ORIGINAL
pub const MAXSPECIALCROSS_ORIGINAL: usize = 8;
pub static mut spechit: [*mut LineT; MAXSPECIALCROSS] = [std::ptr::null_mut(); MAXSPECIALCROSS];
pub static mut numspechit: i32 = 0;

pub static mut linetarget: *mut MobjT = std::ptr::null_mut();

/// C #define: MAXINTERCEPTS_ORIGINAL
pub const MAXINTERCEPTS_ORIGINAL: usize = 128;
/// C #define: MAXINTERCEPTS
pub const MAXINTERCEPTS: usize = MAXINTERCEPTS_ORIGINAL + 61;

/// divline_t
#[repr(C)]
/// C typedef: divline_t
pub struct DivlineT {
    pub x: FixedT,
    pub y: FixedT,
    pub dx: FixedT,
    pub dy: FixedT,
}

/// intercept_t - d is union { thing, line }
#[repr(C)]
#[derive(Clone)]
/// C typedef: intercept_t
pub struct InterceptT {
    pub frac: FixedT,
    pub isaline: boolean,
    pub d_thing: Option<Arc<Mutex<MobjT>>>, // use when !isaline
    pub d_line: Option<Arc<Mutex<LineT>>>,  // use when isaline
}

pub static INTERCEPTS: Lazy<Mutex<[InterceptT; MAXINTERCEPTS]>> = Lazy::new(|| {
    Mutex::new(std::array::from_fn(|_| InterceptT {
        frac: 0,
        isaline: Boolean::False,
        d_thing: None,
        d_line: None,
    }))
});
pub static mut intercept_p: *mut InterceptT = std::ptr::null_mut();

/// C typedef: traverser_t
pub type TraverserT = fn(&mut InterceptT) -> boolean;

pub static mut opentop: FixedT = 0;
pub static mut openbottom: FixedT = 0;
pub static mut openrange: FixedT = 0;
pub static mut lowfloor: FixedT = 0;

pub static mut trace: DivlineT = DivlineT {
    x: 0,
    y: 0,
    dx: 0,
    dy: 0,
};

/// C #define: PT_ADDLINES
pub const PT_ADDLINES: i32 = 1;
/// C #define: PT_ADDTHINGS
pub const PT_ADDTHINGS: i32 = 2;
/// C #define: PT_EARLYOUT
pub const PT_EARLYOUT: i32 = 4;

/// C function: P_InitThinkers
pub fn p_init_thinkers() {
    todo!("original: P_InitThinkers")
}

/// C function: P_AddThinker
pub fn p_add_thinker(thinker: &mut ThinkerT) {
    todo!("original: P_AddThinker")
}

/// C function: P_RemoveThinker
pub fn p_remove_thinker(thinker: &mut ThinkerT) {
    todo!("original: P_RemoveThinker")
}

/// C function: P_SetupPsprites
pub fn p_setup_psprites(curplayer: &mut PlayerT) {
    todo!("original: P_SetupPsprites")
}

/// C function: P_MovePsprites
pub fn p_move_psprites(curplayer: &mut PlayerT) {
    todo!("original: P_MovePsprites")
}

/// C function: P_DropWeapon
pub fn p_drop_weapon(player: &mut PlayerT) {
    todo!("original: P_DropWeapon")
}

/// C function: P_PlayerThink
pub fn p_player_think(player: &mut PlayerT) {
    todo!("original: P_PlayerThink")
}

/// C function: P_RespawnSpecials
pub fn p_respawn_specials() {
    todo!("original: P_RespawnSpecials")
}

/// C function: P_SpawnMobj
pub fn p_spawn_mobj(x: FixedT, y: FixedT, z: FixedT, type_: MobjtypeT) -> Arc<Mutex<MobjT>> {
    todo!("original: P_SpawnMobj")
}

/// C function: P_RemoveMobj
pub fn p_remove_mobj(th: &mut MobjT) {
    todo!("original: P_RemoveMobj")
}

/// C function: P_SubstNullMobj
pub fn p_subst_null_mobj(th: &mut MobjT) -> Arc<Mutex<MobjT>> {
    todo!("original: P_SubstNullMobj")
}

/// C function: P_SetMobjState
pub fn p_set_mobj_state(mobj: &mut MobjT, state: StatenumT) -> boolean {
    todo!("original: P_SetMobjState")
}

/// C function: P_MobjThinker
pub fn p_mobj_thinker(mobj: &mut MobjT) {
    todo!("original: P_MobjThinker")
}

/// C function: P_SpawnPuff
pub fn p_spawn_puff(x: FixedT, y: FixedT, z: FixedT) {
    todo!("original: P_SpawnPuff")
}

/// C function: P_SpawnBlood
pub fn p_spawn_blood(x: FixedT, y: FixedT, z: FixedT, damage: i32) {
    todo!("original: P_SpawnBlood")
}

/// C function: P_SpawnMissile
pub fn p_spawn_missile(
    source: &mut MobjT,
    dest: &mut MobjT,
    type_: MobjtypeT,
) -> Arc<Mutex<MobjT>> {
    todo!("original: P_SpawnMissile")
}

/// C function: P_SpawnPlayerMissile
pub fn p_spawn_player_missile(source: &mut MobjT, type_: MobjtypeT) {
    todo!("original: P_SpawnPlayerMissile")
}

/// C function: P_NoiseAlert
pub fn p_noise_alert(target: &mut MobjT, emmiter: &mut MobjT) {
    todo!("original: P_NoiseAlert")
}

/// C function: P_AproxDistance
pub fn p_aprox_distance(dx: FixedT, dy: FixedT) -> FixedT {
    todo!("original: P_AproxDistance")
}

/// C function: P_PointOnLineSide
pub fn p_point_on_line_side(x: FixedT, y: FixedT, line: &mut LineT) -> i32 {
    todo!("original: P_PointOnLineSide")
}

/// C function: P_PointOnDivlineSide
pub fn p_point_on_divline_side(x: FixedT, y: FixedT, line: &mut DivlineT) -> i32 {
    todo!("original: P_PointOnDivlineSide")
}

/// C function: P_MakeDivline
pub fn p_make_divline(li: &mut LineT, dl: &mut DivlineT) {
    todo!("original: P_MakeDivline")
}

/// C function: P_InterceptVector
pub fn p_intercept_vector(v2: &mut DivlineT, v1: &mut DivlineT) -> FixedT {
    todo!("original: P_InterceptVector")
}

/// C function: P_BoxOnLineSide
pub fn p_box_on_line_side(tmbox: &mut [FixedT], ld: &mut LineT) -> i32 {
    todo!("original: P_BoxOnLineSide")
}

/// C function: P_LineOpening
pub fn p_line_opening(linedef: &mut LineT) {
    todo!("original: P_LineOpening")
}

/// C function: P_BlockLinesIterator
pub fn p_block_lines_iterator(x: i32, y: i32, func: Option<fn(&mut LineT) -> boolean>) -> boolean {
    todo!("original: P_BlockLinesIterator")
}

/// C function: P_BlockThingsIterator
pub fn p_block_things_iterator(x: i32, y: i32, func: Option<fn(&mut MobjT) -> boolean>) -> boolean {
    todo!("original: P_BlockThingsIterator")
}

/// C function: P_PathTraverse
pub fn p_path_traverse(
    x1: FixedT,
    y1: FixedT,
    x2: FixedT,
    y2: FixedT,
    flags: i32,
    trav: Option<fn(&mut InterceptT) -> boolean>,
) -> boolean {
    todo!("original: P_PathTraverse")
}

/// C function: P_UnsetThingPosition
pub fn p_unset_thing_position(thing: &mut MobjT) {
    todo!("original: P_UnsetThingPosition")
}

/// C function: P_SetThingPosition
pub fn p_set_thing_position(thing: &mut MobjT) {
    todo!("original: P_SetThingPosition")
}

/// C function: P_CheckPosition
pub fn p_check_position(thing: &mut MobjT, x: FixedT, y: FixedT) -> boolean {
    todo!("original: P_CheckPosition")
}

/// C function: P_TryMove
pub fn p_try_move(thing: &mut MobjT, x: FixedT, y: FixedT) -> boolean {
    todo!("original: P_TryMove")
}

/// C function: P_TeleportMove
pub fn p_teleport_move(thing: &mut MobjT, x: FixedT, y: FixedT) -> boolean {
    todo!("original: P_TeleportMove")
}

/// C function: P_SlideMove
pub fn p_slide_move(mo: &mut MobjT) {
    todo!("original: P_SlideMove")
}

/// C function: P_CheckSight
pub fn p_check_sight(t1: &mut MobjT, t2: &mut MobjT) -> boolean {
    todo!("original: P_CheckSight")
}

/// C function: P_UseLines
pub fn p_use_lines(player: &mut PlayerT) {
    todo!("original: P_UseLines")
}

/// C function: P_ChangeSector
pub fn p_change_sector(sector: &mut SectorT, crunch: boolean) -> boolean {
    todo!("original: P_ChangeSector")
}

/// C function: P_AimLineAttack
pub fn p_aim_line_attack(t1: &mut MobjT, angle: crate::tables::AngleT, distance: FixedT) -> FixedT {
    todo!("original: P_AimLineAttack")
}

/// C function: P_LineAttack
pub fn p_line_attack(
    t1: &mut MobjT,
    angle: crate::tables::AngleT,
    distance: FixedT,
    slope: FixedT,
    damage: i32,
) {
    todo!("original: P_LineAttack")
}

/// C function: P_RadiusAttack
pub fn p_radius_attack(spot: &mut MobjT, source: &mut MobjT, damage: i32) {
    todo!("original: P_RadiusAttack")
}
