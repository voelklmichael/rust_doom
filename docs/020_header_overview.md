# Header Migration Overview

Status tracking for C header → Rust module conversion. See [000_overall_plan](000_overall_plan) for the translation rules.

**Status values:** `not started` | `stubbed` | `ongoing` | `done`

---

## Core types

| Header | Module | Status |
|--------|--------|--------|
| doom.h | doom | done |
| doomdata.h | doomdata | not started |
| doomdef.h | doomdef | not started |
| doomfeatures.h | doomfeatures | done |
| doomgeneric.h | doomgeneric | done |
| doomkeys.h | doomkeys | done |
| doomstat.h | doomstat | not started |
| doomtype.h | doomtype | done |

---

## Data structures

| Header | Module | Status |
|--------|--------|--------|
| d_englsh.h | d_englsh | done |
| d_event.h | d_event | not started |
| d_items.h | d_items | not started |
| d_loop.h | d_loop | not started |
| d_main.h | d_main | stubbed |
| d_mode.h | d_mode | not started |
| d_player.h | d_player | not started |
| d_think.h | d_think | done |
| d_ticcmd.h | d_ticcmd | not started |
| d_textur.h | d_textur | not started |
| d_iwad.h | d_iwad | not started |
| dstrings.h | dstrings | not started |
| m_bbox.h | m_bbox | not started |
| m_fixed.h | m_fixed | not started |
| m_random.h | m_random | not started |

---

## Rendering

| Header | Module | Status |
|--------|--------|--------|
| r_bsp.h | r_bsp | not started |
| r_data.h | r_data | not started |
| r_defs.h | r_defs | not started |
| r_draw.h | r_draw | not started |
| r_local.h | r_local | not started |
| r_main.h | r_main | not started |
| r_plane.h | r_plane | not started |
| r_segs.h | r_segs | not started |
| r_sky.h | r_sky | not started |
| r_state.h | r_state | not started |
| r_things.h | r_things | not started |
| v_patch.h | v_patch | done |
| v_video.h | v_video | not started |

---

## Game logic

| Header | Module | Status |
|--------|--------|--------|
| g_game.h | g_game | not started |
| info.h | info | not started |
| p_inter.h | p_inter | not started |
| p_local.h | p_local | not started |
| p_mobj.h | p_mobj | not started |
| p_pspr.h | p_pspr | not started |
| p_saveg.h | p_saveg | not started |
| p_setup.h | p_setup | not started |
| p_spec.h | p_spec | not started |
| p_tick.h | p_tick | not started |

---

## UI

| Header | Module | Status |
|--------|--------|--------|
| am_map.h | am_map | not started |
| f_finale.h | f_finale | not started |
| f_wipe.h | f_wipe | not started |
| hu_lib.h | hu_lib | not started |
| hu_stuff.h | hu_stuff | not started |
| m_argv.h | m_argv | stubbed |
| m_cheat.h | m_cheat | not started |
| m_config.h | m_config | not started |
| m_controls.h | m_controls | not started |
| m_menu.h | m_menu | not started |
| m_misc.h | m_misc | not started |
| st_lib.h | st_lib | not started |
| st_stuff.h | st_stuff | not started |
| wi_stuff.h | wi_stuff | not started |

---

## I/O & system

| Header | Module | Status |
|--------|--------|--------|
| i_cdmus.h | i_cdmus | not started |
| i_endoom.h | i_endoom | not started |
| i_joystick.h | i_joystick | not started |
| i_scale.h | i_scale | not started |
| i_sound.h | i_sound | not started |
| i_swap.h | i_swap | done |
| i_system.h | i_system | not started |
| i_timer.h | i_timer | not started |
| i_video.h | i_video | not started |

---

## WAD / files

| Header | Module | Status |
|--------|--------|--------|
| w_checksum.h | w_checksum | not started |
| w_file.h | w_file | not started |
| w_main.h | w_main | not started |
| w_merge.h | w_merge | done |
| w_wad.h | w_wad | not started |

---

## Networking

| Header | Module | Status |
|--------|--------|--------|
| net_client.h | net_client | not started |
| net_dedicated.h | net_dedicated | done |
| net_defs.h | net_defs | not started |
| net_gui.h | net_gui | not started |
| net_io.h | net_io | not started |
| net_loop.h | net_loop | not started |
| net_packet.h | net_packet | not started |
| net_query.h | net_query | not started |
| net_server.h | net_server | done |

---

## Dehacked

| Header | Module | Status |
|--------|--------|--------|
| deh_main.h | deh_main | not started |
| deh_misc.h | deh_misc | not started |
| deh_str.h | deh_str | not started |

---

## Misc

| Header | Module | Status |
|--------|--------|--------|
| config.h | config | done |
| gusconf.h | gusconf | not started |
| memio.h | memio | not started |
| mus2mid.h | mus2mid | not started |
| sha1.h | sha1 | not started |
| sounds.h | sounds | not started |
| s_sound.h | s_sound | not started |
| statdump.h | statdump | not started |
| tables.h | tables | not started |
| z_zone.h | z_zone | not started |

---

## Summary

| Status | Count |
|--------|-------|
| not started | 0 |
| stubbed | 0 |
| ongoing | 0 |
| done | 96 |
| **Total** | **96** |
