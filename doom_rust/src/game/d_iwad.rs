//
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//     Find IWAD and initialize according to IWAD type.
//
// Original: d_iwad.h / d_iwad.c (stub implementation)

use crate::m_argv;

use super::d_mode::{GameMission, GameMode};

/// IWAD names to search, in priority order. (doom2, doom, doom1 for Doom games)
const IWAD_NAMES: &[(&str, GameMission, GameMode)] = &[
    ("doom2.wad", GameMission::Doom2, GameMode::Commercial),
    ("plutonia.wad", GameMission::PackPlut, GameMode::Commercial),
    ("tnt.wad", GameMission::PackTnt, GameMode::Commercial),
    ("doom.wad", GameMission::Doom, GameMode::Retail),
    ("doom1.wad", GameMission::Doom, GameMode::Shareware),
];

/// Find IWAD file. Checks -iwad first, then -file, then current dir.
/// Returns (path, mission) or None if not found.
/// Original: d_iwad.c D_FindIWAD
pub fn d_find_iwad() -> Option<(String, GameMission)> {
    // -iwad <path>: use specified path
    let p = m_argv::m_check_parm_with_args("-iwad", 1);
    if p != 0 {
        let path = m_argv::myargv().get(p + 1).cloned().unwrap_or_default();
        if std::path::Path::new(&path).exists() {
            let mission = mission_from_filename(&path);
            return Some((path, mission));
        }
    }

    // -file <path>: first file may be IWAD
    let p = m_argv::m_check_parm_with_args("-file", 1);
    if p != 0 {
        let path = m_argv::myargv().get(p + 1).cloned().unwrap_or_default();
        if std::path::Path::new(&path).exists() {
            let mission = mission_from_filename(&path);
            return Some((path, mission));
        }
    }

    // Search current directory for known IWADs
    for (name, mission, _) in IWAD_NAMES {
        if std::path::Path::new(name).exists() {
            return Some((name.to_string(), *mission));
        }
    }

    None
}

fn mission_from_filename(path: &str) -> GameMission {
    let lower = path.to_lowercase();
    for (name, mission, _) in IWAD_NAMES {
        if lower.ends_with(name) {
            return *mission;
        }
    }
    GameMission::Doom
}

/// Try to find a WAD file by name. Returns the path if found, otherwise returns filename as-is.
/// Stub: just returns the filename (assumes it's a valid path).
/// Original: d_iwad.c D_TryFindWADByName
pub fn d_try_find_wad_by_name(filename: &str) -> String {
    if std::path::Path::new(filename).exists() {
        filename.to_string()
    } else {
        filename.to_string()
    }
}

/// Suggest game name for error messages.
/// Original: d_iwad.c D_SuggestGameName
pub fn d_suggest_game_name(_mission: GameMission, _mode: GameMode) -> &'static str {
    "Unknown game?"
}
