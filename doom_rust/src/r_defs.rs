//! Rust translation of doomgeneric/r_defs.h
//! Refresh/rendering module, shared data struct definitions.

use crate::doomtype::*;
use crate::d_think::*;
use crate::i_video::*;
use crate::m_fixed::*;
use crate::p_mobj::MobjT;
use crate::v_patch::*;
use std::sync::{Arc, Mutex};

/// C #define: SIL_NONE
pub const SIL_NONE: i32 = 0;
/// C #define: SIL_BOTTOM
pub const SIL_BOTTOM: i32 = 1;
/// C #define: SIL_TOP
pub const SIL_TOP: i32 = 2;
/// C #define: SIL_BOTH
pub const SIL_BOTH: i32 = 3;

/// C #define: MAXDRAWSEGS
pub const MAXDRAWSEGS: usize = 256;

/// vertex_t
#[repr(C)]
/// C typedef: vertex_t
pub struct VertexT {
    pub x: FixedT,
    pub y: FixedT,
}

/// degenmobj_t
#[repr(C)]
/// C typedef: degenmobj_t
pub struct DegenmobjT {
    pub thinker: ThinkerT,
    pub x: FixedT,
    pub y: FixedT,
    pub z: FixedT,
}

/// sector_t (defined before LineT - LineT references SectorT)
#[repr(C)]
/// C typedef: sector_t
pub struct SectorT {
    pub floorheight: FixedT,
    pub ceilingheight: FixedT,
    pub floorpic: i16,
    pub ceilingpic: i16,
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
    pub soundtraversed: i32,
    pub soundtarget: Option<Arc<Mutex<MobjT>>>,
    pub blockbox: [i32; 4],
    pub soundorg: DegenmobjT,
    pub validcount: i32,
    pub thinglist: Option<Arc<Mutex<MobjT>>>,
    pub specialdata: Option<Arc<Mutex<Vec<u8>>>>,
    pub linecount: i32,
    pub lines: Option<Arc<Mutex<Vec<Arc<Mutex<LineT>>>>>>,
}

/// line_t
#[repr(C)]
/// C typedef: line_t
pub struct LineT {
    pub v1: Option<Arc<Mutex<VertexT>>>,
    pub v2: Option<Arc<Mutex<VertexT>>>,
    pub dx: FixedT,
    pub dy: FixedT,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
    pub bbox: [FixedT; 4],
    pub slopetype: SlopetypeT,
    pub frontsector: Option<Arc<Mutex<SectorT>>>,
    pub backsector: Option<Arc<Mutex<SectorT>>>,
    pub validcount: i32,
    pub specialdata: Option<Arc<Mutex<Vec<u8>>>>,
}

/// side_t
#[repr(C)]
/// C typedef: side_t
pub struct SideT {
    pub textureoffset: FixedT,
    pub rowoffset: FixedT,
    pub toptexture: i16,
    pub bottomtexture: i16,
    pub midtexture: i16,
    pub sector: Option<Arc<Mutex<SectorT>>>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: slopetype_t
pub enum SlopetypeT {
    SilHorizontal,
    SilVertical,
    SilPositive,
    SilNegative,
}

/// subsector_t
#[repr(C)]
/// C typedef: subsector_t
pub struct SubsectorT {
    pub sector: Option<Arc<Mutex<SectorT>>>,
    pub numlines: i16,
    pub firstline: i16,
}

/// seg_t
#[repr(C)]
/// C typedef: seg_t
pub struct SegT {
    pub v1: Option<Arc<Mutex<VertexT>>>,
    pub v2: Option<Arc<Mutex<VertexT>>>,
    pub offset: FixedT,
    pub angle: crate::tables::AngleT,
    pub sidedef: Option<Arc<Mutex<SideT>>>,
    pub linedef: Option<Arc<Mutex<LineT>>>,
    pub frontsector: Option<Arc<Mutex<SectorT>>>,
    pub backsector: Option<Arc<Mutex<SectorT>>>,
}

/// node_t
#[repr(C)]
/// C typedef: node_t
pub struct NodeT {
    pub x: FixedT,
    pub y: FixedT,
    pub dx: FixedT,
    pub dy: FixedT,
    pub bbox: [[FixedT; 4]; 2],
    pub children: [u16; 2],
}

/// C typedef: lighttable_t
pub type LighttableT = byte;

/// drawseg_t
#[repr(C)]
#[derive(Clone)]
/// C typedef: drawseg_t
pub struct DrawsegT {
    pub curline: Option<Arc<Mutex<SegT>>>,
    pub x1: i32,
    pub x2: i32,
    pub scale1: FixedT,
    pub scale2: FixedT,
    pub scalestep: FixedT,
    pub silhouette: i32,
    pub bsilheight: FixedT,
    pub tsilheight: FixedT,
    pub sprtopclip: Option<Arc<Mutex<Vec<i16>>>>,
    pub sprbottomclip: Option<Arc<Mutex<Vec<i16>>>>,
    pub maskedtexturecol: Option<Arc<Mutex<Vec<i16>>>>,
}

impl DrawsegT {
    pub const fn new() -> Self {
        Self {
            curline: None,
            x1: 0,
            x2: 0,
            scale1: 0,
            scale2: 0,
            scalestep: 0,
            silhouette: 0,
            bsilheight: 0,
            tsilheight: 0,
            sprtopclip: None,
            sprbottomclip: None,
            maskedtexturecol: None,
        }
    }
}

impl Default for DrawsegT {
    fn default() -> Self {
        Self::new()
    }
}

/// vissprite_t
#[repr(C)]
#[derive(Clone)]
/// C typedef: vissprite_t
pub struct VisspriteT {
    pub prev: Option<Arc<Mutex<VisspriteT>>>,
    pub next: Option<Arc<Mutex<VisspriteT>>>,
    pub x1: i32,
    pub x2: i32,
    pub gx: FixedT,
    pub gy: FixedT,
    pub gz: FixedT,
    pub gzt: FixedT,
    pub startfrac: FixedT,
    pub scale: FixedT,
    pub xiscale: FixedT,
    pub texturemid: FixedT,
    pub patch: i32,
    pub colormap: Option<Arc<Mutex<Vec<LighttableT>>>>,
    pub mobjflags: i32,
}

/// spriteframe_t
#[repr(C)]
/// C typedef: spriteframe_t
pub struct SpriteframeT {
    pub rotate: boolean,
    pub lump: [i16; 8],
    pub flip: [byte; 8],
}

/// spritedef_t
#[repr(C)]
/// C typedef: spritedef_t
pub struct SpritedefT {
    pub numframes: i32,
    pub spriteframes: Option<Arc<Mutex<Vec<Arc<Mutex<SpriteframeT>>>>>>,
}

/// visplane_t
#[repr(C)]
/// C typedef: visplane_t
pub struct VisplaneT {
    pub height: FixedT,
    pub picnum: i32,
    pub lightlevel: i32,
    pub minx: i32,
    pub maxx: i32,
    pub pad1: byte,
    pub top: [byte; SCREENWIDTH as usize],
    pub pad2: byte,
    pub pad3: byte,
    pub bottom: [byte; SCREENWIDTH as usize],
    pub pad4: byte,
}
