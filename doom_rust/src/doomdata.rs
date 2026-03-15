//! Rust translation of doomgeneric/doomdata.h
//! All external data is defined here.

use crate::doomdef::*;
use crate::doomtype::*;

/// C enum: map lump order
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapLump {
    Label,
    Things,
    Linedefs,
    Sidedefs,
    Vertexes,
    Segs,
    Ssectors,
    Nodes,
    Sectors,
    Reject,
    Blockmap,
}

/// C typedef: mapvertex_t
#[repr(C, packed)]
pub struct MapvertexT {
    pub x: i16,
    pub y: i16,
}

/// C typedef: mapsidedef_t
#[repr(C, packed)]
pub struct MapsidedefT {
    pub textureoffset: i16,
    pub rowoffset: i16,
    pub toptexture: [i8; 8],
    pub bottomtexture: [i8; 8],
    pub midtexture: [i8; 8],
    pub sector: i16,
}

/// C typedef: maplinedef_t
#[repr(C, packed)]
pub struct MaplinedefT {
    pub v1: i16,
    pub v2: i16,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
}

pub const ML_BLOCKING: i16 = 1;
pub const ML_BLOCKMONSTERS: i16 = 2;
pub const ML_TWOSIDED: i16 = 4;
pub const ML_DONTPEGTOP: i16 = 8;
pub const ML_DONTPEGBOTTOM: i16 = 16;
pub const ML_SECRET: i16 = 32;
pub const ML_SOUNDBLOCK: i16 = 64;
pub const ML_DONTDRAW: i16 = 128;
pub const ML_MAPPED: i16 = 256;

/// C typedef: mapsector_t
#[repr(C, packed)]
pub struct MapsectorT {
    pub floorheight: i16,
    pub ceilingheight: i16,
    pub floorpic: [i8; 8],
    pub ceilingpic: [i8; 8],
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
}

/// C typedef: mapsubsector_t
#[repr(C, packed)]
pub struct MapsubsectorT {
    pub numsegs: i16,
    pub firstseg: i16,
}

/// C typedef: mapseg_t
#[repr(C, packed)]
pub struct MapsegT {
    pub v1: i16,
    pub v2: i16,
    pub angle: i16,
    pub linedef: i16,
    pub side: i16,
    pub offset: i16,
}

pub const NF_SUBSECTOR: u16 = 0x8000;

/// C typedef: mapnode_t
#[repr(C, packed)]
pub struct MapnodeT {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub bbox: [[i16; 4]; 2],
    pub children: [u16; 2],
}

/// C typedef: mapthing_t
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MapthingT {
    pub x: i16,
    pub y: i16,
    pub angle: i16,
    pub type_: i16,
    pub options: i16,
}
