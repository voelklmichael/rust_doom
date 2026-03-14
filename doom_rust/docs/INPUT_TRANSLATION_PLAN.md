# Input / Platform I/O Translation Plan

Plan for porting the system-specific I/O modules: timer, video, input, joystick, scaling, ENDOOM, CD music.

**Source:** `doomgeneric/doomgeneric/i_*.c` and `i_*.h`

**Current state:** `i_timer` done. `input` module has all submodules: `i_video`, `i_input`, `i_joystick`, `i_scale`, `i_endoom`, `i_cdmus`. `i_endoom` implemented (stdout); others stubbed.

---

## Module Overview

| C Module   | Purpose                    | Rust Target              | Status      |
|------------|----------------------------|--------------------------|-------------|
| i_timer    | Timing (ticks, sleep)      | `src/i_timer.rs`         | ✅ Done     |
| i_video    | Video init, palette, blit  | `src/input/i_video.rs`   | Stub        |
| i_input    | Keyboard/mouse → events    | `src/input/i_input.rs`   | Stub        |
| i_joystick | Gamepad → ticcmd           | `src/input/i_joystick.rs`| Stub        |
| i_scale    | Resolution scaling         | `src/input/i_scale.rs`   | Stub        |
| i_endoom   | ENDOOM lump on exit        | `src/input/i_endoom.rs`   | ✅ Done     |
| i_cdmus    | CD music playback          | `src/input/i_cdmus.rs`   | Stub        |

---

## Dependency Graph

```
i_timer ──────────────────────────────────────────────────────┐
  │                                                             │
  └──► d_loop (TryRunTics timing), G_BuildTiccmd                │
                                                                 │
i_video ──────────────────────────────────────────────────────┤
  │                                                             │
  └──► I_VideoBuffer (→ v_video VIEWIMAGE), I_ReadScreen,       │
       I_SetPalette, I_UpdateNoBlit, I_FinishUpdate              │
                                                                 │
i_input ───────────────────────────────────────────────────────┤
  │                                                             │
  └──► D_PostEvent, doomkeys, m_controls, G_BuildTiccmd         │
                                                                 │
i_joystick ────────────────────────────────────────────────────┤
  │                                                             │
  └──► i_input (often integrated), G_BuildTiccmd                │
                                                                 │
i_scale ───────────────────────────────────────────────────────┤
  │                                                             │
  └──► i_video (resolution), aspect ratio                       │
                                                                 │
i_endoom ─────────────────────────────────────────────────────┤
  │                                                             │
  └──► w_wad (ENDOOM lump), shutdown path                       │
                                                                 │
i_cdmus ──────────────────────────────────────────────────────┘
  └──► s_sound (CD music fallback), optional
```

---

## Phase 1: i_timer ✅ Done

Already implemented in `src/i_timer.rs`:
- `i_get_time` – ticks since start
- `i_get_time_ms` – milliseconds
- `i_sleep` – sleep for ms
- `i_init_timer` – init
- `i_wait_vbl` – stub

---

## Phase 2: i_video

**Goal:** Video buffer, palette, screen blit. Core for display.

| Item        | C Source   | Rust Target              | Notes                                      |
|-------------|------------|--------------------------|--------------------------------------------|
| I_VideoBuffer | i_video.h | `rendering::VIEWIMAGE`   | Already provided by v_video                |
| I_ReadScreen  | i_video.c | `v_read_screen`         | ✅ Done in v_video                          |
| I_InitGraphics | i_video.c | `input::i_video::i_init_graphics` | Platform: create window, alloc buffer |
| I_SetPalette  | i_video.c | same                     | Apply 256-color palette to display         |
| I_UpdateNoBlit | i_video.c | same                    | Copy buffer to screen (no vsync)            |
| I_FinishUpdate | i_video.c | same                    | Present frame, vsync                        |
| I_SetWindowTitle | i_video.c | same                  | Window title                                |
| screen_mode_t | i_video.h | struct                   | width, height, InitMode, DrawScreen, poor_quality |

**Dependencies:** doomdef (SCREENWIDTH, SCREENHEIGHT), v_video, z_zone.

**Strategy:** Stub initially. Real impl needs winit/SDL/minifb or similar. v_video already provides SCREENS buffer; i_video would blit it to the platform window.

---

## Phase 3: i_input

**Goal:** Poll keyboard/mouse, post events to `D_PostEvent`.

| Item              | C Source   | Rust Target                    | Notes                          |
|-------------------|------------|--------------------------------|--------------------------------|
| I_InitInput       | i_input.c  | `input::i_input::i_init_input` | Platform init                  |
| I_ShutdownInput   | i_input.c  | same                           | Cleanup                        |
| I_GetEvent        | i_input.c  | (internal)                     | Poll, return Event             |
| at_to_doom[]      | i_input.c  | key map                        | Scancode → doom key            |
| vanilla_keyboard_mapping | i_input.c | config                     | Use vanilla vs. modern mapping  |
| shiftdown         | i_input.c  | state                          | Shift key state                |

**Dependencies:** d_event, doomkeys, m_controls, m_config.

**Strategy:** Platform-specific. Options: winit, sdl2, or stdio for testing. Call `d_post_event` with `EvType::KeyDown`/`KeyUp`/`Mouse`.

---

## Phase 4: i_joystick

**Goal:** Poll joystick/gamepad, feed into ticcmd or events.

| Item           | C Source    | Rust Target                      | Notes              |
|----------------|-------------|----------------------------------|--------------------|
| I_InitJoystick | i_joystick.c| `input::i_joystick::i_init_joystick` | Platform init  |
| I_ShutdownJoystick | i_joystick.c | same                        | Cleanup            |
| I_UpdateJoystick | i_joystick.c | same                        | Poll axes/buttons  |

**Dependencies:** d_ticcmd (Ticcmd), m_controls (bindings), i_input (often integrated).

**Strategy:** Stub. Real impl needs gilrs, sdl2, or platform API. Can feed into G_BuildTiccmd or post as events.

---

## Phase 5: i_scale

**Goal:** Resolution scaling, aspect ratio correction.

| Item              | C Source  | Rust Target                   | Notes                    |
|-------------------|-----------|-------------------------------|--------------------------|
| I_InitScale       | i_scale.c | `input::i_scale::i_init_scale`| Set up scaling           |
| I_SetScaleFactor  | i_scale.c | same                          | Scale factor             |
| SCREENWIDTH_4_3   | i_video.h | const                          | 256 for squash           |
| SCREENHEIGHT_4_3  | i_video.h | const                          | 240 for stretch          |

**Dependencies:** i_video, doomdef.

**Strategy:** Defer until i_video is in place. Often integrated with video driver.

---

## Phase 6: i_endoom ✅ Done

**Goal:** Display ENDOOM lump (80×25 text screen) on shutdown.

| Item       | C Source   | Rust Target                    | Notes                    |
|------------|------------|--------------------------------|--------------------------|
| I_Endoom   | i_endoom.c | `input::i_endoom::i_endoom`    | Display ENDOOM lump      |
| (helper)   | —          | `input::i_endoom::i_endoom_from_wad` | Load ENDOOM from WAD, print to stdout |

**Dependencies:** w_wad (ENDOOM lump), deh_string.

**Implementation:** Parses 80×25×2 byte IBM PC format; prints chars to stdout. `i_endoom_from_wad()` looks up ENDOOM lump and displays it.

---

## Phase 7: i_cdmus

**Goal:** CD music playback (legacy, optional).

| Item         | C Source   | Rust Target                     | Notes                |
|--------------|------------|---------------------------------|----------------------|
| I_InitCDMusic| i_cdmus.c  | `input::i_cdmus::i_init_cdmusic`| Init CD drive        |
| I_ShutdownCDMusic | i_cdmus.c | same                        | Cleanup              |
| I_PlayCDTrack| i_cdmus.c  | same                            | Play CD track        |

**Dependencies:** s_sound (fallback to MIDI when no CD).

**Strategy:** Stub. CD music is legacy; most ports use OPL/MIDI or digital music. Low priority.

---

## Implementation Order

1. **i_timer** – ✅ Done
2. **i_input** – ✅ Stub (i_init_input, i_shutdown_input)
3. **i_joystick** – ✅ Stub (i_init_joystick, i_shutdown_joystick, i_update_joystick)
4. **i_endoom** – ✅ Done (i_endoom, i_endoom_from_wad)
5. **i_video** – ✅ Stub (i_init_graphics, i_set_palette, i_finish_update, etc.)
6. **i_scale** – ✅ Stub (i_init_scale, i_reset_scale_tables)
7. **i_cdmus** – ✅ Stub (i_cdmus_init, i_cdmus_play, etc.)

---

## Integration

- Add `pub mod input;` to `src/lib.rs` ✅
- **d_loop:** Uses `i_timer::i_init_timer` ✅
- **d_main:** `D_ProcessEvents` would call `i_input` poll loop (when implemented)
- **G_BuildTiccmd:** Would read from `i_input` + `i_joystick` (when implemented)
- **Display loop:** Would call `i_video::I_FinishUpdate` after drawing (when implemented)

---

## Summary

| Phase | Module   | Effort | Status   |
|-------|----------|--------|----------|
| 1     | i_timer  | —      | ✅ Done  |
| 2     | i_video  | Large  | Stub     |
| 3     | i_input  | Medium | Stub     |
| 4     | i_joystick | Medium | Stub   |
| 5     | i_scale  | Small  | Stub     |
| 6     | i_endoom | Small  | ✅ Done  |
| 7     | i_cdmus  | Small  | Stub     |

See also: `C_TO_RUST_OVERVIEW.md`, `GAME_CORE_TRANSLATION_PLAN.md`, `UI_HUD_TRANSLATION_PLAN.md`
