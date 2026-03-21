// doomgeneric/info.h — thing states / mobj info tables (data tables stubbed; see info.c)
#![allow(non_camel_case_types)]

use std::cell::RefCell;

pub use crate::d_think::*;

// Enums copied from info.h (generated body; do not hand-edit enum variants).
include!("info_data.rs");

// Original: NUMSPRITES / NUMSTATES / NUMMOBJTYPES (last enumerator = count)
pub const NUMSPRITES: i32 = SpritenumT::NUMSPRITES as i32;
pub const NUMSTATES: i32 = StatenumT::NUMSTATES as i32;
pub const NUMMOBJTYPES: i32 = MobjtypeT::NUMMOBJTYPES as i32;

/// Original: typedef struct { ... } state_t
#[repr(C)]
pub struct StateT {
    pub sprite: SpritenumT,
    pub frame: i32,
    pub tics: i32,
    pub action: ActionfT,
    pub nextstate: StatenumT,
    pub misc1: i32,
    pub misc2: i32,
}

/// Original: typedef struct { ... } mobjinfo_t
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MobjinfoT {
    pub doomednum: i32,
    pub spawnstate: i32,
    pub spawnhealth: i32,
    pub seestate: i32,
    pub seesound: i32,
    pub reactiontime: i32,
    pub attacksound: i32,
    pub painstate: i32,
    pub painchance: i32,
    pub painsound: i32,
    pub meleestate: i32,
    pub missilestate: i32,
    pub deathstate: i32,
    pub xdeathstate: i32,
    pub deathsound: i32,
    pub speed: i32,
    pub radius: i32,
    pub height: i32,
    pub mass: i32,
    pub damage: i32,
    pub activesound: i32,
    pub flags: i32,
    pub raisestate: i32,
}

/// Original: globals from info.c — `states`, `sprnames`, `mobjinfo`
#[allow(non_camel_case_types)]
pub struct InfoState {
    /// Original: state_t states[NUMSTATES]
    pub states: RefCell<Vec<StateT>>,
    /// Original: char *sprnames[]
    pub sprnames: RefCell<Vec<*const std::ffi::c_char>>,
    /// Original: mobjinfo_t mobjinfo[NUMMOBJTYPES]
    pub mobjinfo: RefCell<Vec<MobjinfoT>>,
}

impl InfoState {
    pub fn new() -> Self {
        Self {
            states: RefCell::new(Vec::new()),
            sprnames: RefCell::new(Vec::new()),
            mobjinfo: RefCell::new(Vec::new()),
        }
    }

    /// Populating `states` / `mobjinfo` belongs in info.c translation (>10 lines).
    pub fn init_from_info_c(&self) {
        todo!("init_from_info_c — table data from info.c");
    }
}
