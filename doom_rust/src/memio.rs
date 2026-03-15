//! Rust translation of doomgeneric/memio.h
//! Memory file I/O (in-memory FILE-like interface).

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: mem_rel_t
pub enum MemRelT {
    SeekSet,
    SeekCur,
    SeekEnd,
}

/// Opaque MEMFILE handle (C: struct _MEMFILE)
#[repr(C)]
pub struct Memfile;

/// C function: mem_fopen_read
pub fn mem_fopen_read(buf: &mut [u8], buflen: usize) -> *mut Memfile {
    todo!("original: mem_fopen_read")
}

/// C function: mem_fread
pub fn mem_fread(buf: &mut [u8], size: usize, nmemb: usize, stream: &mut Memfile) -> usize {
    todo!("original: mem_fread")
}

/// C function: mem_fopen_write
pub fn mem_fopen_write() -> *mut Memfile {
    todo!("original: mem_fopen_write")
}

/// C function: mem_fwrite
pub fn mem_fwrite(ptr: *const u8, size: usize, nmemb: usize, stream: &mut Memfile) -> usize {
    todo!("original: mem_fwrite")
}

/// C function: mem_get_buf
pub fn mem_get_buf(stream: &mut Memfile, buf: &mut *mut u8, buflen: &mut usize) {
    todo!("original: mem_get_buf")
}

/// C function: mem_fclose
pub fn mem_fclose(stream: &mut Memfile) {
    todo!("original: mem_fclose")
}

/// C function: mem_ftell
pub fn mem_ftell(stream: &mut Memfile) -> i64 {
    todo!("original: mem_ftell")
}

/// C function: mem_fseek
pub fn mem_fseek(stream: &mut Memfile, offset: i64, whence: MemRelT) -> i32 {
    todo!("original: mem_fseek")
}
