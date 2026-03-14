# Next Phase Translation Plan

Plan for the next set of C modules to port. Focus: unblock player physics (p_map, p_sight), mobj spawning (info), and reduce dependency stubs (deh_*, doomkeys).

**Source:** `doomgeneric/doomgeneric/*.c` and `*.h`

**Current state:** Game loop (d_think, d_event, d_items, d_main, d_loop, g_game), dstrings, rendering, WAD, sound, zone are done. Player module has p_map, p_sight, p_mobj, blockmap, P_SpawnPlayer. example_render_scene blocked by zone allocator bug.

**Completed:** deh_* stubs, doomkeys, info (minimal types + tables: State, Mobjinfo, states(), MOBJINFO for MT_PLAYER, MT_POSSESSED, MT_TROOP, MT_SERGEANT, MT_HEAD), blockmap in p_setup (P_LoadBlockMap, P_GroupLines, sector.lines, blocklinks, line bbox), p_maputl (P_BlockLinesIterator, P_BlockThingsIterator, P_PathTraverse, P_SetThingPosition, P_UnsetThingPosition), p_map (P_CheckPosition, P_TryMove, P_TeleportMove, P_SlideMove), p_sight (P_CheckSight with REJECT+BSP), p_mobj (P_SpawnMobj, P_RemoveMobj, P_SetMobjState, P_ExplodeMissile, P_MobjThinker with P_XYMovement, P_ZMovement, state tic countdown), P_SpawnMapThing, P_LoadThings, **P_SpawnPlayer** (spawns MT_PLAYER at player starts 1–4), **G_PlayerReborn** (g_game), **player_t** (doomstat: mo, viewz, viewheight, extralight, fixedcolormap, playerstate, health), **P_SetupPsprites** (stub). Mobj extended with thinker, floorz, ceilingz, momx, momy, momz, bnext, bprev, sprev, subsector, radius, height, type, info, state, health, player ptr, etc. example_render_scene uses spawned player mobj for ViewPlayerStub.

**Known issues:** z_zone: base becomes null during Z_Malloc (zone corruption); WAD cache uses PU_STATIC workaround for purgable lumps (avoids Z_ChangeTag user requirement); r_data texture hash table fix (ptr::write_bytes size, hash key).

---

## Dependency Overview

```
info ──────────────────────────────────────────────────────────────┐
  │                                                                  │
  └──► mobjinfo_t, state_t, mobjtype_t, statenum_t                  │
       (p_mobj spawn, p_enemy, p_inter)                             │
                                                                     │
p_setup (blockmap) ─────────────────────────────────────────────────┤
  │                                                                  │
  └──► blockmap[], sector.lines[], P_BlockLinesIterator,            │
       P_BlockThingsIterator (p_maputl, p_map)                     │
                                                                     │
deh_main, deh_misc, deh_str (stubs) ───────────────────────────────┤
  │                                                                  │
  └──► Many modules: p_spec, p_switch, p_inter, p_doors,           │
       m_menu, st_stuff, r_data, s_sound, etc.                      │
                                                                     │
doomkeys ───────────────────────────────────────────────────────────┘
  └──► Key codes for i_input, m_menu, G_BuildTiccmd
```

---

## Phase 1: Info Module ✅ Done (minimal)

**Goal:** Enable mobj spawning from map things, state machine for p_enemy, p_inter.

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| Enums | info.h | `src/info/` or `src/info.rs` | spritenum_t (~150), statenum_t (~1000), mobjtype_t (~150) |
| Structs | info.h | same | state_t, mobjinfo_t |
| Data | info.c | same | states[], mobjinfo[] (~4600 lines) |
| Dependencies | d_think, sounds | d_think, sound | actionf_t, sfxenum_t |

**Complexity:** info.h ~1300 lines, info.c ~4600 lines. Mostly data.

**Strategy:**
1. **Minimal first:** Types + small subset of states/mobjinfo for player, imp, shotgun guy. Get p_mobj spawn working.
2. **Split:** `info/types.rs` (enums, structs), `info/tables.rs` (states, mobjinfo) or `info/doom.rs` for Doom 1 data.
3. **Generate:** Consider build script to parse info.c or extract from C at build time.
4. **Defer:** Full monster roster; add incrementally as p_enemy needs them.

**state_t:** sprite, frame, tics, action (actionf_v), nextstate, misc1, misc2  
**mobjinfo_t:** doomednum, spawnstate, spawnhealth, seestate, seesound, reactiontime, attacksound, painstate, painchance, painstate, meleestate, missilestate, deathstate, xdeathstate, deathsound, speed, radius, height, mass, damage, activesound, flags, raisestate

---

## Phase 2: Blockmap & p_maputl Extension ✅ Done

**Goal:** Enable p_map collision (P_CheckPosition, P_TryMove), line/thing iteration.

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| Blockmap build | p_setup.c | player/p_setup.rs | blockmap, blocklinks, bmapwidth, bmapheight, bmaporgx, bmaporgy |
| sector.lines | p_setup.c | rendering/defs | Sector needs lines[] for twoSided, getSector |
| P_BlockLinesIterator | p_maputl.c | player/p_maputl.rs | Iterate lines in blockmap cells |
| P_BlockThingsIterator | p_maputl.c | player/p_maputl.rs | Iterate things in blockmap cells |
| P_PathTraverse | p_maputl.c | player/p_maputl.rs | Traverse line/thing intercepts along path |
| P_SetThingPosition, P_UnsetThingPosition | p_maputl.c | player/p_maputl.rs | Update blocklinks when thing moves |

**Dependencies:** m_bbox (done), rendering defs (sector, line), player p_mobj.

**Blockmap format:** From MAP lump; grid of (bmapwidth × bmapheight) cells; each cell lists line indices and thing pointers.

---

## Phase 3: p_map & p_sight Implementation ✅ Done

**Goal:** Real collision and line-of-sight.

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| P_CheckPosition | p_map.c | player/p_map.rs | Check if position is valid (no solid lines/things) |
| P_TryMove | p_map.c | player/p_map.rs | Move mobj, slide on walls |
| P_SlideMove | p_map.c | player/p_map.rs | Slide along blocking line |
| P_TeleportMove | p_map.c | player/p_map.rs | Instant move (teleporter) |
| P_LineOpening | p_maputl.c | player/p_maputl.rs | Already done |
| P_CheckSight | p_sight.c | player/p_sight.rs | REJECT + BSP traversal (P_CrossBSPNode, P_CrossSubsector) ✅ |

**Dependencies:** blockmap, p_maputl (P_PathTraverse, P_BlockLinesIterator), reject matrix (from p_setup).

---

## Phase 4: DeHackEd Stubs ✅ Done

**Goal:** Remove compile-time dependency on deh_* so more modules can be ported. Many C modules include deh_main.h; provide no-op stubs.

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| deh_main | deh_main.h/c | `src/deh/` or stubs | DEH_AddStringReplacement, etc. – stub as no-op |
| deh_misc | deh_misc.h/c | same | DEH_snprintf, etc. – use std::fmt or no-op |
| deh_str | deh_str.h/c | same | DEH_String – return input unchanged |

**Strategy:** Create `src/deh/` with minimal stubs. Real DeHackEd support later.

---

## Phase 5: doomkeys & i_input (Optional)

**Goal:** Real keyboard/mouse input for G_BuildTiccmd, D_ProcessEvents.

| Item | C Source | Rust Target | Notes |
|------|----------|-------------|-------|
| doomkeys | doomkeys.h | `src/doomkeys.rs` | KEY_* constants (KEY_RIGHTARROW, KEY_ESCAPE, etc.) |
| i_input | i_input.c | `src/i_input.rs` | Poll keyboard/mouse, call D_PostEvent |

**Dependencies:** d_event, doomkeys. Platform-specific (SDL, winit, or stdio).

---

## Implementation Order (Recommended)

1. **deh_main, deh_misc, deh_str** – Stubs (unblocks p_spec, p_switch, p_doors, etc.)
2. **doomkeys** – Key constants (small, no .c)
3. **info** – Minimal: types + player/imp/shotgun mobjinfo + states. Extend later.
4. **Blockmap in p_setup** – Build blockmap, sector.lines from level data
5. **p_maputl** – P_BlockLinesIterator, P_BlockThingsIterator, P_PathTraverse, P_SetThingPosition, P_UnsetThingPosition
6. **p_map** – P_CheckPosition, P_TryMove, P_SlideMove
7. **p_sight** – P_CheckSight (REJECT + intercept traversal)
8. **p_mobj** – Extend with thinker, spawn from info (P_SpawnMobj) ✅
9. **P_SpawnPlayer** – Spawn player mobj at player starts, link to player_t ✅
10. **z_zone** – Fix zone allocator corruption (base null during purge) ⏳

---

## Effort Estimate

| Phase | Effort | Risk |
|-------|--------|------|
| Phase 1 (info) | Large (2–5 days) | Data-heavy; consider codegen |
| Phase 2 (blockmap) | Medium (1–2 days) | Well-defined format |
| Phase 3 (p_map, p_sight) | Medium (1–2 days) | Depends on Phase 2 |
| Phase 4 (deh stubs) | Small (0.5 day) | Straightforward |
| Phase 5 (doomkeys, i_input) | Small (0.5–1 day) | Platform-dependent |

---

## Out of Scope (Later)

- m_menu, hu_stuff, st_stuff, wi_stuff – UI/HUD (complete: all 9 modules done; see UI_HUD_TRANSLATION_PLAN.md)
- f_finale, f_wipe – End-game, screen wipe ✅ Done (game/f_finale.rs, game/f_wipe.rs)
- am_map – Automap (complete: ui_hud/am_map.rs; see UI_HUD_TRANSLATION_PLAN.md)
- net_* – Networking
- Full g_game (G_InitNew, G_LoadGame, etc.)
- deh_* full implementation

---

## Summary

| Priority | Module | Unblocks |
|----------|--------|----------|
| 1 | deh_* stubs | p_spec, p_switch, p_doors, m_menu, etc. |
| 2 | doomkeys | Input handling |
| 3 | info (minimal) | p_mobj spawn, p_enemy, p_inter |
| 4 | Blockmap + p_maputl | p_map collision |
| 5 | p_map, p_sight | Player movement, monster AI |
| 6 | P_SpawnPlayer | View from player mobj, game start ✅ |
| 7 | z_zone fix | example_render_scene completion ⏳ |

## Remaining Work (Next)

1. **z_zone** – Fix base=null corruption during Z_Malloc purge path.
2. **Real states** – Add states with positive tics for animations.
3. **Monster AI** – p_enemy (A_Look, A_Chase, etc.).

See also: `PLAYER_TRANSLATION_PLAN.md`, `GAME_CORE_TRANSLATION_PLAN.md`, `C_TO_RUST_OVERVIEW.md`
