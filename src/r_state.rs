//! Refresh internal state (r_state.h)
//! Original: r_state.h

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_player::PlayerT;
use crate::m_fixed::FixedT;

pub struct R_StateState {
    pub textureheight: Arc<Mutex<Vec<FixedT>>>,
    pub spritewidth: Arc<Mutex<Vec<FixedT>>>,
    pub spriteoffset: Arc<Mutex<Vec<FixedT>>>,
    pub spritetopoffset: Arc<Mutex<Vec<FixedT>>>,
    pub colormaps: Arc<Mutex<Option<Vec<u8>>>>,
    pub viewwidth: Arc<Mutex<i32>>,
    pub scaledviewwidth: Arc<Mutex<i32>>,
    pub viewheight: Arc<Mutex<i32>>,
    pub firstflat: Arc<Mutex<i32>>,
    pub flattranslation: Arc<Mutex<Vec<i32>>>,
    pub texturetranslation: Arc<Mutex<Vec<i32>>>,
    pub firstspritelump: Arc<Mutex<i32>>,
    pub lastspritelump: Arc<Mutex<i32>>,
    pub numspritelumps: Arc<Mutex<i32>>,
    pub numsprites: Arc<Mutex<i32>>,
    pub sprites: Arc<Mutex<Vec<u8>>>, // spritedef_t*
    pub numvertexes: Arc<Mutex<i32>>,
    pub vertexes: Arc<Mutex<Vec<u8>>>, // vertex_t*
    pub numsegs: Arc<Mutex<i32>>,
    pub segs: Arc<Mutex<Vec<u8>>>, // seg_t*
    pub numsectors: Arc<Mutex<i32>>,
    pub sectors: Arc<Mutex<Vec<u8>>>, // sector_t*
    pub numsubsectors: Arc<Mutex<i32>>,
    pub subsectors: Arc<Mutex<Vec<u8>>>, // subsector_t*
    pub numnodes: Arc<Mutex<i32>>,
    pub nodes: Arc<Mutex<Vec<u8>>>, // node_t*
    pub numlines: Arc<Mutex<i32>>,
    pub lines: Arc<Mutex<Vec<u8>>>, // line_t*
    pub numsides: Arc<Mutex<i32>>,
    pub sides: Arc<Mutex<Vec<u8>>>, // side_t*
    pub viewx: Arc<Mutex<FixedT>>,
    pub viewy: Arc<Mutex<FixedT>>,
    pub viewz: Arc<Mutex<FixedT>>,
    pub viewangle: Arc<Mutex<i32>>, // angle_t
    pub viewplayer: Arc<Mutex<Option<PlayerT>>>,
    pub clipangle: Arc<Mutex<i32>>,
    pub viewangletox: Arc<Mutex<Vec<i32>>>,
    pub xtoviewangle: Arc<Mutex<Vec<i32>>>,
    pub rw_distance: Arc<Mutex<FixedT>>,
    pub rw_normalangle: Arc<Mutex<i32>>,
    pub rw_angle1: Arc<Mutex<i32>>,
    pub sscount: Arc<Mutex<i32>>,
    pub floorplane: Arc<Mutex<Vec<u8>>>, // visplane_t*
    pub ceilingplane: Arc<Mutex<Vec<u8>>>,
}
