// d_mode.h / d_mode.c

pub use crate::doomtype::*;

use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameMissionT {
    Doom = 0,
    Doom2 = 1,
    PackTnt = 2,
    PackPlut = 3,
    PackChex = 4,
    PackHacx = 5,
    Heretic = 6,
    Hexen = 7,
    Strife = 8,
    None = 9,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameModeT {
    Shareware = 0,
    Registered = 1,
    Commercial = 2,
    Retail = 3,
    Indetermined = 4,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameVersionT {
    ExeDoom12 = 0,
    ExeDoom1666 = 1,
    ExeDoom17 = 2,
    ExeDoom18 = 3,
    ExeDoom19 = 4,
    ExeHacx = 5,
    ExeUltimate = 6,
    ExeFinal = 7,
    ExeFinal2 = 8,
    ExeChex = 9,
    ExeHeretic13 = 10,
    ExeHexen11 = 11,
    ExeStrife12 = 12,
    ExeStrife131 = 13,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SkillT {
    NoItems = -1,
    Baby = 0,
    Easy = 1,
    Medium = 2,
    Hard = 3,
    Nightmare = 4,
}

#[derive(Clone, Copy)]
struct ValidMode {
    // Original: mission
    mission: GameMissionT,
    // Original: mode
    mode: GameModeT,
    // Original: episode
    episode: i32,
    // Original: map
    map: i32,
}

#[derive(Clone, Copy)]
struct ValidVersion {
    // Original: mission
    mission: GameMissionT,
    // Original: version
    version: GameVersionT,
}

#[allow(dead_code)]
static VALID_MODES: &[ValidMode] = &[
    ValidMode { mission: GameMissionT::PackChex, mode: GameModeT::Shareware, episode: 1, map: 5 },
    ValidMode { mission: GameMissionT::Doom, mode: GameModeT::Shareware, episode: 1, map: 9 },
    ValidMode { mission: GameMissionT::Doom, mode: GameModeT::Registered, episode: 3, map: 9 },
    ValidMode { mission: GameMissionT::Doom, mode: GameModeT::Retail, episode: 4, map: 9 },
    ValidMode { mission: GameMissionT::Doom2, mode: GameModeT::Commercial, episode: 1, map: 32 },
    ValidMode { mission: GameMissionT::PackTnt, mode: GameModeT::Commercial, episode: 1, map: 32 },
    ValidMode { mission: GameMissionT::PackPlut, mode: GameModeT::Commercial, episode: 1, map: 32 },
    ValidMode { mission: GameMissionT::PackHacx, mode: GameModeT::Commercial, episode: 1, map: 32 },
    ValidMode { mission: GameMissionT::Heretic, mode: GameModeT::Shareware, episode: 1, map: 9 },
    ValidMode { mission: GameMissionT::Heretic, mode: GameModeT::Registered, episode: 3, map: 9 },
    ValidMode { mission: GameMissionT::Heretic, mode: GameModeT::Retail, episode: 5, map: 9 },
    ValidMode { mission: GameMissionT::Hexen, mode: GameModeT::Commercial, episode: 1, map: 60 },
    ValidMode { mission: GameMissionT::Strife, mode: GameModeT::Commercial, episode: 1, map: 34 },
];

#[allow(dead_code)]
static VALID_VERSIONS: &[ValidVersion] = &[
    ValidVersion { mission: GameMissionT::Doom, version: GameVersionT::ExeDoom19 },
    ValidVersion { mission: GameMissionT::Doom, version: GameVersionT::ExeHacx },
    ValidVersion { mission: GameMissionT::Doom, version: GameVersionT::ExeUltimate },
    ValidVersion { mission: GameMissionT::Doom, version: GameVersionT::ExeFinal },
    ValidVersion { mission: GameMissionT::Doom, version: GameVersionT::ExeFinal2 },
    ValidVersion { mission: GameMissionT::Doom, version: GameVersionT::ExeChex },
    ValidVersion { mission: GameMissionT::Heretic, version: GameVersionT::ExeHeretic13 },
    ValidVersion { mission: GameMissionT::Hexen, version: GameVersionT::ExeHexen11 },
    ValidVersion { mission: GameMissionT::Strife, version: GameVersionT::ExeStrife12 },
    ValidVersion { mission: GameMissionT::Strife, version: GameVersionT::ExeStrife131 },
];

#[allow(non_camel_case_types)]
pub struct D_ModeState {
    _ph: RefCell<()>,
}

impl D_ModeState {
    pub fn new() -> Self {
        Self { _ph: RefCell::new(()) }
    }

    // Original: D_ValidGameMode
    pub fn d_valid_game_mode(&self, mission: GameMissionT, mode: GameModeT) -> bool {
        VALID_MODES.iter().any(|m| m.mode == mode && m.mission == mission)
    }

    // Original: D_ValidEpisodeMap - complex, skip
    pub fn d_valid_episode_map(&self, _mission: GameMissionT, _mode: GameModeT, _episode: i32, _map: i32) -> bool {
        todo!("D_ValidEpisodeMap")
    }

    // Original: D_GetNumEpisodes - complex (calls D_ValidEpisodeMap)
    pub fn d_get_num_episodes(&self, _mission: GameMissionT, _mode: GameModeT) -> i32 {
        todo!("D_GetNumEpisodes")
    }

    // Original: D_ValidGameVersion - complex
    pub fn d_valid_game_version(&self, _mission: GameMissionT, _version: GameVersionT) -> bool {
        todo!("D_ValidGameVersion")
    }

    // Original: D_IsEpisodeMap - complex switch
    pub fn d_is_episode_map(&self, _mission: GameMissionT) -> bool {
        todo!("D_IsEpisodeMap")
    }

    // Original: D_GameMissionString - complex switch
    pub fn d_game_mission_string(&self, _mission: GameMissionT) -> &'static str {
        todo!("D_GameMissionString")
    }
}
