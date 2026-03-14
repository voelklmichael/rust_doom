//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//      Zone Memory Allocation, perhaps NeXT ObjectiveC inspired.
//
// Original: z_zone.h / z_zone.c

use crate::doomtype::Byte;
use crate::i_system;
use std::ptr::{null, null_mut};

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

const MEM_ALIGN: usize = std::mem::size_of::<*mut u8>();
const ZONEID: i32 = 0x1d4a11;
const MINFRAGMENT: usize = 64;

#[repr(C)]
struct Memblock {
    size: usize,
    tag: i32,
    id: i32,
    next: *mut Memblock,
    prev: *mut Memblock,
}

#[repr(C)]
struct Memzone {
    size: usize,
    blocklist: Memblock,
    rover: *mut Memblock,
}

static mut MAINZONE: *mut Memzone = null_mut();

fn block_from_ptr(ptr: *mut u8) -> *mut Memblock {
    unsafe { (ptr as *mut u8).sub(std::mem::size_of::<Memblock>()) as *mut Memblock }
}

fn ptr_from_block(block: *mut Memblock) -> *mut u8 {
    unsafe { (block as *mut u8).add(std::mem::size_of::<Memblock>()) }
}

/// Original: Z_Init
pub fn z_init() {}

/// Original: Z_Free
pub fn z_free(ptr: *mut u8) {}

/// Original: Z_Malloc
pub fn z_malloc(size: usize, tag: i32, user: *mut *mut u8) -> *mut u8 {
    dbg!(size);
    Box::leak(Box::new(vec![0u8; size])).as_mut_ptr()
}
