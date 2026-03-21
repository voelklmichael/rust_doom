// doomgeneric/d_iwad.h

use std::cell::RefCell;
use std::ffi::c_char;

pub use crate::d_mode::*;

// Original: IWAD_MASK_* macros
pub const IWAD_MASK_DOOM: i32 = (1 << GameMissionT::Doom as i32)
    | (1 << GameMissionT::Doom2 as i32)
    | (1 << GameMissionT::PackTnt as i32)
    | (1 << GameMissionT::PackPlut as i32)
    | (1 << GameMissionT::PackChex as i32)
    | (1 << GameMissionT::PackHacx as i32);
pub const IWAD_MASK_HERETIC: i32 = 1 << GameMissionT::Heretic as i32;
pub const IWAD_MASK_HEXEN: i32 = 1 << GameMissionT::Hexen as i32;
pub const IWAD_MASK_STRIFE: i32 = 1 << GameMissionT::Strife as i32;

/// Original: typedef struct { char *name; GameMission_t mission; ... } iwad_t
#[repr(C)]
pub struct IwadT {
    pub name: *mut c_char,
    pub mission: GameMissionT,
    pub mode: GameModeT,
    pub description: *mut c_char,
}

#[allow(non_camel_case_types)]
pub struct D_IwadState {
    pub _scratch: RefCell<i32>,
}

impl D_IwadState {
    pub fn new() -> Self {
        Self {
            _scratch: RefCell::new(0),
        }
    }

    // Original: D_FindWADByName
    pub fn d_find_wad_by_name(&self, _filename: *mut c_char) -> *mut c_char {
        todo!("D_FindWADByName");
    }

    // Original: D_TryFindWADByName
    pub fn d_try_find_wad_by_name(&self, _filename: *mut c_char) -> *mut c_char {
        todo!("D_TryFindWADByName");
    }

    // Original: D_FindIWAD
    pub fn d_find_iwad(&self, _mask: i32, _mission: *mut GameMissionT) -> *mut c_char {
        todo!("D_FindIWAD");
    }

    // Original: D_FindAllIWADs
    pub fn d_find_all_iwads(&self, _mask: i32) -> *const *const IwadT {
        todo!("D_FindAllIWADs");
    }

    // Original: D_SaveGameIWADName
    pub fn d_save_game_iwad_name(&self, _gamemission: GameMissionT) -> *mut c_char {
        todo!("D_SaveGameIWADName");
    }

    // Original: D_SuggestIWADName
    pub fn d_suggest_iwad_name(&self, _mission: GameMissionT, _mode: GameModeT) -> *mut c_char {
        todo!("D_SuggestIWADName");
    }

    // Original: D_SuggestGameName
    pub fn d_suggest_game_name(&self, _mission: GameMissionT, _mode: GameModeT) -> *mut c_char {
        todo!("D_SuggestGameName");
    }

    // Original: D_CheckCorrectIWAD
    pub fn d_check_correct_iwad(&self, _mission: GameMissionT) {
        todo!("D_CheckCorrectIWAD");
    }
}
