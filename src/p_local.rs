//! Play functions, animation, global header (p_local.h)
//! Original: p_local.h

use crate::m_fixed::FixedT;

// #define FLOATSPEED (FRACUNIT*4)
// #define MAXHEALTH 100
// #define VIEWHEIGHT (41*FRACUNIT)
// #define MAPBLOCKUNITS 128
// #define MAPBLOCKSIZE, MAPBLOCKSHIFT, MAPBMASK, MAPBTOFRAC
// #define PLAYERRADIUS 16*FRACUNIT
// #define MAXRADIUS 32*FRACUNIT
// #define GRAVITY FRACUNIT
// #define MAXMOVE (30*FRACUNIT)
// #define USERANGE (64*FRACUNIT)
// #define MELEERANGE (64*FRACUNIT)
// #define MISSILERANGE (32*64*FRACUNIT)
// #define BASETHRESHOLD 100
// #define ONFLOORZ INT_MIN
// #define ONCEILINGZ INT_MAX
// #define ITEMQUESIZE 128
// #define MAX_DM_STARTS 10
// #define MAXSPECIALCROSS 20
// #define MAXSPECIALCROSS_ORIGINAL 8
// #define MAXINTERCEPTS_ORIGINAL 128
// #define MAXINTERCEPTS (MAXINTERCEPTS_ORIGINAL + 61)

pub const MAXCEILINGS: usize = 30;
pub const MAXPLATS: usize = 30;

pub struct DivlineT {
    pub x: FixedT,
    pub y: FixedT,
    pub dx: FixedT,
    pub dy: FixedT,
}

pub struct P_LocalState;

impl P_LocalState {
    /// Original: void P_InitThinkers(void)
    pub fn p_init_thinkers(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_AddThinker(thinker_t *thinker)
    pub fn p_add_thinker(&self, _thinker: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_RemoveThinker(thinker_t *thinker)
    pub fn p_remove_thinker(&self, _thinker: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SetupPsprites(player_t *curplayer)
    pub fn p_setup_psprites(&self, _curplayer: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_MovePsprites(player_t *curplayer)
    pub fn p_move_psprites(&self, _curplayer: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_DropWeapon(player_t *player)
    pub fn p_drop_weapon(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_PlayerThink(player_t *player)
    pub fn p_player_think(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_RespawnSpecials(void)
    pub fn p_respawn_specials(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: mobj_t *P_SpawnMobj(fixed_t x, fixed_t y, fixed_t z, mobjtype_t type)
    pub fn p_spawn_mobj(&self, _x: FixedT, _y: FixedT, _z: FixedT, _mobj_type: i32) -> Option<()> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_RemoveMobj(mobj_t *th)
    pub fn p_remove_mobj(&self, _th: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: mobj_t *P_SubstNullMobj(mobj_t *th)
    pub fn p_subst_null_mobj(&self, _th: &()) -> Option<()> {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_SetMobjState(mobj_t *mobj, statenum_t state)
    pub fn p_set_mobj_state(&self, _mobj: &(), _state: i32) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_MobjThinker(mobj_t *mobj)
    pub fn p_mobj_thinker(&self, _mobj: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SpawnPuff(fixed_t x, fixed_t y, fixed_t z)
    pub fn p_spawn_puff(&self, _x: FixedT, _y: FixedT, _z: FixedT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SpawnBlood(fixed_t x, fixed_t y, fixed_t z, int damage)
    pub fn p_spawn_blood(&self, _x: FixedT, _y: FixedT, _z: FixedT, _damage: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: mobj_t *P_SpawnMissile(mobj_t *source, mobj_t *dest, mobjtype_t type)
    pub fn p_spawn_missile(&self, _source: &(), _dest: &(), _mobj_type: i32) -> Option<()> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SpawnPlayerMissile(mobj_t *source, mobjtype_t type)
    pub fn p_spawn_player_missile(&self, _source: &(), _mobj_type: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_NoiseAlert(mobj_t *target, mobj_t *emmiter)
    pub fn p_noise_alert(&self, _target: &(), _emmiter: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t P_AproxDistance(fixed_t dx, fixed_t dy)
    pub fn p_aprox_distance(&self, _dx: FixedT, _dy: FixedT) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: int P_PointOnLineSide(fixed_t x, fixed_t y, line_t *line)
    pub fn p_point_on_line_side(&self, _x: FixedT, _y: FixedT, _line: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int P_PointOnDivlineSide(fixed_t x, fixed_t y, divline_t *line)
    pub fn p_point_on_divline_side(&self, _x: FixedT, _y: FixedT, _line: &DivlineT) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_MakeDivline(line_t *li, divline_t *dl)
    pub fn p_make_divline(&self, _li: &(), _dl: &mut DivlineT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t P_InterceptVector(divline_t *v2, divline_t *v1)
    pub fn p_intercept_vector(&self, _v2: &DivlineT, _v1: &DivlineT) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: int P_BoxOnLineSide(fixed_t *tmbox, line_t *ld)
    pub fn p_box_on_line_side(&self, _tmbox: &[FixedT; 4], _ld: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_LineOpening(line_t *linedef)
    pub fn p_line_opening(&self, _linedef: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_BlockLinesIterator(int x, int y, boolean (*func)(line_t*))
    pub fn p_block_lines_iterator(&self, _x: i32, _y: i32) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_BlockThingsIterator(int x, int y, boolean (*func)(mobj_t*))
    pub fn p_block_things_iterator(&self, _x: i32, _y: i32) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_PathTraverse(...)
    pub fn p_path_traverse(&self, _x1: FixedT, _y1: FixedT, _x2: FixedT, _y2: FixedT, _flags: i32) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_UnsetThingPosition(mobj_t *thing)
    pub fn p_unset_thing_position(&self, _thing: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SetThingPosition(mobj_t *thing)
    pub fn p_set_thing_position(&self, _thing: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_CheckPosition(mobj_t *thing, fixed_t x, fixed_t y)
    pub fn p_check_position(&self, _thing: &(), _x: FixedT, _y: FixedT) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_TryMove(mobj_t *thing, fixed_t x, fixed_t y)
    pub fn p_try_move(&self, _thing: &(), _x: FixedT, _y: FixedT) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_TeleportMove(mobj_t *thing, fixed_t x, fixed_t y)
    pub fn p_teleport_move(&self, _thing: &(), _x: FixedT, _y: FixedT) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SlideMove(mobj_t *mo)
    pub fn p_slide_move(&self, _mo: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_CheckSight(mobj_t *t1, mobj_t *t2)
    pub fn p_check_sight(&self, _t1: &(), _t2: &()) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_UseLines(player_t *player)
    pub fn p_use_lines(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_ChangeSector(sector_t *sector, boolean crunch)
    pub fn p_change_sector(&self, _sector: &(), _crunch: bool) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t P_AimLineAttack(mobj_t *t1, angle_t angle, fixed_t distance)
    pub fn p_aim_line_attack(&self, _t1: &(), _angle: u32, _distance: FixedT) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_LineAttack(...)
    pub fn p_line_attack(&self, _t1: &(), _angle: u32, _distance: FixedT, _slope: FixedT, _damage: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_RadiusAttack(mobj_t *spot, mobj_t *source, int damage)
    pub fn p_radius_attack(&self, _spot: &(), _source: &(), _damage: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_TouchSpecialThing(...)
    pub fn p_touch_special_thing(&self, _special: &(), _toucher: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_DamageMobj(...)
    pub fn p_damage_mobj(&self, _target: &(), _inflictor: &(), _source: &(), _damage: i32) {
        todo!("Basic stage-0 stub")
    }
}
