// doomgeneric/memio.h

use std::ffi::c_void;
use std::os::raw::{c_int, c_long};

/// Original: typedef struct _MEMFILE MEMFILE (opaque)
#[repr(C)]
pub struct Memfile {
    _private: [u8; 0],
}

/// Original: typedef enum { MEM_SEEK_SET, ... } mem_rel_t
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MemRelT {
    MemSeekSet = 0,
    MemSeekCur = 1,
    MemSeekEnd = 2,
}

#[allow(non_camel_case_types)]
pub struct MemioState;

impl MemioState {
    pub fn new() -> Self {
        Self
    }

    // Original: mem_fopen_read
    pub fn mem_fopen_read(&self, _buf: *mut c_void, _buflen: usize) -> *mut Memfile {
        todo!("mem_fopen_read");
    }

    // Original: mem_fread
    pub fn mem_fread(&self, _buf: *mut c_void, _size: usize, _nmemb: usize, _stream: *mut Memfile) -> usize {
        todo!("mem_fread");
    }

    // Original: mem_fopen_write
    pub fn mem_fopen_write(&self) -> *mut Memfile {
        todo!("mem_fopen_write");
    }

    // Original: mem_fwrite
    pub fn mem_fwrite(
        &self,
        _ptr: *const c_void,
        _size: usize,
        _nmemb: usize,
        _stream: *mut Memfile,
    ) -> usize {
        todo!("mem_fwrite");
    }

    // Original: mem_get_buf
    pub fn mem_get_buf(&self, _stream: *mut Memfile, _buf: *mut *mut c_void, _buflen: *mut usize) {
        todo!("mem_get_buf");
    }

    // Original: mem_fclose
    pub fn mem_fclose(&self, _stream: *mut Memfile) {
        todo!("mem_fclose");
    }

    // Original: mem_ftell
    pub fn mem_ftell(&self, _stream: *mut Memfile) -> c_long {
        todo!("mem_ftell");
    }

    // Original: mem_fseek
    pub fn mem_fseek(&self, _stream: *mut Memfile, _offset: c_long, _whence: MemRelT) -> c_int {
        todo!("mem_fseek");
    }
}
