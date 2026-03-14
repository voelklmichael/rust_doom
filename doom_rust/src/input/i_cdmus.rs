//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 1993-2008 Raven Software
//
// DESCRIPTION:
//  CD music playback. Legacy; most ports use OPL/MIDI.
//
// Original: i_cdmus.h + i_cdmus.c

/// CD error codes.
pub const CDERR_NOTINSTALLED: i32 = 10;
pub const CDERR_NOAUDIOSUPPORT: i32 = 11;
pub const CDERR_NOAUDIOTRACKS: i32 = 12;
pub const CDERR_BADDRIVE: i32 = 20;
pub const CDERR_BADTRACK: i32 = 21;
pub const CDERR_IOCTLBUFFMEM: i32 = 22;
pub const CDERR_DEVREQBASE: i32 = 100;

/// Last CD error.
pub static mut CD_ERROR: i32 = 0;

/// Initialize CD music.
/// Original: I_CDMusInit
pub fn i_cdmus_init() -> i32 {
    unsafe {
        CD_ERROR = CDERR_NOTINSTALLED;
    }
    -1
}

/// Print CD startup message.
/// Original: I_CDMusPrintStartup
pub fn i_cdmus_print_startup() {
    // Stub
}

/// Play CD track.
/// Original: I_CDMusPlay
pub fn i_cdmus_play(_track: i32) -> i32 {
    -1
}

/// Stop CD playback.
/// Original: I_CDMusStop
pub fn i_cdmus_stop() -> i32 {
    -1
}

/// Resume CD playback.
/// Original: I_CDMusResume
pub fn i_cdmus_resume() -> i32 {
    -1
}

/// Set CD volume.
/// Original: I_CDMusSetVolume
pub fn i_cdmus_set_volume(_volume: i32) -> i32 {
    -1
}

/// First audio track number.
/// Original: I_CDMusFirstTrack
pub fn i_cdmus_first_track() -> i32 {
    -1
}

/// Last audio track number.
/// Original: I_CDMusLastTrack
pub fn i_cdmus_last_track() -> i32 {
    -1
}

/// Track length in seconds.
/// Original: I_CDMusTrackLength
pub fn i_cdmus_track_length(_track: i32) -> i32 {
    -1
}
