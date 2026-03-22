//! Special effects (p_spec.h, p_spec.c)
//! Original: p_spec.h, p_spec.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::m_fixed::FixedT;
use crate::doomtype::Boolean;

pub const MO_TELEPORTMAN: i32 = 14;

pub struct P_SpecState {
    pub level_timer: Arc<Mutex<Boolean>>,
    pub level_time_count: Arc<Mutex<i32>>,
}

impl P_SpecState {
    /// Original: void P_InitPicAnims(void)
    pub fn p_init_pic_anims(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SpawnSpecials(void)
    pub fn p_spawn_specials(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_UpdateSpecials(void)
    pub fn p_update_specials(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_UseSpecialLine(mobj_t *thing, line_t *line, int side)
    pub fn p_use_special_line(&self, _thing: &(), _line: &(), _side: i32) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_ShootSpecialLine(mobj_t *thing, line_t *line)
    pub fn p_shoot_special_line(&self, _thing: &(), _line: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_CrossSpecialLine(int linenum, int side, mobj_t *thing)
    pub fn p_cross_special_line(&self, _linenum: i32, _side: i32, _thing: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_PlayerInSpecialSector(player_t *player)
    pub fn p_player_in_special_sector(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: int twoSided(int sector, int line)
    pub fn two_sided(&self, _sector: i32, _line: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t P_FindLowestFloorSurrounding(sector_t *sec)
    pub fn p_find_lowest_floor_surrounding(&self, _sec: &()) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t P_FindHighestFloorSurrounding(sector_t *sec)
    pub fn p_find_highest_floor_surrounding(&self, _sec: &()) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t P_FindNextHighestFloor(sector_t *sec, int currentheight)
    pub fn p_find_next_highest_floor(&self, _sec: &(), _currentheight: i32) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t P_FindLowestCeilingSurrounding(sector_t *sec)
    pub fn p_find_lowest_ceiling_surrounding(&self, _sec: &()) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t P_FindHighestCeilingSurrounding(sector_t *sec)
    pub fn p_find_highest_ceiling_surrounding(&self, _sec: &()) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: int P_FindSectorFromLineTag(line_t *line, int start)
    pub fn p_find_sector_from_line_tag(&self, _line: &(), _start: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int P_FindMinSurroundingLight(sector_t *sector, int max)
    pub fn p_find_min_surrounding_light(&self, _sector: &(), _max: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int EV_DoDonut(line_t *line)
    pub fn ev_do_donut(&self, _line: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }
}
