//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Platform movement (up/down, by sector).
//
// Original: p_plats.c (stub)

use crate::game::d_think::Thinker;
use crate::m_fixed::Fixed;
use crate::rendering::defs::{Line, Sector};

pub const MAXPLATS: usize = 30;

/// Platform thinker. Original: plat_t
#[repr(C)]
pub struct Plat {
    pub thinker: Thinker,
    pub sector: *mut Sector,
    pub speed: Fixed,
    pub low: Fixed,
    pub high: Fixed,
    pub wait: i32,
    pub count: i32,
    pub status: i32,
    pub oldstatus: i32,
    pub crush: bool,
    pub tag: i32,
    pub plattype: i32,
}

static mut ACTIVEPLATS: [*mut Plat; MAXPLATS] = [std::ptr::null_mut(); MAXPLATS];

/// Original: P_AddActivePlat
pub fn p_add_active_plat(plat: *mut Plat) {
    if plat.is_null() {
        return;
    }
    unsafe {
        for slot in &mut ACTIVEPLATS {
            if (*slot).is_null() {
                *slot = plat;
                return;
            }
        }
    }
}

/// Platform thinker. Original: T_PlatRaise (stub - no-op for savegame compatibility)
pub unsafe extern "C" fn t_plat_raise(plat: *mut ()) {
    let _ = plat;
}

/// Execute platform special. Original: EV_DoPlat
/// Returns true if a platform mover was started.
pub fn ev_do_plat(
    _line: *const Line,
    _plattype: i32,
    _amount: i32,
) -> bool {
    let _ = (_line, _plattype, _amount);
    false
}
