//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Level loading - parses map lumps and populates rendering state.
//
// Original: p_setup.h / p_setup.c (minimal subset for scene rendering)

use crate::doomdata::MapThing;
use crate::i_swap;
use crate::m_fixed::{Fixed, FRACBITS, FRACUNIT};
use crate::rendering::defs::{DegenMobj, Line, Node, Seg, Sector, SideDef, Subsector, Vertex};
use crate::rendering::{m_add_to_box, m_clear_box, BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::rendering::r_data::{r_check_flat_num_for_name, r_check_texture_num_for_name};
use crate::rendering::state;
use crate::wad::{w_get_num_for_name, w_read_lump, w_lump_length};

use super::{MAPBLOCKSHIFT, MAXRADIUS};
use crate::game::d_mode::GameMode;

/// Build map lump name from episode/map. Used by G_InitNew, G_DoLoadLevel.
/// Original: P_SetupLevel lump name logic
pub fn p_map_name_from_episode_map(episode: i32, map: i32, gamemode: GameMode) -> String {
    if gamemode == GameMode::Commercial {
            if map < 10 {
                format!("MAP0{}", map)
            } else {
                format!("MAP{}", map)
        }
    } else {
        format!("E{}M{}", episode, map)
    }
}

fn read_i16(data: &[u8], offset: usize) -> i16 {
    i_swap::short([data[offset], data[offset + 1]])
}

fn name_from_8(buf: &[u8]) -> String {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(8).min(8);
    String::from_utf8_lossy(&buf[..end]).trim().to_string()
}

/// Load a Doom map from WAD and populate rendering state.
/// Map name: "E1M1", "MAP01", etc.
pub fn p_load_level(map_name: &str) -> Result<(), String> {
    let map_lump = w_get_num_for_name(map_name) as usize;

    // Map lumps follow the map marker: THINGS, LINEDEFS, SIDEDEFS, VERTEXES, SEGS, SSECTORS, NODES
    let vertexes_lump = map_lump + 4;
    let linedefs_lump = map_lump + 2;
    let sidedefs_lump = map_lump + 3;
    let segs_lump = map_lump + 5;
    let ssectors_lump = map_lump + 6;
    let nodes_lump = map_lump + 7;

    // Load raw data (w_lump_length returns i32)
    let num_vertexes = (crate::wad::w_lump_length(vertexes_lump) / 4) as usize;
    let num_linedefs = (crate::wad::w_lump_length(linedefs_lump) / 14) as usize;
    let num_sidedefs = (crate::wad::w_lump_length(sidedefs_lump) / 30) as usize;
    let num_segs = (crate::wad::w_lump_length(segs_lump) / 12) as usize;
    let num_ssectors = (crate::wad::w_lump_length(ssectors_lump) / 4) as usize;
    let num_nodes = (crate::wad::w_lump_length(nodes_lump) / 28) as usize;

    if num_vertexes == 0 || num_nodes == 0 {
        return Err("Map has no vertexes or nodes".to_string());
    }

    let mut vertexes_buf = vec![0u8; num_vertexes * 4];
    w_read_lump(vertexes_lump, &mut vertexes_buf);

    let mut linedefs_buf = vec![0u8; num_linedefs * 14];
    w_read_lump(linedefs_lump, &mut linedefs_buf);

    let mut sidedefs_buf = vec![0u8; num_sidedefs * 30];
    w_read_lump(sidedefs_lump, &mut sidedefs_buf);

    let mut segs_buf = vec![0u8; num_segs * 12];
    w_read_lump(segs_lump, &mut segs_buf);

    let mut ssectors_buf = vec![0u8; num_ssectors * 4];
    w_read_lump(ssectors_lump, &mut ssectors_buf);

    let mut nodes_buf = vec![0u8; num_nodes * 28];
    w_read_lump(nodes_lump, &mut nodes_buf);

    // Lump order: THINGS(1), LINEDEFS(2), SIDEDEFS(3), VERTEXES(4), SEGS(5), SSECTORS(6), NODES(7), SECTORS(8), REJECT(9), BLOCKMAP(10)
    let sectors_lump = map_lump + 8;
    let num_sectors_from_lump = (crate::wad::w_lump_length(sectors_lump) / 26) as usize;
    let mut sectors_buf = vec![0u8; num_sectors_from_lump * 26];
    w_read_lump(sectors_lump, &mut sectors_buf);

    let num_sectors = num_sectors_from_lump;

    // Build vertexes
    let mut vertexes: Vec<Vertex> = (0..num_vertexes)
        .map(|i| {
            let x = read_i16(&vertexes_buf, i * 4) as i32 * FRACUNIT;
            let y = read_i16(&vertexes_buf, i * 4 + 2) as i32 * FRACUNIT;
            Vertex { x, y }
        })
        .collect();

    // Build sectors
    let mut sectors: Vec<Sector> = (0..num_sectors)
        .map(|i| {
            let floorheight = read_i16(&sectors_buf, i * 26) as i32 * FRACUNIT;
            let ceilingheight = read_i16(&sectors_buf, i * 26 + 2) as i32 * FRACUNIT;
            let floorpic_name = name_from_8(&sectors_buf[i * 26 + 4..i * 26 + 12]);
            let ceilingpic_name = name_from_8(&sectors_buf[i * 26 + 12..i * 26 + 20]);
            let lightlevel = read_i16(&sectors_buf, i * 26 + 20);
            let special = read_i16(&sectors_buf, i * 26 + 22);
            let tag = read_i16(&sectors_buf, i * 26 + 24);
            let floorpic = if floorpic_name.is_empty() || floorpic_name == "-" {
                0
            } else {
                r_check_flat_num_for_name(&floorpic_name).max(0) as i16
            };
            let ceilingpic = if ceilingpic_name.is_empty() || ceilingpic_name == "-" {
                0
            } else {
                r_check_flat_num_for_name(&ceilingpic_name).max(0) as i16
            };
            Sector {
                floorheight,
                ceilingheight,
                floorpic,
                ceilingpic,
                lightlevel,
                special,
                tag,
                soundtraversed: 0,
                soundtarget: None,
                blockbox: [0, 0, 0, 0],
                soundorg: DegenMobj {
                    thinker: crate::game::d_think::Thinker::default(),
                    x: 0,
                    y: 0,
                    z: 0,
                },
                validcount: 0,
                thinglist: None,
                specialdata: None,
                linecount: 0,
                lines: Vec::new(),
            }
        })
        .collect();

    // Build sidedefs
    let mut sides: Vec<SideDef> = (0..num_sidedefs)
        .map(|i| {
            let x = read_i16(&sidedefs_buf, i * 30) as i32 * FRACUNIT;
            let y = read_i16(&sidedefs_buf, i * 30 + 2) as i32 * FRACUNIT;
            let uppertex = name_from_8(&sidedefs_buf[i * 30 + 4..i * 30 + 12]);
            let lowertex = name_from_8(&sidedefs_buf[i * 30 + 12..i * 30 + 20]);
            let midtex = name_from_8(&sidedefs_buf[i * 30 + 20..i * 30 + 28]);
            let sector_idx = read_i16(&sidedefs_buf, i * 30 + 28) as usize;
            let sector_idx = if sector_idx < num_sectors { sector_idx } else { 0 };
            let toptexture = if uppertex.is_empty() || uppertex == "-" {
                0
            } else {
                r_check_texture_num_for_name(&uppertex).max(0) as i16
            };
            let bottomtexture = if lowertex.is_empty() || lowertex == "-" {
                0
            } else {
                r_check_texture_num_for_name(&lowertex).max(0) as i16
            };
            let midtexture = if midtex.is_empty() || midtex == "-" {
                0
            } else {
                r_check_texture_num_for_name(&midtex).max(0) as i16
            };
            SideDef {
                textureoffset: x,
                rowoffset: y,
                toptexture,
                bottomtexture,
                midtexture,
                sector_idx,
            }
        })
        .collect();

    // Build lines
    let mut lines: Vec<Line> = (0..num_linedefs)
        .map(|i| {
            let v1_idx = read_i16(&linedefs_buf, i * 14) as usize;
            let v2_idx = read_i16(&linedefs_buf, i * 14 + 2) as usize;
            let flags = read_i16(&linedefs_buf, i * 14 + 4);
            let special = read_i16(&linedefs_buf, i * 14 + 6);
            let tag = read_i16(&linedefs_buf, i * 14 + 8);
            let sidenum0 = read_i16(&linedefs_buf, i * 14 + 10);
            let sidenum1 = read_i16(&linedefs_buf, i * 14 + 12);

            let v1_idx = v1_idx.min(num_vertexes.saturating_sub(1));
            let v2_idx = v2_idx.min(num_vertexes.saturating_sub(1));

            let (dx, dy, bbox) = if v1_idx < num_vertexes && v2_idx < num_vertexes {
                let v1x = vertexes[v1_idx].x;
                let v1y = vertexes[v1_idx].y;
                let v2x = vertexes[v2_idx].x;
                let v2y = vertexes[v2_idx].y;
                (
                    v2x - v1x,
                    v2y - v1y,
                    [
                        v1y.max(v2y),
                        v1y.min(v2y),
                        v1x.min(v2x),
                        v1x.max(v2x),
                    ],
                )
            } else {
                (0, 0, [0, 0, 0, 0])
            };

            let slopetype = crate::rendering::slope_type_from_dx_dy(dx, dy);

            let (frontsector_idx, backsector_idx) = if sidenum0 >= 0 && (sidenum0 as usize) < num_sidedefs {
                let fs = sides[sidenum0 as usize].sector_idx;
                let bs = if sidenum1 >= 0 && (sidenum1 as usize) < num_sidedefs {
                    Some(sides[sidenum1 as usize].sector_idx)
                } else {
                    None
                };
                (fs, bs)
            } else {
                (0, None)
            };

            Line {
                v1_idx,
                v2_idx,
                dx,
                dy,
                flags,
                special,
                tag,
                sidenum: [sidenum0, sidenum1],
                bbox,
                slopetype,
                frontsector_idx,
                backsector_idx,
                validcount: 0,
                specialdata: None,
            }
        })
        .collect();

    // Build segs
    let mut segs: Vec<Seg> = (0..num_segs)
        .map(|i| {
            let v1_idx = read_i16(&segs_buf, i * 12) as usize;
            let v2_idx = read_i16(&segs_buf, i * 12 + 2) as usize;
            let angle_raw = read_i16(&segs_buf, i * 12 + 4) as u16;
            let linedef_idx = read_i16(&segs_buf, i * 12 + 6) as usize;
            let side = read_i16(&segs_buf, i * 12 + 8);
            let offset = read_i16(&segs_buf, i * 12 + 10) as i32 * FRACUNIT;
            let angle = (angle_raw as u32) << 16;

            let v1_idx = v1_idx.min(num_vertexes.saturating_sub(1));
            let v2_idx = v2_idx.min(num_vertexes.saturating_sub(1));

            let (sidedef_idx, frontsector_idx, backsector_idx) = if linedef_idx < num_linedefs {
                let ld = &lines[linedef_idx];
                let side_idx = if side != 0 { ld.sidenum[1] } else { ld.sidenum[0] };
                let sidedef_idx = if side_idx >= 0 && (side_idx as usize) < num_sidedefs {
                    side_idx as usize
                } else {
                    0
                };
                let fs = ld.frontsector_idx;
                let bs = ld.backsector_idx.unwrap_or(fs);
                (sidedef_idx, fs, bs)
            } else {
                (0, 0, 0)
            };

            Seg {
                v1_idx,
                v2_idx,
                offset,
                angle,
                sidedef_idx,
                linedef_idx,
                frontsector_idx,
                backsector_idx,
            }
        })
        .collect();

    // Build subsectors
    let subsectors: Vec<Subsector> = (0..num_ssectors)
        .map(|i| {
            let numlines = read_i16(&ssectors_buf, i * 4);
            let firstline = read_i16(&ssectors_buf, i * 4 + 2);
            let seg_idx = firstline as usize;
            let sector_idx = if seg_idx < segs.len() {
                segs[seg_idx].frontsector_idx
            } else {
                0
            };
            Subsector {
                sector_idx,
                numlines,
                firstline,
            }
        })
        .collect();

    // Build nodes
    let nodes: Vec<Node> = (0..num_nodes)
        .map(|i| {
            let x = read_i16(&nodes_buf, i * 28) as i32 * FRACUNIT;
            let y = read_i16(&nodes_buf, i * 28 + 2) as i32 * FRACUNIT;
            let dx = read_i16(&nodes_buf, i * 28 + 4) as i32 * FRACUNIT;
            let dy = read_i16(&nodes_buf, i * 28 + 6) as i32 * FRACUNIT;
            let bbox_front_right = read_i16(&nodes_buf, i * 28 + 8) as i32 * FRACUNIT;
            let bbox_front_top = read_i16(&nodes_buf, i * 28 + 10) as i32 * FRACUNIT;
            let bbox_front_left = read_i16(&nodes_buf, i * 28 + 12) as i32 * FRACUNIT;
            let bbox_front_bottom = read_i16(&nodes_buf, i * 28 + 14) as i32 * FRACUNIT;
            let bbox_back_right = read_i16(&nodes_buf, i * 28 + 16) as i32 * FRACUNIT;
            let bbox_back_top = read_i16(&nodes_buf, i * 28 + 18) as i32 * FRACUNIT;
            let bbox_back_left = read_i16(&nodes_buf, i * 28 + 20) as i32 * FRACUNIT;
            let bbox_back_bottom = read_i16(&nodes_buf, i * 28 + 22) as i32 * FRACUNIT;
            let child0 = read_i16(&nodes_buf, i * 28 + 24) as u16;
            let child1 = read_i16(&nodes_buf, i * 28 + 26) as u16;
            Node {
                x,
                y,
                dx,
                dy,
                bbox: [
                    [
                        bbox_front_top,
                        bbox_front_bottom,
                        bbox_front_left,
                        bbox_front_right,
                    ],
                    [
                        bbox_back_top,
                        bbox_back_bottom,
                        bbox_back_left,
                        bbox_back_right,
                    ],
                ],
                children: [child0, child1],
            }
        })
        .collect();

    // Set rendering state
    state::with_state_mut(|s| {
        s.numvertexes = num_vertexes as i32;
        s.vertexes = vertexes;
        s.numsegs = num_segs as i32;
        s.segs = segs;
        s.numsectors = num_sectors as i32;
        s.sectors = sectors;
        s.numsubsectors = num_ssectors as i32;
        s.subsectors = subsectors;
        s.numnodes = num_nodes as i32;
        s.nodes = nodes;
        s.numlines = num_linedefs as i32;
        s.lines = lines;
        s.numsides = num_sidedefs as i32;
        s.sides = sides;
    });

    // Load REJECT (lump 9) - for P_CheckSight
    let reject_lump = (map_lump + 9) as usize;
    p_load_reject(reject_lump, num_sectors);

    // Load blockmap (lump 10)
    let blockmap_lump = (map_lump + 10) as usize;
    p_load_blockmap(blockmap_lump);

    // Build sector line lists and blockboxes
    p_group_lines();

    // Initialize thinkers and spawn map things
    super::p_tick::p_init_thinkers();
    // Single player: ensure player 1 is in game
    crate::doomstat::with_doomstat_state(|st| st.playeringame[0] = true);
    let things_lump = (map_lump + 1) as usize;
    p_load_things(things_lump);

    Ok(())
}

/// Load THINGS lump and spawn map things. Original: P_LoadThings
fn p_load_things(lump: usize) {
    const MAPTHING_SIZE: usize = 10; // 5 x i16
    let lumplen = w_lump_length(lump) as usize;
    if lumplen < MAPTHING_SIZE {
        return;
    }
    let numthings = lumplen / MAPTHING_SIZE;
    let mut buf = vec![0u8; lumplen];
    w_read_lump(lump, &mut buf);

    for i in 0..numthings {
        let off = i * MAPTHING_SIZE;
        let mthing = MapThing {
            x: read_i16(&buf, off),
            y: read_i16(&buf, off + 2),
            angle: read_i16(&buf, off + 4),
            type_: read_i16(&buf, off + 6),
            options: read_i16(&buf, off + 8),
        };
        super::p_mobj::p_spawn_map_thing(&mthing);
    }
}

/// Load REJECT lump for fast sight rejection. Original: P_LoadReject
fn p_load_reject(lump: usize, num_sectors: usize) {
    let minlength = (num_sectors * num_sectors + 7) / 8;
    if minlength == 0 {
        return;
    }
    let lumplen = w_lump_length(lump) as usize;
    let copy_len = lumplen.min(minlength);

    let mut reject = vec![0u8; minlength];
    if lumplen > 0 {
        let mut buf = vec![0u8; lumplen];
        w_read_lump(lump, &mut buf);
        reject[..copy_len].copy_from_slice(&buf[..copy_len]);
    }
    state::with_state_mut(|s| s.rejectmatrix = reject);
}

/// Load blockmap from WAD. Original: P_LoadBlockMap
fn p_load_blockmap(lump: usize) {
    let lumplen = w_lump_length(lump) as usize;
    if lumplen < 8 {
        return;
    }
    let count = lumplen / 2;
    let mut blockmaplump = vec![0i16; count];
    let mut buf = vec![0u8; lumplen];
    w_read_lump(lump, &mut buf);
    for i in 0..count {
        blockmaplump[i] = read_i16(&buf, i * 2) as i16;
    }
    let bmaporgx = (blockmaplump[0] as i32) << FRACBITS;
    let bmaporgy = (blockmaplump[1] as i32) << FRACBITS;
    let bmapwidth = blockmaplump[2] as i32;
    let bmapheight = blockmaplump[3] as i32;
    let blocklinks_count = (bmapwidth * bmapheight) as usize;
    let blocklinks = vec![std::ptr::null_mut::<std::ffi::c_void>(); blocklinks_count];
    state::with_state_mut(|s| {
        s.blockmaplump = blockmaplump;
        s.bmaporgx = bmaporgx;
        s.bmaporgy = bmaporgy;
        s.bmapwidth = bmapwidth;
        s.bmapheight = bmapheight;
        s.blocklinks = blocklinks;
    });
}

/// Build sector line lists and blockboxes. Original: P_GroupLines
fn p_group_lines() {
    state::with_state_mut(|s| {
        let num_sectors = s.sectors.len();
        let num_lines = s.lines.len();
        let vertexes = &s.vertexes;

        // Reset linecount and lines for each sector
        for sector in s.sectors.iter_mut() {
            sector.linecount = 0;
            sector.lines.clear();
        }

        // First pass: count lines per sector
        for i in 0..num_lines {
            let line = &s.lines[i];
            let fs = line.frontsector_idx;
            if fs < num_sectors {
                s.sectors[fs].linecount += 1;
            }
            if let Some(bs) = line.backsector_idx {
                if bs < num_sectors && bs != fs {
                    s.sectors[bs].linecount += 1;
                }
            }
        }

        // Pre-allocate lines Vec for each sector
        for sector in s.sectors.iter_mut() {
            sector.lines.reserve(sector.linecount as usize);
        }

        // Second pass: add line indices to each sector's lines
        for i in 0..num_lines {
            let line = &s.lines[i];
            let fs = line.frontsector_idx;
            if fs < num_sectors {
                s.sectors[fs].lines.push(i);
            }
            if let Some(bs) = line.backsector_idx {
                if bs < num_sectors && bs != fs {
                    s.sectors[bs].lines.push(i);
                }
            }
        }

        let (bmaporgx, bmaporgy, bmapwidth, bmapheight) =
            (s.bmaporgx, s.bmaporgy, s.bmapwidth, s.bmapheight);

        // Compute blockboxes and soundorg for each sector
        for sector in s.sectors.iter_mut() {
            let mut bbox: [Fixed; 4] = [0; 4];
            m_clear_box(&mut bbox);
            for &line_idx in &sector.lines {
                if line_idx < num_lines {
                    let line = &s.lines[line_idx];
                    let v1_idx = line.v1_idx;
                    let v2_idx = line.v2_idx;
                    if v1_idx < vertexes.len() {
                        let v = &vertexes[v1_idx];
                        m_add_to_box(&mut bbox, v.x, v.y);
                    }
                    if v2_idx < vertexes.len() {
                        let v = &vertexes[v2_idx];
                        m_add_to_box(&mut bbox, v.x, v.y);
                    }
                }
            }
            let (bx_top, bx_bottom, bx_left, bx_right) = (
                bbox[BOXTOP],
                bbox[BOXBOTTOM],
                bbox[BOXLEFT],
                bbox[BOXRIGHT],
            );
            sector.soundorg.x = (bx_right + bx_left) / 2;
            sector.soundorg.y = (bx_top + bx_bottom) / 2;
            let mut block = (bx_top - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;
            block = if block >= bmapheight { bmapheight - 1 } else { block };
            sector.blockbox[BOXTOP] = block;
            let mut block = (bx_bottom - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
            block = if block < 0 { 0 } else { block };
            sector.blockbox[BOXBOTTOM] = block;
            let mut block = (bx_right - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
            block = if block >= bmapwidth { bmapwidth - 1 } else { block };
            sector.blockbox[BOXRIGHT] = block;
            let mut block = (bx_left - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
            block = if block < 0 { 0 } else { block };
            sector.blockbox[BOXLEFT] = block;
        }
    });
}
