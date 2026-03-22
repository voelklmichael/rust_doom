//! Doom rewritten in Rust - stage 0 migration from C
//! Restarted: Tier 1 migration per docs/010_basic_plan.md
//! Preserves C-style naming (X_State) per plan.
#![allow(non_camel_case_types)]

pub mod config;
pub mod dstrings;
pub mod d_think;
pub mod d_ticcmd;
pub mod d_textur;
pub mod deh_main;
pub mod deh_misc;
pub mod deh_str;
pub mod doom;
pub mod doomfeatures;
pub mod doomgeneric;
pub mod doomkeys;
pub mod doomtype;
pub mod dummy;
pub mod i_swap;
pub mod m_bbox;
pub mod m_fixed;
pub mod m_random;
pub mod r_local;
pub mod r_sky;
pub mod r_state;
pub mod v_patch;
pub mod w_checksum;
pub mod w_file_stdc;
pub mod w_merge;
