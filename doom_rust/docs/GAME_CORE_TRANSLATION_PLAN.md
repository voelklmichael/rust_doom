# Game Core Translation Plan

Plan for porting the next set of C modules that unblock the player module and enable the game loop. These modules are prerequisites for full p_* logic, G_Ticker, and interactive gameplay.

**Source:** `doomgeneric/doomgeneric/*.c` and `*.h`

**Goal:** Enable `G_Ticker` → `P_Ticker` → mobj thinkers, and `G_Responder` → input handling. This requires d_think, info, d_event, d_items, i_timer, d_main, d_loop, and g_game.

---

## Current Status

| Phase | Status | Notes |
|-------|--------|-------|
| **Phase 1** | ✅ Done | d_think, d_event |
| **Phase 2** | ✅ Done | d_items, info (minimal) |
| **Phase 3** | ✅ Done | i_timer, doomdef extension |
| **Phase 4** | ✅ Done | d_main, d_loop, g_game (stubs + G_PlayerReborn) |
| **Phase 5** | ✅ Done | dstrings |
| **Phase 6** | ✅ Done | g_game (G_InitNew, G_DeferedInitNew, G_LoadGame, G_SaveGame stubs; g_defered_load_game, g_defered_save_game) |
| **Phase 7** | ✅ Done | f_finale (F_StartFinale, F_Responder, F_Ticker, F_Drawer; text scroll, art screen, bunny scroll; cast stub), f_wipe (ColorXForm + Melt) |

---

## Dependency Graph

```
d_think ─────────────────────────────────────────────────────────┐
                                                                  │
d_event ────────────────────────────────────────────────────────┤
                                                                  │
info ───────────────────────────────────────────────────────────┼──► g_game ──► d_loop ──► d_main
  │                                                               │
  └──► mobjinfo_t, state_t (p_mobj, p_enemy)                      │
                                                                  │
d_items ─────────────────────────────────────────────────────────┤
  │                                                               │
  └──► weaponinfo_t (p_pspr)                                      │
                                                                  │
i_timer ─────────────────────────────────────────────────────────┤
  │                                                               │
  └──► I_GetTime (doomdef, d_loop)                               │
                                                                  │
doomdef (gameaction_t, gamestate_t) ─────────────────────────────┘
```

---

## Phase 1: Thinker & Event Foundation

Small, low-dependency modules that unblock mobj_t and input handling.

### 1.1 d_think

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| Types | d_think.h | `src/d_think.rs` | `actionf_v`, `actionf_p1`, `actionf_p2`, `actionf_t`, `think_t`, `thinker_t` |
| Dependencies | (none) | — | Header-only; no .c |

**Rust design:** Use `Fn` trait or enum for action functions. `thinker_t` becomes a doubly-linked node with `prev`, `next`, and a `think_t` callback. Consider `thinker_t` as a struct containing `Option<Box<dyn FnMut()>>` or similar; C uses raw function pointers with `void*` context passed via mobj.

**Key types:**
```c
typedef void (*actionf_v)();
typedef void (*actionf_p1)(void*);
typedef void (*actionf_p2)(void*, void*);
typedef union { ... } actionf_t;
typedef struct thinker_s { prev, next, function; } thinker_t;
```

### 1.2 d_event

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| Types | d_event.h | `src/d_event.rs` | `evtype_t`, `event_t`, `buttoncode_t`, `buttoncode2_t` |
| API | d_event.c | same file | `D_PostEvent`, `D_PopEvent` |
| Dependencies | doomtype | doomtype | Boolean |

**Key API:**
- `D_PostEvent(ev: &event_t)` – push event to ring buffer
- `D_PopEvent() -> Option<&event_t>` – pop next event

**Constants:** MAXEVENTS = 64

---

## Phase 2: Info & Items

Large data tables and weapon definitions. Required for mobj spawning, state machine, and weapon logic.

### 2.1 info

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| Enums | info.h | `src/info.rs` or `src/info/` | `spritenum_t`, `statenum_t`, `mobjtype_t` (100+ each) |
| Structs | info.h | same | `state_t`, `mobjinfo_t` |
| Data | info.c | same | `states[]`, `mobjinfo[]` – large static arrays |
| Dependencies | d_think | d_think | actionf_t for state_t.action |

**Complexity:** info.h + info.c are ~1300+ lines. Consider splitting:
- `info/types.rs` – enums, structs
- `info/tables.rs` – `states`, `mobjinfo` (or generated)

**state_t fields:** sprite, frame, tics, action, nextstate, misc1, misc2  
**mobjinfo_t fields:** doomednum, spawnstate, spawnhealth, seestate, seesound, reactiontime, attacksound, painstate, painchance, pain sound, meleestate, missilestate, deathstate, xdeathstate, deathsound, speed, radius, height, mass, damage, activesound, flags, raisestate

**Stub strategy:** Start with minimal `mobjinfo_t` and `state_t` with only fields needed for rendering (sprite, frame). Extend as p_mobj, p_enemy need more.

### 2.2 d_items

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| Struct | d_items.h | `src/d_items.rs` | `weaponinfo_t` |
| Data | d_items.c | same | `weaponinfo[NUMWEAPONS]` |
| Dependencies | doomdef | doomdef | ammotype_t, weapontype_t |

**weaponinfo_t:** ammo, upstate, downstate, readystate, atkstate, flashstate

---

## Phase 3: Timer & Doomdef Extension

### 3.1 i_timer

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| API | i_timer.h/c | `src/i_timer.rs` | `I_GetTime`, `I_GetTimeMS`, `I_Sleep`, `I_InitTimer`, `I_WaitVBL` |
| Constants | i_timer.h | same | TICRATE = 35 |
| Dependencies | (platform) | i_system? | Platform-specific impl |

**Stub strategy:** `I_GetTime` returns monotonic tick count; `I_InitTimer` no-op; `I_Sleep` uses `std::thread::sleep`.

### 3.2 doomdef extension

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| Types | doomdef.h | doomdef.rs | `gameaction_t`, `gamestate_t` (if not present) |
| Enums | doomdef.h | same | `card_t`, `weapontype_t`, `ammotype_t`, `powertype_t` |

Check existing `doomdef.rs`; add `gameaction_t`, `gamestate_t` if missing.

---

## Phase 4: Game Loop

### 4.1 d_main

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| API | d_main.h/c | `src/d_main.rs` | `D_ProcessEvents`, `D_Display`, `D_Shutdown`, `D_PageTicker`, `D_PageDrawer`, `D_AdvanceDemo`, `D_StartTitle` |
| Globals | d_main.h | same | `gameaction` |
| Dependencies | doomdef | doomdef | gameaction_t |

**Note:** d_main.c also contains `D_DoomMain` – the true entry point. May defer full D_DoomMain; start with stubs for `D_ProcessEvents`, etc. **D_Display** ✅ dispatches by gamestate (Level→ST/R_Render/AM/HU, Intermission→WI, Finale→F_Drawer, DemoScreen→D_PageDrawer), then M_Drawer, I_FinishUpdate. **D_Shutdown** ✅ calls I_EndoomFromWad on exit.

### 4.2 d_loop

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| Types | d_loop.h | `src/d_loop.rs` | `loop_interface_t`, `netgame_startup_callback_t` |
| API | d_loop.h/c | same | `D_RegisterLoopCallbacks`, `TryRunTics`, `D_StartGameLoop`, `NetUpdate`, `D_InitNetGame`, `D_StartNetGame` |
| Globals | d_loop.h | same | `singletics`, `gametic`, `ticdup` |
| Dependencies | net_defs, d_ticcmd | (stub net) | ticcmd_t, net types |

**loop_interface_t:** ProcessEvents, BuildTiccmd, RunTic, RunMenu – callbacks for the main loop.

**Stub strategy:** Single-player only: `singletics = true`, no net. `TryRunTics` runs `RunTic` with local ticcmd.

### 4.3 g_game

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| API | g_game.h/c | `src/g_game.rs` | G_InitNew, G_DeferedInitNew, G_LoadGame, G_SaveGame, G_Ticker, G_Responder, G_BuildTiccmd, G_ExitLevel, etc. |
| Dependencies | doomdef, d_event, d_ticcmd, p_setup, p_tick, doomstat, s_sound, ... | many | Core game logic |

**Key functions:**
- `G_Ticker` – calls `P_Ticker` (or `F_Ticker` when GAMESTATE=Finale), updates level ✅
- `G_Responder` – handles events (keys, menu); dispatches to `F_Responder` when in Finale ✅
- `G_BuildTiccmd` – builds ticcmd from input – stub ✅
- `G_PlayerReborn` – reset player for respawn ✅
- `G_InitNew` – starts new game, calls `P_SetupLevel`

**Complexity:** g_game.c is large (~1500+ lines). Implement incrementally: start with `G_Ticker` (calls P_Ticker), `G_Responder` (stub), `G_BuildTiccmd` (stub).

---

## Phase 5: Optional / Deferred

| Module | Purpose | When |
|--------|---------|------|
| Blockmap in p_setup | P_BlockLinesIterator, P_BlockThingsIterator | ✅ Done |
| deh_* | DeHackEd | Stubs done; full impl when mod support needed |
| i_input | Keyboard/mouse → events | When D_ProcessEvents needs real input |
| z_zone | Fix purge/corruption bug (base=null) | Blocks example_render_scene ⏳ |

---

## Implementation Order (Recommended)

1. **d_think** – thinker_t, actionf_t (enables mobj_t.function, p_tick)
2. **d_event** – event queue (enables G_Responder stub)
3. **doomdef extension** – gameaction_t, gamestate_t, ammotype_t, weapontype_t
4. **d_items** – weaponinfo_t (enables p_pspr weapon states)
5. **info** – minimal mobjinfo_t, state_t (enables p_mobj spawn, p_enemy)
6. **i_timer** – I_GetTime stub (enables d_loop timing)
7. **d_main** – D_ProcessEvents, gameaction stub
8. **d_loop** – TryRunTics, loop_interface_t (single-player)
9. **g_game** – G_Ticker → P_Ticker, G_Responder stub, G_BuildTiccmd stub

---

## Integration with doom_rust

- Add new modules to `src/lib.rs`
- **d_think:** p_mobj extends Mobj to include thinker_t linkage; p_tick uses thinker_t
- **d_event:** g_game::G_Responder consumes events
- **info:** p_mobj uses mobjinfo_t for spawn; p_enemy uses state_t, mobjinfo_t
- **d_items:** p_pspr uses weaponinfo_t
- **g_game:** Main loop calls G_Ticker, G_Responder

---

## Summary

| Phase | Modules | Unblocks |
|-------|---------|----------|
| 1 | d_think, d_event | mobj thinker list, input events |
| 2 | info, d_items | mobj spawning, weapon states |
| 3 | i_timer, doomdef | Timing, game state |
| 4 | d_main, d_loop, g_game | Game loop, P_Ticker, G_Responder |

**Estimated effort:** Phase 1–2 are manageable (1–2 days each). Phase 3 is small. Phase 4 (g_game) is the largest; stub aggressively and grow incrementally.

See also: `PLAYER_TRANSLATION_PLAN.md`, `C_TO_RUST_OVERVIEW.md`

---

## Conversion Plan: d_main, d_loop, g_game (Full Implementation)

Plan to complete the game core modules: D_DoomMain/D_DoomLoop, TryRunTics/gameaction dispatch, G_InitNew, G_DoLoadLevel, G_DoSaveGame, G_DoLoadGame.

### Phase A: Game Action Dispatch (d_main + d_loop)

**Goal:** Main loop checks `gameaction` and dispatches to G_DoLoadLevel, G_DoNewGame, G_DoLoadGame, G_DoSaveGame.

| Step | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| A.1 | D_DoomLoop gameaction check | d_main.rs | Before TryRunTics, if gameaction != Nothing: match and call g_do_load_level, g_do_new_game, g_do_load_game, g_do_save_game |
| A.2 | G_DoNewGame | g_game.rs | Resets playeringame[1..3], calls G_InitNew(d_skill, d_episode, d_map) |
| A.3 | d_skill, d_episode, d_map | g_game.rs | Globals set by menu; G_DeferedInitNew stores to these, G_DoNewGame reads |

**Dependencies:** G_InitNew, G_DoLoadLevel (can stub initially).

---

### Phase B: G_InitNew (g_game)

**Goal:** Start new game – set gameskill, gameepisode, gamemap, reset players, call P_SetupLevel.

| Step | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| B.1 | G_InitNew(skill, episode, map) | g_game.rs | Validate episode/map for gamemode; set gameskill, gameepisode, gamemap |
| B.2 | Player reset | g_game.rs | For each player: reset health, ammo, weapons, frags, etc. |
| B.3 | P_SetupLevel call | g_game.rs | p_setup::p_load_level(episode, map, 0, skill) – already exists as p_load_level |
| B.4 | Sky setup | g_game.rs | R_FlatNumForName(SKYFLATNAME), R_TextureNumForName(SKY1/SKY2/SKY3) for commercial |

**Dependencies:** p_setup::p_load_level, r_data (R_FlatNumForName, R_TextureNumForName), r_sky.

---

### Phase C: G_DoLoadLevel (g_game)

**Goal:** Load a level – set sky, gamestate=Level, reset dead players, call P_SetupLevel.

| Step | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| C.1 | Sky flat/texture | g_game.rs | skyflatnum = R_FlatNumForName(SKYFLATNAME); skytexture for commercial |
| C.2 | levelstarttic | g_game.rs | levelstarttic = gametic |
| C.3 | Player state reset | g_game.rs | Dead players → Reborn; clear frags; turbodetected = false |
| C.4 | P_SetupLevel | g_game.rs | p_load_level(gameepisode, gamemap, 0, gameskill) |
| C.5 | Input clear | g_game.rs | Clear gamekeydown, mouse, joy; sendpause, sendsave, paused = false |

**Dependencies:** Same as Phase B.

---

### Phase D: Save/Load (g_game + p_saveg)

**Goal:** G_DoSaveGame, G_DoLoadGame using p_saveg archive functions.

| Step | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| D.1 | P_ArchivePlayers, P_UnArchivePlayers | p_saveg.rs | Serialize/deserialize PLAYERS, doomstat globals |
| D.2 | P_ArchiveWorld, P_UnArchiveWorld | p_saveg.rs | Sectors, lines, sides, etc. |
| D.3 | P_ArchiveThinkers, P_UnArchiveThinkers | p_saveg.rs | Mobj, ceiling/floor/plat thinkers |
| D.4 | P_ArchiveSpecials, P_UnArchiveSpecials | p_saveg.rs | Sector specials, line specials |
| D.5 | P_WriteSaveGameHeader, P_ReadSaveGameHeader | p_saveg.rs | Version, description, episode/map |
| D.6 | G_DoSaveGame | g_game.rs | Open file, P_WriteSaveGameHeader, P_Archive*, P_WriteSaveGameEOF |
| D.7 | G_DoLoadGame | g_game.rs | Open file, P_ReadSaveGameHeader, G_InitNew, P_UnArchive* |

**Dependencies:** p_saveg (stub exists), z_zone (for thinker iteration), p_setup, p_floor, p_ceilng, p_doors, p_plats, p_lights, p_switch, p_inter.

---

### Phase E: D_DoomMain (d_main)

**Goal:** Full startup – Z_Init, config, WAD loading, R_Init, etc.

| Step | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| E.1 | Z_Init | d_main | Zone allocator init |
| E.2 | M_SetConfigDir, M_LoadDefaults | d_main | Config load (m_config) |
| E.3 | D_FindIWAD, D_AddFile | d_main | Find IWAD, load WAD |
| E.4 | W_CheckCorrectIWAD, D_IdentifyVersion | d_main | Validate IWAD, set gamemode |
| E.5 | W_ParseCommandLine | d_main | Load -file PWADs |
| E.6 | V_Init, R_Init | d_main | Video, rendering init |
| E.7 | S_Init, M_Init, etc. | d_main | Sound, menu, controls |
| E.8 | G_DeferedInitNew or D_StartTitle | d_main | Start game or title/demo |

**Dependencies:** z_zone, w_wad, m_config, m_controls, m_menu, v_video, rendering::r_init, s_sound.

---

### Phase F: G_BuildTiccmd (g_game)

**Goal:** Build ticcmd from keyboard/mouse/joystick input.

| Step | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| F.1 | gamekeydown, key_* | g_game.rs or input | Key state array; m_controls key bindings |
| F.2 | mousex, mousey, mousebuttons | g_game.rs | From i_input (stub → platform) |
| F.3 | joyxmove, joyymove, joybuttons | g_game.rs | From i_joystick (stub) |
| F.4 | forwardmove, sidemove, angleturn | g_game.rs | Lookup tables; speed from key_speed |
| F.5 | Strafe, turn, forward, side | g_game.rs | G_BuildTiccmd logic |
| F.6 | BT_ATTACK, BT_USE, weapon keys | g_game.rs | Button encoding |

**Dependencies:** i_input (keyboard/mouse), i_joystick, m_controls (key bindings), hu_stuff (HU_dequeueChatChar).

---

### Implementation Order

1. **Phase A** – Game action dispatch ✅ (unblocks menu New Game → level load)
2. **Phase B** – G_InitNew ✅ (enables new game start)
3. **Phase C** – G_DoLoadLevel ✅ (enables level transitions, idclev)
4. **Phase F** – G_BuildTiccmd ✅ (keyboard + mouse movement, attack/use, strafe, speed, sendpause/sendsave)
5. **Phase D** – Save/load ✅ (file I/O, header, G_DoSaveGame/G_DoLoadGame; P_ArchivePlayers, P_ArchiveWorld, P_ArchiveThinkers; P_ArchiveSpecials writes tc_endspecials only)
6. **Phase E** – D_DoomMain ✅ (full startup: config, IWAD, WAD, video, rendering, menu, loop)
