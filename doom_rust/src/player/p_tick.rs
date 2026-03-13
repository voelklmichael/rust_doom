//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Thinker list, level ticker.
//
// Original: p_tick.h / p_tick.c (partial - thinker list stubs)

use crate::game::d_think::Thinker;
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

/// Remove thinker - deferred deallocation.
/// Original: P_RemoveThinker (lazy; marks for removal)
pub fn p_remove_thinker(_thinker: *mut Thinker) {
    // TODO: Set function to invalid, defer actual removal until tick
}

/// Advance all thinkers one tic. Stub - iterates list but does not call think functions yet.
/// Original: P_Ticker
pub fn p_ticker() {
    // TODO: iterate thinkercap, call thinker.function.acp1(thinker) for each
    // For now, no-op
}

// TODO: leveltime - require full thinker iteration and mobj/player logic
