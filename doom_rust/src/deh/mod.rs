//! DeHackEd support (stubs when FEATURE_DEHACKED is false).
//!
//! Original: deh_main.h, deh_misc.h, deh_str.h

pub mod deh_main;
pub mod misc;
pub mod deh_str;

pub use deh_str::deh_string;
