//! Rust translation of doomgeneric/d_iwad.h
//! Find IWAD and initialize according to IWAD type.

use crate::d_mode::*;
use std::sync::{Arc, Mutex};

/// C #define: IWAD_MASK_DOOM
pub const IWAD_MASK_DOOM: i32 = (1 << GameMissionT::Doom as i32)
    | (1 << GameMissionT::Doom2 as i32)
    | (1 << GameMissionT::PackTnt as i32)
    | (1 << GameMissionT::PackPlut as i32)
    | (1 << GameMissionT::PackChex as i32)
    | (1 << GameMissionT::PackHacx as i32);
/// C #define: IWAD_MASK_HERETIC
pub const IWAD_MASK_HERETIC: i32 = 1 << GameMissionT::Heretic as i32;
/// C #define: IWAD_MASK_HEXEN
pub const IWAD_MASK_HEXEN: i32 = 1 << GameMissionT::Hexen as i32;
/// C #define: IWAD_MASK_STRIFE
pub const IWAD_MASK_STRIFE: i32 = 1 << GameMissionT::Strife as i32;

/// C typedef: iwad_t
#[repr(C)]
/// C typedef: iwad_t
pub struct IwadT {
    pub name: String,
    pub mission: GameMissionT,
    pub mode: GameModeT,
    pub description: String,
}

/// C function: D_FindWADByName
pub fn d_find_wad_by_name(filename: &str) -> String {
    todo!("original: D_FindWADByName")
}

/// C function: D_TryFindWADByName
pub fn d_try_find_wad_by_name(filename: &str) -> String {
    todo!("original: D_TryFindWADByName")
}

/// C function: D_FindIWAD
pub fn d_find_iwad(mask: i32, mission: &mut GameMissionT) -> String {
    todo!("original: D_FindIWAD")
}

/// C function: D_FindAllIWADs
pub fn d_find_all_iwads(mask: i32) -> Vec<Arc<Mutex<IwadT>>> {
    todo!("original: D_FindAllIWADs")
}

/// C function: D_SaveGameIWADName
pub fn d_save_game_iwad_name(gamemission: GameMissionT) -> Arc<Mutex<String>> {
    todo!("original: D_SaveGameIWADName")
}

/// C function: D_SuggestIWADName
pub fn d_suggest_iwad_name(mission: GameMissionT, mode: GameModeT) -> Arc<Mutex<String>> {
    todo!("original: D_SuggestIWADName")
}

/// C function: D_SuggestGameName
pub fn d_suggest_game_name(mission: GameMissionT, mode: GameModeT) -> Arc<Mutex<String>> {
    todo!("original: D_SuggestGameName")
}

/// C function: D_CheckCorrectIWAD
pub fn d_check_correct_iwad(mission: GameMissionT) {
    todo!("original: D_CheckCorrectIWAD")
}
