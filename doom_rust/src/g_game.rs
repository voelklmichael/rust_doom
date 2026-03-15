//! Rust translation of doomgeneric/g_game.h

use crate::d_event::*;
use crate::d_mode::*;
use crate::d_ticcmd::*;
use crate::doomdef::*;
use crate::doomtype::*;

/// C function: G_DeathMatchSpawnPlayer
pub fn g_death_match_spawn_player(playernum: i32) {
    todo!("original: G_DeathMatchSpawnPlayer")
}

/// C function: G_InitNew
pub fn g_init_new(skill: SkillT, episode: i32, map: i32) {
    todo!("original: G_InitNew")
}

/// C function: G_DeferedInitNew
pub fn g_defered_init_new(skill: SkillT, episode: i32, map: i32) {
    todo!("original: G_DeferedInitNew")
}

/// C function: G_DeferedPlayDemo
pub fn g_defered_play_demo(demo: &str) {
    todo!("original: G_DeferedPlayDemo")
}

/// C function: G_LoadGame
pub fn g_load_game(name: &str) {
    todo!("original: G_LoadGame")
}

/// C function: G_DoLoadGame
pub fn g_do_load_game() {
    todo!("original: G_DoLoadGame")
}

/// C function: G_SaveGame
pub fn g_save_game(slot: i32, description: &str) {
    todo!("original: G_SaveGame")
}

/// C function: G_RecordDemo
pub fn g_record_demo(name: &str) {
    todo!("original: G_RecordDemo")
}

/// C function: G_BeginRecording
pub fn g_begin_recording() {
    todo!("original: G_BeginRecording")
}

/// C function: G_PlayDemo
pub fn g_play_demo(name: &str) {
    todo!("original: G_PlayDemo")
}

/// C function: G_TimeDemo
pub fn g_time_demo(name: &str) {
    todo!("original: G_TimeDemo")
}

/// C function: G_CheckDemoStatus
pub fn g_check_demo_status() -> boolean {
    todo!("original: G_CheckDemoStatus")
}

/// C function: G_ExitLevel
pub fn g_exit_level() {
    todo!("original: G_ExitLevel")
}

/// C function: G_SecretExitLevel
pub fn g_secret_exit_level() {
    todo!("original: G_SecretExitLevel")
}

/// C function: G_WorldDone
pub fn g_world_done() {
    todo!("original: G_WorldDone")
}

/// C function: G_BuildTiccmd
pub fn g_build_ticcmd(cmd: &mut TiccmdT, maketic: i32) {
    todo!("original: G_BuildTiccmd")
}

/// C function: G_Ticker
pub fn g_ticker() {
    todo!("original: G_Ticker")
}

/// C function: G_Responder
pub fn g_responder(ev: &mut EventT) -> boolean {
    todo!("original: G_Responder")
}

/// C function: G_ScreenShot
pub fn g_screen_shot() {
    todo!("original: G_ScreenShot")
}

/// C function: G_DrawMouseSpeedBox
pub fn g_draw_mouse_speed_box() {
    todo!("original: G_DrawMouseSpeedBox")
}

/// C function: G_VanillaVersionCode
pub fn g_vanilla_version_code() -> i32 {
    todo!("original: G_VanillaVersionCode")
}

pub static mut vanilla_savegame_limit: i32 = 0;
pub static mut vanilla_demo_limit: i32 = 0;
