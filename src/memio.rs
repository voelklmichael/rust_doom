//! Memory file I/O (memio.h, memio.c)
//! Original: memio.h, memio.c

use std::sync::Arc;
use std::sync::Mutex;

// typedef enum mem_rel_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemRelT {
    MemSeekSet,
    MemSeekCur,
    MemSeekEnd,
}

pub struct Memfile {
    buf: Vec<u8>,
    position: usize,
    mode: MemfileMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemfileMode {
    Read,
    Write,
}

pub struct MemioState;

impl MemioState {
    /// Original: MEMFILE *mem_fopen_read(void *buf, size_t buflen)
    pub fn mem_fopen_read(&self, _buf: &[u8]) -> Arc<Mutex<Memfile>> {
        todo!("Basic stage-0 stub")
    }

    /// Original: size_t mem_fread(void *buf, size_t size, size_t nmemb, MEMFILE *stream)
    pub fn mem_fread(&self, _stream: &Memfile, _buf: &mut [u8]) -> usize {
        todo!("Basic stage-0 stub")
    }

    /// Original: MEMFILE *mem_fopen_write(void)
    pub fn mem_fopen_write(&self) -> Arc<Mutex<Memfile>> {
        todo!("Basic stage-0 stub")
    }

    /// Original: size_t mem_fwrite(const void *ptr, size_t size, size_t nmemb, MEMFILE *stream)
    pub fn mem_fwrite(&self, _stream: &Memfile, _ptr: &[u8]) -> usize {
        todo!("Basic stage-0 stub")
    }

    /// Original: void mem_get_buf(MEMFILE *stream, void **buf, size_t *buflen)
    pub fn mem_get_buf(&self, _stream: &Memfile) -> (Vec<u8>, usize) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void mem_fclose(MEMFILE *stream)
    pub fn mem_fclose(&self, _stream: &Memfile) {
        todo!("Basic stage-0 stub")
    }

    /// Original: long mem_ftell(MEMFILE *stream)
    pub fn mem_ftell(&self, _stream: &Memfile) -> i64 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int mem_fseek(MEMFILE *stream, signed long offset, mem_rel_t whence)
    pub fn mem_fseek(&self, _stream: &mut Memfile, _offset: i64, _whence: MemRelT) -> i32 {
        todo!("Basic stage-0 stub")
    }
}
