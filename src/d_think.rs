// d_think.h - thinker/action function types
// No dependencies (leaf module)

use std::ffi::c_void;

// Original: typedef void (*actionf_v)()
pub type ActionfV = unsafe extern "C" fn();

// Original: typedef void (*actionf_p1)(void*)
pub type ActionfP1 = unsafe extern "C" fn(*mut c_void);

// Original: typedef void (*actionf_p2)(void*, void*)
pub type ActionfP2 = unsafe extern "C" fn(*mut c_void, *mut c_void);

// Original: typedef union { actionf_v acv; actionf_p1 acp1; actionf_p2 acp2; } actionf_t
#[repr(C)]
pub union ActionfT {
    // Original: acv
    pub acv: ActionfV,
    // Original: acp1
    pub acp1: ActionfP1,
    // Original: acp2
    pub acp2: ActionfP2,
}

// Original: typedef actionf_t think_t
pub type ThinkT = ActionfT;

// Original: struct thinker_s
#[repr(C)]
pub struct ThinkerS {
    // Original: prev
    pub prev: *mut ThinkerS,
    // Original: next
    pub next: *mut ThinkerS,
    // Original: function
    pub function: ThinkT,
}

pub type ThinkerT = ThinkerS;
