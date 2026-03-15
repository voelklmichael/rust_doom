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
    pub vertexes: *mut Vertex,

    pub numsegs: i32,
    pub segs: *mut Seg,

    pub numsectors: i32,
    pub sectors: *mut Sector,

    pub numsubsectors: i32,
    pub subsectors: *mut Subsector,

    pub numnodes: i32,
    pub nodes: *mut Node,

    pub numlines: i32,
    pub lines: *mut Line,

    pub numsides: i32,
    pub sides: *mut SideDef,

    // Blockmap (from p_setup, used by p_maputl)
    pub bmaporgx: Fixed,
    pub bmaporgy: Fixed,
    pub bmapwidth: i32,
    pub bmapheight: i32,
    pub blockmap: *mut i16,
    pub blockmaplump: *mut i16,
    pub blocklinks: *mut *mut std::ffi::c_void,
    pub rejectmatrix: *mut u8,

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

    // BSP/seg state
    pub curline: *mut Seg,
    pub sidedef: *mut SideDef,
    pub linedef: *mut Line,
    pub frontsector: *mut Sector,
    pub backsector: *mut Sector,
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
            vertexes: std::ptr::null_mut(),
            numsegs: 0,
            segs: std::ptr::null_mut(),
            numsectors: 0,
            sectors: std::ptr::null_mut(),
            numsubsectors: 0,
            subsectors: std::ptr::null_mut(),
            numnodes: 0,
            nodes: std::ptr::null_mut(),
            numlines: 0,
            lines: std::ptr::null_mut(),
            numsides: 0,
            sides: std::ptr::null_mut(),
            bmaporgx: 0,
            bmaporgy: 0,
            bmapwidth: 0,
            bmapheight: 0,
            blockmap: std::ptr::null_mut(),
            blockmaplump: std::ptr::null_mut(),
            blocklinks: std::ptr::null_mut(),
            rejectmatrix: std::ptr::null_mut(),
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
            curline: std::ptr::null_mut(),
            sidedef: std::ptr::null_mut(),
            linedef: std::ptr::null_mut(),
            frontsector: std::ptr::null_mut(),
            backsector: std::ptr::null_mut(),
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
