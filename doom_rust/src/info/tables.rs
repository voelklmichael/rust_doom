//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// Thing frame/state LUT data (minimal subset).
//
// Original: info.c

use std::sync::OnceLock;

use crate::game::d_think::{no_op_acp1, ActionF};
use crate::info::types::{Mobjinfo, Mobjtype, Spritenum, State, S_NULL};

// Sprite indices (from spritenum_t)
const _SPR_TROO: Spritenum = 0;  // Imp
const _SPR_POSS: Spritenum = 29; // Shotgun guy
const _SPR_PLAY: Spritenum = 58; // Player

/// Number of states. Full Doom has ~1000.
pub const NUMSTATES: usize = 10;

/// Number of mobj types. Full Doom has ~150.
pub const NUMMOBJTYPES: usize = 5;

/// MT_PLAYER
pub const MT_PLAYER: Mobjtype = 0;
/// MT_POSSESSED (shotgun guy)
pub const MT_POSSESSED: Mobjtype = 1;
/// MT_TROOP (imp)
pub const MT_TROOP: Mobjtype = 2;
/// MT_SERGEANT (shotgun guy - alias)
pub const MT_SERGEANT: Mobjtype = 3;
/// MT_HEAD (cacodemon - placeholder)
pub const MT_HEAD: Mobjtype = 4;

fn state_null() -> State {
    State {
        sprite: 0,
        frame: 0,
        tics: -1,
        action: ActionF { acp1: no_op_acp1 },
        nextstate: S_NULL,
        misc1: 0,
        misc2: 0,
    }
}

static STATES_INNER: OnceLock<[State; NUMSTATES]> = OnceLock::new();

/// States array. Minimal: S_NULL and a few placeholders.
/// Lazy-initialized because State contains function pointers.
pub fn states() -> &'static [State; NUMSTATES] {
    STATES_INNER.get_or_init(|| {
        let null = state_null();
        [
            null, null, null, null, null, null, null, null, null, null,
        ]
    })
}

/// Mobjinfo array. Minimal: player, shotgun guy, imp.
pub static MOBJINFO: [Mobjinfo; NUMMOBJTYPES] = [
    Mobjinfo {
        doomednum: 1,
        spawnstate: 1,
        spawnhealth: 100,
        seestate: 1,
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
        radius: 16,
        height: 56,
        mass: 100,
        damage: 0,
        activesound: 0,
        flags: 0,
        raisestate: 0,
    }, // MT_PLAYER
    Mobjinfo {
        doomednum: 3004,
        spawnstate: 1,
        spawnhealth: 20,
        seestate: 1,
        seesound: 0,
        reactiontime: 8,
        attacksound: 0,
        painstate: 0,
        painchance: 200,
        painsound: 0,
        meleestate: 0,
        missilestate: 0,
        deathstate: 0,
        xdeathstate: 0,
        deathsound: 0,
        speed: 8,
        radius: 20,
        height: 56,
        mass: 100,
        damage: 0,
        activesound: 0,
        flags: 0,
        raisestate: 0,
    }, // MT_POSSESSED
    Mobjinfo {
        doomednum: 3002,
        spawnstate: 1,
        spawnhealth: 60,
        seestate: 1,
        seesound: 0,
        reactiontime: 8,
        attacksound: 0,
        painstate: 0,
        painchance: 200,
        painsound: 0,
        meleestate: 0,
        missilestate: 0,
        deathstate: 0,
        xdeathstate: 0,
        deathsound: 0,
        speed: 8,
        radius: 20,
        height: 56,
        mass: 100,
        damage: 0,
        activesound: 0,
        flags: 0,
        raisestate: 0,
    }, // MT_TROOP
    Mobjinfo {
        doomednum: 3003,
        spawnstate: 1,
        spawnhealth: 30,
        seestate: 1,
        seesound: 0,
        reactiontime: 8,
        attacksound: 0,
        painstate: 0,
        painchance: 200,
        painsound: 0,
        meleestate: 0,
        missilestate: 0,
        deathstate: 0,
        xdeathstate: 0,
        deathsound: 0,
        speed: 8,
        radius: 20,
        height: 56,
        mass: 100,
        damage: 0,
        activesound: 0,
        flags: 0,
        raisestate: 0,
    }, // MT_SERGEANT
    Mobjinfo {
        doomednum: 3005,
        spawnstate: 1,
        spawnhealth: 400,
        seestate: 1,
        seesound: 0,
        reactiontime: 8,
        attacksound: 0,
        painstate: 0,
        painchance: 100,
        painsound: 0,
        meleestate: 0,
        missilestate: 0,
        deathstate: 0,
        xdeathstate: 0,
        deathsound: 0,
        speed: 8,
        radius: 31,
        height: 56,
        mass: 400,
        damage: 0,
        activesound: 0,
        flags: 0,
        raisestate: 0,
    }, // MT_HEAD
];
