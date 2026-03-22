//! Bounding box (m_bbox.h, m_bbox.c)
//! Original: m_bbox.h, m_bbox.c

use crate::m_fixed::FixedT;

// enum { BOXTOP, BOXBOTTOM, BOXLEFT, BOXRIGHT }
pub const BOXTOP: usize = 0;
pub const BOXBOTTOM: usize = 1;
pub const BOXLEFT: usize = 2;
pub const BOXRIGHT: usize = 3;

pub struct M_BboxState;

impl M_BboxState {
    /// Original: void M_ClearBox(fixed_t* box)
    pub fn m_clear_box(&self, _box: &mut [FixedT; 4]) {
        // C body: (from m_bbox.c - would clear box to empty bounds)
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_AddToBox(fixed_t* box, fixed_t x, fixed_t y)
    pub fn m_add_to_box(&self, _box: &mut [FixedT; 4], _x: FixedT, _y: FixedT) {
        // C body: (from m_bbox.c - would expand box to include point)
        todo!("Basic stage-0 stub")
    }
}
