//! Rust translation of doomgeneric/p_saveg.h
//! Savegame I/O, archiving, persistence.

use crate::doomtype::*;

/// C #define: SAVESTRINGSIZE
pub const SAVESTRINGSIZE: usize = 24;

/// C function: P_TempSaveGameFile
pub fn p_temp_save_game_file() -> String {
    todo!("original: P_TempSaveGameFile")
}

/// C function: P_SaveGameFile
pub fn p_save_game_file(slot: i32) -> String {
    todo!("original: P_SaveGameFile")
}

/// C function: P_ReadSaveGameHeader
pub fn p_read_save_game_header() -> boolean {
    todo!("original: P_ReadSaveGameHeader")
}

/// C function: P_WriteSaveGameHeader
pub fn p_write_save_game_header(description: &mut [u8]) {
    todo!("original: P_WriteSaveGameHeader")
}

/// C function: P_ReadSaveGameEOF
pub fn p_read_save_game_eof() -> boolean {
    todo!("original: P_ReadSaveGameEOF")
}

/// C function: P_WriteSaveGameEOF
pub fn p_write_save_game_eof() {
    todo!("original: P_WriteSaveGameEOF")
}

/// C function: P_ArchivePlayers
pub fn p_archive_players() {
    todo!("original: P_ArchivePlayers")
}

/// C function: P_UnArchivePlayers
pub fn p_unarchive_players() {
    todo!("original: P_UnArchivePlayers")
}

/// C function: P_ArchiveWorld
pub fn p_archive_world() {
    todo!("original: P_ArchiveWorld")
}

/// C function: P_UnArchiveWorld
pub fn p_unarchive_world() {
    todo!("original: P_UnArchiveWorld")
}

/// C function: P_ArchiveThinkers
pub fn p_archive_thinkers() {
    todo!("original: P_ArchiveThinkers")
}

/// C function: P_UnArchiveThinkers
pub fn p_unarchive_thinkers() {
    todo!("original: P_UnArchiveThinkers")
}

/// C function: P_ArchiveSpecials
pub fn p_archive_specials() {
    todo!("original: P_ArchiveSpecials")
}

/// C function: P_UnArchiveSpecials
pub fn p_unarchive_specials() {
    todo!("original: P_UnArchiveSpecials")
}

pub static mut save_stream: *mut std::ffi::c_void = std::ptr::null_mut(); // FILE*
pub static mut savegame_error: boolean = crate::doomtype::Boolean::False;
