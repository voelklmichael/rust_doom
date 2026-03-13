//! Sound and music subsystem.

mod i_sound;
mod memio;
mod mus2mid;
mod r_angle;
mod s_sound;
mod sounds;
mod tables;

pub use i_sound::{
    i_get_sfx_lump_num, i_init_music, i_init_sound, i_music_is_playing, i_pause_song,
    i_play_song, i_precache_sounds, i_register_song, i_resume_song, i_set_music_volume,
    i_shutdown_music, i_shutdown_sound, i_sound_is_playing, i_start_sound, i_stop_sound,
    i_stop_song, i_un_register_song, i_update_sound, i_update_sound_params, MusicInfo, SfxInfo,
    SndDevice, SoundModule, MusicModule,
};
pub use memio::{mem_fopen_read, mem_fopen_write, MemFileRead, MemFileWrite, MemSeek};
pub use mus2mid::mus2mid;
pub use r_angle::{r_point_to_angle2, Angle};
pub use s_sound::{
    s_change_music, s_init, s_music_playing, s_pause_sound, s_resume_sound, s_set_music_volume,
    s_set_sfx_volume, s_shutdown, s_start, s_start_music, s_start_sound, s_stop_music,
    s_stop_sound, s_update_sounds, SND_CHANNELS,
};
pub use sounds::{s_music, s_sfx, MusicEnum, SfxEnum, NUMMUSIC, NUMSFX};
pub use tables::{finesine, ANGLETOFINESHIFT, FINEANGLES};
