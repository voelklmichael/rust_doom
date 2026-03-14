//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Ceiling movement (raising/lowering sectors).
//
// Original: p_ceilng.c (stub)

use crate::game::d_think::Thinker;
use crate::m_fixed::Fixed;
use crate::rendering::defs::{Line, Sector};

pub const MAXCEILINGS: usize = 30;

/// Ceiling mover thinker. Original: ceiling_t
#[repr(C)]
pub struct CeilingMover {
    pub thinker: Thinker,
    pub ceilingtype: i32,
    pub sector: *mut Sector,
    pub bottomheight: Fixed,
    pub topheight: Fixed,
    pub speed: Fixed,
    pub crush: bool,
    pub direction: i32,
    pub tag: i32,
    pub olddirection: i32,
}

/// Ceiling thinker. Original: T_MoveCeiling (stub - no-op for savegame compatibility)
pub unsafe extern "C" fn t_move_ceiling(ceiling: *mut ()) {
    let _ = ceiling;
}

static mut ACTIVECEILINGS: [*mut CeilingMover; MAXCEILINGS] = [std::ptr::null_mut(); MAXCEILINGS];

/// Original: P_AddActiveCeiling
pub fn p_add_active_ceiling(c: *mut CeilingMover) {
    if c.is_null() {
        return;
    }
    unsafe {
        for slot in &mut ACTIVECEILINGS {
            if (*slot).is_null() {
                *slot = c;
                return;
            }
        }
    }
}

/// Execute ceiling special. Original: EV_DoCeiling
/// Returns true if a ceiling mover was started.
pub fn ev_do_ceiling(
    _line: *const Line,
    _ceilingtype: i32,
) -> bool {
    let _ = (_line, _ceilingtype);
    false
}
