//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Sky rendering. The DOOM sky is a texture map like any wall, wrapping around.
//
// Original: r_sky.h + r_sky.c

use crate::m_fixed::FRACUNIT;
use std::sync::{Mutex, OnceLock};

// =============================================================================
// RSkyState - thread-safe via OnceLock + Mutex
// =============================================================================

static R_SKY_STATE: OnceLock<Mutex<RSkyState>> = OnceLock::new();

pub struct RSkyState {
    pub skytexture: i32,
    pub skytexturemid: i32,
    pub skyflatnum: i32,
}

impl Default for RSkyState {
    fn default() -> Self {
        Self {
            skytexture: 0,
            skytexturemid: 0,
            skyflatnum: 0,
        }
    }
}

fn get_r_sky_state() -> &'static Mutex<RSkyState> {
    R_SKY_STATE.get_or_init(|| Mutex::new(RSkyState::default()))
}

/// Access RSkyState.
pub fn with_r_sky_state<F, R>(f: F) -> R
where
    F: FnOnce(&RSkyState) -> R,
{
    let guard = get_r_sky_state().lock().unwrap();
    f(&guard)
}

/// Mutably access RSkyState.
pub fn with_r_sky_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut RSkyState) -> R,
{
    let mut guard = get_r_sky_state().lock().unwrap();
    f(&mut guard)
}

// =============================================================================
// Public API (from .h)
// =============================================================================

pub const SKYFLATNAME: &str = "F_SKY1";
pub const ANGLETOSKYSHIFT: i32 = 22;

/// Called whenever the view size changes.
pub fn r_init_sky_map() {
    with_r_sky_state_mut(|s| {
        s.skytexturemid = 100 * FRACUNIT;
        s.skyflatnum = crate::rendering::r_data::r_flat_num_for_name(SKYFLATNAME);
    });
}
