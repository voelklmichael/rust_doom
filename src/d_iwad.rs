//! IWAD detection (d_iwad.h, d_iwad.c)
//! Original: d_iwad.h, d_iwad.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_mode::{GameMissionT, GameModeT};

// doom=0, doom2=1, pack_tnt=2, pack_plut=3, pack_chex=4, pack_hacx=5
pub const IWAD_MASK_DOOM: u32 = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5);
pub const IWAD_MASK_HERETIC: u32 = 1 << 6;
pub const IWAD_MASK_HEXEN: u32 = 1 << 7;
pub const IWAD_MASK_STRIFE: u32 = 1 << 8;

pub struct IwadT {
    pub name: String,
    pub mission: GameMissionT,
    pub mode: GameModeT,
    pub description: String,
}

pub struct D_IwadState;

impl D_IwadState {
    /// Original: char *D_FindWADByName(char *filename)
    pub fn d_find_wad_by_name(&self, _filename: &str) -> Option<String> {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *D_TryFindWADByName(char *filename)
    pub fn d_try_find_wad_by_name(&self, _filename: &str) -> Option<String> {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *D_FindIWAD(int mask, GameMission_t *mission)
    pub fn d_find_iwad(&self, _mask: u32, _mission: &mut GameMissionT) -> Option<String> {
        todo!("Basic stage-0 stub")
    }

    /// Original: const iwad_t **D_FindAllIWADs(int mask)
    pub fn d_find_all_iwads(&self, _mask: u32) -> Vec<IwadT> {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *D_SaveGameIWADName(GameMission_t gamemission)
    pub fn d_save_game_iwad_name(&self, _gamemission: GameMissionT) -> Option<String> {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *D_SuggestIWADName(GameMission_t mission, GameMode_t mode)
    pub fn d_suggest_iwad_name(&self, _mission: GameMissionT, _mode: GameModeT) -> Option<String> {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *D_SuggestGameName(GameMission_t mission, GameMode_t mode)
    pub fn d_suggest_game_name(&self, _mission: GameMissionT, _mode: GameModeT) -> Option<String> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void D_CheckCorrectIWAD(GameMission_t mission)
    pub fn d_check_correct_iwad(&self, _mission: GameMissionT) {
        todo!("Basic stage-0 stub")
    }
}
