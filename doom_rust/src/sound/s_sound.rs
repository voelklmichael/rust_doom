//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Sound effect and music playback logic.
//
// Original: s_sound.h / s_sound.c

use std::sync::{Mutex, OnceLock};

use crate::doomfeatures::FEATURE_SOUND;
use crate::d_mode::GameMode;
use crate::doomstat;
use crate::m_fixed::{fixed_mul, Fixed, FRACBITS, FRACUNIT};
use crate::p_mobj::Mobj;

use super::i_sound::{self, SfxInfo};
use super::r_angle::{r_point_to_angle2, Angle};
use super::sounds::{self, MusicEnum, NUMSFX};
use crate::geometry::{finesine, ANGLETOFINESHIFT};

const S_CLIPPING_DIST: Fixed = 1200 * FRACUNIT;
const S_CLOSE_DIST: Fixed = 200 * FRACUNIT;
const S_ATTENUATOR: Fixed = (S_CLIPPING_DIST - S_CLOSE_DIST) >> FRACBITS;
const S_STEREO_SWING: Fixed = 96 * FRACUNIT;
const NORM_PITCH: i32 = 128;
const NORM_PRIORITY: i32 = 64;
const NORM_SEP: i32 = 128;

struct Channel {
    sfxinfo: Option<usize>,
    origin: Option<usize>, // raw pointer as usize for Send
    handle: i32,
}

static CHANNELS: OnceLock<Mutex<Vec<Channel>>> = OnceLock::new();
static SND_SFX_VOLUME: OnceLock<Mutex<i32>> = OnceLock::new();
static MUS_PAUSED: OnceLock<Mutex<bool>> = OnceLock::new();
static MUS_PLAYING: OnceLock<Mutex<Option<usize>>> = OnceLock::new();

pub const SND_CHANNELS: i32 = 8;

fn channels() -> &'static Mutex<Vec<Channel>> {
    CHANNELS.get_or_init(|| {
        Mutex::new(
            (0..SND_CHANNELS as usize)
                .map(|_| Channel {
                    sfxinfo: None,
                    origin: None, // usize for *const Mobj (Send)
                    handle: -1,
                })
                .collect(),
        )
    })
}

fn snd_sfx_volume() -> i32 {
    *SND_SFX_VOLUME
        .get_or_init(|| Mutex::new(8))
        .lock()
        .unwrap()
}

fn set_snd_sfx_volume(v: i32) {
    *SND_SFX_VOLUME
        .get_or_init(|| Mutex::new(8))
        .lock()
        .unwrap() = v;
}

/// Initialize sound.
pub fn s_init(sfx_volume: i32, music_volume: i32) {
    if !FEATURE_SOUND {
        return;
    }
    i_sound::i_precache_sounds(&sounds::s_sfx().lock().unwrap());
    s_set_sfx_volume(sfx_volume);
    s_set_music_volume(music_volume);

    let mut ch = channels().lock().unwrap();
    for c in ch.iter_mut() {
        c.sfxinfo = None;
        c.origin = None;
        c.handle = -1;
    }

    *MUS_PAUSED.get_or_init(|| Mutex::new(false)).lock().unwrap() = false;

    let mut sfx = sounds::s_sfx().lock().unwrap();
    for i in 1..NUMSFX {
        sfx[i].lumpnum = -1;
        sfx[i].usefulness = -1;
    }
}

/// Shutdown sound.
pub fn s_shutdown() {
    i_sound::i_shutdown_sound();
    i_sound::i_shutdown_music();
}

fn s_stop_channel(cnum: usize) {
    let (sfx_idx, handle) = {
        let mut ch = channels().lock().unwrap();
        if cnum >= ch.len() {
            return;
        }
        let c = &mut ch[cnum];
        let sfx_idx = c.sfxinfo;
        let handle = c.handle;
        c.sfxinfo = None;
        c.origin = None;
        c.handle = -1;
        (sfx_idx, handle)
    };
    if let Some(idx) = sfx_idx {
        if i_sound::i_sound_is_playing(handle) {
            i_sound::i_stop_sound(handle);
        }
        let mut sfx = sounds::s_sfx().lock().unwrap();
        if idx < sfx.len() {
            sfx[idx].usefulness -= 1;
        }
    }
}

/// Per-level startup. Kill sounds, start level music.
pub fn s_start() {
    if !FEATURE_SOUND {
        return;
    }
    let to_stop: Vec<usize> = {
        let ch = channels().lock().unwrap();
        ch.iter()
            .enumerate()
            .filter(|(_, c)| c.sfxinfo.is_some())
            .map(|(i, _)| i)
            .collect()
    };
    for cnum in to_stop {
        s_stop_channel(cnum);
    }

    *MUS_PAUSED.get_or_init(|| Mutex::new(false)).lock().unwrap() = false;

    let mnum = level_music();
    s_change_music(mnum, true);
}

fn level_music() -> usize {
    let (gamemode, gameepisode, gamemap) = unsafe {
        (doomstat::GAMEMODE, doomstat::GAMEEPISODE, doomstat::GAMEMAP)
    };

    if gamemode == GameMode::Commercial {
        return (MusicEnum::Runnin as usize) + gamemap as usize - 1;
    }

    const SPMUS: [MusicEnum; 9] = [
        MusicEnum::E3m4,
        MusicEnum::E3m2,
        MusicEnum::E3m3,
        MusicEnum::E1m5,
        MusicEnum::E2m7,
        MusicEnum::E2m4,
        MusicEnum::E2m6,
        MusicEnum::E2m5,
        MusicEnum::E1m9,
    ];

    if gameepisode < 4 {
        (MusicEnum::E1m1 as usize) + (gameepisode as usize - 1) * 9 + gamemap as usize - 1
    } else {
        SPMUS[gamemap as usize - 1] as usize
    }
}

/// Stop sound for thing at origin.
pub fn s_stop_sound(origin: Option<*const Mobj>) {
    let origin_id = origin.map(|p| p as usize);
    let cnum = {
        let ch = channels().lock().unwrap();
        ch.iter()
            .position(|c| c.sfxinfo.is_some() && c.origin == origin_id)
    };
    if let Some(idx) = cnum {
        s_stop_channel(idx);
    }
}

/// Adjust volume and stereo separation based on listener/source positions.
fn s_adjust_sound_params(
    listener: &Mobj,
    source: &Mobj,
    vol: &mut i32,
    sep: &mut i32,
) -> bool {
    let adx = (listener.x - source.x).abs() as u32;
    let ady = (listener.y - source.y).abs() as u32;
    let approx_dist = (adx + ady).saturating_sub(adx.min(ady) >> 1) as Fixed;

    let gamemap = unsafe { doomstat::GAMEMAP };
    if gamemap != 8 && approx_dist > S_CLIPPING_DIST {
        return false;
    }

    let angle: Angle = r_point_to_angle2(listener.x, listener.y, source.x, source.y);
    let mut rel_angle = if angle > listener.angle {
        angle.wrapping_sub(listener.angle)
    } else {
        angle.wrapping_add(0xFFFF_FFFF - listener.angle)
    };
    rel_angle >>= ANGLETOFINESHIFT;
    let fine_idx = (rel_angle as usize) % 8192;

    *sep = 128 - (fixed_mul(S_STEREO_SWING, finesine(fine_idx)) >> FRACBITS);

    if approx_dist < S_CLOSE_DIST {
        *vol = snd_sfx_volume();
    } else if gamemap == 8 {
        let dist = approx_dist.min(S_CLIPPING_DIST);
        *vol = 15
            + ((snd_sfx_volume() - 15) * ((S_CLIPPING_DIST - dist) >> FRACBITS))
                / S_ATTENUATOR;
    } else {
        *vol = (snd_sfx_volume() * ((S_CLIPPING_DIST - approx_dist) >> FRACBITS)) / S_ATTENUATOR;
    }

    *vol > 0
}

/// Start sound for thing at origin. Optional listener for stereo/volume.
pub fn s_start_sound(origin: Option<*const Mobj>, sound_id: i32, listener: Option<*const Mobj>) {
    if !FEATURE_SOUND {
        return;
    }
    if sound_id < 1 || sound_id >= NUMSFX as i32 {
        return;
    }

    let sfx_guard = sounds::s_sfx().lock().unwrap();
    let sfx_idx = sound_id as usize;
    let sfx = sfx_guard.get(sfx_idx).cloned().unwrap();
    drop(sfx_guard);

    let mut volume = snd_sfx_volume();
    if let Some(_link_idx) = sfx.link {
        volume += sfx.volume;
        if volume < 1 {
            return;
        }
        if volume > snd_sfx_volume() {
            volume = snd_sfx_volume();
        }
    }

    let (volume, sep) = if let (Some(orig), Some(list)) = (origin, listener) {
        let orig = unsafe { &*orig };
        let list = unsafe { &*list };
        if list.x == orig.x && list.y == orig.y {
            (volume, NORM_SEP)
        } else {
            let mut vol = volume;
            let mut sep_val = NORM_SEP;
            if !s_adjust_sound_params(list, orig, &mut vol, &mut sep_val) {
                return;
            }
            (vol, sep_val)
        }
    } else {
        (volume, NORM_SEP)
    };

    s_stop_sound(origin);

    let origin_id = origin.map(|p| p as usize);
    let cnum = s_get_channel(origin_id, &sfx, sfx_idx);
    if cnum < 0 {
        return;
    }

    let mut sfx_guard = sounds::s_sfx().lock().unwrap();
    if sfx_guard[sfx_idx].usefulness < 0 {
        sfx_guard[sfx_idx].usefulness = 1;
    } else {
        sfx_guard[sfx_idx].usefulness += 1;
    }
    if sfx_guard[sfx_idx].lumpnum < 0 {
        sfx_guard[sfx_idx].lumpnum = i_sound::i_get_sfx_lump_num(&sfx_guard[sfx_idx]);
    }
    let handle = i_sound::i_start_sound(&sfx_guard[sfx_idx], cnum, volume, sep);
    drop(sfx_guard);

    let mut ch = channels().lock().unwrap();
    if (cnum as usize) < ch.len() {
        ch[cnum as usize].sfxinfo = Some(sfx_idx);
        ch[cnum as usize].origin = origin_id;
        ch[cnum as usize].handle = handle;
    }
}

fn s_get_channel(origin: Option<usize>, sfxinfo: &SfxInfo, sfx_idx: usize) -> i32 {
    let mut stop_cnum: Option<usize> = None;
    let mut cnum;

    {
        let ch = channels().lock().unwrap();
        let n = ch.len();
        cnum = 0usize;

        while cnum < n {
            if ch[cnum].sfxinfo.is_none() {
                break;
            }
            if origin.is_some() && ch[cnum].origin == origin {
                stop_cnum = Some(cnum);
                break;
            }
            cnum += 1;
        }

        if cnum == n {
            for i in 0..n {
                if let Some(ci) = ch[i].sfxinfo {
                    let sfx = sounds::s_sfx().lock().unwrap();
                    if sfx.get(ci).map(|s| s.priority).unwrap_or(0) >= sfxinfo.priority {
                        drop(sfx);
                        stop_cnum = Some(i);
                        cnum = i;
                        break;
                    }
                }
            }
            if stop_cnum.is_none() {
                return -1;
            }
        }
    }

    if let Some(idx) = stop_cnum {
        s_stop_channel(idx);
    }

    let mut ch = channels().lock().unwrap();
    if cnum < ch.len() {
        ch[cnum].sfxinfo = Some(sfx_idx);
        ch[cnum].origin = origin;
    }
    cnum as i32
}

/// Start music.
pub fn s_start_music(music_id: i32) {
    s_change_music(music_id as usize, true);
}

/// Change music.
pub fn s_change_music(music_id: usize, looping: bool) {
    if !FEATURE_SOUND {
        return;
    }
    s_stop_music();
    if music_id >= sounds::NUMMUSIC {
        return;
    }
    // Stub: no actual music playback yet
    *MUS_PLAYING.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(music_id);
}

/// Query if music is playing.
pub fn s_music_playing() -> bool {
    i_sound::i_music_is_playing()
}

/// Stop music.
pub fn s_stop_music() {
    i_sound::i_stop_song();
    *MUS_PLAYING.get_or_init(|| Mutex::new(None)).lock().unwrap() = None;
}

/// Pause sound.
pub fn s_pause_sound() {
    if MUS_PLAYING.get_or_init(|| Mutex::new(None)).lock().unwrap().is_some()
        && !*MUS_PAUSED.get_or_init(|| Mutex::new(false)).lock().unwrap()
    {
        i_sound::i_pause_song();
        *MUS_PAUSED.get_or_init(|| Mutex::new(false)).lock().unwrap() = true;
    }
}

/// Resume sound.
pub fn s_resume_sound() {
    if MUS_PLAYING.get_or_init(|| Mutex::new(None)).lock().unwrap().is_some()
        && *MUS_PAUSED.get_or_init(|| Mutex::new(false)).lock().unwrap()
    {
        i_sound::i_resume_song();
        *MUS_PAUSED.get_or_init(|| Mutex::new(false)).lock().unwrap() = false;
    }
}

/// Update sounds (stub - no listener yet).
pub fn s_update_sounds(_listener: Option<*const Mobj>) {
    if !FEATURE_SOUND {
        return;
    }
    i_sound::i_update_sound();
}

/// Set music volume.
pub fn s_set_music_volume(volume: i32) {
    i_sound::i_set_music_volume(volume);
}

/// Set SFX volume.
pub fn s_set_sfx_volume(volume: i32) {
    set_snd_sfx_volume(volume);
}
