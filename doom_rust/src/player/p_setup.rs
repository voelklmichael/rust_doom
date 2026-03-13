//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Level loading - parses map lumps and populates rendering state.
//
// Original: p_setup.h / p_setup.c (minimal subset for scene rendering)

use crate::i_swap;
use crate::m_fixed::{Fixed, FRACBITS, FRACUNIT};
use crate::rendering::defs::{Line, Node, Seg, Sector, SideDef, SlopeType, Subsector, Vertex};
use crate::rendering::{m_add_to_box, m_clear_box, BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};
use crate::rendering::r_data::{r_check_flat_num_for_name, r_check_texture_num_for_name};
use crate::rendering::state;
use crate::wad::{w_get_num_for_name, w_read_lump, w_lump_length};
use crate::z_zone::{z_malloc, PU_LEVEL};
use std::ptr;

use super::{MAPBLOCKSHIFT, MAXRADIUS};

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

    // Allocate and fill vertexes
    let vertexes_ptr = z_malloc(
        num_vertexes * std::mem::size_of::<Vertex>(),
        PU_LEVEL,
        ptr::null_mut(),
    ) as *mut Vertex;
    for i in 0..num_vertexes {
        let x = read_i16(&vertexes_buf, i * 4) as i32 * FRACUNIT;
        let y = read_i16(&vertexes_buf, i * 4 + 2) as i32 * FRACUNIT;
        unsafe {
            (*vertexes_ptr.add(i)).x = x;
            (*vertexes_ptr.add(i)).y = y;
        }
    }

    // Allocate sectors
    let sectors_ptr = z_malloc(
        num_sectors * std::mem::size_of::<Sector>(),
        PU_LEVEL,
        ptr::null_mut(),
    ) as *mut Sector;
    for i in 0..num_sectors {
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

        unsafe {
            let sec = sectors_ptr.add(i);
            (*sec).floorheight = floorheight;
            (*sec).ceilingheight = ceilingheight;
            (*sec).floorpic = floorpic;
            (*sec).ceilingpic = ceilingpic;
            (*sec).lightlevel = lightlevel;
            (*sec).special = special;
            (*sec).tag = tag;
            (*sec).soundtraversed = 0;
            (*sec).soundtarget = ptr::null_mut();
            (*sec).blockbox = [0, 0, 0, 0];
            (*sec).soundorg = std::mem::zeroed();
            (*sec).validcount = 0;
            (*sec).thinglist = ptr::null_mut();
            (*sec).specialdata = ptr::null_mut();
            (*sec).linecount = 0;
            (*sec).lines = ptr::null_mut();
        }
    }

    // Allocate sidedefs
    let sides_ptr = z_malloc(
        num_sidedefs * std::mem::size_of::<SideDef>(),
        PU_LEVEL,
        ptr::null_mut(),
    ) as *mut SideDef;
    for i in 0..num_sidedefs {
        let x = read_i16(&sidedefs_buf, i * 30) as i32 * FRACUNIT;
        let y = read_i16(&sidedefs_buf, i * 30 + 2) as i32 * FRACUNIT;
        let uppertex = name_from_8(&sidedefs_buf[i * 30 + 4..i * 30 + 12]);
        let lowertex = name_from_8(&sidedefs_buf[i * 30 + 12..i * 30 + 20]);
        let midtex = name_from_8(&sidedefs_buf[i * 30 + 20..i * 30 + 28]);
        let sector_idx = read_i16(&sidedefs_buf, i * 30 + 28);

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

        let sector_ptr = if sector_idx >= 0 && (sector_idx as usize) < num_sectors {
            unsafe { sectors_ptr.add(sector_idx as usize) }
        } else {
            ptr::null_mut()
        };

        unsafe {
            let side = sides_ptr.add(i);
            (*side).textureoffset = x;
            (*side).rowoffset = y;
            (*side).toptexture = toptexture;
            (*side).bottomtexture = bottomtexture;
            (*side).midtexture = midtexture;
            (*side).sector = sector_ptr;
        }
    }

    // Allocate lines
    let lines_ptr = z_malloc(
        num_linedefs * std::mem::size_of::<Line>(),
        PU_LEVEL,
        ptr::null_mut(),
    ) as *mut Line;
    for i in 0..num_linedefs {
        let v1_idx = read_i16(&linedefs_buf, i * 14) as usize;
        let v2_idx = read_i16(&linedefs_buf, i * 14 + 2) as usize;
        let flags = read_i16(&linedefs_buf, i * 14 + 4);
        let special = read_i16(&linedefs_buf, i * 14 + 6);
        let tag = read_i16(&linedefs_buf, i * 14 + 8);
        let sidenum0 = read_i16(&linedefs_buf, i * 14 + 10);
        let sidenum1 = read_i16(&linedefs_buf, i * 14 + 12);

        let v1 = if v1_idx < num_vertexes {
            unsafe { vertexes_ptr.add(v1_idx) }
        } else {
            ptr::null_mut()
        };
        let v2 = if v2_idx < num_vertexes {
            unsafe { vertexes_ptr.add(v2_idx) }
        } else {
            ptr::null_mut()
        };

        let (dx, dy, bbox) = if !v1.is_null() && !v2.is_null() {
            unsafe {
                let v1x = (*v1).x;
                let v1y = (*v1).y;
                let v2x = (*v2).x;
                let v2y = (*v2).y;
                let left = v1x.min(v2x);
                let right = v1x.max(v2x);
                let bottom = v1y.min(v2y);
                let top = v1y.max(v2y);
                (
                    v2x - v1x,
                    v2y - v1y,
                    [top, bottom, left, right], // BOXTOP, BOXBOTTOM, BOXLEFT, BOXRIGHT
                )
            }
        } else {
            (0, 0, [0, 0, 0, 0])
        };

        let slopetype = if dy == 0 {
            SlopeType::Horizontal
        } else if dx == 0 {
            SlopeType::Vertical
        } else if (dx as i64 * dy as i64) > 0 {
            SlopeType::Positive
        } else {
            SlopeType::Negative
        };

        let front_side = if sidenum0 >= 0 && (sidenum0 as usize) < num_sidedefs {
            unsafe { sides_ptr.add(sidenum0 as usize) }
        } else {
            ptr::null_mut()
        };
        let back_side = if sidenum1 >= 0 && (sidenum1 as usize) < num_sidedefs {
            unsafe { sides_ptr.add(sidenum1 as usize) }
        } else {
            ptr::null_mut()
        };

        let frontsector = if !front_side.is_null() {
            unsafe { (*front_side).sector }
        } else {
            ptr::null_mut()
        };
        let backsector = if !back_side.is_null() {
            unsafe { (*back_side).sector }
        } else {
            ptr::null_mut()
        };

        unsafe {
            let line = lines_ptr.add(i);
            (*line).v1 = v1;
            (*line).v2 = v2;
            (*line).dx = dx;
            (*line).dy = dy;
            (*line).flags = flags;
            (*line).special = special;
            (*line).tag = tag;
            (*line).sidenum = [sidenum0, sidenum1];
            (*line).bbox = bbox;
            (*line).slopetype = slopetype;
            (*line).frontsector = frontsector;
            (*line).backsector = backsector;
            (*line).validcount = 0;
            (*line).specialdata = ptr::null_mut();
        }
    }

    // Allocate segs
    let segs_ptr = z_malloc(
        num_segs * std::mem::size_of::<Seg>(),
        PU_LEVEL,
        ptr::null_mut(),
    ) as *mut Seg;
    for i in 0..num_segs {
        let v1_idx = read_i16(&segs_buf, i * 12) as usize;
        let v2_idx = read_i16(&segs_buf, i * 12 + 2) as usize;
        let angle_raw = read_i16(&segs_buf, i * 12 + 4) as u16;
        let linedef_idx = read_i16(&segs_buf, i * 12 + 6) as usize;
        let side = read_i16(&segs_buf, i * 12 + 8);
        let offset = read_i16(&segs_buf, i * 12 + 10) as i32 * FRACUNIT;

        let angle = (angle_raw as u32) << 16; // 16-bit BAM to 32-bit BAM

        let v1 = if v1_idx < num_vertexes {
            unsafe { vertexes_ptr.add(v1_idx) }
        } else {
            ptr::null_mut()
        };
        let v2 = if v2_idx < num_vertexes {
            unsafe { vertexes_ptr.add(v2_idx) }
        } else {
            ptr::null_mut()
        };

        let linedef = if linedef_idx < num_linedefs {
            unsafe { lines_ptr.add(linedef_idx) }
        } else {
            ptr::null_mut()
        };

        let (sidedef, frontsector, backsector) = if !linedef.is_null() {
            unsafe {
                let ld = &*linedef;
                let side_idx = if side != 0 { ld.sidenum[1] } else { ld.sidenum[0] };
                let sidedef_ptr = if side_idx >= 0 && (side_idx as usize) < num_sidedefs {
                    sides_ptr.add(side_idx as usize)
                } else {
                    ptr::null_mut()
                };
                let fs = ld.frontsector;
                let bs = ld.backsector;
                (sidedef_ptr, fs, bs)
            }
        } else {
            (ptr::null_mut(), ptr::null_mut(), ptr::null_mut())
        };

        unsafe {
            let seg = segs_ptr.add(i);
            (*seg).v1 = v1;
            (*seg).v2 = v2;
            (*seg).offset = offset;
            (*seg).angle = angle;
            (*seg).sidedef = sidedef;
            (*seg).linedef = linedef;
            (*seg).frontsector = frontsector;
            (*seg).backsector = backsector;
        }
    }

    // Allocate subsectors
    let subsectors_ptr = z_malloc(
        num_ssectors * std::mem::size_of::<Subsector>(),
        PU_LEVEL,
        ptr::null_mut(),
    ) as *mut Subsector;
    for i in 0..num_ssectors {
        let numlines = read_i16(&ssectors_buf, i * 4);
        let firstline = read_i16(&ssectors_buf, i * 4 + 2);

        let seg_idx = firstline as usize;
        let sector = if seg_idx < num_segs {
            let seg = unsafe { &*segs_ptr.add(seg_idx) };
            if seg.sidedef.is_null() {
                seg.frontsector
            } else {
                unsafe { (*seg.sidedef).sector }
            }
        } else {
            ptr::null_mut()
        };

        unsafe {
            let ss = subsectors_ptr.add(i);
            (*ss).sector = sector;
            (*ss).numlines = numlines;
            (*ss).firstline = firstline;
        }
    }

    // Allocate nodes
    let nodes_ptr = z_malloc(
        num_nodes * std::mem::size_of::<Node>(),
        PU_LEVEL,
        ptr::null_mut(),
    ) as *mut Node;
    for i in 0..num_nodes {
        let x = read_i16(&nodes_buf, i * 28) as i32 * FRACUNIT;
        let y = read_i16(&nodes_buf, i * 28 + 2) as i32 * FRACUNIT;
        let dx = read_i16(&nodes_buf, i * 28 + 4) as i32 * FRACUNIT;
        let dy = read_i16(&nodes_buf, i * 28 + 6) as i32 * FRACUNIT;
        // Doom node bbox order: right, top, left, bottom. We use: top(0), bottom(1), left(2), right(3)
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

        unsafe {
            let node = nodes_ptr.add(i);
            (*node).x = x;
            (*node).y = y;
            (*node).dx = dx;
            (*node).dy = dy;
            // Bbox order: top(0), bottom(1), left(2), right(3)
            (*node).bbox = [
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
            ];
            (*node).children = [child0, child1];
        }
    }

    // Set rendering state
    unsafe {
        state::NUMVERTEXES = num_vertexes as i32;
        state::VERTEXES = vertexes_ptr;
        state::NUMSEGS = num_segs as i32;
        state::SEGS = segs_ptr;
        state::NUMSECTORS = num_sectors as i32;
        state::SECTORS = sectors_ptr;
        state::NUMSUBSECTORS = num_ssectors as i32;
        state::SUBSECTORS = subsectors_ptr;
        state::NUMNODES = num_nodes as i32;
        state::NODES = nodes_ptr;
        state::NUMLINES = num_linedefs as i32;
        state::LINES = lines_ptr;
        state::NUMSIDES = num_sidedefs as i32;
        state::SIDES = sides_ptr;
    }

    // Load blockmap (lump 10)
    let blockmap_lump = (map_lump + 10) as usize;
    p_load_blockmap(blockmap_lump);

    // Build sector line lists and blockboxes
    p_group_lines(num_sectors, num_linedefs, sectors_ptr, lines_ptr);

    Ok(())
}

/// Load blockmap from WAD. Original: P_LoadBlockMap
fn p_load_blockmap(lump: usize) {
    let lumplen = w_lump_length(lump) as usize;
    if lumplen < 8 {
        return;
    }
    let count = lumplen / 2;
    let blockmaplump_ptr = z_malloc(count * 2, PU_LEVEL, ptr::null_mut()) as *mut i16;
    let mut buf = vec![0u8; lumplen];
    w_read_lump(lump, &mut buf);
    for i in 0..count {
        unsafe {
            *blockmaplump_ptr.add(i) = read_i16(&buf, i * 2) as i16;
        }
    }
    let blockmap_ptr = unsafe { blockmaplump_ptr.add(4) };
    let bmaporgx = (unsafe { *blockmaplump_ptr } as i32) << FRACBITS;
    let bmaporgy = (unsafe { *blockmaplump_ptr.add(1) } as i32) << FRACBITS;
    let bmapwidth = unsafe { *blockmaplump_ptr.add(2) } as i32;
    let bmapheight = unsafe { *blockmaplump_ptr.add(3) } as i32;
    let blocklinks_count = (bmapwidth * bmapheight) as usize;
    let blocklinks_ptr =
        z_malloc(blocklinks_count * std::mem::size_of::<*mut std::ffi::c_void>(), PU_LEVEL, ptr::null_mut())
            as *mut *mut std::ffi::c_void;
    unsafe {
        ptr::write_bytes(blocklinks_ptr, 0, blocklinks_count);
    }
    unsafe {
        state::BLOCKMAPLUMP = blockmaplump_ptr;
        state::BLOCKMAP = blockmap_ptr;
        state::BMAPORGX = bmaporgx;
        state::BMAPORGY = bmaporgy;
        state::BMAPWIDTH = bmapwidth;
        state::BMAPHEIGHT = bmapheight;
        state::BLOCKLINKS = blocklinks_ptr;
    }
}

/// Build sector line lists and blockboxes. Original: P_GroupLines
fn p_group_lines(
    num_sectors: usize,
    num_lines: usize,
    sectors_ptr: *mut Sector,
    lines_ptr: *mut Line,
) {
    // Count lines per sector
    for i in 0..num_lines {
        let li = unsafe { &*lines_ptr.add(i) };
        if !li.frontsector.is_null() {
            unsafe {
                (*li.frontsector).linecount += 1;
            }
        }
        if !li.backsector.is_null() && li.backsector != li.frontsector {
            unsafe {
                (*li.backsector).linecount += 1;
            }
        }
    }

    let totallines: i32 = (0..num_sectors)
        .map(|i| unsafe { (*sectors_ptr.add(i)).linecount })
        .sum();

    let linebuffer_ptr = z_malloc(
        (totallines as usize) * std::mem::size_of::<*mut Line>(),
        PU_LEVEL,
        ptr::null_mut(),
    ) as *mut *mut Line;

    let mut linebuffer_offset = 0usize;
    for i in 0..num_sectors {
        let sec = unsafe { sectors_ptr.add(i) };
        let linecount = unsafe { (*sec).linecount } as usize;
        unsafe {
            (*sec).lines = linebuffer_ptr.add(linebuffer_offset);
            (*sec).linecount = 0;
        }
        linebuffer_offset += linecount;
    }

    for i in 0..num_lines {
        let li = unsafe { lines_ptr.add(i) };
        let line = unsafe { &*li };
        if !line.frontsector.is_null() {
            let sector = line.frontsector;
            let idx = unsafe { (*sector).linecount } as usize;
            unsafe {
                *((*sector).lines.add(idx)) = li;
                (*sector).linecount += 1;
            }
        }
        if !line.backsector.is_null() && line.backsector != line.frontsector {
            let sector = line.backsector;
            let idx = unsafe { (*sector).linecount } as usize;
            unsafe {
                *((*sector).lines.add(idx)) = li;
                (*sector).linecount += 1;
            }
        }
    }

    let (bmaporgx, bmaporgy, bmapwidth, bmapheight) = unsafe {
        (
            state::BMAPORGX,
            state::BMAPORGY,
            state::BMAPWIDTH,
            state::BMAPHEIGHT,
        )
    };

    for i in 0..num_sectors {
        let sector = unsafe { sectors_ptr.add(i) };
        let mut bbox: [Fixed; 4] = [0; 4];
        m_clear_box(&mut bbox);
        let linecount = unsafe { (*sector).linecount } as usize;
        for j in 0..linecount {
            let li = unsafe { *((*sector).lines.add(j)) };
            if !li.is_null() {
                let line = unsafe { &*li };
                if !line.v1.is_null() {
                    let v = unsafe { &*line.v1 };
                    m_add_to_box(&mut bbox, v.x, v.y);
                }
                if !line.v2.is_null() {
                    let v = unsafe { &*line.v2 };
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
        unsafe {
            (*sector).soundorg.x = (bx_right + bx_left) / 2;
            (*sector).soundorg.y = (bx_top + bx_bottom) / 2;
        }
        let mut block = (bx_top - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;
        block = if block >= bmapheight { bmapheight - 1 } else { block };
        unsafe { (*sector).blockbox[BOXTOP] = block };
        let mut block = (bx_bottom - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
        block = if block < 0 { 0 } else { block };
        unsafe { (*sector).blockbox[BOXBOTTOM] = block };
        let mut block = (bx_right - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
        block = if block >= bmapwidth { bmapwidth - 1 } else { block };
        unsafe { (*sector).blockbox[BOXRIGHT] = block };
        let mut block = (bx_left - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
        block = if block < 0 { 0 } else { block };
        unsafe { (*sector).blockbox[BOXLEFT] = block };
    }
}
