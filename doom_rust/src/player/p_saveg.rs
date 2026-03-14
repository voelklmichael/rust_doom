//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Savegame I/O, archiving, persistence.
//
// Original: p_saveg.h / p_saveg.c (stub)

/// Maximum size of a savegame description string.
pub const SAVESTRINGSIZE: usize = 24;

/// Temporary filename while saving. Original: P_TempSaveGameFile
pub fn p_temp_save_game_file() -> String {
    crate::game::dstrings::SAVEGAMENAME.to_string()
}

/// Filename for save slot. Original: P_SaveGameFile
pub fn p_save_game_file(slot: i32) -> String {
    format!("{}{}.dsg", crate::game::dstrings::SAVEGAMENAME, slot)
}

/// Read savegame header. Original: P_ReadSaveGameHeader
pub fn p_read_save_game_header() -> bool {
    false
}

/// Write savegame header. Original: P_WriteSaveGameHeader
pub fn p_write_save_game_header(_description: &str) {
    // TODO: require FILE/save stream
}

/// Read savegame EOF marker. Original: P_ReadSaveGameEOF
pub fn p_read_save_game_eof() -> bool {
    false
}

/// Write savegame EOF marker. Original: P_WriteSaveGameEOF
pub fn p_write_save_game_eof() {
    // TODO
}

/// Archive players to save stream. Original: P_ArchivePlayers
pub fn p_archive_players() {
    // TODO: serialize player_t
}

/// Unarchive players from save stream. Original: P_UnArchivePlayers
pub fn p_unarchive_players() {
    // TODO: deserialize player_t
}

/// Archive world (mobjs, sectors). Original: P_ArchiveWorld
pub fn p_archive_world() {
    // TODO: serialize thinkers, sectors
}

/// Unarchive world. Original: P_UnArchiveWorld
pub fn p_unarchive_world() {
    // TODO: deserialize
}

// TODO: P_ArchiveThinkers, P_UnArchiveThinkers, P_ArchiveSpecials, P_UnArchiveSpecials
