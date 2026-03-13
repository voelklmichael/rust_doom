//! Thing/mobj info tables (mobjinfo_t, state_t).
//!
//! Original: info.h + info.c (minimal subset)

pub mod types;
pub mod tables;

pub use tables::{states, MOBJINFO, NUMMOBJTYPES, NUMSTATES};
pub use types::{Mobjinfo, Mobjtype, Spritenum, State, Statenum, S_NULL};