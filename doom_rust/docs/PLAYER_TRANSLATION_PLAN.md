# Player Module Translation Plan

Plan for porting the Doom `p_*.c` / `p_*.h` files from C to Rust. All files go into a `player` submodule. Each logical unit combines its `.h` (public API) and `.c` (private implementation) into a single `.rs` file.

**Source:** `doomgeneric/doomgeneric/p_*.c` and `p_*.h`

---

## Module Structure

```
src/player/
├── mod.rs              # p_local.h role: re-exports public API, constants, shared types
├── p_setup.rs          # p_setup.h + p_setup.c
├── p_mobj.rs           # p_mobj.h + p_mobj.c
├── p_inter.rs          # p_inter.h + p_inter.c
├── p_pspr.rs           # p_pspr.h + p_pspr.c
├── p_saveg.rs          # p_saveg.h + p_saveg.c
├── p_spec.rs           # p_spec.h + p_spec.c
├── p_tick.rs           # p_tick.h + p_tick.c
├── p_ceilng.rs         # p_ceilng.c (no .h)
├── p_doors.rs          # p_doors.c (no .h)
├── p_enemy.rs          # p_enemy.c (no .h)
├── p_floor.rs          # p_floor.c (no .h)
├── p_lights.rs         # p_lights.c (no .h)
├── p_map.rs            # p_map.c (no .h)
├── p_maputl.rs         # p_maputl.c (no .h)
├── p_plats.rs          # p_plats.c (no .h)
├── p_sight.rs          # p_sight.c (no .h)
├── p_switch.rs         # p_switch.c (no .h)
├── p_telept.rs         # p_telept.c (no .h)
└── p_user.rs           # p_user.c (no .h)
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
| d_think | Not started | `thinker_t` |
| doomdata | Started | `mapthing_t` |
| doomdef | Started | `skill_t`, etc. |
| doomstat | Partial | `player_t`, `GAMEMODE`, etc. |
| info | Not started | `mobjinfo_t`, `state_t`, `mobjtype_t`, `statenum_t` |
| r_local / rendering | Started | `line_t`, `sector_t`, `subsector_t`, `side_t`, etc. |
| g_game | Not started | Game logic calls |
| s_sound | Started | Sound playback |
| deh_main, deh_misc | Not started | DeHackEd |
| am_map | Not started | Automap (p_inter) |
| st_stuff, hu_stuff | Not started | Status bar, HUD (p_mobj) |

### Phase 1: Foundation

| Step | File | Dependencies | Notes |
|------|------|--------------|-------|
| 1 | **p_local / mod.rs** | r_local, p_spec | Constants, types, re-exports. Start with minimal stub. ✅ Constants, Divline |
| 2 | **p_mobj.rs** | d_think, doomdata, info, tables, m_fixed | Full `mobj_t`, flags, spawn/remove |
| 3 | **p_maputl.rs** | p_local, m_bbox, r_state | Blockmap, divline, intercept, path traverse. ✅ P_AproxDistance, P_PointOnLineSide, P_PointOnDivlineSide, P_DivlineSide, P_MakeDivline, P_InterceptVector, P_BoxOnLineSide, P_LineOpening |
| 4 | **p_tick.rs** | p_local, z_zone, doomstat | Thinker list, P_Ticker. ✅ P_InitThinkers, P_AddThinker, P_RemoveThinker stubs |
| 5 | **p_setup.rs** | p_local, z_zone, w_wad, m_bbox, g_game, s_sound | Level load, blockmap, reject |

### Phase 2: Movement & Collision

| Step | File | Dependencies | Notes |
|------|------|--------------|-------|
| 6 | **p_map.rs** | p_maputl, p_sight, p_local | P_CheckPosition, P_TryMove, P_SlideMove, P_UseLines, attacks |
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
| 14 | **p_spec.rs** | p_local, r_local, w_wad, g_game | Special sectors, line specials |
| 15 | **p_switch.rs** | p_local, g_game, s_sound, deh_main | Switches, buttons |
| 16 | **p_inter.rs** | p_local, am_map, s_sound | P_TouchSpecialThing, P_DamageMobj |

### Phase 4: Player & Enemies

| Step | File | Dependencies | Notes |
|------|------|--------------|-------|
| 17 | **p_pspr.rs** | p_local, s_sound, deh_misc | Player weapon sprites |
| 18 | **p_user.rs** | p_local, d_event, doomstat | P_PlayerThink |
| 19 | **p_enemy.rs** | p_local, g_game, s_sound | P_NoiseAlert, monster AI |
| 20 | **p_saveg.rs** | p_local, g_game, dstrings | Save/load game |

---

## Integration with doom_rust

1. **Create `src/player/` directory** and add `mod.rs` plus the `p_*.rs` files.
2. **Update `src/lib.rs`**: Replace `pub mod p_mobj` and `pub mod p_setup` with `pub mod player`.
3. **Move existing code**: Current `p_setup.rs` and `p_mobj.rs` in `src/` are partial. Either:
   - Move into `player/` and expand, or
   - Keep as stubs that delegate to `player::` when full port is done.
4. **Fix imports**: All `crate::p_setup`, `crate::p_mobj` become `crate::player::p_setup`, `crate::player::p_mobj` (or `crate::player` with re-exports).

---

## Migration of Existing p_setup / p_mobj

Current `doom_rust` has:

- **p_setup.rs**: Minimal `p_load_level` for scene rendering (different from C `P_SetupLevel`).
- **p_mobj.rs**: Minimal `Mobj` struct (x, y, z, angle, sprite, frame, flags) for sound/rendering.

**Recommendation:** Move both into `player/` as `p_setup.rs` and `p_mobj.rs`. Keep the current minimal API working during migration. As the full C logic is ported, extend the modules and deprecate/remove the minimal stubs where appropriate.

---

## Summary

| Category | Count |
|----------|-------|
| Files with .h + .c | 7 |
| Files with .c only | 12 |
| mod.rs (p_local.h) | 1 |
| **Total .rs files** | **20** |

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
