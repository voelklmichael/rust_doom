//! Map utilities (p_maputl.c)
//! Original: p_maputl.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::m_fixed::FixedT;
use crate::p_local::DivlineT;
use crate::doomtype::Boolean;

pub struct P_MaputlState {
    pub opentop: Arc<Mutex<FixedT>>,
    pub openbottom: Arc<Mutex<FixedT>>,
    pub openrange: Arc<Mutex<FixedT>>,
    pub lowfloor: Arc<Mutex<FixedT>>,
}

impl P_MaputlState {
    /// Original: fixed_t P_AproxDistance(fixed_t dx, fixed_t dy)
    pub fn p_aprox_distance(&self, _dx: FixedT, _dy: FixedT) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: int P_PointOnLineSide(fixed_t x, fixed_t y, line_t *line)
    pub fn p_point_on_line_side(&self, _x: FixedT, _y: FixedT, _line: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int P_PointOnDivlineSide(fixed_t x, fixed_t y, divline_t *line)
    pub fn p_point_on_divline_side(&self, _x: FixedT, _y: FixedT, _line: &DivlineT) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_MakeDivline(line_t *li, divline_t *dl)
    pub fn p_make_divline(&self, _li: &(), _dl: &mut DivlineT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: fixed_t P_InterceptVector(divline_t *v2, divline_t *v1)
    pub fn p_intercept_vector(&self, _v2: &DivlineT, _v1: &DivlineT) -> FixedT {
        todo!("Basic stage-0 stub")
    }

    /// Original: int P_BoxOnLineSide(fixed_t *tmbox, line_t *ld)
    pub fn p_box_on_line_side(&self, _tmbox: &[FixedT; 4], _ld: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_LineOpening(line_t *linedef)
    pub fn p_line_opening(&self, _linedef: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_UnsetThingPosition(mobj_t *thing)
    pub fn p_unset_thing_position(&self, _thing: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SetThingPosition(mobj_t *thing)
    pub fn p_set_thing_position(&self, _thing: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_BlockLinesIterator(int x, int y, boolean (*func)(line_t*))
    pub fn p_block_lines_iterator(&self, _x: i32, _y: i32) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_BlockThingsIterator(int x, int y, boolean (*func)(mobj_t*))
    pub fn p_block_things_iterator(&self, _x: i32, _y: i32) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_PathTraverse(...)
    pub fn p_path_traverse(&self, _x1: FixedT, _y1: FixedT, _x2: FixedT, _y2: FixedT, _flags: i32) -> Boolean {
        todo!("Basic stage-0 stub")
    }
}
