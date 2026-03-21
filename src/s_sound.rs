// s_sound.h - not-so-system-specific sound interface

pub use crate::doomtype::*;
pub use crate::p_mobj::*;
pub use crate::sounds::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct S_SoundState {
    // Original: snd_channels
    pub snd_channels: RefCell<i32>,
}

impl S_SoundState {
    pub fn new() -> Self {
        Self { snd_channels: RefCell::new(0) }
    }

    // Original: S_Init
    pub fn s_init(&self, _sfx_volume: i32, _music_volume: i32) {
        todo!("S_Init")
    }

    // Original: S_Shutdown
    pub fn s_shutdown(&self) {
        todo!("S_Shutdown")
    }

    // Original: S_Start
    pub fn s_start(&self) {
        todo!("S_Start")
    }

    // Original: S_StartSound
    pub fn s_start_sound(&self, _origin: *mut std::ffi::c_void, _sound_id: i32) {
        todo!("S_StartSound")
    }

    // Original: S_StopSound
    pub fn s_stop_sound(&self, _origin: *mut MobjT) {
        todo!("S_StopSound")
    }

    // Original: S_StartMusic
    pub fn s_start_music(&self, _music_id: i32) {
        todo!("S_StartMusic")
    }

    // Original: S_ChangeMusic
    pub fn s_change_music(&self, _music_id: i32, _looping: i32) {
        todo!("S_ChangeMusic")
    }

    // Original: S_MusicPlaying
    pub fn s_music_playing(&self) -> Boolean {
        todo!("S_MusicPlaying")
    }

    // Original: S_StopMusic
    pub fn s_stop_music(&self) {
        todo!("S_StopMusic")
    }

    // Original: S_PauseSound
    pub fn s_pause_sound(&self) {
        todo!("S_PauseSound")
    }

    // Original: S_ResumeSound
    pub fn s_resume_sound(&self) {
        todo!("S_ResumeSound")
    }

    // Original: S_UpdateSounds
    pub fn s_update_sounds(&self, _listener: *mut MobjT) {
        todo!("S_UpdateSounds")
    }

    // Original: S_SetMusicVolume
    pub fn s_set_music_volume(&self, _volume: i32) {
        todo!("S_SetMusicVolume")
    }

    // Original: S_SetSfxVolume
    pub fn s_set_sfx_volume(&self, _volume: i32) {
        todo!("S_SetSfxVolume")
    }
}
