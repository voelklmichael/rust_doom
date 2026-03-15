# Header Translation Plan

This document outlines the plan for translating doomgeneric C header files to Rust modules.

## Translation Rules

| C Construct | Rust Equivalent |
|-------------|-----------------|
| `#include "xxx.h"` | `use crate::xxx::*;` |
| Function declaration | `pub fn name(...) -> ReturnType { todo!("original: C_function_name") }` |
| Type definition (struct, enum, typedef) | `pub struct` / `pub enum` / `pub type` |
| Macro function (e.g. `#define FOO(x) ...`) | `pub fn foo(x: ...) -> ... { /* C macro: FOO */ ... }` |
| `#define STRING "value"` | `pub static STRING: &str = "value";` |
| `#define CONSTANT value` | `pub const CONSTANT: i32 = value;` (or appropriate type) |
| `extern type name[]` | `pub static mut name: [Type; N]` or appropriate |
| `PACKEDATTR` / `__attribute__((packed))` | `#[repr(C, packed)]` on struct |

## Special Cases / Questions

- **Platform-specific defines** (e.g. `DIR_SEPARATOR`, `PATH_SEPARATOR`): Use `#[cfg(target_os = "windows")]` or `std::path::MAIN_SEPARATOR`
- **Feature flags** (`#undef FEATURE_*` in doomfeatures.h): Use `#[cfg(feature = "...")]` or const
- **C stdlib includes** (`<stdio.h>`, `<string.h>`, etc.): Use Rust std equivalents; omit or use `std::ffi` for C interop types
- **Forward declarations**: Rust handles via type definitions; use `Option` or raw pointers as needed for self-referential structs

## Dependency Order (Translation Order)

Headers must be translated in dependency order (dependencies first). Based on `#include` analysis:

### Tier 1 - No doom includes (or only C stdlib)
1. **doomtype** - Base types (byte, boolean); has platform macros
2. **m_fixed** - Fixed-point math; no includes
3. **i_timer** - Timer interface; no includes
4. **doomfeatures** - Feature flags (#undef only)
5. **doomgeneric** - stdlib only

### Tier 2 - Tier 1 dependencies
6. **d_mode** - doomtype
7. **d_think** - (none)
8. **d_ticcmd** - doomtype
9. **d_event** - doomtype
10. **m_argv** - doomtype
11. **m_config** - doomtype
12. **m_random** - doomtype
13. **i_sound** - doomtype
14. **i_scale** - doomtype
15. **i_video** - doomtype
16. **gusconf** - doomtype
17. **d_textur** - doomtype
18. **sha1** - doomtype
19. **w_checksum** - doomtype
20. **tables** - doomtype, m_fixed
21. **net_gui** - doomtype

### Tier 3 - Tier 2 dependencies
22. **doomdef** - doomtype, i_timer, d_mode
23. **memio** - doomtype (mus2mid)
24. **mus2mid** - doomtype, memio
25. **deh_misc** - doomfeatures
26. **deh_str** - doomfeatures
27. **net_defs** - doomtype, d_ticcmd, sha1
28. **net_client** - doomtype, d_ticcmd, sha1, net_defs
29. **net_loop** - net_defs
30. **net_io** - net_defs
31. **net_query** - net_defs
32. **d_loop** - net_defs
33. **m_bbox** - m_fixed
34. **m_misc** - doomtype
35. **m_menu** - d_event
36. **hu_stuff** - d_event
37. **f_finale** - doomtype, d_event
38. **am_map** - d_event, m_cheat
39. **i_system** - d_ticcmd, d_event

### Tier 4 - Core data structures
40. **doomdata** - doomtype, doomdef
41. **info** - d_think
42. **m_cheat** - (needed by am_map, st_stuff)
43. **d_items** - doomdef
44. **d_iwad** - d_mode
45. **w_file** - doomtype
46. **deh_main** - doomtype, doomfeatures, deh_str, sha1

### Tier 5 - Game/rendering core
47. **p_mobj** - tables, m_fixed, d_think, doomdata, info
48. **p_pspr** - m_fixed, tables, info
49. **r_defs** - doomdef, m_fixed, d_think, p_mobj, i_video, v_patch
50. **r_state** - d_player, r_data
51. **r_data** - r_defs, r_state
52. **v_patch** - (check deps)
53. **d_player** - d_items, p_pspr, p_mobj, d_ticcmd, net_defs
54. **sounds** - i_sound
55. **s_sound** - p_mobj, sounds
56. **st_lib** - r_defs
57. **st_stuff** - doomtype, d_event, m_cheat
58. **w_wad** - doomtype, d_mode, w_file
59. **r_plane** - r_data
60. **r_main** - d_player, r_data
61. **r_draw** - (r_defs)
62. **r_things** - (r_defs)
63. **r_segs** - (r_defs)
64. **r_bsp** - (r_defs)
65. **r_local** - tables, doomdef, r_data, r_main, r_bsp, r_segs, r_plane, r_things, r_draw
66. **p_local** - r_local, p_spec
67. **p_spec** - (check)
68. **p_setup** - (check)
69. **p_saveg** - (stdio)
70. **p_inter** - (check)
71. **p_tick** - (check)
72. **g_game** - doomdef, d_event, d_ticcmd
73. **d_main** - doomdef
74. **wi_stuff** - doomdef
75. **v_video** - doomtype, v_patch
76. **doomstat** - doomdata, d_loop, d_player, d_mode, net_defs
77. **dstrings** - d_englsh
78. **d_englsh** - (large string table)
79. **config** - (check)
80. **doomkeys** - (check)
81. **doom** - (main)
82. **Remaining** - f_wipe, hu_lib, net_packet, net_server, net_dedicated, w_main, w_merge, z_zone, statdump, etc.

## Progress Tracker

- [x] **Tier 1** – doomtype, m_fixed, i_timer, doomfeatures, doomgeneric
- [x] **Tier 2** – d_mode, d_think, d_ticcmd, d_event, m_argv, m_config, m_random, i_sound, i_scale, i_video, gusconf, d_textur, sha1, w_checksum, tables, net_gui
- [x] **Tier 3** – doomdef, memio, mus2mid, deh_misc, deh_str, net_defs, net_client, net_loop, net_io, net_query, d_loop, m_bbox, m_misc, m_menu, hu_stuff, f_finale, am_map, i_system
- [x] **Tier 4** – doomdata, info, m_cheat, d_items, d_iwad, w_file, deh_main
- [x] **Tier 5** – p_mobj, p_pspr, r_defs, r_state, r_data, v_patch, d_player, sounds, s_sound, st_lib, st_stuff, w_wad, r_plane, r_main, r_draw, r_things, r_segs, r_bsp, r_local, p_local, p_spec, p_setup, p_saveg, p_inter, p_tick, g_game, d_main, wi_stuff, v_video, doomstat, dstrings, d_englsh, doom, f_wipe, hu_lib, w_main, w_merge, z_zone, statdump, i_cdmus, i_endoom, i_joystick, i_swap

### Translated (78 headers)
All doomgeneric header files have been translated to Rust with full type definitions, function stubs, and proper dependency handling.

## Notes

- Some headers have circular or complex dependencies (e.g. r_data ↔ r_state); may need to split or use `Option`/raw pointers
- `info.h` is very large (~1100 lines) - consider splitting or generating
- `d_englsh.h` contains many string constants - translate to `pub static X: &str = "...";`
