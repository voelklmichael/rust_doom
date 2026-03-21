# Header Dependency Graph

Dependencies derived from `#include "*.h"` in `doomgeneric/*.h` and `doomgeneric/*.c` files.  
Arrow convention: **A → B** means "A includes B" (A depends on B).

---

## Dependency Table

Each header and the headers it directly includes:

| Header | Includes |
|--------|----------|
| am_map | d_event, deh_main, doomdef, doomkeys, doomstat, dstrings, i_system, m_cheat, m_controls, m_misc, p_local, r_state, st_stuff, v_video, w_wad, z_zone |
| config | — |
| d_englsh | — |
| d_event | doomtype |
| d_items | doomdef, info |
| d_iwad | config, d_mode, deh_str, doomkeys, i_system, m_argv, m_config, m_misc, w_wad, z_zone |
| d_loop | d_event, d_ticcmd, doomfeatures, i_system, i_timer, i_video, m_argv, m_fixed, net_client, net_defs, net_gui, net_io, net_loop, net_query, net_server |
| d_main | am_map, config, d_iwad, deh_main, doomdef, doomfeatures, doomstat, dstrings, f_finale, f_wipe, g_game, hu_stuff, i_endoom, i_joystick, i_system, i_timer, i_video, m_argv, m_config, m_controls, m_menu, m_misc, net_client, net_dedicated, net_query, p_saveg, p_setup, r_local, s_sound, sounds, st_stuff, statdump, v_video, w_main, w_wad, wi_stuff, z_zone |
| d_mode | doomtype |
| d_net | d_loop, d_main, deh_main, doomdef, doomfeatures, doomstat, g_game, i_system, i_timer, i_video, m_argv, m_menu, m_misc, w_checksum, w_wad |
| d_player | d_items, d_ticcmd, net_defs, p_mobj, p_pspr |
| d_textur | doomtype |
| d_think | — |
| d_ticcmd | doomtype |
| deh_main | deh_str, doomfeatures, doomtype, sha1 |
| deh_misc | doomfeatures |
| deh_str | doomfeatures |
| doom | — |
| doomdata | doomdef, doomtype |
| doomdef | d_mode, doomtype, i_timer |
| doomfeatures | — |
| doomgeneric | m_argv |
| doomkeys | — |
| doomstat | d_loop, d_mode, d_player, doomdata, net_defs |
| doomtype | — |
| dstrings | d_englsh |
| d_think | — |
| d_ticcmd | doomtype |
| f_finale | d_event, d_main, deh_main, doomstat, doomtype, dstrings, hu_stuff, i_swap, i_system, r_local, r_state, s_sound, sounds, v_video, w_wad, z_zone |
| f_wipe | doomtype, i_video, m_random, v_video, z_zone |
| g_game | am_map, d_event, d_main, d_ticcmd, deh_main, deh_misc, doomdef, doomkeys, doomstat, dstrings, f_finale, hu_stuff, i_system, i_timer, i_video, m_argv, m_controls, m_menu, m_misc, m_random, p_local, p_saveg, p_setup, p_tick, r_data, r_sky, s_sound, sounds, st_stuff, statdump, v_video, w_wad, wi_stuff, z_zone |
| gusconf | doomtype, w_wad, z_zone |
| hu_lib | doomdef, doomkeys, i_swap, r_defs, r_draw, r_local, v_video |
| hu_stuff | d_event, deh_main, doomdef, doomkeys, doomstat, dstrings, hu_lib, i_swap, i_video, m_controls, m_misc, s_sound, sounds, w_wad, z_zone |
| i_cdmus | doomtype |
| i_endoom | config, doomtype, i_video |
| i_input | config, deh_str, doomgeneric, doomkeys, doomtype, i_joystick, i_scale, i_swap, i_system, i_timer, i_video, m_argv, m_config, m_misc, tables, v_video, w_wad, z_zone |
| i_joystick | d_event, doomtype, i_system, m_config, m_misc |
| i_scale | doomtype, i_video, m_argv, z_zone |
| i_sound | config, doomfeatures, doomtype, gusconf, i_video, m_argv, m_config |
| i_swap | — |
| i_system | config, d_event, d_ticcmd, deh_str, doomtype, i_joystick, i_sound, i_timer, i_video, m_argv, m_config, m_misc, w_wad, z_zone |
| i_timer | doomgeneric, doomtype |
| i_video | config, d_event, d_main, doomgeneric, doomkeys, doomtype, i_system, m_argv, tables, v_video, z_zone |
| info | d_think, m_fixed, p_mobj, sounds |
| m_argv | doomtype, i_system, m_misc |
| m_bbox | m_fixed |
| m_cheat | doomtype |
| m_config | config, doomfeatures, doomkeys, doomtype, i_system, m_argv, m_misc, z_zone |
| m_controls | doomkeys, doomtype, m_config, m_misc |
| m_fixed | doomtype, i_system |
| m_menu | d_event, d_main, deh_main, doomdef, doomkeys, doomstat, dstrings, g_game, hu_stuff, i_swap, i_system, i_timer, i_video, m_argv, m_controls, m_misc, p_saveg, r_local, s_sound, sounds, v_video, w_wad, z_zone |
| m_misc | deh_str, doomtype, i_swap, i_system, i_video, v_video, w_wad, z_zone |
| m_random | doomtype |
| memio | z_zone |
| mus2mid | doomtype, i_swap, m_misc, memio, z_zone |
| net_client | d_ticcmd, doomtype, net_defs, sha1 |
| net_dedicated | — |
| net_defs | d_ticcmd, doomtype, sha1 |
| net_gui | doomtype |
| net_io | net_defs |
| net_loop | net_defs |
| net_packet | net_defs |
| net_query | net_defs |
| net_server | — |
| p_ceilng | doomdef, doomstat, p_local, r_state, s_sound, sounds, z_zone |
| p_doors | deh_main, doomdef, doomstat, dstrings, p_local, r_state, s_sound, sounds, z_zone |
| p_enemy | doomdef, doomstat, g_game, i_system, m_random, p_local, r_state, s_sound, sounds |
| p_floor | doomdef, doomstat, p_local, r_state, s_sound, sounds, z_zone |
| p_inter | am_map, deh_main, deh_misc, doomdef, doomstat, dstrings, i_system, m_random, p_local, s_sound, sounds |
| p_lights | doomdef, m_random, p_local, r_state, z_zone |
| p_local | p_spec, r_local |
| p_map | deh_misc, doomdef, doomstat, i_system, m_argv, m_bbox, m_misc, m_random, p_local, r_state, s_sound, sounds |
| p_maputl | doomdef, doomstat, m_bbox, p_local, r_state |
| p_mobj | d_think, doomdata, doomdef, doomstat, hu_stuff, i_system, info, m_fixed, m_random, p_local, s_sound, sounds, st_stuff, tables, z_zone |
| p_plats | doomdef, doomstat, i_system, m_random, p_local, r_state, s_sound, sounds, z_zone |
| p_pspr | d_event, deh_misc, doomdef, doomstat, info, m_fixed, m_random, p_local, s_sound, sounds, tables |
| p_saveg | deh_main, doomstat, dstrings, g_game, i_system, m_misc, p_local, r_state, z_zone |
| p_setup | deh_main, doomdef, doomstat, g_game, i_swap, i_system, m_argv, m_bbox, p_local, s_sound, w_wad, z_zone |
| p_sight | doomdef, i_system, p_local, r_state |
| p_spec | deh_main, doomdef, doomstat, g_game, i_system, m_argv, m_misc, m_random, p_local, r_local, r_state, s_sound, sounds, w_wad, z_zone |
| p_switch | deh_main, doomdef, doomstat, g_game, i_system, p_local, r_state, s_sound, sounds |
| p_telept | doomdef, doomstat, p_local, r_state, s_sound, sounds |
| p_tick | doomstat, p_local, z_zone |
| p_user | d_event, doomdef, doomstat, p_local |
| r_bsp | doomdef, doomstat, i_system, m_bbox, r_local, r_main, r_plane, r_state, r_things |
| r_data | deh_main, doomdef, doomstat, i_swap, i_system, m_misc, p_local, r_defs, r_local, r_sky, r_state, w_wad, z_zone |
| r_defs | d_think, doomdef, i_video, m_fixed, p_mobj, v_patch |
| r_draw | deh_main, doomdef, doomstat, i_system, r_local, v_video, w_wad, z_zone |
| r_local | doomdef, r_bsp, r_data, r_draw, r_main, r_plane, r_segs, r_things, tables |
| r_main | d_loop, d_player, doomdef, m_bbox, m_menu, r_data, r_local, r_sky |
| r_plane | doomdef, doomstat, i_system, r_data, r_local, r_sky, w_wad, z_zone |
| r_segs | doomdef, doomstat, i_system, r_local, r_sky |
| r_sky | m_fixed, r_data |
| r_state | d_player, r_data |
| r_things | deh_main, doomdef, doomstat, i_swap, i_system, r_local, w_wad, z_zone |
| s_sound | deh_str, doomfeatures, doomstat, doomtype, i_sound, i_system, m_argv, m_misc, m_random, p_local, p_mobj, sounds, w_wad, z_zone |
| sha1 | doomtype, i_swap |
| sounds | doomtype, i_sound |
| st_lib | deh_main, doomdef, i_swap, i_system, r_defs, r_local, st_stuff, v_video, w_wad, z_zone |
| st_stuff | am_map, d_event, deh_main, deh_misc, doomdef, doomkeys, doomstat, doomtype, dstrings, g_game, i_system, i_video, m_cheat, m_misc, m_random, p_inter, p_local, r_local, s_sound, sounds, st_lib, v_video, w_wad, z_zone |
| statdump | d_mode, d_player, m_argv |
| tables | doomtype, m_fixed |
| v_patch | — |
| v_video | config, deh_str, doomtype, i_swap, i_system, i_video, m_bbox, m_misc, v_patch, w_wad, z_zone |
| w_checksum | doomtype, m_misc, sha1, w_wad |
| w_file | config, doomtype, m_argv |
| w_file_stdc | m_misc, w_file, z_zone |
| w_main | d_iwad, doomfeatures, m_argv, w_merge, w_wad, z_zone |
| w_merge | — |
| w_wad | config, d_iwad, d_mode, doomtype, i_swap, i_system, i_video, m_misc, w_file, z_zone |
| wi_stuff | deh_main, doomdef, doomstat, g_game, i_swap, i_system, m_misc, m_random, r_local, s_sound, sounds, v_video, w_wad, z_zone |
| z_zone | doomtype, i_system |

---

## Leaf Nodes (no dependencies)

Headers that do not include any other project headers:

- `config`, `d_englsh`, `d_think`, `doom`, `doomfeatures`, `doomkeys`, `doomtype`, `i_swap`, `net_dedicated`, `net_server`, `v_patch`, `w_merge`

---

## Root Nodes (nothing depends on them in this table)

Headers that are not included by any other project header (may be entry points or optional):

- `doom`, `d_englsh`, `d_net`, `dummy`, `i_input`, `w_file_stdc`

---

## DOT Format (for Graphviz)

Render with: `dot -Tpng deps.dot -o deps.png` or `dot -Tsvg deps.dot -o deps.svg`

See [deps.dot](deps.dot) in this directory.
