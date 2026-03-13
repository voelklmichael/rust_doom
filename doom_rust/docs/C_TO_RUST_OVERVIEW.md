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
| **rendering/** | | Scene rendering (BSP, visplanes, sprites) |
| ↳ defs | r_defs.h | vertex_t, sector_t, line_t, seg_t, node_t, subsector_t, etc. |
| ↳ m_bbox | m_bbox.h/c | M_ClearBox, M_AddToBox, bbox indices; shared util (also used by player::p_maputl) |
| ↳ r_main | r_main.h/c | R_Init, R_RenderPlayerView, R_PointToAngle, R_PointInSubsector |
| ↳ r_bsp | r_bsp.h/c | R_RenderBSPNode |
| ↳ r_data | r_data.h/c | R_InitData, R_PrecacheLevel, texture/flat/sprite loading |
| ↳ r_segs | r_segs.h/c | R_StoreWallRange, R_RenderMaskedSegRange |
| ↳ r_plane | r_plane.h/c | R_CheckPlane, R_DrawPlanes |
| ↳ r_draw | r_draw.h/c | Column drawing |
| ↳ r_things | r_things.h/c | Sprite/thing rendering |
| ↳ r_sky | r_sky.h/c | Sky texture |
| ↳ v_patch | v_patch.h | patch_t, post_t |
| ↳ v_video | v_video.h/c | V_Init, V_DrawPatch, screen buffer |

---

## Started (Stub / Partial)

These modules exist but have minimal or stub implementations.

| Rust Module | C Source(s) | Status |
|-------------|-------------|--------|
| **d_iwad** | d_iwad.h/c | Stub: D_TryFindWADByName, D_SuggestGameName (minimal logic) |
| **i_system** | i_system.h/c | Stub: I_ZoneBase, I_Error, I_BeginRead, I_EndRead (no I_GetTime, I_Init, etc.) |
| **doomstat** | doomstat.h/c | Partial: globals (GAMEMODE, GAMEMAP, etc.), Player/WbStartStruct stubs |
| **player/** | p_*.h / p_*.c | Scaffolded; see below |

### player/ module (src/player/)

All p_* C modules are scaffolded in `player/`. See `docs/PLAYER_TRANSLATION_PLAN.md` for details.

| Submodule | Status | Notes |
|-----------|--------|-------|
| **p_setup** | Working | p_load_level for scene rendering (no blockmap yet) |
| **p_mobj** | Partial | Mobj (x, y, z, angle, sprite, frame, flags) for sound/rendering |
| **p_maputl** | Partial | P_AproxDistance, P_PointOnLineSide, P_LineOpening, etc. (no blockmap iterators) |
| **p_tick** | Stub | P_InitThinkers, P_AddThinker, P_RemoveThinker |
| **p_map** | Stub | API only; P_CheckPosition, P_TryMove, etc. need blockmap |
| **p_sight** | Stub | P_CheckSight always returns true |
| **p_spec** | Partial | get_next_sector |
| **p_floor, p_ceilng, p_doors, p_plats, p_lights, p_telept** | Stub | Module structure only |
| **p_switch, p_inter** | Stub | API stubs |
| **p_pspr, p_user, p_enemy, p_saveg** | Stub | API stubs |

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
| info | Thing/mobjs info tables (mobjinfo_t, state_t) |
| d_think | thinker_t, action function pointers |
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
| **Fully rewritten** | ~35 modules (incl. rendering/) |
| **Started (stub)** | 4 + player/ (20 submodules scaffolded) |
| **Not started** | ~45 C modules |

**Foundation:** WAD, zone, sound, geometry, types, rendering (scene rendering works).  
**Player:** All p_* modules scaffolded; blockmap, d_think, info, g_game needed for full game logic.

See also: `PLAYER_TRANSLATION_PLAN.md`, `RENDERING_TRANSLATION_PLAN.md`
