//! Game logic (g_game.h, g_game.c)
//! Original: g_game.h, g_game.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_event::EventT;
use crate::d_mode::SkillT;
use crate::d_ticcmd::TiccmdT;
use crate::doomtype::Boolean;

pub struct G_GameState {
    pub vanilla_savegame_limit: Arc<Mutex<i32>>,
    pub vanilla_demo_limit: Arc<Mutex<i32>>,
}

impl G_GameState {
    /// Original: void G_DeathMatchSpawnPlayer(int playernum)
    pub fn g_death_match_spawn_player(&self, _playernum: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_InitNew(skill_t skill, int episode, int map)
    pub fn g_init_new(&self, _skill: SkillT, _episode: i32, _map: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_DeferedInitNew(skill_t skill, int episode, int map)
    pub fn g_defered_init_new(&self, _skill: SkillT, _episode: i32, _map: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_DeferedPlayDemo(char *demo)
    pub fn g_defered_play_demo(&self, _demo: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_LoadGame(char *name)
    pub fn g_load_game(&self, _name: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_DoLoadGame(void)
    pub fn g_do_load_game(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_SaveGame(int slot, char *description)
    pub fn g_save_game(&self, _slot: i32, _description: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_RecordDemo(char *name)
    pub fn g_record_demo(&self, _name: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_BeginRecording(void)
    pub fn g_begin_recording(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_PlayDemo(char *name)
    pub fn g_play_demo(&self, _name: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_TimeDemo(char *name)
    pub fn g_time_demo(&self, _name: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean G_CheckDemoStatus(void)
    pub fn g_check_demo_status(&self) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_ExitLevel(void)
    pub fn g_exit_level(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_SecretExitLevel(void)
    pub fn g_secret_exit_level(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_WorldDone(void)
    pub fn g_world_done(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_BuildTiccmd(ticcmd_t *cmd, int maketic)
    pub fn g_build_ticcmd(&self, _cmd: &mut TiccmdT, _maketic: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_Ticker(void)
    pub fn g_ticker(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean G_Responder(event_t *ev)
    pub fn g_responder(&self, _ev: &EventT) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_ScreenShot(void)
    pub fn g_screenshot(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void G_DrawMouseSpeedBox(void)
    pub fn g_draw_mouse_speed_box(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: int G_VanillaVersionCode(void)
    pub fn g_vanilla_version_code(&self) -> i32 {
        todo!("Basic stage-0 stub")
    }
}
