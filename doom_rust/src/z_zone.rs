//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//      Zone Memory Allocation, perhaps NeXT ObjectiveC inspired.
//
// Original: z_zone.h / z_zone.c

// PU - purge tags.
pub const PU_STATIC: i32 = 1;
pub const PU_SOUND: i32 = 2;
pub const PU_MUSIC: i32 = 3;
pub const PU_FREE: i32 = 4;
pub const PU_LEVEL: i32 = 5;
pub const PU_LEVSPEC: i32 = 6;
pub const PU_PURGELEVEL: i32 = 7;
pub const PU_CACHE: i32 = 8;
pub const PU_NUM_TAGS: i32 = 9;

/// Original: Z_Init
pub fn z_init() {}

/// Original: Z_Free
pub fn z_free(_ptr: *mut u8) {}

/// Original: Z_Malloc
pub fn z_malloc(size: usize, _tag: i32, _user: *mut *mut u8) -> *mut u8 {
    dbg!(size);
    Box::leak(Box::new(vec![0u8; size])).as_mut_ptr()
}
