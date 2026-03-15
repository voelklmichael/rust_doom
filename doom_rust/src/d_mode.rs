//! Rust translation of doomgeneric/d_mode.h
//! Functions and definitions relating to the game type and operational mode.

use crate::doomtype::*;

/// C typedef: GameMission_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: game_mission_t
pub enum GameMissionT {
    Doom,       // Doom 1
    Doom2,      // Doom 2
    PackTnt,    // Final Doom: TNT: Evilution
    PackPlut,   // Final Doom: The Plutonia Experiment
    PackChex,   // Chex Quest (modded doom)
    PackHacx,   // Hacx (modded doom2)
    Heretic,    // Heretic
    Hexen,      // Hexen
    Strife,     // Strife
    None,
}

/// C typedef: GameMode_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: game_mode_t
pub enum GameModeT {
    Shareware,     // Doom/Heretic shareware
    Registered,    // Doom/Heretic registered
    Commercial,    // Doom II/Hexen
    Retail,        // Ultimate Doom
    Indetermined,  // Unknown
}

/// C typedef: GameVersion_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: game_version_t
pub enum GameVersionT {
    ExeDoom12,      // Doom 1.2
    ExeDoom1666,    // Doom 1.666
    ExeDoom17,      // Doom 1.7/1.7a
    ExeDoom18,      // Doom 1.8
    ExeDoom19,      // Doom 1.9
    ExeHacx,        // Hacx
    ExeUltimate,    // Ultimate Doom (retail)
    ExeFinal,       // Final Doom
    ExeFinal2,      // Final Doom (alternate exe)
    ExeChex,        // Chex Quest executable
    ExeHeretic13,   // Heretic 1.3
    ExeHexen11,     // Hexen 1.1
    ExeStrife12,    // Strife v1.2
    ExeStrife131,   // Strife v1.31
}

/// C typedef: skill_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: skill_t
pub enum SkillT {
    NoItems = -1,  // the "-skill 0" hack
    Baby = 0,
    Easy,
    Medium,
    Hard,
    Nightmare,
}

/// C function: D_ValidGameMode
pub fn d_valid_game_mode(mission: GameMissionT, mode: GameModeT) -> boolean {
    todo!("original: D_ValidGameMode")
}

/// C function: D_ValidGameVersion
pub fn d_valid_game_version(mission: GameMissionT, version: GameVersionT) -> boolean {
    todo!("original: D_ValidGameVersion")
}

/// C function: D_ValidEpisodeMap
pub fn d_valid_episode_map(
    mission: GameMissionT,
    mode: GameModeT,
    episode: i32,
    map: i32,
) -> boolean {
    todo!("original: D_ValidEpisodeMap")
}

/// C function: D_GetNumEpisodes
pub fn d_get_num_episodes(mission: GameMissionT, mode: GameModeT) -> i32 {
    todo!("original: D_GetNumEpisodes")
}

/// C function: D_IsEpisodeMap
pub fn d_is_episode_map(mission: GameMissionT) -> boolean {
    todo!("original: D_IsEpisodeMap")
}

/// C function: D_GameMissionString
pub fn d_game_mission_string(mission: GameMissionT) -> *mut i8 {
    todo!("original: D_GameMissionString")
}
