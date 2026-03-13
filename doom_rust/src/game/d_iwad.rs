//
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//     Find IWAD and initialize according to IWAD type.
//
// Original: d_iwad.h / d_iwad.c (stub implementation)

use super::d_mode::{GameMission, GameMode};

/// Try to find a WAD file by name. Returns the path if found, otherwise returns filename as-is.
/// Stub: just returns the filename (assumes it's a valid path).
/// Original: d_iwad.c D_TryFindWADByName
pub fn d_try_find_wad_by_name(filename: &str) -> String {
    // Stub: check if file exists, otherwise return filename
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
