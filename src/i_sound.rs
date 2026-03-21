// doomgeneric/i_sound.h

use std::cell::RefCell;
use std::ffi::c_char;

pub use crate::doomtype::*;

/// Original: typedef struct sfxinfo_struct sfxinfo_t
#[repr(C)]
pub struct SfxinfoT {
    pub tagname: *mut c_char,
    pub name: [c_char; 9],
    pub priority: i32,
    pub link: *mut SfxinfoT,
    pub pitch: i32,
    pub volume: i32,
    pub usefulness: i32,
    pub lumpnum: i32,
    pub numchannels: i32,
    pub driver_data: *mut std::ffi::c_void,
}

/// Original: typedef struct { ... } musicinfo_t
#[repr(C)]
pub struct MusicinfoT {
    pub name: *mut c_char,
    pub lumpnum: i32,
    pub data: *mut std::ffi::c_void,
    pub handle: *mut std::ffi::c_void,
}

/// Original: typedef enum { SNDDEVICE_NONE, ... } snddevice_t
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SnddeviceT {
    SnddeviceNone = 0,
    SnddevicePcspeaker = 1,
    SnddeviceAdlib = 2,
    SnddeviceSb = 3,
    SnddevicePas = 4,
    SnddeviceGus = 5,
    SnddeviceWaveblaster = 6,
    SnddeviceSoundcanvas = 7,
    SnddeviceGenmidi = 8,
    SnddeviceAwe32 = 9,
    SnddeviceCd = 10,
}

/// Original: typedef struct { ... } sound_module_t
#[repr(C)]
pub struct SoundModuleT {
    pub sound_devices: *mut SnddeviceT,
    pub num_sound_devices: i32,
    pub init: Option<unsafe extern "C" fn(Boolean) -> Boolean>,
    pub shutdown: Option<unsafe extern "C" fn()>,
    pub get_sfx_lump_num: Option<unsafe extern "C" fn(*mut SfxinfoT) -> i32>,
    pub update: Option<unsafe extern "C" fn()>,
    pub update_sound_params: Option<unsafe extern "C" fn(i32, i32, i32)>,
    pub start_sound: Option<unsafe extern "C" fn(*mut SfxinfoT, i32, i32, i32) -> i32>,
    pub stop_sound: Option<unsafe extern "C" fn(i32)>,
    pub sound_is_playing: Option<unsafe extern "C" fn(i32) -> Boolean>,
    pub cache_sounds: Option<unsafe extern "C" fn(*mut SfxinfoT, i32)>,
}

/// Original: typedef struct { ... } music_module_t
#[repr(C)]
pub struct MusicModuleT {
    pub sound_devices: *mut SnddeviceT,
    pub num_sound_devices: i32,
    pub init: Option<unsafe extern "C" fn() -> Boolean>,
    pub shutdown: Option<unsafe extern "C" fn()>,
    pub set_music_volume: Option<unsafe extern "C" fn(i32)>,
    pub pause_music: Option<unsafe extern "C" fn()>,
    pub resume_music: Option<unsafe extern "C" fn()>,
    pub register_song: Option<unsafe extern "C" fn(*mut std::ffi::c_void, i32) -> *mut std::ffi::c_void>,
    pub un_register_song: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
    pub play_song: Option<unsafe extern "C" fn(*mut std::ffi::c_void, Boolean)>,
    pub stop_song: Option<unsafe extern "C" fn()>,
    pub music_is_playing: Option<unsafe extern "C" fn() -> Boolean>,
    pub poll: Option<unsafe extern "C" fn()>,
}

#[allow(non_camel_case_types)]
pub struct I_SoundState {
    pub snd_sfxdevice: RefCell<i32>,
    pub snd_musicdevice: RefCell<i32>,
    pub snd_samplerate: RefCell<i32>,
    pub snd_cachesize: RefCell<i32>,
    pub snd_maxslicetime_ms: RefCell<i32>,
    pub snd_musiccmd: RefCell<*mut c_char>,
    pub opl_io_port: RefCell<i32>,
    pub timidity_cfg_path: RefCell<*mut c_char>,
    /// Original: sound_module_t / music_module_t externs — not populated until drivers load
    pub dg_sound_module: RefCell<Option<SoundModuleT>>,
    pub dg_music_module: RefCell<Option<MusicModuleT>>,
    pub sound_pcsound_module: RefCell<Option<SoundModuleT>>,
    pub music_opl_module: RefCell<Option<MusicModuleT>>,
}

impl I_SoundState {
    pub fn new() -> Self {
        Self {
            snd_sfxdevice: RefCell::new(0),
            snd_musicdevice: RefCell::new(0),
            snd_samplerate: RefCell::new(0),
            snd_cachesize: RefCell::new(0),
            snd_maxslicetime_ms: RefCell::new(0),
            snd_musiccmd: RefCell::new(std::ptr::null_mut()),
            opl_io_port: RefCell::new(0),
            timidity_cfg_path: RefCell::new(std::ptr::null_mut()),
            dg_sound_module: RefCell::new(None),
            dg_music_module: RefCell::new(None),
            sound_pcsound_module: RefCell::new(None),
            music_opl_module: RefCell::new(None),
        }
    }

    // Original: I_InitSound
    pub fn i_init_sound(&self, _use_sfx_prefix: Boolean) {
        todo!("I_InitSound");
    }

    // Original: I_ShutdownSound
    pub fn i_shutdown_sound(&self) {
        todo!("I_ShutdownSound");
    }

    // Original: I_GetSfxLumpNum
    pub fn i_get_sfx_lump_num(&self, _sfxinfo: *mut SfxinfoT) -> i32 {
        todo!("I_GetSfxLumpNum");
    }

    // Original: I_UpdateSound
    pub fn i_update_sound(&self) {
        todo!("I_UpdateSound");
    }

    // Original: I_UpdateSoundParams
    pub fn i_update_sound_params(&self, _channel: i32, _vol: i32, _sep: i32) {
        todo!("I_UpdateSoundParams");
    }

    // Original: I_StartSound
    pub fn i_start_sound(&self, _sfxinfo: *mut SfxinfoT, _channel: i32, _vol: i32, _sep: i32) -> i32 {
        todo!("I_StartSound");
    }

    // Original: I_StopSound
    pub fn i_stop_sound(&self, _channel: i32) {
        todo!("I_StopSound");
    }

    // Original: I_SoundIsPlaying
    pub fn i_sound_is_playing(&self, _channel: i32) -> Boolean {
        todo!("I_SoundIsPlaying");
    }

    // Original: I_PrecacheSounds
    pub fn i_precache_sounds(&self, _sounds: *mut SfxinfoT, _num_sounds: i32) {
        todo!("I_PrecacheSounds");
    }

    // Original: I_InitMusic
    pub fn i_init_music(&self) {
        todo!("I_InitMusic");
    }

    // Original: I_ShutdownMusic
    pub fn i_shutdown_music(&self) {
        todo!("I_ShutdownMusic");
    }

    // Original: I_SetMusicVolume
    pub fn i_set_music_volume(&self, _volume: i32) {
        todo!("I_SetMusicVolume");
    }

    // Original: I_PauseSong
    pub fn i_pause_song(&self) {
        todo!("I_PauseSong");
    }

    // Original: I_ResumeSong
    pub fn i_resume_song(&self) {
        todo!("I_ResumeSong");
    }

    // Original: I_RegisterSong
    pub fn i_register_song(&self, _data: *mut std::ffi::c_void, _len: i32) -> *mut std::ffi::c_void {
        todo!("I_RegisterSong");
    }

    // Original: I_UnRegisterSong
    pub fn i_un_register_song(&self, _handle: *mut std::ffi::c_void) {
        todo!("I_UnRegisterSong");
    }

    // Original: I_PlaySong
    pub fn i_play_song(&self, _handle: *mut std::ffi::c_void, _looping: Boolean) {
        todo!("I_PlaySong");
    }

    // Original: I_StopSong
    pub fn i_stop_song(&self) {
        todo!("I_StopSong");
    }

    // Original: I_MusicIsPlaying
    pub fn i_music_is_playing(&self) -> Boolean {
        todo!("I_MusicIsPlaying");
    }

    // Original: I_BindSoundVariables
    pub fn i_bind_sound_variables(&self) {
        todo!("I_BindSoundVariables");
    }

    // Original: I_InitTimidityConfig
    pub fn i_init_timidity_config(&self) {
        todo!("I_InitTimidityConfig");
    }
}
