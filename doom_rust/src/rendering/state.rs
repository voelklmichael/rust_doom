// TODO(UNSAFE_ELIMINATION): Remove when migrated to Arc<Mutex<T>>
#[allow(unsafe_code)]
//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Refresh/render internal state variables.
//
// Original: r_state.h

use crate::geometry::FINEANGLES;
use crate::m_fixed::Fixed;
use crate::rendering::defs::{
    Angle, DrawSeg, LightTable, Line, Node, Seg, Sector, SideDef, Spritedef, Subsector, Vertex,
    Visplane,
};
use std::sync::{Arc, Mutex, OnceLock};

// =============================================================================
// State struct - thread-safe via OnceLock + Arc + Mutex
// =============================================================================

static RENDER_STATE: OnceLock<Arc<Mutex<RenderState>>> = OnceLock::new();

/// Safety: Raw pointers in RenderState are only used while holding the Mutex lock.
/// They point to zone-allocated data that outlives the render state.
unsafe impl Send for RenderState {}

pub struct RenderState {
    // Texture pegging
    pub textureheight: *mut Fixed,
    pub spritewidth: *mut Fixed,
    pub spriteoffset: *mut Fixed,
    pub spritetopoffset: *mut Fixed,
    pub colormaps: *mut LightTable,

    pub viewwidth: i32,
    pub scaledviewwidth: i32,
    pub viewheight: i32,

    pub firstflat: i32,
    pub flattranslation: *mut i32,
    pub texturetranslation: *mut i32,

    pub firstspritelump: i32,
    pub lastspritelump: i32,
    pub numspritelumps: i32,

    pub numsprites: i32,
    pub sprites: *mut Spritedef,

    pub numvertexes: i32,
    pub vertexes: Vec<Vertex>,

    pub numsegs: i32,
    pub segs: Vec<Seg>,

    pub numsectors: i32,
    pub sectors: Vec<Sector>,

    pub numsubsectors: i32,
    pub subsectors: Vec<Subsector>,

    pub numnodes: i32,
    pub nodes: Vec<Node>,

    pub numlines: i32,
    pub lines: Vec<Line>,

    pub numsides: i32,
    pub sides: Vec<SideDef>,

    // Blockmap (from p_setup, used by p_maputl) - Vec-based, no raw pointers
    pub bmaporgx: Fixed,
    pub bmaporgy: Fixed,
    pub bmapwidth: i32,
    pub bmapheight: i32,
    pub blockmaplump: Vec<i16>,
    pub blocklinks: Vec<*mut std::ffi::c_void>,
    pub rejectmatrix: Vec<u8>,

    // POV data
    pub viewx: Fixed,
    pub viewy: Fixed,
    pub viewz: Fixed,
    pub viewangle: Angle,
    pub viewangleoffset: Angle,
    pub clipangle: Angle,

    pub viewangletox: [i32; FINEANGLES / 2],
    pub xtoviewangle: [Angle; 321],

    pub rw_distance: Fixed,
    pub rw_normalangle: Angle,
    pub rw_angle1: i32,
    pub sscount: i32,

    pub floorplane: *mut Visplane,
    pub ceilingplane: *mut Visplane,

    // BSP/seg state (indices into Vecs above)
    pub curline_idx: Option<usize>,
    pub sidedef_idx: Option<usize>,
    pub linedef_idx: Option<usize>,
    pub frontsector_idx: Option<usize>,
    pub backsector_idx: Option<usize>,
    pub drawsegs: [DrawSeg; crate::rendering::defs::MAXDRAWSEGS],
    pub ds_p: *mut DrawSeg,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            textureheight: std::ptr::null_mut(),
            spritewidth: std::ptr::null_mut(),
            spriteoffset: std::ptr::null_mut(),
            spritetopoffset: std::ptr::null_mut(),
            colormaps: std::ptr::null_mut(),
            viewwidth: 0,
            scaledviewwidth: 0,
            viewheight: 0,
            firstflat: 0,
            flattranslation: std::ptr::null_mut(),
            texturetranslation: std::ptr::null_mut(),
            firstspritelump: 0,
            lastspritelump: 0,
            numspritelumps: 0,
            numsprites: 0,
            sprites: std::ptr::null_mut(),
            numvertexes: 0,
            vertexes: Vec::new(),
            numsegs: 0,
            segs: Vec::new(),
            numsectors: 0,
            sectors: Vec::new(),
            numsubsectors: 0,
            subsectors: Vec::new(),
            numnodes: 0,
            nodes: Vec::new(),
            numlines: 0,
            lines: Vec::new(),
            numsides: 0,
            sides: Vec::new(),
            bmaporgx: 0,
            bmaporgy: 0,
            bmapwidth: 0,
            bmapheight: 0,
            blockmaplump: Vec::new(),
            blocklinks: Vec::new(),
            rejectmatrix: Vec::new(),
            viewx: 0,
            viewy: 0,
            viewz: 0,
            viewangle: 0,
            viewangleoffset: 0,
            clipangle: 0,
            viewangletox: [0; FINEANGLES / 2],
            xtoviewangle: [0; 321],
            rw_distance: 0,
            rw_normalangle: 0,
            rw_angle1: 0,
            sscount: 0,
            floorplane: std::ptr::null_mut(),
            ceilingplane: std::ptr::null_mut(),
            curline_idx: None,
            sidedef_idx: None,
            linedef_idx: None,
            frontsector_idx: None,
            backsector_idx: None,
            drawsegs: unsafe { std::mem::zeroed() },
            ds_p: std::ptr::null_mut(),
        }
    }
}

fn get_state() -> &'static Arc<Mutex<RenderState>> {
    RENDER_STATE.get_or_init(|| Arc::new(Mutex::new(RenderState::default())))
}

/// Read from render state.
pub fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(&RenderState) -> R,
{
    let guard = get_state().lock().unwrap();
    f(&guard)
}

/// Write to render state.
pub fn with_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut RenderState) -> R,
{
    let mut guard = get_state().lock().unwrap();
    f(&mut guard)
}
