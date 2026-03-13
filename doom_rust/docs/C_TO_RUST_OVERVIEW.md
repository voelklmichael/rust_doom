# C → Rust Translation Overview

Overview of the doom_rust port: which modules are fully rewritten, partially started, or not yet begun.

---

## Fully Rewritten

These modules have been ported from C with full or near-full functionality.

| Rust Module | C Source(s) | Notes |
|-------------|-------------|-------|
| **m_random** | m_random.h/c | RNDTABLE, M_Random, P_Random, M_ClearRandom |
| **m_argv** | m_argv.h/c | argc/argv, M_CheckParm, M_CheckParmWithArgs, M_ParmExists |
| **m_fixed** | m_fixed.h/c | fixed_t, FRACUNIT, FRACBITS, FixedMul |
| **m_misc** | m_misc.h/c | M_StringCopy, M_ExtractFileBase, M_FileLength |
| **doomtype** | doomtype.h | Boolean, Byte, strcasecmp, DIR_SEPARATOR, etc. |
| **doomdef** | doomdef.h (partial) | MAXPLAYERS, Gamestate |
| **doomdata** | doomdata.h (partial) | MapThing |
| **doomfeatures** | doomfeatures.h | FEATURE_SOUND, etc. |
| **d_mode** | d_mode.h | GameMission, GameMode, GameVersion, Skill |
| **d_ticcmd** | d_ticcmd.h | Ticcmd |
| **config** | config.h | Build configuration constants |
| **i_swap** | i_swap.h | Long/Short byte swapping |
| **geometry/tables** | tables.h/c | finesine, finetangent, tantoangle, gammatable, slope_div, angle constants |
| **geometry/tables_data** | tables.c | Lookup table data (FINESINE, FINETANGENT, TANTOANGLE, GAMMATABLE) |
| **sound/** | | |
| ↳ i_sound | i_sound.h/c | SfxInfo, MusicInfo, SoundModule/MusicModule traits, I_* stubs |
| ↳ sounds | sounds.h/c | S_sfx, S_music, MusicEnum, SfxEnum |
| ↳ s_sound | s_sound.h/c | S_Init, S_StartSound, S_StopSound, channel logic, stereo |
| ↳ memio | memio.h/c | MemFileRead, MemFileWrite, memory streams |
| ↳ mus2mid | mus2mid.h/c | MUS→MIDI conversion |
| ↳ r_angle | r_main.c (partial) | R_PointToAngle2 for stereo positioning |
| **z_zone** | z_zone.h/c | Z_Init, Z_Malloc, Z_Free, Z_ChangeTag, purge tags |
| **wad/** | | |
| ↳ w_wad | w_wad.h/c | W_AddFile, W_ReadLump, W_CacheLump*, hash table |
| ↳ w_file | w_file.h/c | WadFile, W_OpenFile |
| ↳ w_checksum | w_checksum.c | W_Checksum |
| ↳ w_main | w_main.c | W_ParseCommandLine |
| ↳ w_merge | w_merge.h/c | W_MergeFile, NWT merge |
| ↳ w_file_stdc | w_file_stdc.c | Stdio-based file I/O |
| **sha1_mod** | sha1.c | SHA1 hashing |

---

## Started (Stub / Partial)

These modules exist but have minimal or stub implementations.

| Rust Module | C Source(s) | Status |
|-------------|-------------|--------|
| **d_iwad** | d_iwad.h/c | Stub: D_TryFindWADByName, D_SuggestGameName (minimal logic) |
| **i_system** | i_system.h/c | Stub: I_ZoneBase, I_Error, I_BeginRead, I_EndRead (no I_GetTime, I_Init, etc.) |
| **doomstat** | doomstat.h/c | Partial: globals (GAMEMODE, GAMEMAP, etc.), Player/WbStartStruct stubs |
| **p_mobj** | p_mobj.h (partial) | Minimal stub: Mobj with x, y, z, angle only (for sound) |

---

## Not Started

C modules with no Rust equivalent yet.

### Game Core
| C Module | Purpose |
|----------|---------|
| d_main | Main game loop, D_DoomMain |
| d_loop | Game loop, netgame sync |
| d_event | Event handling |
| d_net | Networking |
| d_items | Item pickup logic |
| dstrings | Game strings |
| g_game | Game logic, G_Ticker, G_Responder |

### Player / Physics
| C Module | Purpose |
|----------|---------|
| p_mobj | Full map object (sprites, state, thinker) |
| p_map | Collision, movement |
| p_maputl | Map utilities |
| p_mobj | Full implementation |
| p_user | Player movement |
| p_plats | Platforms |
| p_doors | Doors |
| p_floor | Floor movement |
| p_ceilng | Ceiling movement |
| p_lights | Lighting |
| p_spec | Special sectors |
| p_switch | Switches |
| p_inter | Item interaction |
| p_sight | Line-of-sight |
| p_telept | Teleporters |
| p_tick | Thinker ticks |
| p_pspr | Player weapons/sprites |
| p_setup | Level setup |
| p_enemy | Monster AI |
| p_saveg | Savegame |

### Rendering
| C Module | Purpose |
|----------|---------|
| r_main | Main renderer, R_RenderPlayerView |
| r_bsp | BSP traversal |
| r_segs | Seg rendering |
| r_plane | Floors/ceilings |
| r_draw | Column drawing |
| r_data | Texture/flat/sprite loading |
| r_things | Thing rendering |
| r_sky | Sky rendering |
| v_video | Video buffer |
| v_patch | Patch drawing |

### UI / HUD
| C Module | Purpose |
|----------|---------|
| m_menu | Main menu |
| m_controls | Control config |
| m_cheat | Cheat codes |
| m_config | Configuration |
| hu_stuff | Heads-up display |
| hu_lib | HUD font |
| st_stuff | Status bar |
| st_lib | Status bar lib |
| wi_stuff | Intermission screen |

### Other
| C Module | Purpose |
|----------|---------|
| info | Thing/mobjs info tables |
| m_bbox | Bounding boxes |
| f_finale | End-game screens |
| f_wipe | Screen wipe |
| i_timer | Timing |
| i_video | Video init |
| i_input | Input handling |
| i_joystick | Joystick |
| i_scale | Resolution scaling |
| i_endoom | ENDOOM screen |
| i_cdmus | CD music |
| deh_* | DeHackEd |
| am_map | Automap |
| statdump | Statistics |
| dummy | Placeholder |
| doomgeneric* | Platform-specific (Linux, Win, etc.) |

### Network (if applicable)
| C Module | Purpose |
|----------|---------|
| net_* | Network protocol, client, server |

---

## Summary

| Category | Count |
|----------|-------|
| **Fully rewritten** | ~25 modules |
| **Started (stub)** | 4 modules |
| **Not started** | ~60+ C modules |

The foundation (WAD, zone, sound, geometry, types) is in place. Game logic, rendering, and UI are not yet ported.
