# Sound/Music Translation Plan: C → Rust

## Overview

The Doom sound system has three layers:

1. **Data layer** – Sound/music lookup tables (`sounds.h/c`)
2. **Game logic layer** – Channel management, volume, stereo, level music (`s_sound.h/c`)
3. **Platform layer** – Actual audio output (`i_sound.h/c` + backend implementation)

Supporting modules: `memio` (memory streams), `mus2mid` (MUS→MIDI conversion).

**Note:** SDL-based backends (`i_sdlsound.c`, `i_sdlmusic.c`) have been removed. The Rust port will use a **pure Rust** backend (rodio, etc.) – no SDL or other native audio libs.

---

## File Inventory

| C File | Lines | Purpose | Dependencies |
|--------|-------|---------|--------------|
| `sounds.h` | ~230 | Enums, extern S_sfx/S_music | i_sound |
| `sounds.c` | ~230 | S_sfx[], S_music[] data | doomtype |
| `s_sound.h` | ~90 | S_Init, S_StartSound, etc. | p_mobj, sounds |
| `s_sound.c` | ~671 | Channel logic, volume, stereo | i_sound, p_local, w_wad, z_zone, mobj_t |
| `i_sound.h` | ~255 | sfxinfo_t, musicinfo_t, module traits | doomtype |
| `i_sound.c` | ~420 | Module selection, I_* wrappers | config, doomfeatures |
| `mus2mid.h` | 8 | mus2mid() declaration | memio |
| `mus2mid.c` | ~738 | MUS→MIDI conversion | memio, i_swap |
| `memio.h` | 40 | MEMFILE, mem_fread, etc. | - |
| `memio.c` | ~150 | Memory stream implementation | - |

**Remaining C backends** (reference only, not porting): `i_allegrosound.c`, `i_allegromusic.c` (Allegro). We implement a **pure Rust** backend instead.

---

## Dependency Graph

```
sounds.rs          (no deps)
    ↓
i_sound.rs        (doomtype, doomfeatures, config)
    ↓
memio.rs          (no deps)
    ↓
mus2mid.rs        (memio, i_swap)
    ↓
i_sound_rodio.rs  (rodio, midly, w_wad, z_zone, memio, mus2mid)   ← pure Rust
    ↓
s_sound.rs        (i_sound, p_mobj, m_fixed, r_main?, w_wad, z_zone, doomstat)
```

**Critical missing deps:** `p_mobj` (mobj_t), `m_fixed` (fixed_t, FRACUNIT), `r_main` (R_PointToAngle2), `p_local` (game map).

---

## Recommended Translation Order

### Phase 1: Foundation (no game logic)

| Step | Module | Rust File | Notes |
|------|--------|-----------|-------|
| 1 | memio | `memio.rs` | Memory streams, ~150 lines. Pure I/O. |
| 2 | sounds | `sounds.rs` | S_sfx[], S_music[] data + enums. |
| 3 | i_sound | `i_sound.rs` | Types: `SfxInfo`, `MusicInfo`, `SndDevice`, `SoundModule`, `MusicModule` traits. |
| 4 | mus2mid | `mus2mid.rs` | MUS→MIDI. Pure logic, ~700 lines. |

### Phase 2: Stubs (compile, no audio)

| Step | Module | Rust File | Notes |
|------|--------|-----------|-------|
| 5 | m_fixed | `m_fixed.rs` | `fixed_t`, `FRACUNIT`, `FRACBITS` from doomdef.h. |
| 6 | p_mobj | `p_mobj.rs` | Minimal `Mobj` stub (x, y, z, angle) for s_sound. |
| 7 | s_sound | `s_sound.rs` | Stub S_Init, S_StartSound, etc. Return/panic. |

### Phase 3: Platform Backend (pure Rust)

| Step | Module | Rust File | Notes |
|------|--------|-----------|-------|
| 8 | i_sound_impl | `i_sound_rodio.rs` | Implement `SoundModule`/`MusicModule`. |

**Backend (no SDL):**

- **rodio + midly**: Pure Rust, no native libs. rodio for SFX playback; midly for MUS→MIDI parsing (mus2mid) + MIDI playback via rodio or a MIDI synth.
- **cpal + midly**: Lower-level alternative if rodio is insufficient.

### Phase 4: Full s_sound Integration

| Step | Module | Rust File | Notes |
|------|--------|-----------|-------|
| 9 | r_main | `r_main.rs` | `R_PointToAngle2` (or inline in s_sound). |
| 10 | s_sound | `s_sound.rs` | Full channel logic, volume, stereo, per-level music. |

---

## Module Details

### 1. sounds.rs

- **MusicEnum** – `mus_None`, `mus_e1m1`, … `mus_dm2int`, `NUMMUSIC`.
- **SfxEnum** – `sfx_None`, `sfx_pistol`, … `sfx_radio`, `NUMSFX`.
- **S_music: [MusicInfo; NUMMUSIC]** – name, lumpnum, data, handle.
- **S_sfx: [SfxInfo; NUMSFX]** – name, priority, link, pitch, volume, lumpnum, etc.

### 2. i_sound.rs

- **SfxInfo** – `tagname`, `name[9]`, `priority`, `link`, `pitch`, `volume`, `usefulness`, `lumpnum`, `numchannels`, `driver_data`.
- **MusicInfo** – `name`, `lumpnum`, `data`, `handle`.
- **SndDevice** enum: `None`, `PcSpeaker`, `Adlib`, `SB`, … `CD`.
- **SoundModule** trait: `Init`, `Shutdown`, `GetSfxLumpNum`, `Update`, `UpdateSoundParams`, `StartSound`, `StopSound`, `SoundIsPlaying`, `CacheSounds`.
- **MusicModule** trait: `Init`, `Shutdown`, `SetMusicVolume`, `PauseMusic`, `ResumeMusic`, `RegisterSong`, `UnRegisterSong`, `PlaySong`, `StopSong`, `MusicIsPlaying`, `Poll`.
- **I_*** functions – dispatch to active module.

### 3. memio.rs

- **MemFile** – `Read` + `Seek` over `&[u8]` or `Vec<u8>`.
- **mem_fopen_read**, **mem_fopen_write**, **mem_fread**, **mem_fwrite**, **mem_fseek**, **mem_ftell**, **mem_fclose**, **mem_get_buf**.

### 4. mus2mid.rs

- **mus2mid(mus_input, midi_output)** – convert MUS bytes to MIDI bytes.
- MUS event codes, channel mapping, MIDI header.
- Can use `std::io::Read`/`Write` for `MemFile`.

### 5. s_sound.rs (full)

- **channel_t** – `sfxinfo`, `origin` (Mobj), `handle`.
- **S_CLIPPING_DIST**, **S_CLOSE_DIST**, **S_ATTENUATOR**, **S_STEREO_SWING**.
- **S_Init**, **S_Shutdown**, **S_Start**, **S_StartSound**, **S_StopSound**, **S_StartMusic**, **S_ChangeMusic**, **S_MusicPlaying**, **S_StopMusic**, **S_PauseSound**, **S_ResumeSound**, **S_UpdateSounds**, **S_SetMusicVolume**, **S_SetSfxVolume**.
- **S_GetChannel**, **S_StopChannel**, **S_AdjustSoundParams** (needs `R_PointToAngle2`).

### 6. i_sound_rodio.rs (pure Rust backend)

- Implement `SoundModule`: load WAD lumps (Doom format → PCM), convert to rodio `Sink`/`Source`, cache.
- Implement `MusicModule`: load MUS→MIDI via mus2mid; play MIDI via rodio + soft synth, or use `midly` + a Rust MIDI renderer.
- **FEATURE_SOUND** gate – no-op when disabled.

---

## Cargo.toml Additions

```toml
# Pure Rust audio (no SDL)
rodio = { version = "0.17", optional = true }
midly = { version = "0.5", optional = true }

[features]
default = []
sound = ["rodio", "midly"]
```

---

## Risks / Mitigations

| Risk | Mitigation |
|------|------------|
| **p_mobj / game logic** not yet ported | Use `Mobj` stub (x, y, z, angle) or `Option<MobjId>` for origin. |
| **R_PointToAngle2** not ported | Implement in `m_angle.rs` or inline in s_sound (simple atan2). |
| **fixed_t** | Use `i32` (FRACUNIT = 1<<16). |
| **MIDI playback** | rodio plays raw PCM; MUS→MIDI via mus2mid; need soft synth or `midly` + renderer for music. |
| **deh_str** (Dehacked sound names) | Stub or defer; use lump names for now. |

---

## Estimated Effort

| Phase | Effort | Notes |
|-------|--------|-------|
| Phase 1 | 1–2 days | memio, sounds, i_sound types, mus2mid |
| Phase 2 | 0.5 day | m_fixed, p_mobj stub, s_sound stub |
| Phase 3 | 1–2 days | rodio backend (pure Rust) |
| Phase 4 | 1 day | Full s_sound, R_PointToAngle2 |

**Total:** ~4–6 days for a working sound/music system.

---

## Checklist

- [x] memio.rs
- [x] sounds.rs (data + enums)
- [x] i_sound.rs (types + traits)
- [x] mus2mid.rs
- [x] m_fixed.rs
- [x] p_mobj.rs stub
- [x] s_sound.rs (full channel logic, stub I_* backend)
- [x] R_PointToAngle2 / r_angle – for stereo positioning
- [ ] i_sound_rodio.rs – skipped (no rodio backend)
