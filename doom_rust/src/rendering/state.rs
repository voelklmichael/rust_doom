//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Refresh/render internal state variables (global).
//
// Original: r_state.h

use crate::geometry::FINEANGLES;
use crate::m_fixed::Fixed;
use crate::rendering::defs::{
    Angle, DrawSeg, LightTable, Line, Node, Seg, Sector, SideDef, Spritedef, Subsector, Vertex,
    Visplane,
};

// =============================================================================
// Public API (from .h) - state variables
// =============================================================================

// Texture pegging
pub static mut TEXTUREHEIGHT: *mut Fixed = std::ptr::null_mut();

// Pre-rendering (fracs)
pub static mut SPRITEWIDTH: *mut Fixed = std::ptr::null_mut();
pub static mut SPRITEOFFSET: *mut Fixed = std::ptr::null_mut();
pub static mut SPRITETOPOFFSET: *mut Fixed = std::ptr::null_mut();

pub static mut COLORMAPS: *mut LightTable = std::ptr::null_mut();

pub static mut VIEWWIDTH: i32 = 0;
pub static mut SCALEDVIEWWIDTH: i32 = 0;
pub static mut VIEWHEIGHT: i32 = 0;

pub static mut FIRSTFLAT: i32 = 0;

pub static mut FLATTRANSLATION: *mut i32 = std::ptr::null_mut();
pub static mut TEXTURETRANSLATION: *mut i32 = std::ptr::null_mut();

pub static mut FIRSTSPRITELUMP: i32 = 0;
pub static mut LASTSPRITELUMP: i32 = 0;
pub static mut NUMSPRITELUMPS: i32 = 0;

pub static mut NUMSPRITES: i32 = 0;
pub static mut SPRITES: *mut Spritedef = std::ptr::null_mut();

pub static mut NUMVERTEXES: i32 = 0;
pub static mut VERTEXES: *mut Vertex = std::ptr::null_mut();

pub static mut NUMSEGS: i32 = 0;
pub static mut SEGS: *mut Seg = std::ptr::null_mut();

pub static mut NUMSECTORS: i32 = 0;
pub static mut SECTORS: *mut Sector = std::ptr::null_mut();

pub static mut NUMSUBSECTORS: i32 = 0;
pub static mut SUBSECTORS: *mut Subsector = std::ptr::null_mut();

pub static mut NUMNODES: i32 = 0;
pub static mut NODES: *mut Node = std::ptr::null_mut();

pub static mut NUMLINES: i32 = 0;
pub static mut LINES: *mut Line = std::ptr::null_mut();

pub static mut NUMSIDES: i32 = 0;
pub static mut SIDES: *mut SideDef = std::ptr::null_mut();

// Blockmap (from p_setup, used by p_maputl)
pub static mut BMAPORGX: Fixed = 0;
pub static mut BMAPORGY: Fixed = 0;
pub static mut BMAPWIDTH: i32 = 0;
pub static mut BMAPHEIGHT: i32 = 0;
pub static mut BLOCKMAP: *mut i16 = std::ptr::null_mut();
pub static mut BLOCKMAPLUMP: *mut i16 = std::ptr::null_mut();
/// Thing chains per block (mobj_t** in C). Cast to *mut *mut Mobj when used.
pub static mut BLOCKLINKS: *mut *mut std::ffi::c_void = std::ptr::null_mut();

// POV data (viewplayer is from d_player - stub for now)
pub static mut VIEWX: Fixed = 0;
pub static mut VIEWY: Fixed = 0;
pub static mut VIEWZ: Fixed = 0;
pub static mut VIEWANGLE: Angle = 0;

pub static mut CLIPANGLE: Angle = 0;

pub static mut VIEWANGLETOX: [i32; FINEANGLES / 2] = [0; FINEANGLES / 2];
pub static mut XTOVIEWANGLE: [Angle; 321] = [0; 321]; // SCREENWIDTH+1

pub static mut RW_DISTANCE: Fixed = 0;
pub static mut RW_NORMALANGLE: Angle = 0;
pub static mut RW_ANGLE1: i32 = 0;
pub static mut SSCOUNT: i32 = 0;

pub static mut FLOORPLANE: *mut Visplane = std::ptr::null_mut();
pub static mut CEILINGPLANE: *mut Visplane = std::ptr::null_mut();

// BSP/seg state (set by r_bsp, read/written by r_segs)
pub static mut CURLINE: *mut Seg = std::ptr::null_mut();
pub static mut SIDEDEF: *mut SideDef = std::ptr::null_mut();
pub static mut LINEDEF: *mut Line = std::ptr::null_mut();
pub static mut FRONTSECTOR: *mut Sector = std::ptr::null_mut();
pub static mut BACKSECTOR: *mut Sector = std::ptr::null_mut();
pub static mut DRAWSEGS: [DrawSeg; crate::rendering::defs::MAXDRAWSEGS] =
    unsafe { std::mem::zeroed() };
pub static mut DS_P: *mut DrawSeg = std::ptr::null_mut();
