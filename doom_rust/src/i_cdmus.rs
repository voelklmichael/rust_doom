//! Rust translation of doomgeneric/i_cdmus.h
//! CD music interface.

pub const CDERR_NOTINSTALLED: i32 = 10;
pub const CDERR_NOAUDIOSUPPORT: i32 = 11;
pub const CDERR_NOAUDIOTRACKS: i32 = 12;
pub const CDERR_BADDRIVE: i32 = 20;
pub const CDERR_BADTRACK: i32 = 21;
pub const CDERR_IOCTLBUFFMEM: i32 = 22;
pub const CDERR_DEVREQBASE: i32 = 100;

pub static mut cd_error: i32 = 0;

/// C function: I_CDMusInit
pub fn i_cdmus_init() -> i32 {
    todo!("original: I_CDMusInit")
}

/// C function: I_CDMusPrintStartup
pub fn i_cdmus_print_startup() {
    todo!("original: I_CDMusPrintStartup")
}

/// C function: I_CDMusPlay
pub fn i_cdmus_play(track: i32) -> i32 {
    todo!("original: I_CDMusPlay")
}

/// C function: I_CDMusStop
pub fn i_cdmus_stop() -> i32 {
    todo!("original: I_CDMusStop")
}

/// C function: I_CDMusResume
pub fn i_cdmus_resume() -> i32 {
    todo!("original: I_CDMusResume")
}

/// C function: I_CDMusSetVolume
pub fn i_cdmus_set_volume(volume: i32) -> i32 {
    todo!("original: I_CDMusSetVolume")
}

/// C function: I_CDMusFirstTrack
pub fn i_cdmus_first_track() -> i32 {
    todo!("original: I_CDMusFirstTrack")
}

/// C function: I_CDMusLastTrack
pub fn i_cdmus_last_track() -> i32 {
    todo!("original: I_CDMusLastTrack")
}

/// C function: I_CDMusTrackLength
pub fn i_cdmus_track_length(track: i32) -> i32 {
    todo!("original: I_CDMusTrackLength")
}
