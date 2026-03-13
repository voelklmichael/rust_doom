//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//	System specific interface stuff.
//
// Original: i_system.h / i_system.c

use crate::doomtype::Byte;

/// Called by startup code to get the amount of memory to allocate for the zone management.
/// Returns (pointer, size). Stub: allocates a fixed size (e.g. 16MB).
/// The returned pointer must remain valid for the program's lifetime.
pub fn i_zone_base() -> (*mut Byte, usize) {
    const ZONE_SIZE: usize = 32 * 1024 * 1024; // 32MB (PU_STATIC cache uses more)
    let zone = vec![0u8; ZONE_SIZE];
    let size = zone.len();
    let ptr = Box::leak(zone.into_boxed_slice()).as_mut_ptr();
    (ptr, size)
}

/// Fatal error - exit the program.
pub fn i_error(msg: &str) {
    panic!("I_Error: {}", msg);
}

/// Begin a read operation (stub - no-op for sync I/O).
pub fn i_begin_read() {}

/// End a read operation (stub - no-op for sync I/O).
pub fn i_end_read() {}
