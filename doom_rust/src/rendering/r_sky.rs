//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Sky rendering. The DOOM sky is a texture map like any wall, wrapping around.
//
// Original: r_sky.h + r_sky.c

use crate::m_fixed::FRACUNIT;

// =============================================================================
// Public API (from .h)
// =============================================================================

pub const SKYFLATNAME: &str = "F_SKY1";
pub const ANGLETOSKYSHIFT: i32 = 22;

pub static mut SKYTEXTURE: i32 = 0;
pub static mut SKYTEXTUREMID: i32 = 0;
pub static mut SKYFLATNUM: i32 = 0;

/// Called whenever the view size changes.
pub fn r_init_sky_map() {
    unsafe {
        SKYTEXTUREMID = 100 * FRACUNIT;
        SKYFLATNUM = crate::rendering::r_data::r_flat_num_for_name(SKYFLATNAME);
    }
}
