//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  MapObj data. Map Objects or mobjs are actors, entities,
//  thinker, take-your-pick... anything that moves, acts, or
//  suffers state changes of more or less violent nature.
//
// Original: d_think.h

use std::ptr;

// =============================================================================
// Action function pointer types (C: actionf_v, actionf_p1, actionf_p2)
// =============================================================================

/// No-argument action (C: actionf_v).
pub type ActionFV = unsafe extern "C" fn();

/// Single-pointer action - receives thinker context (C: actionf_p1).
pub type ActionFP1 = unsafe extern "C" fn(*mut ());

/// Two-pointer action (C: actionf_p2).
pub type ActionFP2 = unsafe extern "C" fn(*mut (), *mut ());

/// Union of action function types (C: actionf_t).
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(clippy::missing_inline_in_public_items)]
pub union ActionF {
    pub acv: ActionFV,
    pub acp1: ActionFP1,
    pub acp2: ActionFP2,
}

/// Think function type - historically another name for actionf_t.
pub type ThinkT = ActionF;

// =============================================================================
// Thinker - doubly linked list node for actors/thinkers
// =============================================================================

/// Doubly linked list of actors. Mobj embeds this as first member.
/// Original: thinker_t
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Thinker {
    pub prev: *mut Thinker,
    pub next: *mut Thinker,
    pub function: ThinkT,
}

impl std::fmt::Debug for Thinker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Thinker")
            .field("prev", &self.prev)
            .field("next", &self.next)
            .field("function", &"<action>")
            .finish()
    }
}

impl Thinker {
    /// Call the thinker's action with self as context (acp1 convention).
    /// Safe only when the function was registered as ActionFP1.
    #[inline]
    pub unsafe fn think_acp1(&mut self) {
        let f = self.function.acp1;
        f(self as *mut Thinker as *mut ());
    }
}

impl Default for Thinker {
    fn default() -> Self {
        Self {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            function: ActionF { acv: no_op },
        }
    }
}

/// No-op action for thinkers that don't need a think function.
pub unsafe extern "C" fn no_op() {}

/// Sentinel: when thinker.function.acp1 == this, the thinker is marked for removal.
/// P_RemoveThinker sets this; P_RunThinkers unlinks and frees.
pub unsafe extern "C" fn thinker_marked_removed(_: *mut ()) {}
