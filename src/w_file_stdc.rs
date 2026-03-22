//! Stdio WAD file implementation (w_file_stdc.c)
//! Original: w_file_stdc.c

use std::sync::Arc;
use std::sync::Mutex;

pub struct W_FileStdcState;

impl W_FileStdcState {
    /// Original: wad_file_t *W_StdC_OpenFile(char *path)
    pub fn w_stdc_open_file(&self, _path: &str) -> Option<Arc<Mutex<()>>> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void W_StdC_CloseFile(wad_file_t *wad)
    pub fn w_stdc_close_file(&self, _wad: &Arc<Mutex<()>>) {
        todo!("Basic stage-0 stub")
    }

    /// Original: size_t W_StdC_Read(wad_file_t *wad, unsigned int offset, void *buffer, size_t buffer_len)
    pub fn w_stdc_read(&self, _wad: &Arc<Mutex<()>>, _offset: u32, _buffer: &mut [u8]) -> usize {
        todo!("Basic stage-0 stub")
    }
}
