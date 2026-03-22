# Stage 0 Migration Overview

Track migration status for each C module → Rust `x.rs`. See [010_basic_plan.md](010_basic_plan.md) for conversion rules.

**Status:** `not started` | `in progress` | `done`

**Restart:** All Rust code removed and Tier 1 restarted per plan. See [040_plan_deviations.md](040_plan_deviations.md).

---

## Tier 1 (shortest, <150 lines)

| Module | Status |
|--------|--------|
| config | done |
| dstrings | not started |
| d_think | done |
| d_ticcmd | done |
| d_textur | done |
| deh_main | not started |
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
| r_local | not started |
| r_sky | not started |
| r_state | not started |
| v_patch | done |
| w_checksum | not started |
| w_file_stdc | not started |
| w_merge | done |

## Tier 2 (150–350 lines)

| Module | Status |
|--------|--------|
| d_event | not started |
| d_items | not started |
| d_mode | not started |
| d_player | not started |
| doomdata | not started |
| doomdef | not started |
| i_endoom | not started |
| i_timer | not started |
| m_cheat | not started |
| memio | not started |
| p_telept | not started |
| p_tick | not started |
| w_file | not started |
| w_main | not started |

## Tier 3 (350–600 lines)

| Module | Status |
|--------|--------|
| d_mode | not started |
| doomstat | not started |
| f_wipe | not started |
| i_input | not started |
| icon | not started |
| m_argv | not started |
| p_ceilng | not started |
| p_lights | not started |
| p_local | not started |
| p_plats | not started |
| p_sight | not started |
| p_user | not started |
| sha1 | not started |

## Tier 4 (600–1100 lines)

| Module | Status |
|--------|--------|
| d_englsh | not started |
| d_iwad | not started |
| f_finale | not started |
| hu_lib | not started |
| hu_stuff | not started |
| i_joystick | not started |
| i_system | not started |
| i_video | not started |
| m_controls | not started |
| m_misc | not started |
| mus2mid | not started |
| p_doors | not started |
| p_floor | not started |
| p_inter | not started |
| p_maputl | not started |
| p_pspr | not started |
| p_setup | not started |
| p_switch | not started |
| r_bsp | not started |
| r_data | not started |
| r_defs | not started |
| r_plane | not started |
| r_segs | not started |
| st_lib | not started |
| statdump | not started |
| v_video | not started |
| w_wad | not started |
| z_zone | not started |

## Tier 5 (1100–2500 lines)

| Module | Status |
|--------|--------|
| am_map | not started |
| d_main | not started |
| g_game | not started |
| i_scale | not started |
| m_config | not started |
| m_menu | not started |
| p_enemy | not started |
| p_map | not started |
| p_mobj | not started |
| p_saveg | not started |
| p_spec | not started |
| r_draw | not started |
| r_main | not started |
| r_things | not started |
| st_stuff | not started |
| tables | not started |
| wi_stuff | not started |

## Tier 6 (2500+ lines)

| Module | Status |
|--------|--------|
| info | not started |

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
