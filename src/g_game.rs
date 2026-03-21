// doomgeneric/g_game.h

pub use crate::am_map::*;
pub use crate::d_event::*;
pub use crate::d_main::*;
pub use crate::d_ticcmd::*;
pub use crate::deh_main::*;
pub use crate::deh_misc::*;
pub use crate::doomdef::*;
pub use crate::doomkeys::*;
pub use crate::doomstat::*;
pub use crate::dstrings::*;
pub use crate::f_finale::*;
pub use crate::hu_stuff::*;
pub use crate::i_system::*;
pub use crate::i_timer::*;
pub use crate::i_video::*;
pub use crate::m_argv::*;
pub use crate::m_controls::*;
pub use crate::m_menu::*;
pub use crate::m_misc::*;
pub use crate::m_random::*;
pub use crate::p_local::*;
pub use crate::p_saveg::*;
pub use crate::p_setup::*;
pub use crate::p_tick::*;
pub use crate::r_data::*;
pub use crate::r_sky::*;
pub use crate::s_sound::*;
pub use crate::sounds::*;
pub use crate::st_stuff::*;
pub use crate::statdump::*;
pub use crate::v_video::*;
pub use crate::w_wad::*;
pub use crate::wi_stuff::*;
pub use crate::z_zone::*;

use crate::d_mode::SkillT;

use std::cell::RefCell;
use std::ffi::c_char;

#[allow(non_camel_case_types)]
pub struct G_GameState {
    pub vanilla_savegame_limit: RefCell<i32>,
    pub vanilla_demo_limit: RefCell<i32>,
}

impl G_GameState {
    pub fn new() -> Self {
        Self {
            vanilla_savegame_limit: RefCell::new(0),
            vanilla_demo_limit: RefCell::new(0),
        }
    }

    pub fn g_death_match_spawn_player(&self, _playernum: i32) {
        todo!("G_DeathMatchSpawnPlayer");
    }

    pub fn g_init_new(&self, _skill: SkillT, _episode: i32, _map: i32) {
        todo!("G_InitNew");
    }

    pub fn g_defered_init_new(&self, _skill: SkillT, _episode: i32, _map: i32) {
        todo!("G_DeferedInitNew");
    }

    pub fn g_defered_play_demo(&self, _demo: *mut c_char) {
        todo!("G_DeferedPlayDemo");
    }

    pub fn g_load_game(&self, _name: *mut c_char) {
        todo!("G_LoadGame");
    }

    pub fn g_do_load_game(&self) {
        todo!("G_DoLoadGame");
    }

    pub fn g_save_game(&self, _slot: i32, _description: *mut c_char) {
        todo!("G_SaveGame");
    }

    pub fn g_record_demo(&self, _name: *mut c_char) {
        todo!("G_RecordDemo");
    }

    pub fn g_begin_recording(&self) {
        todo!("G_BeginRecording");
    }

    pub fn g_play_demo(&self, _name: *mut c_char) {
        todo!("G_PlayDemo");
    }

    pub fn g_time_demo(&self, _name: *mut c_char) {
        todo!("G_TimeDemo");
    }

    pub fn g_check_demo_status(&self) -> crate::doomtype::Boolean {
        todo!("G_CheckDemoStatus");
    }

    pub fn g_exit_level(&self) {
        todo!("G_ExitLevel");
    }

    pub fn g_secret_exit_level(&self) {
        todo!("G_SecretExitLevel");
    }

    pub fn g_world_done(&self) {
        todo!("G_WorldDone");
    }

    pub fn g_build_ticcmd(&self, _cmd: *mut TiccmdT, _maketic: i32) {
        todo!("G_BuildTiccmd");
    }

    pub fn g_ticker(&self) {
        todo!("G_Ticker");
    }

    pub fn g_responder(&self, _ev: *mut crate::d_event::EventT) -> crate::doomtype::Boolean {
        todo!("G_Responder");
    }

    pub fn g_screen_shot(&self) {
        todo!("G_ScreenShot");
    }

    pub fn g_draw_mouse_speed_box(&self) {
        todo!("G_DrawMouseSpeedBox");
    }

    pub fn g_vanilla_version_code(&self) -> i32 {
        todo!("G_VanillaVersionCode");
    }
}
