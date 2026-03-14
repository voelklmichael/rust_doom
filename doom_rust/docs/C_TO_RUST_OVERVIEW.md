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
| **doomdef** | doomdef.h | MAXPLAYERS, Gamestate, Gameaction, Card, Weapontype, Ammotype, Powertype, screen dims, power durations, MTF_* skill flags, BLOCK_SIZE |
| **doomdata** | doomdata.h | MapThing, MapVertex, MapLineDef, MapSideDef, MapSector, MapSeg, MapSubsector, MapNode, MTF_NOTSINGLE |
| **doomstat** | doomstat.h/c, d_player.h | player_t (Player), wbplayerstruct_t (WbPlayerStruct), wbstartstruct_t (WbStartStruct), Pspdef, all globals (GAMEMODE, GAMEMAP, PLAYERS, etc.), logical_gamemission |
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
| ↳ g_game | g_game.h/c | G_Ticker, G_Responder, G_BuildTiccmd; F_Ticker/F_Responder when GAMESTATE=Finale |
| ↳ f_finale | f_finale.h/c | F_StartFinale, F_Responder, F_Ticker, F_Drawer; F_TextWrite, F_ArtScreenDrawer, F_BunnyScroll; F_CastDrawer/CastTicker (stub) |
| ↳ f_wipe | f_wipe.h/c | wipe_StartScreen, wipe_EndScreen, wipe_ScreenWipe; ColorXForm + Melt |
| ↳ statdump | statdump.h/c | Statdump capture, mission discovery, output |
| ↳ dstrings | dstrings.h/c, d_englsh.h | SAVEGAMENAME, quit messages, save/load prompts |
| **i_timer** | i_timer.h/c | i_get_time, i_sleep, i_init_timer |
| **rendering/** | | Scene rendering (BSP, visplanes, sprites) |
| ↳ defs | r_defs.h | vertex_t, sector_t, line_t, seg_t, node_t, subsector_t, etc. |
| ↳ m_bbox | m_bbox.h/c | M_ClearBox, M_AddToBox, bbox indices; shared util (also used by player::p_maputl) |
| ↳ r_main | r_main.h/c | R_Init, R_RenderPlayerView, R_SetViewSize, R_ExecuteSetViewSize (calls r_init_buffer), R_PointToAngle, R_PointToAngle2, R_PointToDist, R_PointInSubsector, R_PointOnSide, R_PointOnSegSide, R_ScaleFromGlobalAngle, R_AddPointToBox, R_InitTextureMapping, R_InitLightTables, viewangleoffset, pspritescale, pspriteiscale, screenheightarray, yslope, distscale |
| ↳ r_bsp | r_bsp.h/c | R_RenderBSPNode |
| ↳ r_data | r_data.h/c | R_InitData, R_PrecacheLevel, texture/flat/sprite loading |
| ↳ r_segs | r_segs.h/c | R_StoreWallRange, R_RenderMaskedSegRange |
| ↳ r_plane | r_plane.h/c | R_CheckPlane, R_DrawPlanes |
| ↳ r_draw | r_draw.h/c | colfunc, spanfunc, fuzzcolfunc, transcolfunc (high/low by detailshift), R_InitBuffer, R_VideoErase |
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
| **p_spec** | Partial | get_next_sector, P_CrossSpecialLine, P_UseSpecialLine, P_ShootSpecialLine; dispatches to EV_* (floor, teleport working) |
| **p_floor** | Working | EV_DoFloor (types 1,2,4,5,6), T_MoveFloor, find_lowest/highest_floor_surrounding |
| **p_ceilng, p_doors, p_plats** | Stub | EV_DoCeiling, EV_DoDoor, EV_DoPlat API stubs |
| **p_lights** | Partial | EV_LightTurnOn; EV_StartLightStrobing/Flickering stubs (need T_* thinkers) |
| **p_telept** | Working | EV_Teleport: find MO_TELEPORTMAN by tag, P_TeleportMove |
| **p_switch, p_inter** | Stub | P_UseSpecialLine, P_ChangeSwitchTexture; P_TouchSpecialThing, P_DamageMobj, P_Give*Ammo/Weapon/Body/Armor |
| **p_pspr, p_user, p_saveg** | Stub | P_SetupPsprites, P_MovePsprites, P_DropWeapon; P_PlayerThink, P_Thrust, P_CalcHeight, P_MovePlayer; P_Archive* |
| **p_enemy** | Partial | P_NoiseAlert (sets sector.soundtarget for adjacent sectors) |

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

### Game Core (partial – missing pieces)
| C Module | Purpose |
|----------|---------|
| d_main | D_DoomMain, D_DoomInit, D_DoomLoop, full startup; D_ProcessEvents/D_Display/D_Shutdown done |
| d_loop | Game action dispatch; TryRunTics, LoopInterface done |
| g_game | G_InitNew, G_DoLoadLevel, G_DoSaveGame, G_DoLoadGame, G_BuildTiccmd; G_Ticker, G_Responder, G_PlayerReborn done |

**Conversion plan:** See `GAME_CORE_TRANSLATION_PLAN.md` § Conversion Plan: d_main, d_loop, g_game.

### Info / Data Tables
| C Module | Purpose |
|----------|---------|
| info | Full thing/mobj info tables – State, Mobjinfo, states(), S_sfx; minimal done (MT_PLAYER, MT_POSSESSED, MT_TROOP, MT_SERGEANT, MT_HEAD) |

### Platform I/O (see `docs/INPUT_TRANSLATION_PLAN.md`)
| C Module | Purpose |
|----------|---------|
| i_video | Video init, palette, blit – I_InitGraphics, I_SetPalette, I_FinishUpdate (stub) |
| i_input | Keyboard/mouse → events (stub) |
| i_joystick | Gamepad → ticcmd (stub) |
| i_scale | Resolution scaling (stub) |
| i_cdmus | CD music playback (stub) |
| i_allegrosound | Allegro sound backend |
| i_allegromusic | Allegro music backend |

### DeHackEd
| C Module | Purpose |
|----------|---------|
| deh_main | DeHackEd main, BEX parsing |
| deh_misc | DeHackEd misc (DEH_AddStringReplacement, etc.) |
| deh_str | DeHackEd string tables |

### Networking (out of scope – local play only)
| C Module | Purpose |
|----------|---------|
| d_net | Netgame init, D_ArbitrateNetStart |
| net_defs | Net packet types |
| net_client, net_server | Client/server |
| net_loop | Loopback |
| net_io, net_packet | I/O, packet handling |
| net_query, net_gui, net_dedicated | Query, GUI, dedicated server |

### Platform / Build
| C Module | Purpose |
|----------|---------|
| doomgeneric | Platform abstraction (doomgeneric.c, doomgeneric.h) |
| doomgeneric_linuxvt | Linux VT backend |
| doomgeneric_win | Windows backend |
| doomgeneric_xlib | X11 backend |
| doomgeneric_emscripten | Emscripten/Web |
| doomgeneric_allegro | Allegro backend |
| doomgeneric_soso, doomgeneric_sosox | Soso backends |

### Misc / Optional
| C Module | Purpose |
|----------|---------|
| gusconf | Gravis Ultrasound config |
| icon | ✅ Done – asserts/icon.png (converted from icon.c RGB data) |
| dummy | Placeholder / unused |
| d_textur | Texture format (Heretic/Hexen) |

---

## Summary

| Category | Count |
|----------|-------|
| **Fully rewritten** | ~38 modules (incl. rendering/, f_finale, f_wipe, statdump) |
| **Started (stub)** | 2 + player/ (20 submodules scaffolded) |
| **Not started** | ~33 C modules (game core 3, info 1, platform I/O 7, DeHackEd 3, networking 8, platform 7, misc 4) |

**Foundation:** WAD, zone, sound, geometry, types, rendering (scene rendering works).  
**Player:** p_setup, p_mobj, p_map, p_sight, p_maputl, p_tick working; P_SpawnPlayer, G_PlayerReborn done. example_render_scene blocked by z_zone corruption.  
**UI/HUD:** All 9 modules complete (cheat, controls, config, hu_lib, hu_stuff, st_lib, st_stuff, wi_stuff, menu).  
**Game:** f_finale (end-game screens), f_wipe (ColorXForm + Melt), statdump; g_game wired to F_Ticker/F_Responder when GAMESTATE=Finale. `d_display()` dispatches to ST_Drawer, R_RenderPlayerView/AM_Drawer, HU_Drawer, WI_Drawer, F_Drawer, D_PageDrawer by gamestate; calls `i_finish_update()`. `d_shutdown()` displays ENDOOM on exit.

See also: `PLAYER_TRANSLATION_PLAN.md`, `RENDERING_TRANSLATION_PLAN.md`, `GAME_CORE_TRANSLATION_PLAN.md`, `NEXT_PHASE_TRANSLATION_PLAN.md`, `UI_HUD_TRANSLATION_PLAN.md`, `INPUT_TRANSLATION_PLAN.md`
