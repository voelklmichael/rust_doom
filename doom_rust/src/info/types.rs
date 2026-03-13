//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// Thing frame/state LUT types (minimal subset).
//
// Original: info.h

use crate::game::d_think::ActionF;

/// State index (statenum_t). Full enum has ~1000 entries.
pub type Statenum = i32;

/// Sprite index (spritenum_t). Full enum has ~150 entries.
pub type Spritenum = i32;

/// Mobj type index (mobjtype_t). Full enum has ~150 entries.
pub type Mobjtype = i32;

/// S_NULL - null state.
pub const S_NULL: Statenum = 0;

/// State structure - one frame of a thing's animation.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct State {
    pub sprite: Spritenum,
    pub frame: i32,
    pub tics: i32,
    pub action: ActionF,
    pub nextstate: Statenum,
    pub misc1: i32,
    pub misc2: i32,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("sprite", &self.sprite)
            .field("frame", &self.frame)
            .field("tics", &self.tics)
            .field("action", &"<action>")
            .field("nextstate", &self.nextstate)
            .field("misc1", &self.misc1)
            .field("misc2", &self.misc2)
            .finish()
    }
}

/// Mobj info - defines a thing type's behavior and appearance.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Mobjinfo {
    pub doomednum: i32,
    pub spawnstate: Statenum,
    pub spawnhealth: i32,
    pub seestate: Statenum,
    pub seesound: i32,
    pub reactiontime: i32,
    pub attacksound: i32,
    pub painstate: Statenum,
    pub painchance: i32,
    pub painsound: i32,
    pub meleestate: Statenum,
    pub missilestate: Statenum,
    pub deathstate: Statenum,
    pub xdeathstate: Statenum,
    pub deathsound: i32,
    pub speed: i32,
    pub radius: i32,
    pub height: i32,
    pub mass: i32,
    pub damage: i32,
    pub activesound: i32,
    pub flags: i32,
    pub raisestate: Statenum,
}
