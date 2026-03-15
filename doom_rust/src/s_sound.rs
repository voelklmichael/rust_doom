//! Rust translation of doomgeneric/s_sound.h
//! The not so system specific sound interface.

use crate::doomtype::*;
use crate::p_mobj::*;
use crate::sounds::*;

/// C function: S_Init
pub fn s_init(sfx_volume: i32, music_volume: i32) {
    todo!("original: S_Init")
}

/// C function: S_Shutdown
pub fn s_shutdown() {
    todo!("original: S_Shutdown")
}

/// C function: S_Start
pub fn s_start() {
    todo!("original: S_Start")
}

/// C function: S_StartSound
pub fn s_start_sound(origin: &mut [u8], sound_id: i32) {
    todo!("original: S_StartSound")
}

/// C function: S_StopSound
pub fn s_stop_sound(origin: &mut MobjT) {
    todo!("original: S_StopSound")
}

/// C function: S_StartMusic
pub fn s_start_music(music_id: i32) {
    todo!("original: S_StartMusic")
}

/// C function: S_ChangeMusic
pub fn s_change_music(music_id: i32, looping: i32) {
    todo!("original: S_ChangeMusic")
}

/// C function: S_MusicPlaying
pub fn s_music_playing() -> boolean {
    todo!("original: S_MusicPlaying")
}

/// C function: S_StopMusic
pub fn s_stop_music() {
    todo!("original: S_StopMusic")
}

/// C function: S_PauseSound
pub fn s_pause_sound() {
    todo!("original: S_PauseSound")
}

/// C function: S_ResumeSound
pub fn s_resume_sound() {
    todo!("original: S_ResumeSound")
}

/// C function: S_UpdateSounds
pub fn s_update_sounds(listener: &mut MobjT) {
    todo!("original: S_UpdateSounds")
}

/// C function: S_SetMusicVolume
pub fn s_set_music_volume(volume: i32) {
    todo!("original: S_SetMusicVolume")
}

/// C function: S_SetSfxVolume
pub fn s_set_sfx_volume(volume: i32) {
    todo!("original: S_SetSfxVolume")
}

pub static mut snd_channels: i32 = 0;
