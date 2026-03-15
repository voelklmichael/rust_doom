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
use std::sync::{Mutex, OnceLock};

// =============================================================================
// PTickState - thread-safe via OnceLock + Mutex
// =============================================================================

/// Safety: Thinker contains raw pointers; access is serialized by Mutex.
unsafe impl Send for PTickState {}

static P_TICK_STATE: OnceLock<Mutex<PTickState>> = OnceLock::new();

pub struct PTickState {
    /// Both the head and tail of the thinker list. Original: thinkercap
    pub thinkercap: Thinker,
}

fn get_p_tick_state() -> &'static Mutex<PTickState> {
    P_TICK_STATE.get_or_init(|| {
        Mutex::new(PTickState {
            thinkercap: Thinker {
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
                function: crate::game::d_think::ActionF {
                    acv: crate::game::d_think::no_op,
                },
            },
        })
    })
}

/// Access PTickState.
pub fn with_ptick_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut PTickState) -> R,
{
    let mut guard = get_p_tick_state().lock().unwrap();
    f(&mut guard)
}

/// Initialize thinker list.
/// Original: P_InitThinkers
pub fn p_init_thinkers() {
    with_ptick_state(|s| {
        let cap = &mut s.thinkercap as *mut Thinker;
        s.thinkercap.prev = cap;
        s.thinkercap.next = cap;
    });
}

/// Add a new thinker at the end of the list.
/// Original: P_AddThinker
pub fn p_add_thinker(thinker: *mut Thinker) {
    if thinker.is_null() {
        return;
    }
    with_ptick_state(|s| {
        let cap = &mut s.thinkercap as *mut Thinker;
        unsafe {
            (*thinker).next = cap;
            (*thinker).prev = s.thinkercap.prev;
            (*s.thinkercap.prev).next = thinker;
            s.thinkercap.prev = thinker;
        }
    });
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
    with_ptick_state(|s| {
        let cap = &mut s.thinkercap as *mut Thinker;
        unsafe {
            let mut current = s.thinkercap.next;
            while current != cap {
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
    });
}

/// Advance all thinkers one tic. Original: P_Ticker (partial - no player/menu logic)
pub fn p_ticker() {
    p_run_thinkers();
}
