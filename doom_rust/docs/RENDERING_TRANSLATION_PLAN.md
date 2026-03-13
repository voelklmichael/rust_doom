# Rendering Module Translation Plan

Plan for porting the Doom rendering subsystem from C to Rust. All files go into a `rendering` module. Each logical unit combines its `.h` (public API) and `.c` (private implementation) into a single `.rs` file.

---

## Module Structure

```
src/rendering/
├── mod.rs              # Re-exports public API from submodules
├── defs.rs             # r_defs.h (definitions only, no .c)
├── state.rs            # r_state.h (extern declarations, state from r_data etc.)
├── r_main.rs           # r_main.h + r_main.c
├── r_data.rs           # r_data.h + r_data.c
├── r_bsp.rs            # r_bsp.h + r_bsp.c
├── r_plane.rs          # r_plane.h + r_plane.c
├── r_segs.rs           # r_segs.h + r_segs.c
├── r_things.rs         # r_things.h + r_things.c
├── r_draw.rs           # r_draw.h + r_draw.c
├── r_sky.rs            # r_sky.h + r_sky.c
├── v_patch.rs          # v_patch.h (definitions only, no .c)
├── v_video.rs          # v_video.h + v_video.c
└── m_bbox.rs           # m_bbox.h + m_bbox.c (used by r_main, r_bsp)
```

**Note:** `r_local.h` is an aggregator that includes all r_*.h; in Rust this is replaced by `mod.rs` re-exports.

---

## Translation Order

Dependencies flow bottom-up. Translate in this order:

### Phase 0: Prerequisites (outside rendering)

These must exist or be stubbed before rendering:

| Module | Status | Notes |
|--------|--------|-------|
| p_mobj | Started (stub) | Full mobj_t needed for sector_t.thinglist, etc. |
| d_think | Not started | thinker_t in degenmobj_t, sector_t.specialdata |
| p_local | Not started | Used by r_data.c for level data |
| i_video | Not started | SCREENWIDTH, SCREENHEIGHT, videomode_t |
| doomdef | Started | SCREENWIDTH may be in doomdef or i_video |
| deh_main | Not started | DeHackEd – R_TextureNumForName, etc. |

### Phase 1: Foundation (no game data)

| Step | File | C Sources | Public (.h) | Private (.c) |
|------|------|-----------|------------|--------------|
| 1 | **v_patch.rs** | v_patch.h | `patch_t`, `post_t`, `column_t` | (none) |
| 2 | **m_bbox.rs** | m_bbox.h/c | `M_ClearBox`, `M_AddToBox`, `BOXTOP` etc. | Implementation |
| 3 | **defs.rs** | r_defs.h | `vertex_t`, `sector_t`, `line_t`, `seg_t`, `node_t`, `subsector_t`, `side_t`, `drawseg_t`, `vissprite_t`, `column_t`, `lighttable_t`, etc. | (definitions only) |
| 4 | **state.rs** | r_state.h | Extern state vars: `viewwidth`, `textureheight`, `colormaps`, `vertexes`, `segs`, `sectors`, etc. | (declarations; state lives in r_data, etc.) |

### Phase 2: Video Buffer

| Step | File | C Sources | Public (.h) | Private (.c) |
|------|------|-----------|------------|--------------|
| 5 | **v_video.rs** | v_video.h/c | `V_Init`, `V_DrawPatch`, `V_DrawBlock`, `V_CopyRect`, `V_MarkRect`, `V_UseBuffer`, `V_RestoreBuffer`, etc. | Buffer management, blitting |

### Phase 3: Data Loading

| Step | File | C Sources | Public (.h) | Private (.c) |
|------|------|-----------|------------|--------------|
| 6 | **r_data.rs** | r_data.h/c | `R_InitData`, `R_PrecacheLevel`, `R_GetColumn`, `R_FlatNumForName`, `R_TextureNumForName`, `R_CheckTextureNumForName` | Texture/flat/sprite loading, caching |

### Phase 4: Sky

| Step | File | C Sources | Public (.h) | Private (.c) |
|------|------|-----------|------------|--------------|
| 7 | **r_sky.rs** | r_sky.h/c | `SKYFLATNAME`, `ANGLETOSKYSHIFT`, `skytexture`, `skytexturemid`, `R_InitSkyMap` | Sky texture setup |

### Phase 5: Main & BSP

| Step | File | C Sources | Public (.h) | Private (.c) |
|------|------|-----------|------------|--------------|
| 8 | **r_main.rs** | r_main.h/c | `R_Init`, `R_RenderPlayerView`, `R_SetViewSize`, `R_PointToAngle`, `R_PointToAngle2`, `R_PointInSubsector`, `R_ScaleFromGlobalAngle`, `viewcos`, `viewsin`, `centerx`, `centery`, lighting LUTs, `colfunc`, `spanfunc`, etc. | POV setup, angle/dist utils |
| 9 | **r_bsp.rs** | r_bsp.h/c | `R_ClearClipSegs`, `R_ClearDrawSegs`, `R_RenderBSPNode`, BSP state (`curline`, `drawsegs`, etc.) | BSP traversal |

### Phase 6: Segments, Planes, Things

| Step | File | C Sources | Public (.h) | Private (.c) |
|------|------|-----------|------------|--------------|
| 10 | **r_segs.rs** | r_segs.h/c | `R_RenderMaskedSegRange` | Seg rendering |
| 11 | **r_plane.rs** | r_plane.h/c | `R_InitPlanes`, `R_ClearPlanes`, `R_MapPlane`, `R_MakeSpans`, `R_DrawPlanes`, plane state | Floor/ceiling visplanes |
| 12 | **r_things.rs** | r_things.h/c | `R_AddSprites`, `R_AddPSprites`, `R_DrawSprites`, `R_InitSprites`, `R_ClearSprites`, `R_DrawMasked`, `R_DrawMaskedColumn`, `R_SortVisSprites`, `R_ClipVisSprite` | Sprite/thing rendering |

### Phase 7: Column Drawing

| Step | File | C Sources | Public (.h) | Private (.c) |
|------|------|-----------|------------|--------------|
| 13 | **r_draw.rs** | r_draw.h/c | `R_DrawColumn`, `R_DrawColumnLow`, `R_DrawFuzzColumn`, `R_DrawTranslatedColumn`, `R_DrawSpan`, `R_VideoErase`, draw state (`dc_*`, `ds_*`) | Low-level column/span blitting |

---

## File-by-File Summary

| Rust File | C .h | C .c | Public API | Private |
|-----------|------|------|------------|---------|
| defs.rs | r_defs.h | — | Types: vertex_t, sector_t, line_t, seg_t, node_t, subsector_t, side_t, drawseg_t, vissprite_t, column_t, lighttable_t, etc. | — |
| state.rs | r_state.h | — | Extern state (viewwidth, textureheight, vertexes, segs, sectors, sprites, etc.) | — |
| v_patch.rs | v_patch.h | — | patch_t, post_t, column_t | — |
| m_bbox.rs | m_bbox.h | m_bbox.c | M_ClearBox, M_AddToBox, BOXTOP/BOTTOM/LEFT/RIGHT | Bbox logic |
| v_video.rs | v_video.h | v_video.c | V_Init, V_DrawPatch*, V_DrawBlock, V_CopyRect, V_MarkRect, V_UseBuffer, V_RestoreBuffer, V_LoadTintTable, etc. | Screen buffer, blit impl |
| r_data.rs | r_data.h | r_data.c | R_InitData, R_PrecacheLevel, R_GetColumn, R_FlatNumForName, R_TextureNumForName | Texture/flat/sprite load |
| r_sky.rs | r_sky.h | r_sky.c | R_InitSkyMap, skytexture, skytexturemid | Sky setup |
| r_main.rs | r_main.h | r_main.c | R_Init, R_RenderPlayerView, R_SetViewSize, R_PointToAngle, R_PointToAngle2, R_PointInSubsector, R_ScaleFromGlobalAngle, R_AddPointToBox, view vars, lighting LUTs | POV, utils |
| r_bsp.rs | r_bsp.h | r_bsp.c | R_ClearClipSegs, R_ClearDrawSegs, R_RenderBSPNode, BSP state | BSP traversal |
| r_segs.rs | r_segs.h | r_segs.c | R_RenderMaskedSegRange | Seg drawing |
| r_plane.rs | r_plane.h | r_plane.c | R_InitPlanes, R_ClearPlanes, R_MapPlane, R_MakeSpans, R_DrawPlanes | Visplane logic |
| r_things.rs | r_things.h | r_things.c | R_AddSprites, R_AddPSprites, R_DrawSprites, R_InitSprites, R_ClearSprites, R_DrawMasked, R_DrawMaskedColumn, R_SortVisSprites, R_ClipVisSprite | Sprite rendering |
| r_draw.rs | r_draw.h | r_draw.c | R_DrawColumn, R_DrawColumnLow, R_DrawFuzzColumn, R_DrawTranslatedColumn, R_DrawSpan, R_VideoErase, dc_*/ds_* state | Column/span blit |

---

## Rust Module Layout (per file)

Each `.rs` file follows this pattern:

```rust
//! Module description. Original: foo.h / foo.c

// =============================================================================
// Public API (from .h)
// =============================================================================

pub struct Vertex { ... }
pub fn r_init() { ... }

// =============================================================================
// Private implementation (from .c)
// =============================================================================

fn helper_from_c_impl() { ... }
static mut INTERNAL_STATE: ... = ...;
```

- **Public:** Types, constants, and functions declared in the `.h` and used by other crates/modules.
- **Private:** Implementation details, static state, and helpers from the `.c` file.

---

## External Dependencies

Rendering depends on (must exist or be stubbed):

| C Module | Rust Module | Purpose |
|----------|-------------|---------|
| doomdef.h | doomdef | SCREENWIDTH, SCREENHEIGHT (or i_video) |
| doomstat.h | doomstat | Game state |
| doomtype.h | doomtype | Boolean, byte |
| m_fixed.h | m_fixed | fixed_t, FRACUNIT |
| m_misc.h | m_misc | M_ExtractFileBase |
| i_swap.h | i_swap | Long/Short |
| i_system.h | i_system | I_Error |
| z_zone.h | z_zone | Z_Malloc, Z_Free |
| w_wad.h | wad | W_CacheLump*, W_ReadLump |
| p_mobj.h | p_mobj | mobj_t |
| p_local.h | (p_local) | Level data (vertexes, segs, sectors) |
| d_think.h | (d_think) | thinker_t |
| d_player.h | (d_player) | player_t |
| d_loop.h | (d_loop) | Game loop |
| m_menu.h | (m_menu) | R_SetViewSize |
| deh_main.h | (deh_main) | Texture name lookup |
| tables.h | geometry::tables | finesine, tantoangle, etc. |

---

## Progress

### Completed (Phase 1–4)
- [x] **m_bbox** – M_ClearBox, M_AddToBox
- [x] **v_patch** – patch_t, post_t, column_t
- [x] **defs** – vertex_t, sector_t, line_t, seg_t, node_t, subsector_t, side_t, drawseg_t, vissprite_t, visplane_t, spritedef_t, etc.
- [x] **state** – viewwidth, vertexes, segs, sectors, etc.
- [x] **v_video** – stub (V_Init, V_DrawPatch, V_DrawBlock, etc. – no-op)
- [x] **r_data** – R_InitData, R_PrecacheLevel, R_GetColumn, R_FlatNumForName, R_TextureNumForName, R_CheckTextureNumForName
- [x] **r_sky** – R_InitSkyMap, skytexture, skytexturemid, SKYFLATNAME

### Next Steps
1. **r_main** – R_Init, R_RenderPlayerView, R_SetViewSize
2. **r_bsp** – R_RenderBSPNode
3. **r_segs** → **r_plane** → **r_things** → **r_draw**
