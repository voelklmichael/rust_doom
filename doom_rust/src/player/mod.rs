//! Player / game logic module.
//!
//! Original: p_local.h (aggregator) + p_*.c / p_*.h
//! See docs/PLAYER_TRANSLATION_PLAN.md for translation plan.

use crate::m_fixed::{Fixed, FRACBITS, FRACUNIT};

pub mod p_ceilng;
pub mod p_doors;
pub mod p_enemy;
pub mod p_floor;
pub mod p_inter;
pub mod p_lights;
pub mod p_map;
pub mod p_maputl;
pub mod p_mobj;
pub mod p_plats;
pub mod p_pspr;
pub mod p_saveg;
pub mod p_setup;
pub mod p_sight;
pub mod p_spec;
pub mod p_switch;
pub mod p_telept;
pub mod p_tick;
pub mod p_user;

// =============================================================================
// p_local.h constants and types
// =============================================================================

/// FLOATSPEED - floating object speed.
pub const FLOATSPEED: Fixed = FRACUNIT * 4;

/// MAXHEALTH - maximum player health.
pub const MAXHEALTH: i32 = 100;

/// VIEWHEIGHT - player view height above floor.
pub const VIEWHEIGHT: Fixed = 41 * FRACUNIT;

/// MAPBLOCKUNITS - blockmap cell size in map units.
pub const MAPBLOCKUNITS: i32 = 128;
/// MAPBLOCKSIZE - blockmap cell size in fixed_t.
pub const MAPBLOCKSIZE: Fixed = (MAPBLOCKUNITS as Fixed) * FRACUNIT;
/// MAPBLOCKSHIFT - shift to convert to block index.
pub const MAPBLOCKSHIFT: i32 = FRACBITS + 7;
/// MAPBTOFRAC - fractional position within block (for P_PathTraverse).
pub const MAPBTOFRAC: i32 = MAPBLOCKSHIFT - FRACBITS;
/// MAPBMASK - mask for block alignment.
pub const MAPBMASK: Fixed = MAPBLOCKSIZE - 1;

/// P_PathTraverse flags.
pub const PT_ADDLINES: i32 = 1;
pub const PT_ADDTHINGS: i32 = 2;
pub const PT_EARLYOUT: i32 = 4;

/// PLAYERRADIUS - player radius for movement checking.
pub const PLAYERRADIUS: Fixed = 16 * FRACUNIT;
/// MAXRADIUS - max thing radius for sector block boxes.
pub const MAXRADIUS: Fixed = 32 * FRACUNIT;

/// GRAVITY - gravity per tic.
pub const GRAVITY: Fixed = FRACUNIT;
/// MAXMOVE - max movement per tic.
pub const MAXMOVE: Fixed = 30 * FRACUNIT;

/// USERANGE - use/interact range.
pub const USERANGE: Fixed = 64 * FRACUNIT;
/// MELEERANGE - melee attack range.
pub const MELEERANGE: Fixed = 64 * FRACUNIT;
/// MISSILERANGE - missile attack range.
pub const MISSILERANGE: Fixed = 32 * 64 * FRACUNIT;

/// BASETHRESHOLD - follow player exclusively for ~3 seconds.
pub const BASETHRESHOLD: i32 = 100;

/// ONFLOORZ - special z value for floor.
pub const ONFLOORZ: i32 = i32::MIN;
/// ONCEILINGZ - special z value for ceiling.
pub const ONCEILINGZ: i32 = i32::MAX;

/// ITEMQUESIZE - item respawn queue size.
pub const ITEMQUESIZE: usize = 128;

/// Divline - line for intercept calculations (from p_local.h P_MAPUTL).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Divline {
    pub x: Fixed,
    pub y: Fixed,
    pub dx: Fixed,
    pub dy: Fixed,
}

/// Intercept - one line or thing crossed along a trace.
#[derive(Clone, Copy)]
pub struct Intercept {
    pub frac: Fixed,
    pub isaline: bool,
    pub line: *mut crate::rendering::defs::Line,
    pub thing: *mut crate::player::p_mobj::Mobj,
}

/// Max intercepts (original Doom had 128; we use 128+61 for overrun safety).
pub const MAXINTERCEPTS: usize = 189;
