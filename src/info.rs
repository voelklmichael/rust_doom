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
    // Original: sprite
    pub sprite: SpritenumT,
    // Original: frame
    pub frame: i32,
    // Original: tics
    pub tics: i32,
    // Original: action
    pub action: ActionfT,
    // Original: nextstate
    pub nextstate: StatenumT,
    // Original: misc1
    pub misc1: i32,
    // Original: misc2
    pub misc2: i32,
}

/// Original: typedef struct { ... } mobjinfo_t
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MobjinfoT {
    // Original: doomednum
    pub doomednum: i32,
    // Original: spawnstate
    pub spawnstate: i32,
    // Original: spawnhealth
    pub spawnhealth: i32,
    // Original: seestate
    pub seestate: i32,
    // Original: seesound
    pub seesound: i32,
    // Original: reactiontime
    pub reactiontime: i32,
    // Original: attacksound
    pub attacksound: i32,
    // Original: painstate
    pub painstate: i32,
    // Original: painchance
    pub painchance: i32,
    // Original: painsound
    pub painsound: i32,
    // Original: meleestate
    pub meleestate: i32,
    // Original: missilestate
    pub missilestate: i32,
    // Original: deathstate
    pub deathstate: i32,
    // Original: xdeathstate
    pub xdeathstate: i32,
    // Original: deathsound
    pub deathsound: i32,
    // Original: speed
    pub speed: i32,
    // Original: radius
    pub radius: i32,
    // Original: height
    pub height: i32,
    // Original: mass
    pub mass: i32,
    // Original: damage
    pub damage: i32,
    // Original: activesound
    pub activesound: i32,
    // Original: flags
    pub flags: i32,
    // Original: raisestate
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
    // Original: init_from_info_c — table data from info.c
    pub fn init_from_info_c(&self) {
        todo!("init_from_info_c — table data from info.c");
    }
}
