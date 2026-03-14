//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Refresh/rendering module, shared data struct definitions.
//  Used by both play and refresh.
//
// Original: r_defs.h

use crate::doomdef::SCREENWIDTH_USIZE;
use crate::m_fixed::Fixed;
use crate::player::p_mobj::Mobj;

// =============================================================================
// Public API (from .h)
// =============================================================================

/// LineDef flags (from doomdata.h).
pub const ML_BLOCKING: i16 = 1;
pub const ML_BLOCKMONSTERS: i16 = 2;
pub const ML_TWOSIDED: i16 = 4;
pub const ML_DONTPEGTOP: i16 = 8;
pub const ML_DONTPEGBOTTOM: i16 = 16;
pub const ML_MAPPED: i16 = 256;
pub const ML_SECRET: i16 = 32;
pub const ML_DONTDRAW: i16 = 128;

/// Silhouette flags for clipping segs and sprites.
pub const SIL_NONE: i32 = 0;
pub const SIL_BOTTOM: i32 = 1;
pub const SIL_TOP: i32 = 2;
pub const SIL_BOTH: i32 = 3;

pub const MAXDRAWSEGS: usize = 256;

/// Angle type (from tables.h).
pub type Angle = u32;

/// Vertex - plain vanilla 2D point.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: Fixed,
    pub y: Fixed,
}

pub use crate::game::d_think::Thinker;

/// Degen mobj for sector sound origin.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DegenMobj {
    pub thinker: Thinker,
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
}

/// Sector - runtime record, stores things/mobjs.
#[repr(C)]
#[derive(Debug)]
pub struct Sector {
    pub floorheight: Fixed,
    pub ceilingheight: Fixed,
    pub floorpic: i16,
    pub ceilingpic: i16,
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
    pub soundtraversed: i32,
    pub soundtarget: *mut Mobj,
    pub blockbox: [i32; 4],
    pub soundorg: DegenMobj,
    pub validcount: i32,
    pub thinglist: *mut Mobj,
    pub specialdata: *mut std::ffi::c_void,
    pub linecount: i32,
    pub lines: *mut *mut Line,
}

/// SideDef.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SideDef {
    pub textureoffset: Fixed,
    pub rowoffset: Fixed,
    pub toptexture: i16,
    pub bottomtexture: i16,
    pub midtexture: i16,
    pub sector: *mut Sector,
}

/// Slope type for line clipping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SlopeType {
    Horizontal = 0,
    Vertical = 1,
    Positive = 2,
    Negative = 3,
}

/// LineDef.
#[repr(C)]
#[derive(Debug)]
pub struct Line {
    pub v1: *mut Vertex,
    pub v2: *mut Vertex,
    pub dx: Fixed,
    pub dy: Fixed,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
    pub bbox: [Fixed; 4],
    pub slopetype: SlopeType,
    pub frontsector: *mut Sector,
    pub backsector: *mut Sector,
    pub validcount: i32,
    pub specialdata: *mut std::ffi::c_void,
}

/// SubSector - BSP leaf, list of line segs.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Subsector {
    pub sector: *mut Sector,
    pub numlines: i16,
    pub firstline: i16,
}

/// LineSeg.
#[repr(C)]
#[derive(Debug)]
pub struct Seg {
    pub v1: *mut Vertex,
    pub v2: *mut Vertex,
    pub offset: Fixed,
    pub angle: Angle,
    pub sidedef: *mut SideDef,
    pub linedef: *mut Line,
    pub frontsector: *mut Sector,
    pub backsector: *mut Sector,
}

/// BSP node.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub x: Fixed,
    pub y: Fixed,
    pub dx: Fixed,
    pub dy: Fixed,
    pub bbox: [[Fixed; 4]; 2],
    pub children: [u16; 2],
}

/// Light table - 8-bit colormap index.
pub type LightTable = u8;

/// Draw segment for BSP/seg rendering.
#[repr(C)]
#[derive(Debug)]
pub struct DrawSeg {
    pub curline: *mut Seg,
    pub x1: i32,
    pub x2: i32,
    pub scale1: Fixed,
    pub scale2: Fixed,
    pub scalestep: Fixed,
    pub silhouette: i32,
    pub bsilheight: Fixed,
    pub tsilheight: Fixed,
    pub sprtopclip: *mut i16,
    pub sprbottomclip: *mut i16,
    pub maskedtexturecol: *mut i16,
}

/// Visible sprite - thing to draw during refresh.
#[repr(C)]
#[derive(Debug)]
pub struct Vissprite {
    pub prev: *mut Vissprite,
    pub next: *mut Vissprite,
    pub x1: i32,
    pub x2: i32,
    pub gx: Fixed,
    pub gy: Fixed,
    pub gz: Fixed,
    pub gzt: Fixed,
    pub startfrac: Fixed,
    pub scale: Fixed,
    pub xiscale: Fixed,
    pub texturemid: Fixed,
    pub patch: i32,
    pub colormap: *mut LightTable,
    pub mobjflags: i32,
}

/// Sprite frame - rotation/flip for one view angle.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpriteFrame {
    pub rotate: bool,
    pub lump: [i16; 8],
    pub flip: [u8; 8],
}

/// Sprite definition - animation frames.
#[repr(C)]
#[derive(Debug)]
pub struct Spritedef {
    pub numframes: i32,
    pub spriteframes: *mut SpriteFrame,
}

/// Visplane - floor/ceiling plane for rendering.
#[repr(C)]
#[derive(Debug)]
pub struct Visplane {
    pub height: Fixed,
    pub picnum: i32,
    pub lightlevel: i32,
    pub minx: i32,
    pub maxx: i32,
    pub pad1: u8,
    pub top: [u8; SCREENWIDTH_USIZE],
    pub pad2: u8,
    pub pad3: u8,
    pub bottom: [u8; SCREENWIDTH_USIZE],
    pub pad4: u8,
}
