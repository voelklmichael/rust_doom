//! WAD file stdc backend (w_file_stdc.c)
//! Original: w_file_stdc.c
//! C-only module.

use crate::w_file::WadFileT;

pub struct W_FileStdcState;

impl W_FileStdcState {
    /// Original: static wad_file_t *W_StdC_OpenFile(char *path)
    pub fn w_stdc_open_file(&self, _path: &str) -> Option<Box<WadFileT>> {
        // C body:
        // fstream = fopen(path, "rb");
        // if (fstream == NULL) return NULL;
        // result = Z_Malloc(sizeof(stdc_wad_file_t), PU_STATIC, 0);
        // result->wad.file_class = &stdc_wad_file;
        // result->wad.mapped = NULL;
        // result->wad.length = M_FileLength(fstream);
        // result->fstream = fstream;
        // return &result->wad;
        todo!("Basic stage-0 stub")
    }

    /// Original: static void W_StdC_CloseFile(wad_file_t *wad)
    pub fn w_stdc_close_file(&self, _wad: Box<WadFileT>) {
        // C body:
        // fclose(stdc_wad->fstream);
        // Z_Free(stdc_wad);
        todo!("Basic stage-0 stub")
    }

    /// Original: size_t W_StdC_Read(wad_file_t *wad, unsigned int offset, void *buffer, size_t buffer_len)
    pub fn w_stdc_read(&self, _wad: &WadFileT, _offset: u32, _buffer: &mut [u8]) -> usize {
        // C body:
        // fseek(stdc_wad->fstream, offset, SEEK_SET);
        // result = fread(buffer, 1, buffer_len, stdc_wad->fstream);
        todo!("Basic stage-0 stub")
    }
}
