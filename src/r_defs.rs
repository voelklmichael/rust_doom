//! Render definitions (r_defs.h)
//! Original: r_defs.h

use crate::m_fixed::FixedT;

pub const SIL_NONE: i32 = 0;
pub const SIL_BOTTOM: i32 = 1;
pub const SIL_TOP: i32 = 2;
pub const SIL_BOTH: i32 = 3;

pub struct VertexT {
    pub x: FixedT,
    pub y: FixedT,
}

pub struct DegenmobjT {
    pub x: FixedT,
    pub y: FixedT,
    pub z: FixedT,
}

pub struct SectorT {
    pub floorheight: FixedT,
    pub ceilingheight: FixedT,
    pub floorpic: i16,
    pub ceilingpic: i16,
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
    pub soundtraversed: i32,
    pub validcount: i32,
}

pub struct LineT;
pub struct SideT;
pub struct SegT;
pub struct SubsectorT;
pub struct NodeT;
pub struct DrawsegT;
