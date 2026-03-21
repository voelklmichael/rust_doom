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
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MapsidedefT {
    pub textureoffset: i16,
    pub rowoffset: i16,
    pub toptexture: [u8; 8],
    pub bottomtexture: [u8; 8],
    pub midtexture: [u8; 8],
    pub sector: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MaplinedefT {
    pub v1: i16,
    pub v2: i16,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MapsectorT {
    pub floorheight: i16,
    pub ceilingheight: i16,
    pub floorpic: [u8; 8],
    pub ceilingpic: [u8; 8],
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MapsubsectorT {
    pub numsegs: i16,
    pub firstseg: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MapsegT {
    pub v1: i16,
    pub v2: i16,
    pub angle: i16,
    pub linedef: i16,
    pub side: i16,
    pub offset: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MapnodeT {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub bbox: [[i16; 4]; 2],
    pub children: [u16; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MapthingT {
    pub x: i16,
    pub y: i16,
    pub angle: i16,
    pub type_: i16,
    pub options: i16,
}

#[allow(non_camel_case_types)]
pub struct DoomdataState;

impl DoomdataState {
    pub fn new() -> Self {
        Self
    }
}
