// w_file.h / w_file.c

pub use crate::doomtype::*;
pub use crate::m_argv::*;

use std::os::raw::{c_char, c_void};

/// Original: typedef struct — forward class for WAD reader vtable
#[allow(non_camel_case_types)]
pub struct WadFileClassT {
    _private: (),
}

// Original: struct _wad_file_s
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct WadFileT {
    // Original: file_class
    pub file_class: *mut WadFileClassT,
    // Original: mapped
    pub mapped: *mut Byte,
    // Original: length
    pub length: u32,
}

#[allow(non_camel_case_types)]
pub struct W_FileState;

impl W_FileState {
    pub fn new() -> Self {
        Self
    }

    // Original: W_OpenFile — tries stdc / mmap classes; >10 lines in C
    pub fn w_open_file(&self, _m_argv: &M_ArgvState, _path: *mut c_char) -> *mut WadFileT {
        todo!("W_OpenFile")
    }

    // Original: W_CloseFile
    pub fn w_close_file(&self, _wad: *mut WadFileT) {
        todo!("W_CloseFile")
    }

    // Original: W_Read
    pub fn w_read(
        &self,
        _wad: *mut WadFileT,
        _offset: u32,
        _buffer: *mut c_void,
        _buffer_len: usize,
    ) -> usize {
        todo!("W_Read")
    }
}
