//! Rust translation of doomgeneric/r_state.h
//! Refresh/render internal state variables (global).

use crate::d_player::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_video::*;
use crate::m_fixed::*;
use crate::tables::*;

pub static mut textureheight: *mut FixedT = std::ptr::null_mut();
pub static mut spritewidth: *mut FixedT = std::ptr::null_mut();
pub static mut spriteoffset: *mut FixedT = std::ptr::null_mut();
pub static mut spritetopoffset: *mut FixedT = std::ptr::null_mut();
pub static mut colormaps: *mut crate::r_defs::LighttableT = std::ptr::null_mut();

pub static mut viewwidth: i32 = 0;
pub static mut scaledviewwidth: i32 = 0;
pub static mut viewheight: i32 = 0;
pub static mut firstflat: i32 = 0;
pub static mut flattranslation: *mut i32 = std::ptr::null_mut();
pub static mut texturetranslation: *mut i32 = std::ptr::null_mut();

pub static mut firstspritelump: i32 = 0;
pub static mut lastspritelump: i32 = 0;
pub static mut numspritelumps: i32 = 0;

pub static mut numsprites: i32 = 0;
pub static mut sprites: *mut crate::r_defs::SpritedefT = std::ptr::null_mut();
pub static mut numvertexes: i32 = 0;
pub static mut vertexes: *mut crate::r_defs::VertexT = std::ptr::null_mut();
pub static mut numsegs: i32 = 0;
pub static mut segs: *mut crate::r_defs::SegT = std::ptr::null_mut();
pub static mut numsectors: i32 = 0;
pub static mut sectors: *mut crate::r_defs::SectorT = std::ptr::null_mut();
pub static mut numsubsectors: i32 = 0;
pub static mut subsectors: *mut crate::r_defs::SubsectorT = std::ptr::null_mut();
pub static mut numnodes: i32 = 0;
pub static mut nodes: *mut crate::r_defs::NodeT = std::ptr::null_mut();
pub static mut numlines: i32 = 0;
pub static mut lines: *mut crate::r_defs::LineT = std::ptr::null_mut();
pub static mut numsides: i32 = 0;
pub static mut sides: *mut crate::r_defs::SideT = std::ptr::null_mut();

pub static mut viewx: FixedT = 0;
pub static mut viewy: FixedT = 0;
pub static mut viewz: FixedT = 0;
pub static mut viewangle: AngleT = 0;
pub static mut viewplayer: *mut PlayerT = std::ptr::null_mut();

pub static mut clipangle: AngleT = 0;
pub static mut viewangletox: [i32; (FINEANGLES / 2) as usize] = [0; (FINEANGLES / 2) as usize];
pub static mut xtoviewangle: [AngleT; (SCREENWIDTH + 1) as usize] = [0; (SCREENWIDTH + 1) as usize];

pub static mut rw_distance: FixedT = 0;
pub static mut rw_normalangle: AngleT = 0;
pub static mut rw_angle1: i32 = 0;
pub static mut sscount: i32 = 0;

pub static mut floorplane: *mut crate::r_defs::VisplaneT = std::ptr::null_mut();
pub static mut ceilingplane: *mut crate::r_defs::VisplaneT = std::ptr::null_mut();
