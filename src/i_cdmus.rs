// doomgeneric/i_cdmus.h

pub use crate::doomtype::*;

use std::cell::RefCell;

pub const CDERR_NOTINSTALLED: i32 = 10;
pub const CDERR_NOAUDIOSUPPORT: i32 = 11;
pub const CDERR_NOAUDIOTRACKS: i32 = 12;
pub const CDERR_BADDRIVE: i32 = 20;
pub const CDERR_BADTRACK: i32 = 21;
pub const CDERR_IOCTLBUFFMEM: i32 = 22;
pub const CDERR_DEVREQBASE: i32 = 100;

#[allow(non_camel_case_types)]
pub struct I_CdmusState {
    /// Original: int cd_Error
    pub cd_error: RefCell<i32>,
}

impl I_CdmusState {
    pub fn new() -> Self {
        Self {
            cd_error: RefCell::new(0),
        }
    }

    // Original: I_CDMusInit
    pub fn i_cd_mus_init(&self) -> i32 {
        todo!("I_CDMusInit");
    }

    // Original: I_CDMusPrintStartup
    pub fn i_cd_mus_print_startup(&self) {
        todo!("I_CDMusPrintStartup");
    }

    // Original: I_CDMusPlay
    pub fn i_cd_mus_play(&self, _track: i32) -> i32 {
        todo!("I_CDMusPlay");
    }

    // Original: I_CDMusStop
    pub fn i_cd_mus_stop(&self) -> i32 {
        todo!("I_CDMusStop");
    }

    // Original: I_CDMusResume
    pub fn i_cd_mus_resume(&self) -> i32 {
        todo!("I_CDMusResume");
    }

    // Original: I_CDMusSetVolume
    pub fn i_cd_mus_set_volume(&self, _volume: i32) -> i32 {
        todo!("I_CDMusSetVolume");
    }

    // Original: I_CDMusFirstTrack
    pub fn i_cd_mus_first_track(&self) -> i32 {
        todo!("I_CDMusFirstTrack");
    }

    // Original: I_CDMusLastTrack
    pub fn i_cd_mus_last_track(&self) -> i32 {
        todo!("I_CDMusLastTrack");
    }

    // Original: I_CDMusTrackLength
    pub fn i_cd_mus_track_length(&self, _track: i32) -> i32 {
        todo!("I_CDMusTrackLength");
    }
}
