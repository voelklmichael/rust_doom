//! Thing/mobj info tables (mobjinfo_t, state_t).
//!
//! Original: info.h + info.c (minimal subset)

pub mod tables;
pub mod types;

pub use tables::{states, MOBJINFO, MT_PLAYER, MT_TELEPORTMAN, NUMMOBJTYPES, NUMSTATES};
pub use types::{Mobjinfo, Mobjtype, Spritenum, State, Statenum, S_NULL};
