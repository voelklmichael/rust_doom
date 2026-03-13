# UI/HUD Translation Plan

Plan for porting the Doom UI and HUD C modules to Rust. All files go into the `ui_hud` module. Each logical unit combines its `.h` (public API) and `.c` (private implementation) into a single `.rs` file.

**Source:** `doomgeneric/doomgeneric/m_*.c`, `hu_*.c`, `st_*.c`, `wi_*.c` and corresponding `.h` files

**Current status:** cheat, st_lib, hu_lib full; hu_stuff (HU_Init, HU_Start, HU_Drawer, HU_Erase, HU_Ticker); st_stuff (ST_Init, ST_Start, ST_Drawer, ST_Ticker, ST_Responder, w_health/w_armor); wi_stuff (WI_Start, WI_Ticker, WI_Drawer, WI_End, INTERPIC/WIMAP); config (M_LoadDefaults, variable store, M_SetVariable, M_GetIntVariable); menu (M_Init syncs from config, m_set_screenblocks, m_set_detail_level); v_video, r_draw; controls stubs.

---

## Module Structure

```
src/ui_hud/
├── mod.rs           # Re-exports, module organization
├── menu.rs          # m_menu.h + m_menu.c  (~2125 lines)
├── controls.rs      # m_controls.h + m_controls.c  (~398 lines)
├── cheat.rs         # m_cheat.h + m_cheat.c  (~89 lines)
├── config.rs        # m_config.h + m_config.c  (~2128 lines)
├── hu_stuff.rs      # hu_stuff.h + hu_stuff.c  (~641 lines)
├── hu_lib.rs        # hu_lib.h + hu_lib.c  (~347 lines)
├── st_stuff.rs      # st_stuff.h + st_stuff.c  (~1416 lines)
├── st_lib.rs        # st_lib.h + st_lib.c  (~284 lines)
└── wi_stuff.rs      # wi_stuff.h + wi_stuff.c  (~1829 lines)
```

**Total:** 9 submodules, ~9,257 lines of C to port.

---

## Public vs Private Convention

For each `*.rs` file:

| Section | C Source | Rust Visibility |
|---------|----------|-----------------|
| **Public** | Content from `*.h` (declarations, types, constants) | `pub fn`, `pub struct`, `pub const`, `pub type` |
| **Private** | Content from `*.c` (implementation) | `fn`, `struct` (no `pub`) |

---

## File Inventory

### 1. menu.rs — m_menu.h + m_menu.c

| C Header | C Implementation | Public API |
|----------|------------------|------------|
| m_menu.h | m_menu.c | `M_Responder`, `M_Ticker`, `M_Drawer`, `M_Init`, `M_StartControlPanel`, `detailLevel`, `screenblocks` |

**Dependencies:** d_event, doomdef, doomkeys, dstrings, d_main, deh_main, i_swap, i_system, i_timer, i_video, m_misc, v_video, w_wad, z_zone, r_local, hu_stuff, g_game, m_argv, m_controls, p_saveg, s_sound, doomstat, sounds.

---

### 2. controls.rs — m_controls.h + m_controls.c

| C Header | C Implementation | Public API |
|----------|------------------|------------|
| m_controls.h | m_controls.c | Key globals (`key_right`, `key_left`, `key_fire`, etc.), mouse/joy globals, `M_BindBaseControls`, `M_BindHereticControls`, `M_BindHexenControls`, `M_BindStrifeControls`, `M_BindWeaponControls`, `M_BindMapControls`, `M_BindMenuControls`, `M_BindChatControls`, `M_ApplyPlatformDefaults` |

**Dependencies:** doomkeys, doomfeatures (for game-specific bindings). Mostly data/globals.

**Note:** m_controls.h is large (~170 lines) with many `extern int` key variables. In Rust, consider a `Controls` struct or `lazy_static` / `AtomicI32` for key bindings.

---

### 3. cheat.rs — m_cheat.h + m_cheat.c

| C Header | C Implementation | Public API |
|----------|------------------|------------|
| m_cheat.h | m_cheat.c | `CHEAT` macro, `MAX_CHEAT_LEN`, `MAX_CHEAT_PARAMS`, `cheatseq_t`, `cht_CheckCheat`, `cht_GetParam` |

**Dependencies:** doomtype only. Smallest module; good starting point.

---

### 4. config.rs — m_config.h + m_config.c

| C Header | C Implementation | Public API |
|----------|------------------|------------|
| m_config.h | m_config.c | `M_LoadDefaults`, `M_SaveDefaults`, `M_SaveDefaultsAlternate`, `M_SetConfigDir`, `M_BindVariable`, `M_SetVariable`, `M_GetIntVariable`, `M_GetStrVariable`, `M_GetFloatVariable`, `M_SetConfigFilenames`, `M_GetSaveGameDir`, `configdir` |

**Dependencies:** config, doomtype, doomkeys, doomfeatures, i_system, m_argv, m_misc, z_zone. Large (~2128 lines); config file parsing, variable binding.

---

### 5. hu_lib.rs — hu_lib.h + hu_lib.c

| C Header | C Implementation | Public API |
|----------|------------------|------------|
| hu_lib.h | hu_lib.c | `HU_CHARERASE`, `HU_MAXLINES`, `HU_MAXLINELENGTH`, `hu_textline_t`, `hu_stext_t`, `hu_itext_t`, `HUlib_init`, `HUlib_clearTextLine`, `HUlib_initTextLine`, `HUlib_addCharToTextLine`, `HUlib_delCharFromTextLine`, `HUlib_drawTextLine`, `HUlib_eraseTextLine`, `HUlib_initSText`, `HUlib_addLineToSText`, `HUlib_addMessageToSText`, `HUlib_drawSText`, `HUlib_eraseSText`, `HUlib_initIText`, `HUlib_delCharFromIText`, `HUlib_eraseLineFromIText`, `HUlib_resetIText`, `HUlib_addPrefixToIText`, `HUlib_keyInIText`, `HUlib_drawIText`, `HUlib_eraseIText` |

**Dependencies:** r_defs (patch_t), doomdef, doomkeys, v_video, i_swap, r_local, r_draw.

---

### 6. hu_stuff.rs — hu_stuff.h + hu_stuff.c

| C Header | C Implementation | Public API |
|----------|------------------|------------|
| hu_stuff.h | hu_stuff.c | `HU_FONTSTART`, `HU_FONTEND`, `HU_FONTSIZE`, `HU_BROADCAST`, `HU_MSGX`, `HU_MSGY`, `HU_MSGWIDTH`, `HU_MSGHEIGHT`, `HU_MSGTIMEOUT`, `HU_Init`, `HU_Start`, `HU_Responder`, `HU_Ticker`, `HU_Drawer`, `HU_dequeueChatChar`, `HU_Erase`, `chat_macros` |

**Dependencies:** doomdef, doomkeys, z_zone, deh_main, i_swap, i_video, hu_lib, m_controls, m_misc, w_wad, s_sound, doomstat, dstrings, sounds.

---

### 7. st_lib.rs — st_lib.h + st_lib.c

| C Header | C Implementation | Public API |
|----------|------------------|------------|
| st_lib.h | st_lib.c | `st_number_t`, `st_percent_t`, `st_multicon_t`, `st_binicon_t`, `STlib_init`, `STlib_initNum`, `STlib_updateNum`, `STlib_initPercent`, `STlib_updatePercent`, `STlib_initMultIcon`, `STlib_updateMultIcon`, `STlib_initBinIcon`, `STlib_updateBinIcon` |

**Dependencies:** r_defs (patch_t), doomtype.

---

### 8. st_stuff.rs — st_stuff.h + st_stuff.c

| C Header | C Implementation | Public API |
|----------|------------------|------------|
| st_stuff.h | st_stuff.c | `ST_HEIGHT`, `ST_WIDTH`, `ST_Y`, `ST_Responder`, `ST_Ticker`, `ST_Drawer`, `ST_Start`, `ST_Init`, `st_stateenum_t`, `st_chatstateenum_t`, `st_backing_screen`, `cheat_mus`, `cheat_god`, `cheat_ammo`, etc. |

**Dependencies:** i_system, i_video, z_zone, m_misc, m_random, w_wad, deh_main, deh_misc, doomdef, doomkeys, g_game, st_lib, r_local, p_local, p_inter, am_map, m_cheat, s_sound, v_video, doomstat, dstrings, sounds.

---

### 9. wi_stuff.rs — wi_stuff.h + wi_stuff.c

| C Header | C Implementation | Public API |
|----------|------------------|------------|
| wi_stuff.h | wi_stuff.c | `stateenum_t` (NoState, StatCount, ShowNextLoc), `WI_Ticker`, `WI_Drawer`, `WI_Start`, `WI_End` |

**Dependencies:** doomdef (wbstartstruct_t), z_zone, m_misc, m_random, deh_main, i_swap, i_system, w_wad, g_game, r_local, s_sound, doomstat, sounds, v_video.

**Note:** `wbstartstruct_t` is in doomstat (d_net.h / d_player.h in C). Rust has `WbStartStruct` with `epsd`, `last`, `next`, `pnum`, `plyr[]` and `WbPlayerStruct` in doomstat.

---

## Dependency Graph

```
                    ┌─────────────┐
                    │   m_cheat   │  (minimal)
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐     ┌─────────────┐
                    │  m_controls │     │  m_config   │
                    └──────┬──────┘     └──────┬──────┘
                           │                   │
              ┌────────────┼───────────────────┼────────────┐
              │            │                   │            │
       ┌──────▼──────┐     │            ┌──────▼──────┐     │
       │   hu_lib    │     │            │   m_menu   │     │
       └──────┬──────┘     │            └─────────────┘     │
              │            │                                 │
       ┌──────▼──────┐     │            ┌─────────────┐     │
       │  hu_stuff   │◄────┘            │  st_lib     │     │
       └─────────────┘                  └──────┬──────┘     │
              │                                 │            │
              │                          ┌──────▼──────┐     │
              └─────────────────────────►│  st_stuff   │◄────┘
                                         └──────┬──────┘
                                                │
                                         ┌──────▼──────┐
                                         │  wi_stuff   │
                                         └─────────────┘
```

---

## Prerequisites (outside ui_hud)

These must exist or be stubbed before full UI/HUD port:

| Module | Status in doom_rust | Notes |
|--------|---------------------|-------|
| d_event | ✅ Done | Event, EvType, D_PostEvent, D_PopEvent |
| doomdef | ✅ Done | SCREENWIDTH, SCREENHEIGHT, TICRATE, gamestate_t, gameaction_t |
| doomkeys | ✅ Done | KEY_* constants |
| doomtype | ✅ Done | Boolean |
| doomstat | Partial | WbStartStruct (epsd, last, next, pnum, plyr), WbPlayerStruct; gameepisode, gamemap, etc. |
| deh_main, deh_misc | ✅ Stubs | DeHackEd |
| i_swap | ✅ Done | SHORT, LONG |
| i_system | Stub | I_Error, I_ZoneBase |
| i_timer | ✅ Done | TICRATE |
| i_video | Not started | Screen buffer, mode set |
| m_misc | ✅ Done | M_StringCopy, etc. |
| v_video | ✅ Done | V_DrawPatch, V_DrawPatchDirect, V_CopyRect, V_UseBuffer, V_RestoreBuffer, screen buffer |
| w_wad | ✅ Done | W_CacheLumpName |
| z_zone | Partial | Zone allocator (has corruption bug) |
| r_defs | ✅ Done | patch_t, etc. |
| r_local | Partial | viewwindowx, etc. |
| r_draw | ✅ Done | Column drawing, R_VideoErase |
| g_game | Partial | G_Responder, game state |
| p_saveg | Stub | Savegame API |
| p_inter | Stub | P_TouchSpecialThing, etc. |
| am_map | Not started | automapactive, AM_* |
| s_sound | ✅ Done | S_StartSound, etc. |
| dstrings | ✅ Done | SAVEGAMENAME, etc. |
| sounds | ✅ Done | S_sfx, S_music |

---

## Implementation Order (Recommended)

Translate in dependency order. Start with stubs for missing deps.

| Phase | File | Deps | Effort | Notes |
|-------|------|-----|--------|-------|
| 1 | **cheat.rs** | doomtype | Small | Minimal; `cheatseq_t`, `cht_CheckCheat`, `cht_GetParam` |
| 2 | **controls.rs** | doomkeys | Small | Key globals; stub M_Bind* as no-ops initially |
| 3 | **st_lib.rs** | r_defs, doomtype | Small | Widget structs, STlib_* |
| 4 | **hu_lib.rs** | r_defs, v_video, r_draw | Medium | Font widgets, HUlib_* |
| 5 | **config.rs** | m_argv, m_misc, z_zone, etc. | Large | Config parsing; stub M_LoadDefaults/M_SaveDefaults |
| 6 | **hu_stuff.rs** | hu_lib, m_controls, doomstat | Medium | HUD init, responder, ticker, drawer |
| 7 | **st_stuff.rs** | st_lib, m_cheat, p_inter, am_map | Large | Status bar; many cheat sequences |
| 8 | **wi_stuff.rs** | doomstat, g_game, v_video | Large | Intermission screen |
| 9 | **menu.rs** | hu_stuff, m_controls, m_config, g_game | Large | Main menu; most complex |

---

## mod.rs Structure

```rust
//! UI and HUD: menus, status bar, heads-up display, intermission.

pub mod cheat;
pub mod config;
pub mod controls;
pub mod hu_lib;
pub mod hu_stuff;
pub mod menu;
pub mod st_lib;
pub mod st_stuff;
pub mod wi_stuff;

// Re-export commonly used public API
pub use cheat::{cheatseq_t, cht_CheckCheat, cht_GetParam};
pub use config::{M_LoadDefaults, M_SaveDefaults, M_BindVariable, /* ... */};
pub use controls::{key_fire, key_use, M_BindBaseControls, /* ... */};
pub use hu_stuff::{HU_Init, HU_Start, HU_Responder, HU_Ticker, HU_Drawer};
pub use st_stuff::{ST_Init, ST_Start, ST_Responder, ST_Ticker, ST_Drawer};
pub use wi_stuff::{WI_Start, WI_Ticker, WI_Drawer, WI_End};
pub use menu::{M_Init, M_Responder, M_Ticker, M_Drawer, M_StartControlPanel};
```

---

## Integration with doom_rust

1. Add `pub mod ui_hud;` to `src/lib.rs`.
2. **d_main / d_loop:** Call `M_Responder`, `M_Ticker`, `M_Drawer` when in menu state; `ST_Responder`, `ST_Ticker`, `ST_Drawer` when in level; `WI_Ticker`, `WI_Drawer` when in intermission.
3. **g_game:** Uses `ST_Start` when player spawns; `WI_Start` when level complete.
4. **i_input:** Feeds events to `M_Responder` / `ST_Responder` / `HU_Responder` via `D_PostEvent`.

---

## Implementation Progress

| Module | Status | Implemented |
|--------|--------|-------------|
| **cheat.rs** | ✅ Full | CheatSeq, cht_CheckCheat, cht_GetParam |
| **controls.rs** | Partial | Key globals; M_BindBaseControls, M_BindWeaponControls, M_BindMapControls, M_BindMenuControls; m_sync_controls_to_config |
| **st_lib.rs** | ✅ Full | StNumber, StPercent, StMultIcon, StBinIcon; stlib_init, stlib_update_* |
| **hu_lib.rs** | ✅ Full | HuTextline, HuStext, HuItext; hulib_draw_*, hulib_erase_*, text manipulation |
| **config.rs** | Partial | M_LoadDefaults (file I/O, -config/-extraconfig); M_SaveDefaults; M_SetConfigFilenames; variable store; M_BindVariable; m_set_variable → m_update_control_from_config (key/control sync) |
| **hu_stuff.rs** | ✅ Full | HU_Init, HU_Start, HU_Drawer, HU_Erase, HU_Ticker, font loading |
| **st_stuff.rs** | Partial | ST_Init, ST_Start, ST_Drawer, ST_Ticker, ST_Responder; w_health, w_armor, w_ready, w_arms, w_armsbg, w_faces, w_keyboxes; cheats: god, idfa, idkfa, noclip, idmus, idclev, idbehold*, idchoppers; palette effects (fixedcolormap update for damage/bonus/invuln flash); Player wired |
| **wi_stuff.rs** | Partial | WI_Start, WI_Ticker, WI_Drawer, WI_End; stats; animated stat counting; par time; wi_set_accelerate; animated background (WIA*); "Finished!" / "Entering" + level names; WILV/CWILV patches |
| **menu.rs** | Partial | M_Init, M_StartControlPanel; M_Responder; M_Drawer (main/episode/newgame/options/sound/load/save, thermo, skull); Load/Save borders + strings + slot activation (g_defered_load_game, g_defered_save_game); m_set_screenblocks, m_set_detail_level |

## Stub Strategy (Historical)

For initial scaffold:

1. **cheat.rs** — Full impl (small). ✅
2. **controls.rs** — Struct with key values; `M_Bind*` no-ops.
3. **st_lib.rs** — Types + impls. ✅
4. **hu_lib.rs** — Types + impls. ✅
5. **config.rs** — Variable store; M_LoadDefaults sets defaults. ✅
6. **hu_stuff.rs** — Full impl. ✅
7. **st_stuff.rs** — Partial: load graphics, w_health/w_armor, cheat handling. ✅
8. **wi_stuff.rs** — Partial: load background, draw. ✅
9. **menu.rs** — M_Init syncs from config. ✅

---

## Effort Estimate

| Phase | Modules | Effort |
|-------|---------|--------|
| Stub scaffold | All 9 | 1–2 days |
| cheat + controls + st_lib | 3 | 0.5 day |
| hu_lib + hu_stuff | 2 | 1–2 days |
| st_stuff | 1 | 1–2 days |
| config | 1 | 2–3 days |
| wi_stuff | 1 | 1–2 days |
| menu | 1 | 2–3 days |

**Total (full impl):** ~2–3 weeks. **Stub scaffold:** 1–2 days.

---

## Remaining Work

| Module | Remaining |
|--------|-----------|
| **st_stuff** | — |
| **wi_stuff** | — |
| **config** | — |
| **menu** | — |
| **controls** | — |

**Done (this session):** config: M_BindVariable pointer binding via m_set_variable → m_update_control_from_config; controls::m_update_control_from_config syncs key/mouse/joy vars to globals when config loads; menu: save string input for new saves (HuItext), Read This screen (HELP1 lump, any key closes); controls: M_BindHereticControls, M_BindHexenControls, M_BindStrifeControls; full sync (Heretic/Hexen/Strife globals, menu keys, mouse/joy extras, dclick_use, key_multi_msgplayer);

---

## Summary

| Category | Count |
|----------|-------|
| Submodules | 9 |
| C lines (approx) | ~9,257 |
| Merge pattern | .h = public, .c = private per file |

See also: `PLAYER_TRANSLATION_PLAN.md`, `GAME_CORE_TRANSLATION_PLAN.md`, `C_TO_RUST_OVERVIEW.md`
