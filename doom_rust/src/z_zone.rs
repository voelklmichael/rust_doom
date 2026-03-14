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
pub fn z_init() {
    let (base, size) = i_system::i_zone_base();
    unsafe {
        let zone = base as *mut Memzone;
        (*zone).size = size;
        let blocklist = &mut (*zone).blocklist;
        let block = (zone as *mut u8).add(std::mem::size_of::<Memzone>()) as *mut Memblock;
        blocklist.next = block;
        blocklist.prev = block;
        blocklist.tag = PU_STATIC;
        (*zone).rover = block;

        (*block).prev = blocklist;
        (*block).next = blocklist;
        (*block).tag = PU_FREE;
        (*block).size = size - std::mem::size_of::<Memzone>();
        (*block).id = 0;

        MAINZONE = zone;
    }
}

/// Original: Z_Free
pub fn z_free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let mut block = block_from_ptr(ptr);
        if (*block).id != ZONEID {
            i_system::i_error("Z_Free: freed a pointer without ZONEID");
        }

        (*block).tag = PU_FREE;
        (*block).id = 0;

        let mut other = (*block).prev;
        if (*other).tag == PU_FREE {
            (*other).size += (*block).size;
            (*other).next = (*block).next;
            (*(*block).next).prev = other;
            if block == (*MAINZONE).rover {
                (*MAINZONE).rover = other;
            }
            block = other;
        }
        other = (*block).next;
        if (*other).tag == PU_FREE {
            (*block).size += (*other).size;
            (*block).next = (*other).next;
            (*(*other).next).prev = block;
            if other == (*MAINZONE).rover {
                (*MAINZONE).rover = block;
            }
        }
    }
}

/// Original: Z_Malloc
pub fn z_malloc(size: usize, tag: i32, user: *mut *mut u8) -> *mut u8 {
    todo!()
    // unsafe {
    //     if MAINZONE.is_null() {
    //         i_system::i_error("Z_Malloc: zone not initialized");
    //     }
    //     let mut size = (size + MEM_ALIGN - 1) & !(MEM_ALIGN - 1);
    //     size += std::mem::size_of::<Memblock>();

    //     if user.is_null() && tag >= PU_PURGELEVEL {
    //         i_system::i_error("Z_Malloc: an owner is required for purgable blocks");
    //     }

    //     let mut base = (*MAINZONE).rover;
    //     if !(*base).prev.is_null() && (*(*base).prev).tag == PU_FREE {
    //         base = (*base).prev;
    //     }
    //     let mut rover = base;
    //     let start = (*base).prev;

    //     loop {
    //         if rover == start {
    //             i_system::i_error(&format!("Z_Malloc: failed on allocation of {} bytes", size));
    //         }
    //         if (*rover).tag != PU_FREE {
    //             if (*rover).tag < PU_PURGELEVEL {
    //                 base = (*rover).next;
    //                 rover = base;
    //             } else {
    //                 base = (*base).prev;
    //                 z_free(ptr_from_block(rover));
    //                 base = (*base).next;
    //                 if base.is_null() {
    //                     i_system::i_error(&format!(
    //                         "Z_Malloc: null base after purge (zone corrupted) for {} bytes",
    //                         size
    //                     ));
    //                 }
    //                 rover = (*base).next;
    //             }
    //         } else {
    //             rover = (*rover).next;
    //         }
    //         if base.is_null() {
    //             i_system::i_error(&format!(
    //                 "Z_Malloc: base is null (zone corrupted) for {} bytes",
    //                 size
    //             ));
    //         }
    //         if (*base).tag == PU_FREE && (*base).size >= size {
    //             break;
    //         }
    //     }

    //     let extra = (*base).size - size;
    //     if extra > MINFRAGMENT {
    //         let newblock = ptr_from_block(base as *mut Memblock).add(size) as *mut Memblock;
    //         (*newblock).size = extra;
    //         (*newblock).tag = PU_FREE;
    //         (*newblock).user = null_mut();
    //         (*newblock).prev = base;
    //         (*newblock).next = (*base).next;
    //         (*(*base).next).prev = newblock;
    //         (*base).next = newblock;
    //         (*base).size = size;
    //     }

    //     (*base).user = user;
    //     (*base).tag = tag;
    //     let result = ptr_from_block(base);
    //     if !user.is_null() {
    //         *user = result;
    //     }
    //     (*MAINZONE).rover = (*base).next;
    //     (*base).id = ZONEID;
    //     result
    // }
}
