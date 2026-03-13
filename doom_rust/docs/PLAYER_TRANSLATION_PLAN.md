# Player Module Translation Plan

Plan for porting the Doom `p_*.c` / `p_*.h` files from C to Rust. All files go into a `player` submodule. Each logical unit combines its `.h` (public API) and `.c` (private implementation) into a single `.rs` file.

**Source:** `doomgeneric/doomgeneric/p_*.c` and `p_*.h`

---

## Current Status

| Phase | Status | Notes |
|-------|--------|-------|
| **Integration** | ✅ Done | `src/player/` exists, `lib.rs` uses `pub mod player`, imports updated |
| **Phase 1** | ✅ Done | mod.rs, p_mobj, p_maputl, p_tick, p_setup |
| **Phase 2** | ✅ Done | p_map, p_sight, p_floor, p_ceilng, p_doors, p_plats, p_lights, p_telept |
| **Phase 3** | ✅ Done | p_spec, p_switch, p_inter |
| **Phase 4** | ✅ Done | p_pspr, p_user, p_enemy, p_saveg |

**Working:** `p_load_level` (blockmap, REJECT, P_LoadThings), `p_maputl` (BlockLinesIterator, BlockThingsIterator, PathTraverse, P_SetThingPosition, P_UnsetThingPosition), `p_map` (P_CheckPosition, P_TryMove, P_TeleportMove, P_SlideMove), `p_sight` (P_CheckSight with REJECT+BSP), `p_tick` (P_RunThinkers, P_Ticker), `p_mobj` (P_SpawnMobj, P_RemoveMobj, P_SpawnMapThing, P_SpawnPlayer, P_MobjThinker with XY/Z movement and state tics), `get_next_sector`.

**Stubs:** p_floor, p_ceilng, p_doors, p_plats, p_lights, p_telept, p_switch, p_inter, p_pspr, p_user, p_enemy, p_saveg.

**Missing:** None – all 20 modules scaffolded; core movement/collision/spawn logic implemented.

---

## Module Structure

```
src/player/
├── mod.rs              # p_local.h: constants, Divline, re-exports ✅
├── p_setup.rs          # p_setup.h + p_setup.c ✅ (p_load_level working)
├── p_mobj.rs           # p_mobj.h + p_mobj.c ✅ (minimal Mobj)
├── p_inter.rs          # p_inter.h + p_inter.c ✅ (stub)
├── p_pspr.rs           # p_pspr.h + p_pspr.c ✅ (stub)
├── p_saveg.rs          # p_saveg.h + p_saveg.c ✅ (stub)
├── p_spec.rs           # p_spec.h + p_spec.c ✅ (get_next_sector)
├── p_tick.rs           # p_tick.h + p_tick.c ✅ (thinker list)
├── p_ceilng.rs         # p_ceilng.c ✅ (stub)
├── p_doors.rs          # p_doors.c ✅ (stub)
├── p_enemy.rs          # p_enemy.c ✅ (stub)
├── p_floor.rs          # p_floor.c ✅ (stub)
├── p_lights.rs         # p_lights.c ✅ (stub)
├── p_map.rs            # p_map.c ✅ (stub; requires blockmap)
├── p_maputl.rs         # p_maputl.c ✅ (line/divline utils)
├── p_plats.rs          # p_plats.c ✅ (stub)
├── p_sight.rs          # p_sight.c ✅ (P_CheckSight stub)
├── p_switch.rs         # p_switch.c ✅ (stub)
├── p_telept.rs         # p_telept.c ✅ (stub)
└── p_user.rs           # p_user.c ✅ (stub)
```

---

## Public vs Private Convention

For each `*.rs` file:

| Section | C Source | Rust Visibility |
|---------|----------|-----------------|
| **Public** | Content from `*.h` (declarations, types, constants) | `pub fn`, `pub struct`, `pub const`, `pub type` |
| **Private** | Content from `*.c` (implementation) | `fn`, `struct` (no `pub`) |

Files with only `.c` (no `.h`): their public API is what other modules call. These are declared in `p_local.h` or used internally. Use `pub` only for items that cross module boundaries.

---

## File Inventory

### With .h + .c (combine into single .rs)

| Rust File | C Header | C Implementation | Public API (from .h) |
|-----------|----------|------------------|----------------------|
| **p_setup.rs** | p_setup.h | p_setup.c | `P_SetupLevel`, `P_Init` |
| **p_mobj.rs** | p_mobj.h | p_mobj.c | `mobj_t`, `mobjflag_t`, mobj flags |
| **p_inter.rs** | p_inter.h | p_inter.c | (minimal; mostly via p_local) |
| **p_pspr.rs** | p_pspr.h | p_pspr.c | `P_SetupPsprites`, `P_MovePsprites`, `P_DropWeapon` |
| **p_saveg.rs** | p_saveg.h | p_saveg.c | Savegame read/write API |
| **p_spec.rs** | p_spec.h | p_spec.c | `P_InitPicAnims`, `P_SpawnSpecials`, `P_UpdateSpecials`, `P_UseSpecialLine`, `P_ShootSpecialLine`, `P_CrossSpecialLine`, `P_PlayerInSpecialSector`, `twoSided`, `getSector`, `getSide`, etc. |
| **p_tick.rs** | p_tick.h | p_tick.c | `P_Ticker` |

### Header-only (p_local.h)

`p_local.h` is an aggregator: it includes `r_local.h` and `p_spec.h`, and declares functions/types from all p_* modules. In Rust, this becomes `mod.rs`:

- Re-export public items from submodules
- Define constants only in p_local: `FLOATSPEED`, `MAXHEALTH`, `VIEWHEIGHT`, `MAPBLOCKUNITS`, `MAPBLOCKSIZE`, etc.
- Define types only in p_local: `divline_t`, `intercept_t`, `traverser_t`
- Declare externs: `thinkercap`, `intercepts`, `intercept_p`, `opentop`, `openbottom`, `floatok`, `tmfloorz`, `tmceilingz`, `ceilingline`, `spechit`, `numspechit`, `linetarget`, `rejectmatrix`, `blockmaplump`, `blockmap`, `bmapwidth`, `bmapheight`, `bmaporgx`, `bmaporgy`, `blocklinks`, `maxammo`, `clipammo`

### .c only (no dedicated .h)

| Rust File | C Source | Public API (declared in p_local.h or used elsewhere) |
|-----------|----------|------------------------------------------------------|
| **p_ceilng.rs** | p_ceilng.c | Ceiling movement (T_MoveCeiling, etc.) |
| **p_doors.rs** | p_doors.c | Door logic (T_VerticalDoor, etc.) |
| **p_enemy.rs** | p_enemy.c | `P_NoiseAlert` |
| **p_floor.rs** | p_floor.c | Floor movement |
| **p_lights.rs** | p_lights.c | Lighting changes |
| **p_map.rs** | p_map.c | `P_CheckPosition`, `P_TryMove`, `P_TeleportMove`, `P_SlideMove`, `P_CheckSight`, `P_UseLines`, `P_ChangeSector`, `P_AimLineAttack`, `P_LineAttack`, `P_RadiusAttack` |
| **p_maputl.rs** | p_maputl.c | `P_InitThinkers`, `P_AddThinker`, `P_RemoveThinker`, `P_AproxDistance`, `P_PointOnLineSide`, `P_PointOnDivlineSide`, `P_MakeDivline`, `P_InterceptVector`, `P_BoxOnLineSide`, `P_LineOpening`, `P_BlockLinesIterator`, `P_BlockThingsIterator`, `P_PathTraverse`, `P_UnsetThingPosition`, `P_SetThingPosition` |
| **p_plats.rs** | p_plats.c | Platform movement |
| **p_sight.rs** | p_sight.c | `P_CheckSight` (used by p_map) |
| **p_switch.rs** | p_switch.c | Switch/button logic |
| **p_telept.rs** | p_telept.c | Teleporter logic |
| **p_user.rs** | p_user.c | `P_PlayerThink` |

---

## Dependency Order

Dependencies flow bottom-up. Translate in this order:

### Phase 0: Prerequisites (outside player)

These must exist or be stubbed before player:

| Module | Status | Notes |
|--------|--------|-------|
| d_think | ✅ Done | `thinker_t` |
| doomdata | ✅ Done | `mapthing_t` |
| doomdef | ✅ Done | `skill_t`, etc. |
| doomstat | ✅ Done | `player_t`, `GAMEMODE`, etc. |
| info | ✅ Done | `mobjinfo_t`, `state_t`, `mobjtype_t`, `statenum_t` |
| r_local / rendering | ✅ Done | `line_t`, `sector_t`, `subsector_t`, `side_t`, etc. |
| m_bbox | ✅ In `rendering/m_bbox` | M_ClearBox, M_AddToBox; shared by rendering and p_maputl |
| g_game | ✅ Done | G_Ticker, G_PlayerReborn, G_BuildTiccmd |
| s_sound | ✅ Done | Sound playback |
| deh_main, deh_misc | Stub | DeHackEd – stubs for compatibility |
| am_map | Not started | Automap (p_inter) |
| st_stuff, hu_stuff | ✅ Done | Status bar, HUD (p_mobj) |

### Phase 1: Foundation

| Step | File | Dependencies | Notes |
|------|------|--------------|-------|
| 1 | **p_local / mod.rs** | r_local, p_spec | Constants, types, re-exports. Start with minimal stub. ✅ Constants, Divline |
| 2 | **p_mobj.rs** | d_think, doomdata, info, tables, m_fixed | Full `mobj_t`, flags, spawn/remove. ✅ Minimal Mobj (x,y,z,angle,sprite,frame,flags) |
| 3 | **p_maputl.rs** | p_local, m_bbox, r_state | Blockmap, divline, intercept, path traverse. ✅ P_AproxDistance, P_PointOnLineSide, P_PointOnDivlineSide, P_DivlineSide, P_MakeDivline, P_InterceptVector, P_BoxOnLineSide, P_LineOpening |
| 4 | **p_tick.rs** | p_local, z_zone, doomstat | Thinker list, P_Ticker. ✅ P_InitThinkers, P_AddThinker, P_RemoveThinker stubs |
| 5 | **p_setup.rs** | p_local, z_zone, w_wad, m_bbox, g_game, s_sound | Level load, blockmap, reject. ✅ p_load_level (rendering subset; no blockmap yet) |

### Phase 2: Movement & Collision

| Step | File | Dependencies | Notes |
|------|------|--------------|-------|
| 6 | **p_map.rs** | p_maputl, p_sight, p_local | P_CheckPosition, P_TryMove, P_SlideMove, P_UseLines, attacks. ✅ Stub (globals, API; blockmap required for impl) |
| 7 | **p_sight.rs** | p_local, r_state | P_CheckSight. ✅ Stub (always true) |
| 8 | **p_floor.rs** | p_local, z_zone, s_sound | Floor movers. ✅ Stub |
| 9 | **p_ceilng.rs** | p_local, z_zone, s_sound | Ceiling movers. ✅ Stub |
| 10 | **p_doors.rs** | p_local, z_zone, s_sound | Door movers. ✅ Stub |
| 11 | **p_plats.rs** | p_local, z_zone, s_sound | Platform movers. ✅ Stub |
| 12 | **p_lights.rs** | p_local, z_zone | Lighting. ✅ Stub |
| 13 | **p_telept.rs** | p_local, s_sound | Teleporters. ✅ Stub |

### Phase 3: Specials & Interaction

| Step | File | Dependencies | Notes |
|------|------|--------------|-------|
| 14 | **p_spec.rs** | p_local, r_local, w_wad, g_game | Special sectors, line specials. ✅ getNextSector, MO_TELEPORTMAN |
| 15 | **p_switch.rs** | p_local, g_game, s_sound, deh_main | Switches, buttons. ✅ Stub |
| 16 | **p_inter.rs** | p_local, am_map, s_sound | P_TouchSpecialThing, P_DamageMobj. ✅ Stub with API |

### Phase 4: Player & Enemies

| Step | File | Dependencies | Notes |
|------|------|--------------|-------|
| 17 | **p_pspr.rs** | p_local, s_sound, deh_misc | Player weapon sprites. ✅ Psprnum, Pspdef, P_SetupPsprites, P_MovePsprites, P_DropWeapon stubs |
| 18 | **p_user.rs** | p_local, d_event, doomstat | P_PlayerThink. ✅ P_PlayerThink stub, INVERSECOLORMAP, MAXBOB |
| 19 | **p_enemy.rs** | p_local, g_game, s_sound | P_NoiseAlert, monster AI. ✅ P_NoiseAlert stub |
| 20 | **p_saveg.rs** | p_local, g_game, dstrings | Save/load game. ✅ SAVESTRINGSIZE, SAVEGAMENAME from dstrings |

---

## Integration with doom_rust

1. ✅ **Create `src/player/` directory** and add `mod.rs` plus the `p_*.rs` files.
2. ✅ **Update `src/lib.rs`**: Replaced `pub mod p_mobj` and `pub mod p_setup` with `pub mod player`.
3. ✅ **Move existing code**: `p_setup.rs` and `p_mobj.rs` moved into `player/`.
4. ✅ **Fix imports**: `crate::rendering::defs`, `crate::sound::s_sound`, `examples` use `crate::player::p_setup`, `crate::player::p_mobj`.

---

## Migration of Existing p_setup / p_mobj

**Done.** Both are in `player/`:

- **p_setup.rs**: `p_load_level` with blockmap, REJECT, P_LoadThings, sector.lines, playeringame init.
- **p_mobj.rs**: Full `Mobj` (thinker, floorz, ceilingz, momx/momy/momz, radius, height, type, info, state, health, player ptr, etc.), P_SpawnMobj, P_RemoveMobj, P_SpawnMapThing, P_SpawnPlayer, P_MobjThinker (XY/Z movement, state tics).

**Next:** Fix z_zone corruption; add monster AI (p_enemy); fill specials stubs.

---

## Summary

| Category | Count |
|----------|-------|
| Files with .h + .c | 7 |
| Files with .c only | 12 |
| mod.rs (p_local.h) | 1 |
| **Total .rs files** | **20** |

---

## Next Steps (implementation order)

1. ~~**Implement p_map**~~ ✅ Done – P_CheckPosition, P_TryMove, P_TeleportMove, P_SlideMove.
2. ~~**Blockmap in p_setup**~~ ✅ Done – blockmap, sector.lines, blocklinks, P_LoadBlockMap, P_GroupLines.
3. ~~**d_think**~~ ✅ Done – thinker_t linkage in mobj, P_RunThinkers.
4. ~~**info**~~ ✅ Done – minimal mobjinfo_t, state_t, states(), MOBJINFO for player/imp/shotgun.
5. ~~**g_game**~~ ✅ Done – G_Ticker, G_PlayerReborn, G_BuildTiccmd stub.
6. **z_zone** – Fix base=null corruption during purge (blocks example_render_scene).
7. **Real states** – Add states with positive tics for animations.
8. **Monster AI** – p_enemy (A_Look, A_Chase, etc.).
9. **Fill stubs** – p_floor, p_ceilng, p_doors, p_plats, p_lights, p_telept, p_spec, p_switch, p_inter, p_pspr, p_user, p_saveg.

---

## Reference: p_* Include Graph

```
p_local.h
  ├── r_local.h
  └── p_spec.h

p_*.c files all include:
  - doomdef.h
  - p_local.h (which pulls in r_local, p_spec)
  - Plus: z_zone, s_sound, doomstat, g_game, i_system, deh_*, m_*, w_wad, r_state, etc.
```
