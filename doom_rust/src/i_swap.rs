//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//	Endianess handling, swapping 16bit and 32bit.
//
// Original: i_swap.h

use std::mem;

/// Convert bytes to little-endian i32.
/// On little-endian systems this is a no-op; on big-endian it swaps.
#[inline]
pub fn long(bytes: [u8; 4]) -> i32 {
    i32::from_le_bytes(bytes)
}

/// Convert bytes to little-endian i16.
#[inline]
pub fn short(bytes: [u8; 2]) -> i16 {
    i16::from_le_bytes(bytes)
}

/// Convert i32 to bytes (little-endian).
#[inline]
pub fn long_to_bytes(val: i32) -> [u8; 4] {
    val.to_le_bytes()
}

/// Convert i32 to bytes for storage (handles endianness).
#[inline]
pub fn long_from_bytes(val: i32) -> [u8; 4] {
    val.to_le_bytes()
}
