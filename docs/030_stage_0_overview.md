# Stage 0 Migration Overview

Track migration status for each C module → Rust `x.rs`. See [010_basic_plan.md](010_basic_plan.md) for conversion rules.

**Status:** `not started` | `in progress` | `done`

---

## Tier 1 (shortest, <150 lines)

| Module | Status |
|--------|--------|
| config | done |
| dstrings | done |
| d_think | done |
| d_ticcmd | done |
| d_textur | done |
| deh_main | done |
| deh_misc | done |
| deh_str | done |
| doom | done |
| doomfeatures | done |
| doomgeneric | done |
| doomkeys | done |
| doomtype | done |
| dummy | done |
| i_swap | done |
| m_bbox | done |
| m_fixed | done |
| m_random | done |
| r_local | done |
| r_sky | done |
| r_state | done |
| v_patch | done |
| w_checksum | done |
| w_file_stdc | done |
| w_merge | done |

## Tier 2 (150–350 lines)

| Module | Status |
|--------|--------|
| d_event | done |
| d_items | done |
| d_mode | done |
| d_player | done |
| doomdata | done |
| doomdef | done |
| i_endoom | done |
| i_timer | done |
| m_cheat | done |
| memio | done |
| p_telept | done |
| p_tick | done |
| w_file | done |
| w_main | done |

## Tier 3 (350–600 lines)

| Module | Status |
|--------|--------|
| d_mode | done |
| doomstat | done |
| f_wipe | done |
| i_input | done |
| icon | done |
| m_argv | done |
| p_ceilng | done |
| p_lights | done |
| p_local | done |
| p_plats | done |
| p_sight | done |
| p_user | done |
| sha1 | done |

## Tier 4 (600–1100 lines)

| Module | Status |
|--------|--------|
| d_englsh | done |
| d_iwad | done |
| f_finale | done |
| hu_lib | done |
| hu_stuff | done |
| i_joystick | done |
| i_system | done |
| i_video | done |
| m_controls | done |
| m_misc | done |
| mus2mid | done |
| p_doors | done |
| p_floor | done |
| p_inter | done |
| p_maputl | done |
| p_pspr | done |
| p_setup | done |
| p_switch | done |
| r_bsp | done |
| r_data | done |
| r_defs | done |
| r_plane | done |
| r_segs | done |
| st_lib | done |
| statdump | done |
| v_video | done |
| w_wad | done |
| z_zone | done |

## Tier 5 (1100–2500 lines)

| Module | Status |
|--------|--------|
| am_map | done |
| d_main | done |
| g_game | done |
| i_scale | done |
| m_config | done |
| m_menu | done |
| p_enemy | done |
| p_map | done |
| p_mobj | done |
| p_saveg | done |
| p_spec | done |
| r_draw | done |
| r_main | done |
| r_things | done |
| st_stuff | done |
| tables | done |
| wi_stuff | done |

## Tier 6 (2500+ lines)

| Module | Status |
|--------|--------|
| info | done |

---

## Excluded (sound, networking)

These modules are not in the migration scope.

| Module | Status |
|--------|--------|
| d_loop | excluded |
| d_net | excluded |
| gusconf | excluded |
| i_allegromusic | excluded |
| i_allegrosound | excluded |
| i_cdmus | excluded |
| i_sound | excluded |
| net_client | excluded |
| net_dedicated | excluded |
| net_defs | excluded |
| net_gui | excluded |
| net_io | excluded |
| net_loop | excluded |
| net_packet | excluded |
| net_query | excluded |
| net_server | excluded |
| s_sound | excluded |
| sounds | excluded |
