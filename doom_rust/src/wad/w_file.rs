//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//	WAD I/O functions.
//
// Original: w_file.h (public) + w_file.c (private)

use crate::config;
use crate::doomtype::Byte;
use crate::m_argv;
use crate::wad::w_file_stdc;

// --- Public API (from w_file.h) ---

/// WadFile handle - corresponds to wad_file_t
pub struct WadFile {
    pub mapped: Option<*mut Byte>,
    pub length: u32,
    pub(crate) inner: WadFileInner,
}

pub(crate) enum WadFileInner {
    StdC(Box<w_file_stdc::StdCBackend>),
}

impl WadFile {
    /// Read data from the specified offset. Returns number of bytes read.
    /// Original: W_Read
    pub fn read(&self, offset: u32, buffer: &mut [u8]) -> usize {
        match &self.inner {
            WadFileInner::StdC(backend) => backend.as_ref().read(offset, buffer),
        }
    }

    /// Close the file and free resources.
    /// Original: W_CloseFile
    pub fn close(self) {
        match self.inner {
            WadFileInner::StdC(backend) => (*backend).close(),
        }
    }
}

/// Open the specified file. Returns None if could not be opened.
/// Original: W_OpenFile
pub fn w_open_file(path: &str) -> Option<WadFile> {
    // Original: w_file.c - if !M_CheckParm("-mmap") use stdc directly
    if !config::HAVE_MMAP || m_argv::m_check_parm("-mmap") == 0 {
        return w_file_stdc::w_stdc_open_file(path);
    }

    // Try all classes - for now only stdc
    w_file_stdc::w_stdc_open_file(path)
}
