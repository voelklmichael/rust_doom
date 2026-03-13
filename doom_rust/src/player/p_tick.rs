//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Thinker list, level ticker.
//
// Original: p_tick.h / p_tick.c

use crate::game::d_think::{thinker_marked_removed, Thinker};
use crate::z_zone::z_free;
use std::ptr;

/// Both the head and tail of the thinker list.
/// Original: thinkercap
pub static mut THINKERCAP: Thinker = Thinker {
    prev: ptr::null_mut(),
    next: ptr::null_mut(),
    function: crate::game::d_think::ActionF {
        acv: crate::game::d_think::no_op,
    },
};

/// Initialize thinker list.
/// Original: P_InitThinkers
pub fn p_init_thinkers() {
    unsafe {
        THINKERCAP.prev = &mut THINKERCAP as *mut Thinker;
        THINKERCAP.next = &mut THINKERCAP as *mut Thinker;
    }
}

/// Add a new thinker at the end of the list.
/// Original: P_AddThinker
pub fn p_add_thinker(thinker: *mut Thinker) {
    if thinker.is_null() {
        return;
    }
    unsafe {
        (*thinker).next = &mut THINKERCAP as *mut Thinker;
        (*thinker).prev = THINKERCAP.prev;
        (*THINKERCAP.prev).next = thinker;
        THINKERCAP.prev = thinker;
    }
}

/// Remove thinker - deferred deallocation. Marks for removal; actual free in p_run_thinkers.
/// Original: P_RemoveThinker
pub fn p_remove_thinker(thinker: *mut Thinker) {
    if thinker.is_null() {
        return;
    }
    unsafe {
        (*thinker).function.acp1 = thinker_marked_removed;
    }
}

/// Run all thinkers. Removes and frees those marked by P_RemoveThinker.
/// Original: P_RunThinkers
pub fn p_run_thinkers() {
    unsafe {
        let mut current = THINKERCAP.next;
        while current != &mut THINKERCAP as *mut Thinker {
            let next = (*current).next;
            if (*current).function.acp1 == thinker_marked_removed {
                (*(*current).next).prev = (*current).prev;
                (*(*current).prev).next = (*current).next;
                z_free(current as *mut u8);
            } else {
                let acp1 = (*current).function.acp1;
                acp1(current as *mut ());
            }
            current = next;
        }
    }
}

/// Advance all thinkers one tic. Original: P_Ticker (partial - no player/menu logic)
pub fn p_ticker() {
    p_run_thinkers();
}
