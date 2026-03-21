// doomdata.h — map / WAD lump structures (no .c)

pub use crate::doomdef::*;
pub use crate::doomtype::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MapLump {
    // Original: ML_LABEL, ML_THINGS, ...
    MlLabel = 0,
    MlThings = 1,
    MlLinedefs = 2,
    MlSidedefs = 3,
    MlVertexes = 4,
    MlSegs = 5,
    MlSsectors = 6,
    MlNodes = 7,
    MlSectors = 8,
    MlReject = 9,
    MlBlockmap = 10,
}

// Original: #define ML_BLOCKING 1
pub const ML_BLOCKING: i32 = 1;
// Original: #define ML_BLOCKMONSTERS 2
pub const ML_BLOCKMONSTERS: i32 = 2;
// Original: #define ML_TWOSIDED 4
pub const ML_TWOSIDED: i32 = 4;
// Original: #define ML_DONTPEGTOP 8
pub const ML_DONTPEGTOP: i32 = 8;
// Original: #define ML_DONTPEGBOTTOM 16
pub const ML_DONTPEGBOTTOM: i32 = 16;
// Original: #define ML_SECRET 32
pub const ML_SECRET: i32 = 32;
// Original: #define ML_SOUNDBLOCK 64
pub const ML_SOUNDBLOCK: i32 = 64;
// Original: #define ML_DONTDRAW 128
pub const ML_DONTDRAW: i32 = 128;
// Original: #define ML_MAPPED 256
pub const ML_MAPPED: i32 = 256;

// Original: #define NF_SUBSECTOR 0x8000
pub const NF_SUBSECTOR: u16 = 0x8000;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MapvertexT {
    // Original: x
    pub x: i16,
    // Original: y
    pub y: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MapsidedefT {
    // Original: textureoffset
    pub textureoffset: i16,
    // Original: rowoffset
    pub rowoffset: i16,
    // Original: toptexture
    pub toptexture: [u8; 8],
    // Original: bottomtexture
    pub bottomtexture: [u8; 8],
    // Original: midtexture
    pub midtexture: [u8; 8],
    // Original: sector
    pub sector: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MaplinedefT {
    // Original: v1
    pub v1: i16,
    // Original: v2
    pub v2: i16,
    // Original: flags
    pub flags: i16,
    // Original: special
    pub special: i16,
    // Original: tag
    pub tag: i16,
    // Original: sidenum
    pub sidenum: [i16; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MapsectorT {
    // Original: floorheight
    pub floorheight: i16,
    // Original: ceilingheight
    pub ceilingheight: i16,
    // Original: floorpic
    pub floorpic: [u8; 8],
    // Original: ceilingpic
    pub ceilingpic: [u8; 8],
    // Original: lightlevel
    pub lightlevel: i16,
    // Original: special
    pub special: i16,
    // Original: tag
    pub tag: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MapsubsectorT {
    // Original: numsegs
    pub numsegs: i16,
    // Original: firstseg
    pub firstseg: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MapsegT {
    // Original: v1
    pub v1: i16,
    // Original: v2
    pub v2: i16,
    // Original: angle
    pub angle: i16,
    // Original: linedef
    pub linedef: i16,
    // Original: side
    pub side: i16,
    // Original: offset
    pub offset: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MapnodeT {
    // Original: x
    pub x: i16,
    // Original: y
    pub y: i16,
    // Original: dx
    pub dx: i16,
    // Original: dy
    pub dy: i16,
    // Original: bbox
    pub bbox: [[i16; 4]; 2],
    // Original: children
    pub children: [u16; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MapthingT {
    // Original: x
    pub x: i16,
    // Original: y
    pub y: i16,
    // Original: angle
    pub angle: i16,
    // Original: type
    pub type_: i16,
    // Original: options
    pub options: i16,
}

#[allow(non_camel_case_types)]
pub struct DoomdataState;

impl DoomdataState {
    pub fn new() -> Self {
        Self
    }
}
