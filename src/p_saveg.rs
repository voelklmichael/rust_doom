//! Savegame I/O (p_saveg.h, p_saveg.c)
//! Original: p_saveg.h, p_saveg.c

use crate::doomtype::Boolean;

pub const SAVESTRINGSIZE: usize = 24;

pub struct P_SavegState {
    pub savegame_error: std::sync::Arc<std::sync::Mutex<Boolean>>,
}

impl P_SavegState {
    /// Original: char *P_TempSaveGameFile(void)
    pub fn p_temp_save_game_file(&self) -> String {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *P_SaveGameFile(int slot)
    pub fn p_save_game_file(&self, _slot: i32) -> String {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_ReadSaveGameHeader(void)
    pub fn p_read_save_game_header(&self) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_WriteSaveGameHeader(char *description)
    pub fn p_write_save_game_header(&self, _description: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_ReadSaveGameEOF(void)
    pub fn p_read_save_game_eof(&self) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_WriteSaveGameEOF(void)
    pub fn p_write_save_game_eof(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_ArchivePlayers(void)
    pub fn p_archive_players(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_UnArchivePlayers(void)
    pub fn p_unarchive_players(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_ArchiveWorld(void)
    pub fn p_archive_world(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_UnArchiveWorld(void)
    pub fn p_unarchive_world(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_ArchiveThinkers(void)
    pub fn p_archive_thinkers(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_UnArchiveThinkers(void)
    pub fn p_unarchive_thinkers(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_ArchiveSpecials(void)
    pub fn p_archive_specials(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_UnArchiveSpecials(void)
    pub fn p_unarchive_specials(&self) {
        todo!("Basic stage-0 stub")
    }
}
