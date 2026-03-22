//! WAD file interface (w_file.h)
//! Original: w_file.h

use crate::doomtype::Byte;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default)]
pub struct WadFileClassT {
    // OpenFile(char *path) -> wad_file_t*
    // CloseFile(wad_file_t *file)
    // Read(wad_file_t *file, offset, buffer, buffer_len)
}

pub struct WadFileT {
    pub file_class: Arc<Mutex<WadFileClassT>>,
    pub mapped: Option<Vec<Byte>>,
    pub length: u32,
}

pub struct W_FileState;

impl W_FileState {
    /// Original: wad_file_t *W_OpenFile(char *path)
    pub fn w_open_file(&self, _path: &str) -> Option<Arc<Mutex<WadFileT>>> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void W_CloseFile(wad_file_t *wad)
    pub fn w_close_file(&self, _wad: &WadFileT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: size_t W_Read(wad_file_t *wad, unsigned int offset, void *buffer, size_t buffer_len)
    pub fn w_read(&self, _wad: &WadFileT, _offset: u32, _buffer: &mut [u8]) -> usize {
        todo!("Basic stage-0 stub")
    }
}
