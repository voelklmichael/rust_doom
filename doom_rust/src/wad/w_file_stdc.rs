//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//	WAD I/O functions.
//
// Original: w_file_stdc.c (private - no .h)

use crate::m_misc;
use crate::wad::w_file::{WadFile, WadFileInner};
use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// StdC backend - holds the file handle.
/// Original: stdc_wad_file_t
pub struct StdCBackend {
    file: RefCell<File>,
}

impl StdCBackend {
    /// Original: W_StdC_Read
    pub fn read(&self, offset: u32, buffer: &mut [u8]) -> usize {
        let mut file = self.file.borrow_mut();
        if file.seek(SeekFrom::Start(offset as u64)).is_err() {
            return 0;
        }
        file.read(buffer).unwrap_or(0)
    }

    /// Original: W_StdC_CloseFile - drop releases the file
    pub fn close(self) {
        // File closes on drop
    }
}

/// Original: W_StdC_OpenFile
pub fn w_stdc_open_file(path: &str) -> Option<WadFile> {
    let mut file = File::open(path).ok()?;
    let length = m_misc::m_file_length(&mut file).ok()? as u32;

    let backend = StdCBackend {
        file: RefCell::new(file),
    };

    Some(WadFile {
        mapped: None,
        length,
        inner: WadFileInner::StdC(Box::new(backend)),
    })
}
