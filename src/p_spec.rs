// doomgeneric/p_spec.h — special effects (stub)

pub use crate::doomtype::*;
pub use crate::m_fixed::*;
pub use crate::r_defs::*;

use crate::d_player::PlayerT;

use std::cell::RefCell;

// Original: #define MO_TELEPORTMAN 14
pub const MO_TELEPORTMAN: i32 = 14;

#[repr(C)]
pub struct LightflashT {
    _opaque: u8,
}

#[repr(C)]
pub struct StrobeT {
    _opaque: u8,
}

#[repr(C)]
pub struct GlowT {
    _opaque: u8,
}

#[repr(C)]
pub struct PlatT {
    _opaque: u8,
}

#[repr(C)]
pub struct VldoorT {
    _opaque: u8,
}

#[repr(C)]
pub struct CeilingT {
    _opaque: u8,
}

#[repr(C)]
pub struct FloormoveT {
    _opaque: u8,
}

#[allow(non_camel_case_types)]
pub struct P_SpecState {
    pub level_timer: RefCell<Boolean>,
    pub level_time_count: RefCell<i32>,
}

impl P_SpecState {
    pub fn new() -> Self {
        Self {
            level_timer: RefCell::new(Boolean::False),
            level_time_count: RefCell::new(0),
        }
    }

    pub fn p_init_pic_anims(&self) {
        todo!("P_InitPicAnims");
    }

    pub fn p_spawn_specials(&self) {
        todo!("P_SpawnSpecials");
    }

    pub fn p_update_specials(&self) {
        todo!("P_UpdateSpecials");
    }

    pub fn p_player_in_special_sector(&self, _player: *mut PlayerT) {
        todo!("P_PlayerInSpecialSector");
    }

    pub fn p_find_lowest_floor_surrounding(&self, _sec: *mut SectorT) -> FixedT {
        todo!("P_FindLowestFloorSurrounding");
    }

    pub fn p_find_highest_floor_surrounding(&self, _sec: *mut SectorT) -> FixedT {
        todo!("P_FindHighestFloorSurrounding");
    }

    pub fn p_find_lowest_ceiling_surrounding(&self, _sec: *mut SectorT) -> FixedT {
        todo!("P_FindLowestCeilingSurrounding");
    }

    pub fn p_find_highest_ceiling_surrounding(&self, _sec: *mut SectorT) -> FixedT {
        todo!("P_FindHighestCeilingSurrounding");
    }

    pub fn ev_do_donut(&self, _line: *mut LineT) -> i32 {
        todo!("EV_DoDonut");
    }

    pub fn p_spawn_fire_flicker(&self, _sector: *mut SectorT) {
        todo!("P_SpawnFireFlicker");
    }

    pub fn t_light_flash(&self, _flash: *mut LightflashT) {
        todo!("T_LightFlash");
    }

    pub fn p_spawn_light_flash(&self, _sector: *mut SectorT) {
        todo!("P_SpawnLightFlash");
    }

    pub fn t_strobe_flash(&self, _flash: *mut StrobeT) {
        todo!("T_StrobeFlash");
    }

    pub fn ev_start_light_strobing(&self, _line: *mut LineT) {
        todo!("EV_StartLightStrobing");
    }

    pub fn ev_turn_tag_lights_off(&self, _line: *mut LineT) {
        todo!("EV_TurnTagLightsOff");
    }

    pub fn t_glow(&self, _g: *mut GlowT) {
        todo!("T_Glow");
    }

    pub fn p_spawn_glowing_light(&self, _sector: *mut SectorT) {
        todo!("P_SpawnGlowingLight");
    }

    pub fn p_init_switch_list(&self) {
        todo!("P_InitSwitchList");
    }

    pub fn t_plat_raise(&self, _plat: *mut PlatT) {
        todo!("T_PlatRaise");
    }

    pub fn p_add_active_plat(&self, _plat: *mut PlatT) {
        todo!("P_AddActivePlat");
    }

    pub fn p_remove_active_plat(&self, _plat: *mut PlatT) {
        todo!("P_RemoveActivePlat");
    }

    pub fn ev_stop_plat(&self, _line: *mut LineT) {
        todo!("EV_StopPlat");
    }

    pub fn p_activate_in_stasis(&self, _tag: i32) {
        todo!("P_ActivateInStasis");
    }

    pub fn t_vertical_door(&self, _door: *mut VldoorT) {
        todo!("T_VerticalDoor");
    }

    pub fn p_spawn_door_close_in30(&self, _sec: *mut SectorT) {
        todo!("P_SpawnDoorCloseIn30");
    }

    pub fn p_init_sliding_door_frames(&self) {
        todo!("P_InitSlidingDoorFrames");
    }

    pub fn t_move_ceiling(&self, _ceiling: *mut CeilingT) {
        todo!("T_MoveCeiling");
    }

    pub fn p_add_active_ceiling(&self, _c: *mut CeilingT) {
        todo!("P_AddActiveCeiling");
    }

    pub fn p_remove_active_ceiling(&self, _c: *mut CeilingT) {
        todo!("P_RemoveActiveCeiling");
    }

    pub fn p_activate_in_stasis_ceiling(&self, _line: *mut LineT) {
        todo!("P_ActivateInStasisCeiling");
    }

    pub fn t_move_floor(&self, _floor: *mut FloormoveT) {
        todo!("T_MoveFloor");
    }
}
