// m_bbox.h / m_bbox.c

pub use crate::m_fixed::*;

// Original: enum BOXTOP, BOXBOTTOM, BOXLEFT, BOXRIGHT
pub const BOXTOP: usize = 0;
pub const BOXBOTTOM: usize = 1;
pub const BOXLEFT: usize = 2;
pub const BOXRIGHT: usize = 3;

#[allow(non_camel_case_types)]
pub struct M_BboxState;

impl M_BboxState {
    pub fn new() -> Self {
        Self
    }

    // Original: M_ClearBox
    pub fn m_clear_box(&self, box_: &mut [FixedT; 4]) {
        box_[BOXTOP] = i32::MIN;
        box_[BOXRIGHT] = i32::MIN;
        box_[BOXBOTTOM] = i32::MAX;
        box_[BOXLEFT] = i32::MAX;
    }

    // Original: M_AddToBox
    pub fn m_add_to_box(&self, box_: &mut [FixedT; 4], x: FixedT, y: FixedT) {
        if x < box_[BOXLEFT] {
            box_[BOXLEFT] = x;
        } else if x > box_[BOXRIGHT] {
            box_[BOXRIGHT] = x;
        }
        if y < box_[BOXBOTTOM] {
            box_[BOXBOTTOM] = y;
        } else if y > box_[BOXTOP] {
            box_[BOXTOP] = y;
        }
    }
}
