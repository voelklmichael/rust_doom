//! Rust translation of doomgeneric/d_think.h
//! MapObj data - thinker/action function types.

use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

/// C typedef: actionf_v - void (*)()
pub type ActionfV = fn();

/// C typedef: actionf_p1 - void (*)(void*)
pub type ActionfP1 = fn(&mut c_void);

/// C typedef: actionf_p2 - void (*)(void*, void*)
pub type ActionfP2 = fn(&mut c_void, &mut c_void);

/// C typedef: actionf_t union
#[repr(C)]
pub union ActionfT {
    pub acv: ActionfV,
    pub acp1: ActionfP1,
    pub acp2: ActionfP2,
}

/// C typedef: think_t
pub type ThinkT = ActionfT;

fn _think_noop() {}

/// C typedef: thinker_t - doubly linked list of actors
#[repr(C)]
/// C typedef: thinker_t
pub struct ThinkerT {
    pub prev: Option<Arc<Mutex<ThinkerT>>>,
    pub next: Option<Arc<Mutex<ThinkerT>>>,
    pub function: ThinkT,
}

impl ThinkerT {
    pub const fn new() -> Self {
        Self {
            prev: None,
            next: None,
            function: ThinkT { acv: _think_noop },
        }
    }
}

impl Default for ThinkerT {
    fn default() -> Self {
        Self::new()
    }
}
