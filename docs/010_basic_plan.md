# Basic Migration Plan: C Doom → Rust

This document defines the staged migration strategy for converting the Doom C codebase (`doomgeneric/`) into Rust modules. Each C module becomes a corresponding Rust module `x.rs`. The goal is a mechanical, stage-0 translation that preserves structure; refactoring comes later.

---

## 1. Module Mapping

### 1.1 Paired Modules (x.h + x.c → x.rs)

For each `x.h`/`x.c` pair, create a single Rust module `x.rs` containing declarations (from `.h`) and implementations (from `.c`).

| Module | Files |
|--------|-------|
| am_map | am_map.h, am_map.c |
| d_event | d_event.h, d_event.c |
| d_items | d_items.h, d_items.c |
| d_iwad | d_iwad.h, d_iwad.c |
| d_loop | d_loop.h, d_loop.c |
| d_main | d_main.h, d_main.c |
| d_mode | d_mode.h, d_mode.c |
| doomdef | doomdef.h, doomdef.c |
| doomgeneric | doomgeneric.h, doomgeneric.c |
| doomstat | doomstat.h, doomstat.c |
| dstrings | dstrings.h, dstrings.c |
| f_finale | f_finale.h, f_finale.c |
| f_wipe | f_wipe.h, f_wipe.c |
| g_game | g_game.h, g_game.c |
| gusconf | gusconf.h, gusconf.c |
| hu_lib | hu_lib.h, hu_lib.c |
| hu_stuff | hu_stuff.h, hu_stuff.c |
| i_cdmus | i_cdmus.h, i_cdmus.c |
| i_endoom | i_endoom.h, i_endoom.c |
| i_joystick | i_joystick.h, i_joystick.c |
| info | info.h, info.c |
| i_scale | i_scale.h, i_scale.c |
| i_sound | i_sound.h, i_sound.c |
| i_system | i_system.h, i_system.c |
| i_timer | i_timer.h, i_timer.c |
| i_video | i_video.h, i_video.c |
| m_argv | m_argv.h, m_argv.c |
| m_bbox | m_bbox.h, m_bbox.c |
| m_cheat | m_cheat.h, m_cheat.c |
| m_config | m_config.h, m_config.c |
| m_controls | m_controls.h, m_controls.c |
| memio | memio.h, memio.c |
| m_fixed | m_fixed.h, m_fixed.c |
| m_menu | m_menu.h, m_menu.c |
| m_misc | m_misc.h, m_misc.c |
| m_random | m_random.h, m_random.c |
| mus2mid | mus2mid.h, mus2mid.c |
| p_inter | p_inter.h, p_inter.c |
| p_mobj | p_mobj.h, p_mobj.c |
| p_pspr | p_pspr.h, p_pspr.c |
| p_saveg | p_saveg.h, p_saveg.c |
| p_setup | p_setup.h, p_setup.c |
| p_spec | p_spec.h, p_spec.c |
| p_tick | p_tick.h, p_tick.c |
| r_bsp | r_bsp.h, r_bsp.c |
| r_data | r_data.h, r_data.c |
| r_draw | r_draw.h, r_draw.c |
| r_main | r_main.h, r_main.c |
| r_plane | r_plane.h, r_plane.c |
| r_segs | r_segs.h, r_segs.c |
| r_sky | r_sky.h, r_sky.c |
| r_things | r_things.h, r_things.c |
| sha1 | sha1.h, sha1.c |
| sounds | sounds.h, sounds.c |
| s_sound | s_sound.h, s_sound.c |
| statdump | statdump.h, statdump.c |
| st_lib | st_lib.h, st_lib.c |
| st_stuff | st_stuff.h, st_stuff.c |
| tables | tables.h, tables.c |
| v_video | v_video.h, v_video.c |
| w_checksum | w_checksum.h, w_checksum.c |
| w_file | w_file.h, w_file.c |
| wi_stuff | wi_stuff.h, wi_stuff.c |
| w_main | w_main.h, w_main.c |
| w_wad | w_wad.h, w_wad.c |
| z_zone | z_zone.h, z_zone.c |

### 1.2 Header-Only Modules (x.h only → x.rs)

No corresponding `.c` file; module contains only declarations.

| Module | File |
|--------|------|
| config | config.h |
| deh_main | deh_main.h |
| deh_misc | deh_misc.h |
| deh_str | deh_str.h |
| d_englsh | d_englsh.h |
| doom | doom.h |
| doomdata | doomdata.h |
| doomfeatures | doomfeatures.h |
| doomkeys | doomkeys.h |
| doomtype | doomtype.h |
| d_player | d_player.h |
| d_textur | d_textur.h |
| d_think | d_think.h |
| d_ticcmd | d_ticcmd.h |
| i_swap | i_swap.h |
| net_client | net_client.h |
| net_dedicated | net_dedicated.h |
| net_defs | net_defs.h |
| net_gui | net_gui.h |
| net_io | net_io.h |
| net_loop | net_loop.h |
| net_packet | net_packet.h |
| net_query | net_query.h |
| net_server | net_server.h |
| p_local | p_local.h |
| r_defs | r_defs.h |
| r_local | r_local.h |
| r_state | r_state.h |
| v_patch | v_patch.h |
| w_merge | w_merge.h |

### 1.3 C-Only Modules (x.c only → x.rs)

No corresponding `.h` file; module contains only implementation.

| Module | File |
|--------|------|
| d_net | d_net.c |
| dummy | dummy.c |
| i_allegromusic | i_allegromusic.c |
| i_allegrosound | i_allegrosound.c |
| icon | icon.c |
| i_input | i_input.c |
| p_ceilng | p_ceilng.c |
| p_doors | p_doors.c |
| p_enemy | p_enemy.c |
| p_floor | p_floor.c |
| p_lights | p_lights.c |
| p_map | p_map.c |
| p_maputl | p_maputl.c |
| p_plats | p_plats.c |
| p_sight | p_sight.c |
| p_switch | p_switch.c |
| p_telept | p_telept.c |
| p_user | p_user.c |
| w_file_stdc | w_file_stdc.c |

---

## 2. Conversion Rules

### 2.1 Imports

| C | Rust | Notes |
|---|------|-------|
| `#include "y.h"` in x.h | `pub use y::*;` | Public re-export of module |
| `#include "y.h"` in x.c | `use y::*;` | Private use |

System headers (`#include <stdio.h>`, etc.) become Rust crates or `std` imports as appropriate.

### 2.2 Constants

| C | Rust | Visibility |
|---|------|------------|
| `#define FOO 42` in x.h | `pub static FOO: i32 = 42;` | pub |
| `#define FOO 42` in x.c | `static FOO: i32 = 42;` | private |

Add a comment with the original macro name:

```rust
// #define SCREENWIDTH 320
pub static SCREENWIDTH: i32 = 320;
```

### 2.3 Macro Functions

| C | Rust | Visibility |
|---|------|------------|
| `#define M_Foo(x) ((x)*2)` in x.h | `pub fn m_foo(x: i32) -> i32 { x * 2 }` | pub |
| Same in x.c | `fn m_foo(x: i32) -> i32 { x * 2 }` | private |

Add a comment with the original macro name.

### 2.4 Type Aliases

| C | Rust | Visibility |
|---|------|------------|
| `typedef int fixed_t;` in x.h | `pub type FixedT = i32;` | pub |
| Same in x.c | `type FixedT = i32;` | private |

Add a comment with the original type name.

Example:

```rust
// typedef fixed_t
pub type FixedT = i32;
```

### 2.5 Struct and Enum Definitions

| C | Rust | Visibility |
|---|------|------------|
| `typedef struct { ... } name_t;` in x.h | `pub struct NameT { ... }` | pub |
| Same in x.c | `struct NameT { ... }` | private |
| `typedef enum { A, B } name_e;` in x.h | `pub enum NameE { A, B }` | pub |
| Same in x.c | `enum NameE { A, B }` | private |

Add a comment with the original type name. Add a comment with the original field name for each struct field, or the original variant name for each enum variant.

Example:

```rust
// typedef struct thinker_s
pub struct ThinkerS {
    // struct thinker_s* prev
    prev: Option<Arc<Mutex<ThinkerS>>>,
    // struct thinker_s* next
    next: Option<Arc<Mutex<ThinkerS>>>,
    // think_t function
    function: ThinkT,
}
```

### 2.6 Union Definitions

| C | Rust | Visibility |
|---|------|------------|
| `typedef union { ... } name_u;` in x.h | `pub enum NameU { VariantA(...), VariantB(...) }` | pub |
| Same in x.c | `enum NameU { ... }` | private |

Always use Rust enums with variants; never use `union` or unsafe code. Add a comment with the original type name and field names (as variant names).

### 2.7 Global Variables

| C | Rust |
|---|------|
| `int global_var;` | `Arc<Mutex<i32>>` inside `X_State` |
| `byte *ptr;` | `Arc<Mutex<Option<Vec<u8>>>>` or similar |
| `extern byte *other;` | In the owning module: `fn other(&self) -> Arc<Mutex<T>>` with `// extern other` comment; using modules call this getter |

- Create a struct `X_State` per module `x` containing all globals as fields.
- Use `Arc<Mutex<T>>` for interior mutability (since methods take `&self`).
- Add comments with original variable names.
- For each `extern` variable: the owning module (where it is defined) must provide an argument-less getter `fn name(&self) -> Arc<Mutex<T>>` that returns the wrapped value; using modules call this getter instead of holding a reference. Add a comment `// extern <c-variable-name>` at the getter.

Example:

```rust
pub struct V_VideoState {
    // byte *tinttable
    pub tinttable: Arc<Mutex<Option<Vec<u8>>>>,
    // byte *xlatab
    pub xlatab: Arc<Mutex<Option<Vec<u8>>>>,
    // static byte *dest_screen
    dest_screen: Arc<Mutex<Option<Vec<u8>>>>,
    // int dirtybox[4]
    pub dirtybox: Arc<Mutex<[i32; 4]>>,
}
```

### 2.8 Functions

| C | Rust |
|---|------|
| Function in x.h | `pub fn foo(&self, ...) -> T` |
| Function in x.c only | `fn foo(&self, ...) -> T` |
| `void f(...)` | `fn f(&self, ...)` |
| `int f(...)` | `fn f(&self, ...) -> i32` |
| `T* f(...)` | `fn f(&self, ...) -> Arc<Mutex<T>>` or appropriate type |
| Pointer args | References `&T` or `&mut T` as needed |

- Migrate as methods on `X_State`.
- Use `&self`, never `&mut self` (mutability via `Arc<Mutex<...>>`).
- Add comment with original function name.
- Do **not** refactor C logic; instead:
  1. Copy the C function body as a comment.
  2. Implement the Rust body as `todo!("Basic stage-0 stub")` for now.

Example:

```rust
/// Original: void V_MarkRect(int x, int y, int width, int height)
pub fn v_mark_rect(&self, x: i32, y: i32, width: i32, height: i32) {
    // C body:
    // if (dest_screen == I_VideoBuffer) {
    //     M_AddToBox(dirtybox, x, y);
    //     M_AddToBox(dirtybox, x + width-1, y + height-1);
    // }
    todo!("Basic stage-0 stub")
}
```

### 2.9 Static Inline Functions

Migrate to freestanding Rust functions (not methods on `X_State`). Use `pub fn` if from `.h`, `fn` if from `.c`.

| C | Rust | Visibility |
|---|------|------------|
| `static inline unsigned short swapLE16(unsigned short val)` in x.h | `#[inline] pub fn swap_le16(val: u16) -> u16` | pub |
| Same in x.c | `#[inline] fn swap_le16(val: u16) -> u16` | private |

Add a comment with the original function name. Copy the C body as a comment.

Example:

```rust
// static inline unsigned short swapLE16(unsigned short val)
#[inline]
pub fn swap_le16(val: u16) -> u16 {
    // C body:
    // return (val>>8) | (val<<8);
    todo!("Basic stage-0 stub")
}
```

### 2.10 Preprocessor Token Concatenation

| C | Rust |
|---|------|
| `#define PASTE(a,b) a##b` | Use macros: `macro_rules! paste { ($a:ident, $b:ident) => { ... } }` |

Example (sha1.c): `#define X(a) do { *p++ = hd->h##a >> 24; ... } while(0)` → expand to explicit field access (e.g. `hd.h0`, `hd.h1`, …) or a macro.

### 2.11 Include Guards

| C | Rust |
|---|------|
| `#ifndef __X_H__` / `#define __X_H__` / `#endif` | Not needed (Rust modules) |

### 2.12 Packed / Alignment

| C | Rust |
|---|------|
| `PACKEDATTR` / `__attribute__((packed))` | `#[repr(C, packed)]` |

### 2.13 Function Pointers

| C | Rust |
|---|------|
| `typedef void (*actionf_v)();` | `type ActionfV = fn();` or `Box<dyn Fn()>` |
| `typedef void (*actionf_p1)(void*);` | `type ActionfP1 = fn(*mut c_void);` |
| Callback in struct | `Option<Box<dyn Fn(...)>>` or raw function pointer |

Add a comment with the original type name. Use `pub type` if from `.h`, `type` if from `.c`.

Example (`d_think.h`):

```rust
// typedef void (*actionf_v)();
pub type ActionfV = fn();

// typedef void (*actionf_p1)(void*);
pub type ActionfP1 = fn(*mut c_void);
```

### 2.14 Variadic Functions

| C | Rust |
|---|------|
| `void I_Error(char *error, ...);` | Use `format!` or `std::fmt::Arguments` |
| `char *M_StringJoin(const char *s, ...);` | Use variadic-like API or iterator |

Rust approach: replace with `fn i_error(&self, fmt: impl std::fmt::Display)` or `fn i_error(&self, args: std::fmt::Arguments)`.

---

## 3. Additional C Constructs (Not in Original List)

These C constructions also need migration rules:

### 3.1 Platform / Compiler Conditionals

| C | Rust |
|---|------|
| `#ifdef _WIN32` | Remove (unused); use single code path |
| `#ifdef __GNUC__` | Remove (unused); use single code path |
| `#if defined(X) \|\| defined(Y)` | Remove if unused; otherwise include C code directly |

---

## 4. Feature Switches

**Used features: migrate these code paths directly. Do not create Rust features (`#[cfg(feature = "...")]`); just use the C code.**
- `SYS_LITTLE_ENDIAN` — little-endian paths (i_swap.h, sha1.c, i_video.c)
- `RANGECHECK` — bounds checking (doomdef.h, v_video.c, r_plane.c, r_draw.c, r_bsp.c, r_segs.c, r_things.c, p_sight.c)
- `ORIGCODE` — unmodified original code paths (i_sound.c, i_cdmus.c, i_joystick.c, i_endoom.c, i_system.c)
- `DOOM_GENERIC` — main guard; `DOOMGENERIC_RESX` (640), `DOOMGENERIC_RESY` (400)

**Unused features: remove the corresponding `#ifdef`'d code entirely during migration.** Do not migrate:
- `FEATURE_WAD_MERGE`, `FEATURE_DEHACKED`, `FEATURE_MULTIPLAYER`, `FEATURE_SOUND` (doomfeatures)
- `HAVE_LIBPNG`, `HAVE_MMAP`, `HAVE_LIBZ`
- `RANGECHECKING`, `CHECK_MUS_HEADER`
- `STANDALONE`, `HAVE_DEV_ISA_SPKRIO_H`, `HAVE_DEV_SPEAKER_SPEAKER_H`, `HAVE_LINUX_KD_H`, `HAVE_IOPERM`
- `_WIN32`, `__MACOSX__`, `__DJGPP__`, `__GNUC__`, `__MSC_VER` (platform/compiler)
- `__cplusplus`
- `CMAP256`, `SYS_BIG_ENDIAN` (pixel format / endianness)

### 4.1 Used Features (include C code directly)

| Switch | Action |
|--------|--------|
| `SYS_LITTLE_ENDIAN` | Use the little-endian C code path directly |
| `RANGECHECK` | Include bounds-checking code (e.g. `debug_assert!` or equivalent) |
| `ORIGCODE` | Use the original C code path directly |
| `DOOM_GENERIC` | Module visibility; `DOOMGENERIC_RESX`/`RESY` as `const` |

### 4.2 Build Defaults (doomgeneric.h)

| Switch | Action |
|--------|--------|
| `DOOMGENERIC_RESX` | `const` 640 |
| `DOOMGENERIC_RESY` | `const` 400 |

### 4.3 Complete List (by file)

*Code under unused features (see list above): remove.*

```
config.h:        HAVE_DEV_ISA_SPKRIO_H, HAVE_DEV_SPEAKER_SPEAKER_H, HAVE_INTTYPES_H,
                 HAVE_IOPERM, HAVE_LIBAMD64, HAVE_LIBI386, HAVE_LIBM, HAVE_LIBPNG,
                 HAVE_LIBSAMPLERATE, HAVE_LIBZ, HAVE_LINUX_KD_H, HAVE_MEMORY_H,
                 HAVE_MMAP, HAVE_SCHED_SETAFFINITY, HAVE_STDINT_H, HAVE_STDLIB_H,
                 HAVE_STRINGS_H, HAVE_STRING_H, HAVE_SYS_STAT_H, HAVE_SYS_TYPES_H,
                 HAVE_UNISTD_H, ORIGCODE, STDC_HEADERS

doomfeatures.h:  FEATURE_WAD_MERGE, FEATURE_DEHACKED, FEATURE_MULTIPLAYER, FEATURE_SOUND

doomgeneric.h:   DOOM_GENERIC, DOOMGENERIC_RESX, DOOMGENERIC_RESY, CMAP256, __cplusplus

doomdef.h:       RANGECHECK

deh_str.h:       FEATURE_DEHACKED
deh_misc.h:      FEATURE_DEHACKED
doomtype.h:      _WIN32, __GNUC__, __cplusplus, (_WIN32 || __DJGPP__)
dummy.c:         FEATURE_SOUND
i_sound.c:       FEATURE_SOUND, ORIGCODE
i_sound.h:       FEATURE_SOUND
i_cdmus.c:       ORIGCODE
i_joystick.c:    ORIGCODE
i_endoom.c:      ORIGCODE, __DJGPP__
i_swap.h:        __DJGPP__, __GNUC__
i_scale.c:       (_MSC_VER && !__cplusplus)
w_file.c:        _WIN32, HAVE_MMAP
w_main.c:        FEATURE_WAD_MERGE
d_iwad.c:        (_WIN32 && !_WIN32_WCE), _WIN32
m_config.c:      FEATURE_MULTIPLAYER, FEATURE_SOUND
d_loop.c:        FEATURE_MULTIPLAYER
d_main.c:        FEATURE_MULTIPLAYER, FEATURE_DEHACKED, _WIN32
m_misc.c:        _WIN32, _MSC_VER
r_plane.c:       RANGECHECK
r_draw.c:        RANGECHECK
r_bsp.c:         RANGECHECK
r_segs.c:        RANGECHECK
r_things.c:      RANGECHECK
p_sight.c:       RANGECHECK
wi_stuff.c:      RANGECHECKING
v_video.c:       HAVE_LIBPNG, RANGECHECK
i_video.c:       CMAP256, SYS_BIG_ENDIAN
sha1.c:          SYS_BIG_ENDIAN
mus2mid.c:       CHECK_MUS_HEADER, STANDALONE
i_system.c:      _WIN32, ORIGCODE, __MACOSX__
```

---

## 5. Suggested Migration Order

Order by total module size (shortest first). Sound and networking excluded. Add stub references as needed until dependencies are migrated.

**Tier 1 (shortest, &lt;150 lines)**  
`doomfeatures`, `doom`, `d_textur`, `w_merge`, `r_local`, `deh_str`, `deh_main`, `v_patch`, `dummy`, `d_ticcmd`, `d_think`, `i_swap`, `doomgeneric`, `deh_misc`, `r_sky`, `w_file_stdc`, `doomkeys`, `config`, `m_bbox`, `m_fixed`, `doomtype`, `m_random`, `dstrings`, `w_checksum`, `r_state`

**Tier 2 (150–350 lines)**  
`i_endoom`, `p_telept`, `i_timer`, `m_cheat`, `d_items`, `doomdef`, `d_event`, `d_player`, `doomdata`, `w_file`, `p_tick`, `w_main`, `memio`

**Tier 3 (350–600 lines)**  
`icon`, `p_local`, `p_plats`, `d_mode`, `m_argv`, `doomstat`, `p_ceilng`, `i_input`, `p_lights`, `p_sight`, `f_wipe`, `sha1`, `p_user`

**Tier 4 (600–1100 lines)**  
`i_joystick`, `statdump`, `r_defs`, `st_lib`, `r_plane`, `hu_lib`, `p_floor`, `z_zone`, `m_controls`, `m_misc`, `r_bsp`, `p_switch`, `i_system`, `i_video`, `w_wad`, `d_englsh`, `hu_stuff`, `mus2mid`, `f_finale`, `r_segs`, `p_doors`, `p_setup`, `d_iwad`, `p_inter`, `p_pspr`, `r_data`, `p_maputl`, `v_video`

**Tier 5 (1100–2500 lines)**  
`r_things`, `r_main`, `r_draw`, `p_mobj`, `am_map`, `p_map`, `i_scale`, `st_stuff`, `wi_stuff`, `d_main`, `p_saveg`, `p_enemy`, `p_spec`, `m_config`, `m_menu`, `tables`, `g_game`

**Tier 6 (2500+ lines)**  
`info`

---

## 6. Summary

- **~56** paired modules, **~30** header-only modules, **~19** C-only modules.
- Each module becomes `x.rs` with `X_State` holding globals as `Arc<Mutex<T>>`.
- Functions become `&self` methods; bodies start as `todo!("Basic stage-0 stub")` with C code in comments.
- For used features (`SYS_LITTLE_ENDIAN`, `RANGECHECK`, `ORIGCODE`, `DOOM_GENERIC`): include C code directly. Do not create Rust features.
- Additional rules cover structs, enums, unions, function pointers, variadics, packed types, and platform conditionals.
