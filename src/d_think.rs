//! Thinker/action function types (d_think.h)
//! Original: d_think.h

// typedef void (*actionf_v)();
pub type ActionfV = fn();

// typedef void (*actionf_p1)(void*);
pub type ActionfP1 = fn(*mut std::ffi::c_void);

// typedef void (*actionf_p2)(void*, void*);
pub type ActionfP2 = fn(*mut std::ffi::c_void, *mut std::ffi::c_void);

// typedef union { actionf_v acv; actionf_p1 acp1; actionf_p2 acp2; } actionf_t
// Plan §2.6: Union → enum with variants
#[derive(Clone, Copy)]
pub enum ActionfT {
    Acv(ActionfV),
    Acp1(ActionfP1),
    Acp2(ActionfP2),
}

// typedef actionf_t think_t
pub type ThinkT = ActionfT;

// typedef struct thinker_s { struct thinker_s* prev, next; think_t function; } thinker_t
#[repr(C)]
pub struct ThinkerS {
    // struct thinker_s* prev
    pub prev: *mut ThinkerS,
    // struct thinker_s* next
    pub next: *mut ThinkerS,
    // think_t function
    pub function: ThinkT,
}
