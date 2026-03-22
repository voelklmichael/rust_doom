//! Thing frame/state LUT (info.h, info.c)
//! Original: info.h, info.c
//!
//! Stage-0: types and minimal stubs. Full spritenum_t, statenum_t, mobjtype_t
//! enums and states/mobjinfo arrays to be migrated in later stages.

use crate::d_think::ActionfT;

// Type aliases for large enums (full variant lists in later stages)
pub type SpritenumT = i32;
pub type StatenumT = i32;
pub type MobjtypeT = i32;

// Key constants from enums
pub const NUMSPRITES: usize = 232;
pub const NUMSTATES: usize = 967;
pub const NUMMOBJTYPES: usize = 159;

/// typedef struct { sprite, frame, tics, action, nextstate, misc1, misc2 } state_t
#[derive(Clone)]
pub struct StateT {
    pub sprite: SpritenumT,
    pub frame: i32,
    pub tics: i32,
    pub action: ActionfT,
    pub nextstate: StatenumT,
    pub misc1: i32,
    pub misc2: i32,
}

/// typedef struct { doomednum, spawnstate, ... } mobjinfo_t
#[derive(Clone)]
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

pub struct InfoState {
    /// Original: state_t states[NUMSTATES]
    pub states: std::sync::Arc<std::sync::Mutex<Vec<StateT>>>,
    /// Original: mobjinfo_t mobjinfo[NUMMOBJTYPES]
    pub mobjinfo: std::sync::Arc<std::sync::Mutex<Vec<MobjinfoT>>>,
}

/// Original: char *sprnames[]
pub static SPRNAMES: &[&str] = &[
    "TROO", "SHTG", "PUNG", "PISG", "PISF", "SHTF", "SHT2", "CHGG", "CHGF", "MISG",
    "MISF", "SAWG", "PLSG", "PLSF", "BFGG", "BFGF", "BLUD", "PUFF", "BAL1", "BAL2",
    "PLSS", "PLSE", "MISL", "BFS1", "BFE1", "BFE2", "TFOG", "IFOG", "PLAY", "POSS",
    "SPOS", "VILE", "FIRE", "FATB", "FBXP", "SKEL", "MANF", "FATT", "CPOS", "SARG",
    "HEAD", "BAL7", "BOSS", "BOS2", "SKUL", "SPID", "BSPI", "APLS", "APBX", "CYBR",
    "PAIN", "SSWV", "KEEN", "BBRN", "BOSF", "ARM1", "ARM2", "BAR1", "BEXP", "FCAN",
    "BON1", "BON2", "BKEY", "RKEY", "YKEY", "BSKU", "RSKU", "YSKU", "STIM", "MEDI",
    "SOUL", "PINV", "PSTR", "PINS", "MEGA", "SUIT", "PMAP", "PVIS", "CLIP", "AMMO",
    "ROCK", "BROK", "CELL", "CELP", "SHEL", "SBOX", "BPAK", "BFUG", "MGUN", "CSAW",
    "LAUN", "PLAS", "SHOT", "SGN2", "COLU", "SMT2", "GOR1", "POL2", "POL5", "POL4",
    "POL3", "POL1", "POL6", "GOR2", "GOR3", "GOR4", "GOR5", "SMIT", "COL1", "COL2",
    "COL3", "COL4", "CAND", "CBRA", "COL6", "TRE1", "TRE2", "ELEC", "CEYE", "FSKU",
    "COL5", "TBLU", "TGRN", "TRED", "SMBT", "SMGT", "SMRT", "HDB1", "HDB2", "HDB3",
    "HDB4", "HDB5", "HDB6", "POB1", "POB2", "BRS1", "TLMP", "TLP2",
];

impl InfoState {
    /// Get state by index (original: states[i])
    pub fn get_state(&self, i: usize) -> Option<StateT> {
        self.states.lock().ok().and_then(|s| s.get(i).cloned())
    }

    /// Get mobjinfo by index (original: mobjinfo[i])
    pub fn get_mobjinfo(&self, i: usize) -> Option<MobjinfoT> {
        self.mobjinfo.lock().ok().and_then(|s| s.get(i).cloned())
    }
}
