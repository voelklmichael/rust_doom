//! Rust translation of doomgeneric/i_sound.h
//! The not so system specific sound interface.

use crate::doomtype::*;

/// C typedef: sfxinfo_t
#[repr(C)]
/// C typedef: sfxinfo_t
pub struct SfxinfoT {
    pub tagname: *mut i8,
    pub name: [i8; 9],
    pub priority: i32,
    pub link: *mut SfxinfoT,
    pub pitch: i32,
    pub volume: i32,
    pub usefulness: i32,
    pub lumpnum: i32,
    pub numchannels: i32,
    pub driver_data: *mut core::ffi::c_void,
}

/// C typedef: musicinfo_t
#[repr(C)]
/// C typedef: musicinfo_t
pub struct MusicinfoT {
    pub name: *mut i8,
    pub lumpnum: i32,
    pub data: *mut core::ffi::c_void,
    pub handle: *mut core::ffi::c_void,
}

/// C typedef: snddevice_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: snddevice_t
pub enum SnddeviceT {
    None = 0,
    Pcspeaker = 1,
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

/// C typedef: sound_module_t
#[repr(C)]
/// C typedef: sound_module_t
pub struct SoundModuleT {
    pub sound_devices: *mut SnddeviceT,
    pub num_sound_devices: i32,
    pub init: Option<extern "C" fn(bool) -> Boolean>,
    pub shutdown: Option<extern "C" fn()>,
    pub get_sfx_lump_num: Option<extern "C" fn(*mut SfxinfoT) -> i32>,
    pub update: Option<extern "C" fn()>,
    pub update_sound_params: Option<extern "C" fn(i32, i32, i32)>,
    pub start_sound: Option<extern "C" fn(*mut SfxinfoT, i32, i32, i32) -> i32>,
    pub stop_sound: Option<extern "C" fn(i32)>,
    pub sound_is_playing: Option<extern "C" fn(i32) -> Boolean>,
    pub cache_sounds: Option<extern "C" fn(*mut SfxinfoT, i32)>,
}

impl SoundModuleT {
    pub const fn default() -> Self {
        Self {
            sound_devices: core::ptr::null_mut(),
            num_sound_devices: 0,
            init: None,
            shutdown: None,
            get_sfx_lump_num: None,
            update: None,
            update_sound_params: None,
            start_sound: None,
            stop_sound: None,
            sound_is_playing: None,
            cache_sounds: None,
        }
    }
}

/// C typedef: music_module_t
#[repr(C)]
/// C typedef: music_module_t
pub struct MusicModuleT {
    pub sound_devices: *mut SnddeviceT,
    pub num_sound_devices: i32,
    pub init: Option<extern "C" fn() -> Boolean>,
    pub shutdown: Option<extern "C" fn()>,
    pub set_music_volume: Option<extern "C" fn(i32)>,
    pub pause_music: Option<extern "C" fn()>,
    pub resume_music: Option<extern "C" fn()>,
    pub register_song: Option<extern "C" fn(*mut core::ffi::c_void, i32) -> *mut core::ffi::c_void>,
    pub un_register_song: Option<extern "C" fn(*mut core::ffi::c_void)>,
    pub play_song: Option<extern "C" fn(*mut core::ffi::c_void, Boolean)>,
    pub stop_song: Option<extern "C" fn()>,
    pub music_is_playing: Option<extern "C" fn() -> Boolean>,
    pub poll: Option<extern "C" fn()>,
}

impl MusicModuleT {
    pub const fn default() -> Self {
        Self {
            sound_devices: core::ptr::null_mut(),
            num_sound_devices: 0,
            init: None,
            shutdown: None,
            set_music_volume: None,
            pause_music: None,
            resume_music: None,
            register_song: None,
            un_register_song: None,
            play_song: None,
            stop_song: None,
            music_is_playing: None,
            poll: None,
        }
    }
}

/// C function: I_InitSound
pub fn i_init_sound(use_sfx_prefix: Boolean) {
    todo!("original: I_InitSound")
}

/// C function: I_ShutdownSound
pub fn i_shutdown_sound() {
    todo!("original: I_ShutdownSound")
}

/// C function: I_GetSfxLumpNum
pub fn i_get_sfx_lump_num(sfxinfo: *mut SfxinfoT) -> i32 {
    todo!("original: I_GetSfxLumpNum")
}

/// C function: I_UpdateSound
pub fn i_update_sound() {
    todo!("original: I_UpdateSound")
}

/// C function: I_UpdateSoundParams
pub fn i_update_sound_params(channel: i32, vol: i32, sep: i32) {
    todo!("original: I_UpdateSoundParams")
}

/// C function: I_StartSound
pub fn i_start_sound(sfxinfo: *mut SfxinfoT, channel: i32, vol: i32, sep: i32) -> i32 {
    todo!("original: I_StartSound")
}

/// C function: I_StopSound
pub fn i_stop_sound(channel: i32) {
    todo!("original: I_StopSound")
}

/// C function: I_SoundIsPlaying
pub fn i_sound_is_playing(channel: i32) -> Boolean {
    todo!("original: I_SoundIsPlaying")
}

/// C function: I_PrecacheSounds
pub fn i_precache_sounds(sounds: *mut SfxinfoT, num_sounds: i32) {
    todo!("original: I_PrecacheSounds")
}

/// C function: I_InitMusic
pub fn i_init_music() {
    todo!("original: I_InitMusic")
}

/// C function: I_ShutdownMusic
pub fn i_shutdown_music() {
    todo!("original: I_ShutdownMusic")
}

/// C function: I_SetMusicVolume
pub fn i_set_music_volume(volume: i32) {
    todo!("original: I_SetMusicVolume")
}

/// C function: I_PauseSong
pub fn i_pause_song() {
    todo!("original: I_PauseSong")
}

/// C function: I_ResumeSong
pub fn i_resume_song() {
    todo!("original: I_ResumeSong")
}

/// C function: I_RegisterSong
pub fn i_register_song(data: *mut core::ffi::c_void, len: i32) -> *mut core::ffi::c_void {
    todo!("original: I_RegisterSong")
}

/// C function: I_UnRegisterSong
pub fn i_un_register_song(handle: *mut core::ffi::c_void) {
    todo!("original: I_UnRegisterSong")
}

/// C function: I_PlaySong
pub fn i_play_song(handle: *mut core::ffi::c_void, looping: Boolean) {
    todo!("original: I_PlaySong")
}

/// C function: I_StopSong
pub fn i_stop_song() {
    todo!("original: I_StopSong")
}

/// C function: I_MusicIsPlaying
pub fn i_music_is_playing() -> Boolean {
    todo!("original: I_MusicIsPlaying")
}

/// C function: I_BindSoundVariables
pub fn i_bind_sound_variables() {
    todo!("original: I_BindSoundVariables")
}

pub static mut snd_sfxdevice: i32 = 0;
pub static mut snd_musicdevice: i32 = 0;
pub static mut snd_samplerate: i32 = 0;
pub static mut snd_cachesize: i32 = 0;
pub static mut snd_maxslicetime_ms: i32 = 0;
pub static mut snd_musiccmd: *mut i8 = core::ptr::null_mut();

/// C function: I_InitTimidityConfig
pub fn i_init_timidity_config() {
    todo!("original: I_InitTimidityConfig")
}

pub static mut sound_pcsound_module: SoundModuleT = SoundModuleT::default();
pub static mut music_opl_module: MusicModuleT = MusicModuleT::default();
pub static mut opl_io_port: i32 = 0;
pub static mut timidity_cfg_path: *mut i8 = core::ptr::null_mut();
