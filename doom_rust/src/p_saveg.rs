//! Rust translation of doomgeneric/p_saveg.h
//! Savegame I/O, archiving, persistence.

use crate::doomtype::*;

pub const SAVESTRINGSIZE: usize = 24;

pub fn p_temp_save_game_file() -> *mut i8 {
    todo!("original: P_TempSaveGameFile")
}

pub fn p_save_game_file(slot: i32) -> *mut i8 {
    todo!("original: P_SaveGameFile")
}

pub fn p_read_save_game_header() -> boolean {
    todo!("original: P_ReadSaveGameHeader")
}

pub fn p_write_save_game_header(description: *mut i8) {
    todo!("original: P_WriteSaveGameHeader")
}

pub fn p_read_save_game_eof() -> boolean {
    todo!("original: P_ReadSaveGameEOF")
}

pub fn p_write_save_game_eof() {
    todo!("original: P_WriteSaveGameEOF")
}

pub fn p_archive_players() {
    todo!("original: P_ArchivePlayers")
}

pub fn p_unarchive_players() {
    todo!("original: P_UnArchivePlayers")
}

pub fn p_archive_world() {
    todo!("original: P_ArchiveWorld")
}

pub fn p_unarchive_world() {
    todo!("original: P_UnArchiveWorld")
}

pub fn p_archive_thinkers() {
    todo!("original: P_ArchiveThinkers")
}

pub fn p_unarchive_thinkers() {
    todo!("original: P_UnArchiveThinkers")
}

pub fn p_archive_specials() {
    todo!("original: P_ArchiveSpecials")
}

pub fn p_unarchive_specials() {
    todo!("original: P_UnArchiveSpecials")
}

pub static mut save_stream: *mut std::ffi::c_void = std::ptr::null_mut(); // FILE*
pub static mut savegame_error: boolean = crate::doomtype::Boolean::False;
