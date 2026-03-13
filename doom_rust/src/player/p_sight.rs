//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Line-of-sight / visibility checks.
//
// Original: p_sight.c (stub - P_CheckSight always returns true)

use super::p_mobj::Mobj;

/// Check if t1 has line of sight to t2.
/// Original: P_CheckSight
/// Stub: always returns true (no blocking). Full impl needs REJECT, BSP, subsectors.
pub fn p_check_sight(_t1: *const Mobj, _t2: *const Mobj) -> bool {
    true
}
