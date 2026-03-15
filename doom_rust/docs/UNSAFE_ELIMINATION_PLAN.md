# Plan: Eliminating Unsafe Code

**Current status (incremental migration):** `#![allow(unsafe_code)]` at crate level; `#[allow(unsafe_code)]` on 15 modules. Build passes. Remove allow from each module as it is migrated.

With `#![forbid(unsafe_code)]` enabled, the compiler reports **~268 errors**:
- **248** `unsafe` blocks (raw pointer dereference, `ptr::write_bytes`, etc.)
- **9** `unsafe impl Send` (for state structs with raw pointers)
- **4** `unsafe extern "C" fn` declarations
- **1** `unsafe fn` method (`think_acp1`)

---

## Example 1: Raw Pointer Dereferencing (most common)

**Location:** `p_setup.rs:107-111`, `p_mobj.rs:404-419`

```rust
// CURRENT (unsafe)
let vertexes_ptr = z_malloc(...) as *mut Vertex;
unsafe {
    (*vertexes_ptr.add(i)).x = x;
    (*vertexes_ptr.add(i)).y = y;
}
```

**Why unsafe:** Dereferencing raw pointers is undefined behavior if the pointer is null, unaligned, or dangling.

**Safe alternative – use `Vec` + indexing:**
```rust
let mut vertexes: Vec<Vertex> = Vec::with_capacity(num_vertexes);
for i in 0..num_vertexes {
    vertexes.push(Vertex { x, y });
}
// Access: vertexes[i].x
```

---

## Example 2: `unsafe impl Send` for State Structs

**Location:** `doomstat.rs:251`, `r_data.rs`, etc.

```rust
// CURRENT (unsafe)
struct DoomstatState {
    players: [Player; MAXPLAYERS],
    // ... contains *mut T fields
}
unsafe impl Send for DoomstatState {}
```

**Why unsafe:** Types with raw pointers are `!Send` by default; we override that.

**Safe alternatives:**
- **A)** Replace `*mut T` in state with `AtomicPtr<T>` (atomic ops, no `unsafe impl`)
- **B)** Use indices (`usize`) instead of pointers; store data in `Vec<T>`
- **C)** Use `Arc<Mutex<T>>` or similar for shared data ✓ *chosen*

**Chosen approach (C):** Wrap shared data in `Arc<Mutex<T>>` so the state holds thread-safe handles instead of raw pointers. The inner `T` uses `Vec` + indices (Example 1) to avoid raw pointers. State becomes `Send` without `unsafe impl`.

---

## Example 3: `unsafe extern "C" fn` (Thinker Callbacks)

**Location:** `d_think.rs:85-92`, `p_mobj.rs:108`

```rust
// CURRENT (unsafe)
pub unsafe extern "C" fn no_op_acp1(_: *mut ()) {}
pub unsafe extern "C" fn thinker_marked_removed(_: *mut ()) {}
pub unsafe extern "C" fn p_mobj_thinker(mobj: *mut ()) { ... }
```

**Why unsafe:** C ABI + raw pointer argument; callers must uphold invariants.

**Safe alternative (Arc<Mutex<_>>, similar to Example 2):** Replace the function-pointer pattern with index-based dispatch:

- Store mobjs in `Arc<Mutex<Vec<Mobj>>>` (or similar shared state).
- Pass an index (`usize`) instead of `*mut ()` to the thinker logic.
- Replace `thinker.function.acp1 = p_mobj_thinker` with an enum or trait:

```rust
enum ThinkerAction {
    Mobj(usize),           // index into Arc<Mutex<Vec<Mobj>>>
    Plat(usize),
    MarkedForRemoval,
    NoOp,
}

impl Thinker {
    fn run(&self, mutex: &Mutex<ThinkerState>) {
        match self.action {
            ThinkerAction::Mobj(idx) => {
                let mut state = mutex.lock().unwrap();
                p_mobj_thinker_safe(&mut state.mobjs[idx]);
            }
            ThinkerAction::MarkedForRemoval => {}
            ThinkerAction::NoOp => {}
            ...
        }
    }
}
```

- `p_mobj_thinker_safe(&mut Mobj)` receives `&mut Mobj` – no raw pointer, no unsafe.

---

## Example 4: `ptr::write_bytes` (Zeroing Memory)

**Location:** `p_mobj.rs:390-392`

```rust
// CURRENT (unsafe)
unsafe {
    ptr::write_bytes(ptr as *mut u8, 0, std::mem::size_of::<Mobj>());
}
```

**Safe alternative (Option + Arc<Mutex<_>>, similar to Examples 2–3):** Store mobjs in shared state; use `Option` for slots and construct values explicitly:

- Store in `Arc<Mutex<Vec<Option<Mobj>>>>` – `Option` represents empty/reserved slots.
- Or `Arc<Mutex<Vec<Mobj>>>` – allocate by pushing `Mobj { /* explicit fields */ }`; no zeroing.
- Replace raw allocation + `write_bytes` with: `state.mobjs.push(Some(Mobj { x, y, ... }))` or `state.mobjs[idx] = Some(mobj)`.
- No `MaybeUninit` or zeroing – construct values directly; `Option::None` for uninitialized slots.

---

## Strategy Options

### Option A: Minimal Change – Allow Unsafe in Isolated Modules

**Approach:** Remove `#![forbid(unsafe_code)]` and use `#![deny(unsafe_code)]` with targeted `#[allow(unsafe_code)]` in modules that need it.

**Pros:** Small change, keeps current design.  
**Cons:** Unsafe remains; only localized.

---

### Option B: Index-Based Architecture (Recommended for full elimination)

**Approach:** Replace pointer-based structures with index-based ones.

| Current | Replacement |
|---------|-------------|
| `*mut Vertex` | `usize` (index into `Vec<Vertex>`) |
| `*mut Sector` | `usize` (index into `Vec<Sector>`) |
| `*mut Mobj` | `usize` (index into `Vec<Mobj>`) or `Option<usize>` |
| `thinglist: *mut Mobj` | `Vec<usize>` (indices of things in sector) |
| Linked list (prev/next) | `Vec` + indices, or `SlotMap` / generational arena |

**Changes:**
1. **Zone allocator:** Return `Box<T>` or indices into a pool instead of `*mut u8`.
2. **Map data:** `vertices: Vec<Vertex>`, `sectors: Vec<Sector>`, etc.
3. **Mobjs:** `mobjs: Vec<Mobj>` with `Mobj { sector_idx: usize, next_thing_idx: Option<usize>, ... }`.
4. **Thinker list:** Store indices; iterate with `for i in thinker_indices`.

**Pros:** No raw pointers, no unsafe.  
**Cons:** Large refactor (hundreds of call sites).

---

### Option C: Safe Wrapper Types

**Approach:** Introduce wrapper types that encapsulate unsafe access.

```rust
struct VertexPtr(NonNull<Vertex>);
impl VertexPtr {
    fn get(&self) -> &Vertex { /* still needs unsafe for deref */ }
}
```

**Reality:** Dereferencing `NonNull` still requires `unsafe`. This only hides it; it does not remove it under `forbid(unsafe_code)`.

---

### Option D: Hybrid – Fix Low-Hanging Fruit First

**Phase 1 (quick wins):**
1. Replace `ptr::write_bytes` (Example 4) with `Option` + `Arc<Mutex<Vec<Option<Mobj>>>>`; construct values explicitly, use `None` for empty slots.
2. Replace `unsafe impl Send` by using `Arc<Mutex<T>>` for shared data (Example 2, Option C).
3. Replace thinker callbacks (Example 3) with enum/trait dispatch + `Arc<Mutex<Vec<Mobj>>>`; pass indices instead of raw pointers.

**Phase 2 (structural):**
4. Introduce index-based map data (`Vec<Vertex>`, `Vec<Sector>`, etc.).
5. Migrate one subsystem at a time (e.g. vertices → sectors → lines → mobjs).

---

## Recommended Execution Order

1. **Phase 1 – Quick fixes**
   - `p_mobj.rs` (Example 4): Use `Option` + `Arc<Mutex<Vec<Option<Mobj>>>>`; construct mobjs explicitly, no `write_bytes`.
   - State structs: Use `Arc<Mutex<T>>` for shared data (Example 2, Option C); inner `T` uses `Vec` + indices to avoid raw pointers.
   - `d_think.rs` / thinker system: Replace function pointers with enum/trait dispatch; store mobjs in `Arc<Mutex<Vec<Mobj>>>`, pass indices (Example 3).

2. **Phase 2 – Zone allocator**
   - Change `z_malloc` to return `Box<[u8]>` or a typed `Pool<T>`.
   - Update callers to use `Box` or pool indices.

3. **Phase 3 – Map data**
   - `p_setup`: Use `Vec<Vertex>`, `Vec<Sector>`, `Vec<SideDef>`, etc.
   - Replace `*mut X` with `usize` and access via `vec[index]`.

4. **Phase 4 – Game objects**
   - `Mobj`, thinker list, thing lists: move to index-based representation.
   - Replace all `(*ptr).field` with `vec[idx].field`.

---

## Estimated Effort

| Phase | Files | Est. changes |
|-------|-------|--------------|
| Phase 1 | 3–5 | ~20 |
| Phase 2 | 1 + callers | ~50 |
| Phase 3 | p_setup, defs, p_map, etc. | ~200 |
| Phase 4 | p_mobj, p_tick, p_* | ~300 |

**Total:** ~600+ edits across 40+ files.

---

## Migration Progress

**Done:**
- Switched from `forbid` to `allow` at crate level for incremental migration
- Added `#[allow(unsafe_code)]` to 15 modules with TODO comments
- Removed `unsafe` from trivial extern fns: `no_op`, `no_op_acp1`, `thinker_marked_removed` (d_think.rs)
- Added `unsafe impl Send` for RDataState (was missing, caused OnceLock compile errors)
- **Phase 1 mobj/thinker refactor (complete):**
  - Created `player/mobjs.rs` with `Arc<Mutex<MobjsState>>`, `MobjIndex`, `mobj_alloc`, `with_mobjs_state`, `with_mobj_ref`
  - Replaced `z_malloc` + `write_bytes` in `p_spawn_mobj` with explicit `Mobj` construction and `mobj_alloc`
  - Replaced thinker linked list with `thinker_indices: Vec<usize>`; `p_run_thinkers` uses `p_mobj_thinker_safe(&mut Mobj)`
  - `Player.mo` now `Option<MobjIndex>`; `r_main::view_player` uses `with_mobj_ref`
  - Removed `#[allow(unsafe_code)]` from p_mobj, p_tick (no unsafe in those modules now)

**Modules with allow (remove as migrated):** doomstat, d_think, p_setup, r_bsp, r_data, r_draw, r_main, r_plane, r_segs, r_things, state, v_video

**Phase 3 progress (map data):**
- Migrated `rejectmatrix` from `z_malloc` to `Vec<u8>` (p_load_reject)
- Migrated `blockmaplump`, `blocklinks` from `z_malloc` to `Vec` (p_load_blockmap)
- RenderState now uses `blockmaplump: Vec<i16>`, `blocklinks: Vec<*mut c_void>`, `rejectmatrix: Vec<u8>`
- Removed 2 `z_malloc` calls and associated `unsafe` from p_setup
- **Nodes:** Migrated from `z_malloc` to `Vec<Node>` in p_setup
- **p_group_lines:** Refactored to use `Vec`s and indices via `state::with_state_mut`
- **r_main, r_bsp, r_segs, r_things:** Updated for `Vec` + indices (curline_idx, frontsector_idx, etc.)
- **r_point_in_subsector:** Returns `usize` (subsector index) instead of `*mut Subsector`
- **p_maputl:** `p_point_on_line_side`, `p_box_on_line_side` take `&Line`; `p_line_opening` takes `line_idx`; `p_block_lines_iterator` passes `line_idx` to callback
- **p_map:** `spechit` and `ceilingline` use indices; `pit_check_line` takes `line_idx`; `bestslideline`/`secondslideline` use indices
- **Intercept:** `line` → `line_idx: Option<usize>`

**Phase 3 continued (ev_do_*, p_enemy, am_map):**
- **ev_do_ceiling, ev_do_door, ev_do_locked_door, ev_do_plat, ev_start_light_strobing, ev_start_light_flickering:** Switched from `*const Line` to `line_idx: usize`
- **ev_teleport:** Takes `line_idx: usize`; `find_teleport_dest` uses `Vec<Sector>` and `find_teleportman_in_sector` uses `sector.thinglist: Option<MobjIndex>`
- **ev_do_floor:** Fixed `has_specialdata` check; uses `state::with_state` for sector lookup
- **ev_light_turn_on_by_tag:** Migrated to `state::with_state_mut` and `Vec<Sector>`
- **p_enemy:** `p_noise_alert` uses `sector.lines: Vec<usize>`, `get_next_sector(line_idx, sec_idx)`, `set_sector_soundtarget(sec_idx, MobjIndex)`
- **am_map:** `am_find_min_max_boundaries` uses `Vec<Vertex>`; `am_draw_walls` uses `Vec<Line>` with `v1_idx`, `v2_idx`, `frontsector_idx`, `backsector_idx`; `am_draw_things` uses `sector.thinglist: Option<MobjIndex>` and `with_mobj_ref`

**p_saveg migration (when module enabled):**
- **p_archive_world / p_unarchive_world:** Use `state::with_state` and `state::with_state_mut` to iterate over `Vec<Sector>`, `Vec<Line>`, `Vec<SideDef>`; `sec.specialdata = None`, `sec.soundtarget = None`
- **sector_index / sector_from_index:** Compute index from pointer via `(ptr - base) / size_of`; `sector_from_index` returns pointer from `s.sectors.as_ptr().add(idx)`
- **set_sector_specialdata:** Helper to set `sector.specialdata = Some(value)` via pointer→index conversion
- **FloorMover:** Uses `sector_idx` directly in archive/unarchive (not `sector` pointer)
- **p_unarchive_thinkers:** floorz/ceilingz from subsector uses `subsector_idx` and `state.subsectors[].sector_idx` → `sectors[].floorheight`

**p_sight migration (when p_maputl enabled):**
- **p_check_sight:** Uses `subsector as usize` for indices; REJECT lookup via `state.rejectmatrix` (Vec); sector indices from `subsector.sector_idx`
- **p_cross_subsector:** Uses `state::with_state_mut`; iterates `subsectors`, `segs`, `lines`, `vertexes`, `sectors` via indices; `Seg` has `linedef_idx`, `v1_idx`, `v2_idx`, `frontsector_idx`, `backsector_idx`; `Line` has `validcount`, `backsector_idx: Option<usize>`
- **p_cross_bsp_node:** Uses `state.nodes.get(bspnum)` (Vec<Node>)
- **Exports:** Added `with_r_main_state`, `with_r_main_state_mut` to `crate::rendering` for p_sight

**Enabled modules:** p_maputl, p_sight (fixed Intercept.line_idx, r_main imports).
**p_maputl:** `p_make_divline` now takes `line_idx: usize` and uses `state::with_state` (removed unsafe pointer deref).
**Next steps:** Phase 2 (zone allocator). Re-enable p_saveg, p_map (requires p_spec, p_inter, r_main fixes), p_ceilng, p_doors, p_floor, etc. when ready.
