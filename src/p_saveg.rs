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

    pub fn p_temp_save_game_file(&self) -> *mut c_char {
        todo!("P_TempSaveGameFile");
    }

    pub fn p_save_game_file(&self, _slot: i32) -> *mut c_char {
        todo!("P_SaveGameFile");
    }

    pub fn p_read_save_game_header(&self) -> Boolean {
        todo!("P_ReadSaveGameHeader");
    }

    pub fn p_write_save_game_header(&self, _description: *mut c_char) {
        todo!("P_WriteSaveGameHeader");
    }

    pub fn p_read_save_game_eof(&self) -> Boolean {
        todo!("P_ReadSaveGameEOF");
    }

    pub fn p_write_save_game_eof(&self) {
        todo!("P_WriteSaveGameEOF");
    }

    pub fn p_archive_players(&self) {
        todo!("P_ArchivePlayers");
    }

    pub fn p_un_archive_players(&self) {
        todo!("P_UnArchivePlayers");
    }

    pub fn p_archive_world(&self) {
        todo!("P_ArchiveWorld");
    }

    pub fn p_un_archive_world(&self) {
        todo!("P_UnArchiveWorld");
    }

    pub fn p_archive_thinkers(&self) {
        todo!("P_ArchiveThinkers");
    }

    pub fn p_un_archive_thinkers(&self) {
        todo!("P_UnArchiveThinkers");
    }

    pub fn p_archive_specials(&self) {
        todo!("P_ArchiveSpecials");
    }

    pub fn p_un_archive_specials(&self) {
        todo!("P_UnArchiveSpecials");
    }
}
