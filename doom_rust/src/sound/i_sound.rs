//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  The not so system specific sound interface.
//
// Original: i_sound.h

use crate::doomtype::Boolean;

/// SoundFX struct.
#[derive(Debug, Clone)]
pub struct SfxInfo {
    pub tagname: Option<String>,
    pub name: [u8; 9],
    pub priority: i32,
    pub link: Option<usize>, // index into S_sfx
    pub pitch: i32,
    pub volume: i32,
    pub usefulness: i32,
    pub lumpnum: i32,
    pub numchannels: i32,
}

/// MusicInfo struct.
#[derive(Debug, Clone)]
pub struct MusicInfo {
    pub name: Option<String>,
    pub lumpnum: i32,
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SndDevice {
    None = 0,
    PcSpeaker = 1,
    Adlib = 2,
    Sb = 3,
    Pas = 4,
    Gus = 5,
    Waveblaster = 6,
    Soundcanvas = 7,
    Genmidi = 8,
    Awe32 = 9,
    Cd = 10,
}

/// Interface for sound modules.
pub trait SoundModule {
    fn init(&mut self, use_sfx_prefix: bool) -> bool;
    fn shutdown(&mut self);
    fn get_sfx_lump_num(&self, sfxinfo: &SfxInfo) -> i32;
    fn update(&mut self);
    fn update_sound_params(&mut self, channel: i32, vol: i32, sep: i32);
    fn start_sound(&mut self, sfxinfo: &SfxInfo, channel: i32, vol: i32, sep: i32) -> i32;
    fn stop_sound(&mut self, channel: i32);
    fn sound_is_playing(&self, channel: i32) -> bool;
    fn cache_sounds(&mut self, sounds: &[SfxInfo]);
}

/// Interface for music modules.
pub trait MusicModule {
    fn init(&mut self) -> bool;
    fn shutdown(&mut self);
    fn set_music_volume(&mut self, volume: i32);
    fn pause_music(&mut self);
    fn resume_music(&mut self);
    fn register_song(&mut self, data: &[u8]) -> Option<Vec<u8>>;
    fn un_register_song(&mut self, handle: &mut Option<Vec<u8>>);
    fn play_song(&mut self, handle: &[u8], looping: bool);
    fn stop_song(&mut self);
    fn music_is_playing(&self) -> bool;
    fn poll(&mut self);
}

// Stub implementations - no audio backend yet.
pub fn i_init_sound(_use_sfx_prefix: bool) {}
pub fn i_shutdown_sound() {}
pub fn i_get_sfx_lump_num(sfxinfo: &SfxInfo) -> i32 {
    sfxinfo.lumpnum
}
pub fn i_update_sound() {}
pub fn i_update_sound_params(_channel: i32, _vol: i32, _sep: i32) {}
pub fn i_start_sound(_sfxinfo: &SfxInfo, _channel: i32, _vol: i32, _sep: i32) -> i32 {
    -1
}
pub fn i_stop_sound(_channel: i32) {}
pub fn i_sound_is_playing(_channel: i32) -> Boolean {
    false
}
pub fn i_precache_sounds(_sounds: &[SfxInfo]) {}

pub fn i_init_music() {}
pub fn i_shutdown_music() {}
pub fn i_set_music_volume(_volume: i32) {}
pub fn i_pause_song() {}
pub fn i_resume_song() {}
pub fn i_register_song(_data: &[u8]) -> Option<Vec<u8>> {
    None
}
pub fn i_un_register_song(_handle: &mut Option<Vec<u8>>) {}
pub fn i_play_song(_handle: &[u8], _looping: bool) {}
pub fn i_stop_song() {}
pub fn i_music_is_playing() -> Boolean {
    false
}
