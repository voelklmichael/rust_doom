//! Rust translation of doomgeneric/p_spec.h
//! Special effects: texture animation, height/lighting changes.

use crate::d_think::*;
use crate::doomtype::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::r_defs::*;

pub static mut level_timer: boolean = crate::doomtype::Boolean::False;
pub static mut level_time_count: i32 = 0;

pub const MO_TELEPORTMAN: i32 = 14;

pub fn p_init_pic_anims() {
    todo!("original: P_InitPicAnims")
}

pub fn p_spawn_specials() {
    todo!("original: P_SpawnSpecials")
}

pub fn p_update_specials() {
    todo!("original: P_UpdateSpecials")
}

pub fn p_use_special_line(thing: *mut MobjT, line: *mut LineT, side: i32) -> boolean {
    todo!("original: P_UseSpecialLine")
}

pub fn p_shoot_special_line(thing: *mut MobjT, line: *mut LineT) {
    todo!("original: P_ShootSpecialLine")
}

pub fn p_cross_special_line(linenum: i32, side: i32, thing: *mut MobjT) {
    todo!("original: P_CrossSpecialLine")
}

pub fn p_player_in_special_sector(player: *mut crate::d_player::PlayerT) {
    todo!("original: P_PlayerInSpecialSector")
}

pub fn two_sided(sector: i32, line: i32) -> i32 {
    todo!("original: twoSided")
}

pub fn get_sector(current_sector: i32, line: i32, side: i32) -> *mut SectorT {
    todo!("original: getSector")
}

pub fn get_side(current_sector: i32, line: i32, side: i32) -> *mut SideT {
    todo!("original: getSide")
}

pub fn p_find_lowest_floor_surrounding(sec: *mut SectorT) -> FixedT {
    todo!("original: P_FindLowestFloorSurrounding")
}

pub fn p_find_highest_floor_surrounding(sec: *mut SectorT) -> FixedT {
    todo!("original: P_FindHighestFloorSurrounding")
}

pub fn p_find_next_highest_floor(sec: *mut SectorT, currentheight: i32) -> FixedT {
    todo!("original: P_FindNextHighestFloor")
}

pub fn p_find_lowest_ceiling_surrounding(sec: *mut SectorT) -> FixedT {
    todo!("original: P_FindLowestCeilingSurrounding")
}

pub fn p_find_highest_ceiling_surrounding(sec: *mut SectorT) -> FixedT {
    todo!("original: P_FindHighestCeilingSurrounding")
}

pub fn p_find_sector_from_line_tag(line: *mut LineT, start: i32) -> i32 {
    todo!("original: P_FindSectorFromLineTag")
}

pub fn p_find_min_surrounding_light(sector: *mut SectorT, max: i32) -> i32 {
    todo!("original: P_FindMinSurroundingLight")
}

pub fn get_next_sector(line: *mut LineT, sec: *mut SectorT) -> *mut SectorT {
    todo!("original: getNextSector")
}

pub fn ev_do_donut(line: *mut LineT) -> i32 {
    todo!("original: EV_DoDonut")
}

/// fireflicker_t
#[repr(C)]
pub struct FireflickerT {
    pub thinker: ThinkerT,
    pub sector: *mut SectorT,
    pub count: i32,
    pub maxlight: i32,
    pub minlight: i32,
}

/// lightflash_t
#[repr(C)]
pub struct LightflashT {
    pub thinker: ThinkerT,
    pub sector: *mut SectorT,
    pub count: i32,
    pub maxlight: i32,
    pub minlight: i32,
    pub maxtime: i32,
    pub mintime: i32,
}

/// strobe_t
#[repr(C)]
pub struct StrobeT {
    pub thinker: ThinkerT,
    pub sector: *mut SectorT,
    pub count: i32,
    pub minlight: i32,
    pub maxlight: i32,
    pub darktime: i32,
    pub brighttime: i32,
}

/// glow_t
#[repr(C)]
pub struct GlowT {
    pub thinker: ThinkerT,
    pub sector: *mut SectorT,
    pub minlight: i32,
    pub maxlight: i32,
    pub direction: i32,
}

pub const GLOWSPEED: i32 = 8;
pub const STROBEBRIGHT: i32 = 5;
pub const FASTDARK: i32 = 15;
pub const SLOWDARK: i32 = 35;

pub fn p_spawn_fire_flicker(sector: *mut SectorT) {
    todo!("original: P_SpawnFireFlicker")
}

pub fn t_light_flash(flash: *mut LightflashT) {
    todo!("original: T_LightFlash")
}

pub fn p_spawn_light_flash(sector: *mut SectorT) {
    todo!("original: P_SpawnLightFlash")
}

pub fn t_strobe_flash(flash: *mut StrobeT) {
    todo!("original: T_StrobeFlash")
}

pub fn p_spawn_strobe_flash(sector: *mut SectorT, fast_or_slow: i32, in_sync: i32) {
    todo!("original: P_SpawnStrobeFlash")
}

pub fn ev_start_light_strobing(line: *mut LineT) {
    todo!("original: EV_StartLightStrobing")
}

pub fn ev_turn_tag_lights_off(line: *mut LineT) {
    todo!("original: EV_TurnTagLightsOff")
}

pub fn ev_light_turn_on(line: *mut LineT, bright: i32) {
    todo!("original: EV_LightTurnOn")
}

pub fn t_glow(g: *mut GlowT) {
    todo!("original: T_Glow")
}

pub fn p_spawn_glowing_light(sector: *mut SectorT) {
    todo!("original: P_SpawnGlowingLight")
}

#[repr(C)]
pub struct SwitchlistT {
    pub name1: [i8; 9],
    pub name2: [i8; 9],
    pub episode: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BwhereE {
    Top,
    Middle,
    Bottom,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ButtonT {
    pub line: *mut LineT,
    pub where_: BwhereE,
    pub btexture: i32,
    pub btimer: i32,
    pub soundorg: *mut DegenmobjT,
}

impl ButtonT {
    pub const fn new() -> Self {
        Self {
            line: std::ptr::null_mut(),
            where_: BwhereE::Top,
            btexture: 0,
            btimer: 0,
            soundorg: std::ptr::null_mut(),
        }
    }
}

impl Default for ButtonT {
    fn default() -> Self {
        Self::new()
    }
}

pub const MAXSWITCHES: usize = 50;
pub const MAXBUTTONS: usize = 16;
pub const BUTTONTIME: i32 = 35;

pub static mut buttonlist: [ButtonT; MAXBUTTONS] = [ButtonT::new(); MAXBUTTONS];

pub fn p_change_switch_texture(line: *mut LineT, use_again: i32) {
    todo!("original: P_ChangeSwitchTexture")
}

pub fn p_init_switch_list() {
    todo!("original: P_InitSwitchList")
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatE {
    Up,
    Down,
    Waiting,
    InStasis,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlattypeE {
    PerpetualRaise,
    DownWaitUpStay,
    RaiseAndChange,
    RaiseToNearestAndChange,
    BlazeDwus,
}

#[repr(C)]
pub struct PlatT {
    pub thinker: ThinkerT,
    pub sector: *mut SectorT,
    pub speed: FixedT,
    pub low: FixedT,
    pub high: FixedT,
    pub wait: i32,
    pub count: i32,
    pub status: PlatE,
    pub oldstatus: PlatE,
    pub crush: boolean,
    pub tag: i32,
    pub type_: PlattypeE,
}

pub const PLATWAIT: i32 = 3;
pub const PLATSPEED: FixedT = FRACUNIT;
pub const MAXPLATS: usize = 30;

pub static mut activeplats: [*mut PlatT; MAXPLATS] = [std::ptr::null_mut(); MAXPLATS];

pub fn t_plat_raise(plat: *mut PlatT) {
    todo!("original: T_PlatRaise")
}

pub fn ev_do_plat(line: *mut LineT, type_: PlattypeE, amount: i32) -> i32 {
    todo!("original: EV_DoPlat")
}

pub fn p_add_active_plat(plat: *mut PlatT) {
    todo!("original: P_AddActivePlat")
}

pub fn p_remove_active_plat(plat: *mut PlatT) {
    todo!("original: P_RemoveActivePlat")
}

pub fn ev_stop_plat(line: *mut LineT) {
    todo!("original: EV_StopPlat")
}

pub fn p_activate_in_stasis(tag: i32) {
    todo!("original: P_ActivateInStasis")
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VldoorE {
    VldNormal,
    VldClose30ThenOpen,
    VldClose,
    VldOpen,
    VldRaiseIn5Mins,
    VldBlazeRaise,
    VldBlazeOpen,
    VldBlazeClose,
}

#[repr(C)]
pub struct VldoorT {
    pub thinker: ThinkerT,
    pub type_: VldoorE,
    pub sector: *mut SectorT,
    pub topheight: FixedT,
    pub speed: FixedT,
    pub direction: i32,
    pub topwait: i32,
    pub topcountdown: i32,
}

pub const VDOORSPEED: FixedT = FRACUNIT * 2;
pub const VDOORWAIT: i32 = 150;

pub fn ev_vertical_door(line: *mut LineT, thing: *mut MobjT) {
    todo!("original: EV_VerticalDoor")
}

pub fn ev_do_door(line: *mut LineT, type_: VldoorE) -> i32 {
    todo!("original: EV_DoDoor")
}

pub fn ev_do_locked_door(line: *mut LineT, type_: VldoorE, thing: *mut MobjT) -> i32 {
    todo!("original: EV_DoLockedDoor")
}

pub fn t_vertical_door(door: *mut VldoorT) {
    todo!("original: T_VerticalDoor")
}

pub fn p_spawn_door_close_in30(sec: *mut SectorT) {
    todo!("original: P_SpawnDoorCloseIn30")
}

pub fn p_spawn_door_raise_in5mins(sec: *mut SectorT, secnum: i32) {
    todo!("original: P_SpawnDoorRaiseIn5Mins")
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CeilingE {
    LowerToFloor,
    RaiseToHighest,
    LowerAndCrush,
    CrushAndRaise,
    FastCrushAndRaise,
    SilentCrushAndRaise,
}

#[repr(C)]
pub struct CeilingT {
    pub thinker: ThinkerT,
    pub type_: CeilingE,
    pub sector: *mut SectorT,
    pub bottomheight: FixedT,
    pub topheight: FixedT,
    pub speed: FixedT,
    pub crush: boolean,
    pub direction: i32,
    pub tag: i32,
    pub olddirection: i32,
}

pub const CEILSPEED: FixedT = FRACUNIT;
pub const CEILWAIT: i32 = 150;
pub const MAXCEILINGS: usize = 30;

pub static mut activeceilings: [*mut CeilingT; MAXCEILINGS] =
    [std::ptr::null_mut(); MAXCEILINGS];

pub fn ev_do_ceiling(line: *mut LineT, type_: CeilingE) -> i32 {
    todo!("original: EV_DoCeiling")
}

pub fn t_move_ceiling(ceiling: *mut CeilingT) {
    todo!("original: T_MoveCeiling")
}

pub fn p_add_active_ceiling(c: *mut CeilingT) {
    todo!("original: P_AddActiveCeiling")
}

pub fn p_remove_active_ceiling(c: *mut CeilingT) {
    todo!("original: P_RemoveActiveCeiling")
}

pub fn ev_ceiling_crush_stop(line: *mut LineT) -> i32 {
    todo!("original: EV_CeilingCrushStop")
}

pub fn p_activate_in_stasis_ceiling(line: *mut LineT) {
    todo!("original: P_ActivateInStasisCeiling")
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloorE {
    LowerFloor,
    LowerFloorToLowest,
    TurboLower,
    RaiseFloor,
    RaiseFloorToNearest,
    RaiseToTexture,
    LowerAndChange,
    RaiseFloor24,
    RaiseFloor24AndChange,
    RaiseFloorCrush,
    RaiseFloorTurbo,
    DonutRaise,
    RaiseFloor512,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StairE {
    Build8,
    Turbo16,
}

#[repr(C)]
pub struct FloormoveT {
    pub thinker: ThinkerT,
    pub type_: FloorE,
    pub crush: boolean,
    pub sector: *mut SectorT,
    pub direction: i32,
    pub newspecial: i32,
    pub texture: i16,
    pub floordestheight: FixedT,
    pub speed: FixedT,
}

pub const FLOORSPEED: FixedT = FRACUNIT;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultE {
    Ok,
    Crushed,
    Pastdest,
}

pub fn t_move_plane(
    sector: *mut SectorT,
    speed: FixedT,
    dest: FixedT,
    crush: boolean,
    floor_or_ceiling: i32,
    direction: i32,
) -> ResultE {
    todo!("original: T_MovePlane")
}

pub fn ev_build_stairs(line: *mut LineT, type_: StairE) -> i32 {
    todo!("original: EV_BuildStairs")
}

pub fn ev_do_floor(line: *mut LineT, floortype: FloorE) -> i32 {
    todo!("original: EV_DoFloor")
}

pub fn t_move_floor(floor: *mut FloormoveT) {
    todo!("original: T_MoveFloor")
}

pub fn ev_teleport(line: *mut LineT, side: i32, thing: *mut MobjT) -> i32 {
    todo!("original: EV_Teleport")
}
