// doomgeneric/p_saveg.h

pub use crate::doomtype::*;

use std::cell::RefCell;
use std::ffi::{c_char, c_void};

// Original: #define SAVESTRINGSIZE 24
pub const SAVESTRINGSIZE: usize = 24;

#[allow(non_camel_case_types)]
pub struct P_SavegState {
    /// Original: FILE *save_stream
    pub save_stream: RefCell<*mut c_void>,
    /// Original: boolean savegame_error
    pub savegame_error: RefCell<Boolean>,
}

impl P_SavegState {
    pub fn new() -> Self {
        Self {
            save_stream: RefCell::new(std::ptr::null_mut()),
            savegame_error: RefCell::new(Boolean::False),
        }
    }

    // Original: P_TempSaveGameFile
    pub fn p_temp_save_game_file(&self) -> *mut c_char {
        todo!("P_TempSaveGameFile");
    }

    // Original: P_SaveGameFile
    pub fn p_save_game_file(&self, _slot: i32) -> *mut c_char {
        todo!("P_SaveGameFile");
    }

    // Original: P_ReadSaveGameHeader
    pub fn p_read_save_game_header(&self) -> Boolean {
        todo!("P_ReadSaveGameHeader");
    }

    // Original: P_WriteSaveGameHeader
    pub fn p_write_save_game_header(&self, _description: *mut c_char) {
        todo!("P_WriteSaveGameHeader");
    }

    // Original: P_ReadSaveGameEOF
    pub fn p_read_save_game_eof(&self) -> Boolean {
        todo!("P_ReadSaveGameEOF");
    }

    // Original: P_WriteSaveGameEOF
    pub fn p_write_save_game_eof(&self) {
        todo!("P_WriteSaveGameEOF");
    }

    // Original: P_ArchivePlayers
    pub fn p_archive_players(&self) {
        todo!("P_ArchivePlayers");
    }

    // Original: P_UnArchivePlayers
    pub fn p_un_archive_players(&self) {
        todo!("P_UnArchivePlayers");
    }

    // Original: P_ArchiveWorld
    pub fn p_archive_world(&self) {
        todo!("P_ArchiveWorld");
    }

    // Original: P_UnArchiveWorld
    pub fn p_un_archive_world(&self) {
        todo!("P_UnArchiveWorld");
    }

    // Original: P_ArchiveThinkers
    pub fn p_archive_thinkers(&self) {
        todo!("P_ArchiveThinkers");
    }

    // Original: P_UnArchiveThinkers
    pub fn p_un_archive_thinkers(&self) {
        todo!("P_UnArchiveThinkers");
    }

    // Original: P_ArchiveSpecials
    pub fn p_archive_specials(&self) {
        todo!("P_ArchiveSpecials");
    }

    // Original: P_UnArchiveSpecials
    pub fn p_un_archive_specials(&self) {
        todo!("P_UnArchiveSpecials");
    }
}
