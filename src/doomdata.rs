//! Map/WAD data structures (doomdata.h)
//! Original: doomdata.h

use crate::doomtype::Byte;

// enum map lump order
pub const ML_LABEL: i32 = 0;
pub const ML_THINGS: i32 = 1;
pub const ML_LINEDEFS: i32 = 2;
pub const ML_SIDEDEFS: i32 = 3;
pub const ML_VERTEXES: i32 = 4;
pub const ML_SEGS: i32 = 5;
pub const ML_SSECTORS: i32 = 6;
pub const ML_NODES: i32 = 7;
pub const ML_SECTORS: i32 = 8;
pub const ML_REJECT: i32 = 9;
pub const ML_BLOCKMAP: i32 = 10;

// typedef struct mapvertex_t
#[repr(C, packed)]
pub struct MapvertexT {
    pub x: i16,
    pub y: i16,
}

// typedef struct mapsidedef_t
#[repr(C, packed)]
pub struct MapsidedefT {
    pub textureoffset: i16,
    pub rowoffset: i16,
    pub toptexture: [Byte; 8],
    pub bottomtexture: [Byte; 8],
    pub midtexture: [Byte; 8],
    pub sector: i16,
}

// typedef struct maplinedef_t
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
