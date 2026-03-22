# Stage 0 Migration Overview

Track migration status for each C module → Rust `x.rs`. See [010_basic_plan.md](010_basic_plan.md) for conversion rules.

**Status:** `not started` | `in progress` | `done`

---

## Tier 1 (shortest, <150 lines)

| Module | Status |
|--------|--------|
| doomfeatures | not started |
| doom | not started |
| d_textur | not started |
| w_merge | not started |
| r_local | not started |
| deh_str | not started |
| deh_main | not started |
| v_patch | not started |
| dummy | not started |
| d_ticcmd | not started |
| d_think | not started |
| i_swap | not started |
| doomgeneric | not started |
| deh_misc | not started |
| r_sky | not started |
| w_file_stdc | not started |
| doomkeys | not started |
| config | not started |
| m_bbox | not started |
| m_fixed | not started |
| doomtype | not started |
| m_random | not started |
| dstrings | not started |
| w_checksum | not started |
| r_state | not started |

## Tier 2 (150–350 lines)

| Module | Status |
|--------|--------|
| i_endoom | not started |
| p_telept | not started |
| i_timer | not started |
| m_cheat | not started |
| d_items | not started |
| doomdef | not started |
| d_event | not started |
| d_player | not started |
| doomdata | not started |
| w_file | not started |
| p_tick | not started |
| w_main | not started |
| memio | not started |

## Tier 3 (350–600 lines)

| Module | Status |
|--------|--------|
| icon | not started |
| p_local | not started |
| p_plats | not started |
| d_mode | not started |
| m_argv | not started |
| doomstat | not started |
| p_ceilng | not started |
| i_input | not started |
| p_lights | not started |
| p_sight | not started |
| f_wipe | not started |
| sha1 | not started |
| p_user | not started |

## Tier 4 (600–1100 lines)

| Module | Status |
|--------|--------|
| i_joystick | not started |
| statdump | not started |
| r_defs | not started |
| st_lib | not started |
| r_plane | not started |
| hu_lib | not started |
| p_floor | not started |
| z_zone | not started |
| m_controls | not started |
| m_misc | not started |
| r_bsp | not started |
| p_switch | not started |
| i_system | not started |
| i_video | not started |
| w_wad | not started |
| d_englsh | not started |
| hu_stuff | not started |
| mus2mid | not started |
| f_finale | not started |
| r_segs | not started |
| p_doors | not started |
| p_setup | not started |
| d_iwad | not started |
| p_inter | not started |
| p_pspr | not started |
| r_data | not started |
| p_maputl | not started |
| v_video | not started |

## Tier 5 (1100–2500 lines)

| Module | Status |
|--------|--------|
| r_things | not started |
| r_main | not started |
| r_draw | not started |
| p_mobj | not started |
| am_map | not started |
| p_map | not started |
| i_scale | not started |
| st_stuff | not started |
| wi_stuff | not started |
| d_main | not started |
| p_saveg | not started |
| p_enemy | not started |
| p_spec | not started |
| m_config | not started |
| m_menu | not started |
| tables | not started |
| g_game | not started |

## Tier 6 (2500+ lines)

| Module | Status |
|--------|--------|
| info | not started |

---

## Excluded (sound, networking)

These modules are not in the migration scope.

| Module | Status |
|--------|--------|
| i_sound | excluded |
| i_cdmus | excluded |
| i_allegromusic | excluded |
| i_allegrosound | excluded |
| s_sound | excluded |
| sounds | excluded |
| gusconf | excluded |
| d_net | excluded |
| d_loop | excluded |
| net_client | excluded |
| net_dedicated | excluded |
| net_defs | excluded |
| net_gui | excluded |
| net_io | excluded |
| net_loop | excluded |
| net_packet | excluded |
| net_query | excluded |
| net_server | excluded |
