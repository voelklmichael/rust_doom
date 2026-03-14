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
| **z_zone** | z_zone.h/c | Z_Init, Z_Malloc, Z_Free, Z_ChangeTag, purge tags. Known: base=null during purge (corruption) |
| **wad/** | | |
| ↳ w_wad | w_wad.h/c | W_AddFile, W_ReadLump, W_CacheLump*, hash table |
| ↳ w_file | w_file.h/c | WadFile, W_OpenFile |
| ↳ w_checksum | w_checksum.c | W_Checksum |
| ↳ w_main | w_main.c | W_ParseCommandLine |
| ↳ w_merge | w_merge.h/c | W_MergeFile, NWT merge |
| ↳ w_file_stdc | w_file_stdc.c | Stdio-based file I/O |
| **sha1_mod** | sha1.c | SHA1 hashing |
| **game/** | | Game loop, events, thinkers |
| ↳ d_think | d_think.h | Thinker, ActionF, thinker_t |
| ↳ d_event | d_event.h/c | Event, EvType, d_post_event, d_pop_event |
| ↳ d_items | d_items.h/c | Weaponinfo, WEAPONINFO table |
| ↳ d_mode | d_mode.h | GameMission, GameMode, Skill |
| ↳ d_ticcmd | d_ticcmd.h | Ticcmd |
| ↳ d_iwad | d_iwad.h/c | D_TryFindWADByName, D_SuggestGameName |
| ↳ d_main | d_main.h/c | D_ProcessEvents, GAMEACTION |
| ↳ d_loop | d_loop.h/c | TryRunTics, LoopInterface |
| ↳ g_game | g_game.h/c | G_Ticker, G_Responder, G_BuildTiccmd |
| ↳ dstrings | dstrings.h/c, d_englsh.h | SAVEGAMENAME, quit messages, save/load prompts |
| **i_timer** | i_timer.h/c | i_get_time, i_sleep, i_init_timer |
| **rendering/** | | Scene rendering (BSP, visplanes, sprites) |
| ↳ defs | r_defs.h | vertex_t, sector_t, line_t, seg_t, node_t, subsector_t, etc. |
| ↳ m_bbox | m_bbox.h/c | M_ClearBox, M_AddToBox, bbox indices; shared util (also used by player::p_maputl) |
| ↳ r_main | r_main.h/c | R_Init, R_RenderPlayerView, R_PointToAngle, R_PointInSubsector |
| ↳ r_bsp | r_bsp.h/c | R_RenderBSPNode |
| ↳ r_data | r_data.h/c | R_InitData, R_PrecacheLevel, texture/flat/sprite loading |
| ↳ r_segs | r_segs.h/c | R_StoreWallRange, R_RenderMaskedSegRange |
| ↳ r_plane | r_plane.h/c | R_CheckPlane, R_DrawPlanes |
| ↳ r_draw | r_draw.h/c | Column drawing, R_VideoErase |
| ↳ r_things | r_things.h/c | Sprite/thing rendering |
| ↳ r_sky | r_sky.h/c | Sky texture |
| ↳ v_patch | v_patch.h | patch_t, post_t |
| ↳ v_video | v_video.h/c | V_Init, V_DrawPatch, V_DrawPatchDirect, V_CopyRect, V_UseBuffer, V_RestoreBuffer, screen buffer |

---

## Started (Stub / Partial)

These modules exist but have minimal or stub implementations.

| Rust Module | C Source(s) | Status |
|-------------|-------------|--------|
| **i_system** | i_system.h/c | Stub: I_ZoneBase, I_Error, I_BeginRead, I_EndRead (no I_GetTime, I_Init, etc.) |
| **doomstat** | doomstat.h/c | Partial: globals (GAMEMODE, GAMEMAP, etc.), Player (mo, viewz, viewheight, extralight, fixedcolormap, playerstate, health), PlayerState enum, PLAYERS, PLAYERINGAME, PLAYERSTARTS; WbStartStruct (epsd, last, next, pnum, plyr), WbPlayerStruct |
| **player/** | p_*.h / p_*.c | Scaffolded; see below |
| **ui_hud** | m_menu, m_controls, m_cheat, m_config, hu_*, st_*, wi_stuff | All 9 modules full: cheat, st_lib, hu_lib, hu_stuff, st_stuff, wi_stuff, config, controls, menu. Save string input, Read This screen, Heretic/Hexen/Strife controls. v_video, r_draw. |

### player/ module (src/player/)

All p_* C modules are scaffolded in `player/`. See `docs/PLAYER_TRANSLATION_PLAN.md` for details.

| Submodule | Status | Notes |
|-----------|--------|-------|
| **p_setup** | Working | p_load_level, blockmap, REJECT, P_LoadThings, playeringame init |
| **p_mobj** | Working | Mobj full, P_SpawnMobj, P_RemoveMobj, P_SpawnMapThing, P_SpawnPlayer, P_MobjThinker (XY/Z movement, state tics) |
| **p_maputl** | Working | BlockLinesIterator, BlockThingsIterator, PathTraverse, P_SetThingPosition, P_UnsetThingPosition |
| **p_tick** | Working | P_InitThinkers, P_AddThinker, P_RemoveThinker, P_RunThinkers, P_Ticker |
| **p_map** | Working | P_CheckPosition, P_TryMove, P_TeleportMove, P_SlideMove |
| **p_sight** | Working | P_CheckSight (REJECT + BSP traversal) |
| **p_spec** | Partial | get_next_sector |
| **p_floor, p_ceilng, p_doors, p_plats, p_lights, p_telept** | Stub | Module structure only |
| **p_switch, p_inter** | Stub | API stubs |
| **p_pspr, p_user, p_enemy, p_saveg** | Stub | API stubs |

### ui_hud/ module (src/ui_hud/)

All 9 UI/HUD modules implemented. See `docs/UI_HUD_TRANSLATION_PLAN.md` for details.

| Submodule | Status | Notes |
|-----------|--------|-------|
| **cheat** | Full | CheatSeq, cht_CheckCheat, cht_GetParam |
| **controls** | Full | Key globals, M_Bind* (base, Heretic, Hexen, Strife, weapon, map, menu, chat) |
| **config** | Full | M_LoadDefaults, M_SaveDefaults, variable store, m_set_variable → m_update_control_from_config |
| **hu_lib** | Full | HuTextline, HuStext, HuItext |
| **hu_stuff** | Full | HU_Init, HU_Start, HU_Drawer, HU_Erase, HU_Ticker; save string input |
| **st_lib** | Full | StNumber, StPercent, StMultIcon, StBinIcon |
| **st_stuff** | Full | ST_Init, ST_Start, ST_Drawer, ST_Ticker, ST_Responder; w_health/w_armor, cheats |
| **wi_stuff** | Full | WI_Start, WI_Ticker, WI_Drawer, WI_End; intermission |
| **menu** | Full | M_Init, M_Responder, M_Drawer; main/episode/newgame/options/sound/load/save; Read This; save string input |

---

## Not Started

C modules with no Rust equivalent yet.

### Game Core
| C Module | Purpose |
|----------|---------|
| d_main (partial) | D_ProcessEvents, gameaction; D_DoomMain not ported |
| d_loop (partial) | TryRunTics, LoopInterface; netgame not ported |
| d_net | Networking |
| g_game (partial) | G_Ticker, G_PlayerReborn, G_BuildTiccmd stub; G_InitNew, save/load not ported |

### Other
| C Module | Purpose |
|----------|---------|
| info | Thing/mobjs info tables – minimal done (State, Mobjinfo, states(), MOBJINFO for MT_PLAYER, MT_POSSESSED, MT_TROOP, MT_SERGEANT, MT_HEAD) |
| f_finale | End-game screens (game/f_finale.rs: F_StartFinale, F_Responder, F_Ticker, F_Drawer stub) |
| f_wipe | Screen wipe (game/f_wipe.rs: wipe_StartScreen, wipe_EndScreen, wipe_ScreenWipe ColorXForm) |
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
**Player:** p_setup, p_mobj, p_map, p_sight, p_maputl, p_tick working; P_SpawnPlayer, G_PlayerReborn done. example_render_scene blocked by z_zone corruption.  
**UI/HUD:** All 9 modules complete (cheat, controls, config, hu_lib, hu_stuff, st_lib, st_stuff, wi_stuff, menu).

See also: `PLAYER_TRANSLATION_PLAN.md`, `RENDERING_TRANSLATION_PLAN.md`, `GAME_CORE_TRANSLATION_PLAN.md`, `NEXT_PHASE_TRANSLATION_PLAN.md`, `UI_HUD_TRANSLATION_PLAN.md`
