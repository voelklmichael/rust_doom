//! Rust translation of doomgeneric/info.h
//! Thing frame/state LUT.

use crate::d_think::*;

fn _state_action_noop() {}

/// C typedef: spritenum_t (abbreviated - full enum has 200+ values)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: spritenum_t
pub enum SpritenumT {
    Troo,
    Play,
    Numsprites,
}

/// C typedef: statenum_t (abbreviated - full enum has 900+ values)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: statenum_t
pub enum StatenumT {
    Null,
    Numstates,
}

/// C typedef: state_t
#[repr(C)]
/// C typedef: state_t
pub struct StateT {
    pub sprite: SpritenumT,
    pub frame: i32,
    pub tics: i32,
    pub action: ActionfT,
    pub nextstate: StatenumT,
    pub misc1: i32,
    pub misc2: i32,
}

pub static mut states: [StateT; 1] = [StateT {
    sprite: SpritenumT::Troo,
    frame: 0,
    tics: 0,
    action: ActionfT {
        acv: _state_action_noop,
    },
    nextstate: StatenumT::Null,
    misc1: 0,
    misc2: 0,
}; 1];

pub static mut sprnames: [&str; 1] = [""; 1];

/// C typedef: mobjtype_t (abbreviated)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: mobjtype_t
pub enum MobjtypeT {
    Player,
    NuMMobjTypes,
}

/// C typedef: mobjinfo_t
#[repr(C)]
#[derive(Clone, Copy)]
/// C typedef: mobjinfo_t
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

/// C #define: NUMMOBJTYPES
pub const NUMMOBJTYPES: usize = 2;
pub static mut mobjinfo: [MobjinfoT; NUMMOBJTYPES] = [MobjinfoT {
    doomednum: 0,
    spawnstate: 0,
    spawnhealth: 0,
    seestate: 0,
    seesound: 0,
    reactiontime: 0,
    attacksound: 0,
    painstate: 0,
    painchance: 0,
    painsound: 0,
    meleestate: 0,
    missilestate: 0,
    deathstate: 0,
    xdeathstate: 0,
    deathsound: 0,
    speed: 0,
    radius: 0,
    height: 0,
    mass: 0,
    damage: 0,
    activesound: 0,
    flags: 0,
    raisestate: 0,
}; NUMMOBJTYPES];
