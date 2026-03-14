//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Screen scale-up: 1x,2x,3x,4x pixel doubling; aspect-ratio stretch/squash.
//
// Original: i_scale.h + i_scale.c

/// Set source and destination buffers for scaling.
/// Original: I_InitScale
pub fn i_init_scale(
    _src_buffer: *mut u8,
    _dest_buffer: *mut u8,
    _dest_pitch: i32,
) {
    // Stub: platform-specific; used when blitting to non-native resolution
}

/// Reset stretch tables from palette.
/// Original: I_ResetScaleTables
pub fn i_reset_scale_tables(_palette: *const u8) {
    // Stub
}
