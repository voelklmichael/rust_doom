//! Rust translation of doomgeneric/w_file.h
//! WAD I/O functions.

use crate::doomtype::*;
use std::sync::{Arc, Mutex};

/// C typedef: wad_file_t
#[repr(C)]
pub struct WadFileT {
    pub file_class: *mut WadFileClassT,
    pub mapped: *mut byte,
    pub length: u32,
}

/// C typedef: wad_file_class_t
#[repr(C)]
pub struct WadFileClassT {
    pub open_file: Option<extern "C" fn(*mut i8) -> *mut WadFileT>,
    pub close_file: Option<extern "C" fn(*mut WadFileT)>,
    pub read: Option<
        extern "C" fn(*mut WadFileT, u32, *mut core::ffi::c_void, usize) -> usize,
    >,
}

/// C function: W_OpenFile
pub fn w_open_file(path: &str) -> Arc<Mutex<WadFileT>> {
    todo!("original: W_OpenFile")
}

/// C function: W_CloseFile
pub fn w_close_file(wad: &mut WadFileT) {
    todo!("original: W_CloseFile")
}

/// C function: W_Read
pub fn w_read(
    wad: &mut WadFileT,
    offset: u32,
    buffer: &mut [u8],
    buffer_len: usize,
) -> usize {
    todo!("original: W_Read")
}
