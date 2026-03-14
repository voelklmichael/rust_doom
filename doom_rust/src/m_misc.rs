//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//      Miscellaneous.
//
// Original: m_misc.h / m_misc.c

use crate::doomtype::{Boolean, DIR_SEPARATOR};
use std::fs::File;
use std::io::{Seek, SeekFrom};

/// Copy string with size limit. Returns true if src fit entirely.
pub fn m_string_copy(dest: &mut [u8], src: &str) -> Boolean {
    if dest.is_empty() {
        return false;
    }
    let max = dest.len() - 1;
    let src_bytes = src.as_bytes();
    let copy_len = src_bytes.len().min(max);
    dest[..copy_len].copy_from_slice(&src_bytes[..copy_len]);
    dest[copy_len] = 0;
    copy_len == src_bytes.len()
}

/// Extract file base name (without path and extension) into dest, max 8 chars, uppercase.
/// Original: m_misc.c M_ExtractFileBase
pub fn m_extract_file_base(path: &str, dest: &mut [u8; 8]) {
    dest.fill(0);
    let path = path.as_bytes();
    if path.is_empty() {
        return;
    }

    // Find start of filename (after last separator)
    let mut start = path.len() - 1;
    while start > 0 && path[start - 1] != DIR_SEPARATOR as u8 {
        start -= 1;
    }
    let filename = &path[start..];

    // Copy up to 8 chars, stop at '.'
    for (len, &b) in filename.iter().enumerate() {
        if b == b'.' || len >= 8 {
            break;
        }
        dest[len] = b.to_ascii_uppercase();
    }
}

trait ToAsciiUppercase {
    fn to_ascii_uppercase(self) -> u8;
}
impl ToAsciiUppercase for u8 {
    fn to_ascii_uppercase(self: u8) -> u8 {
        if self.is_ascii_lowercase() {
            self - 32
        } else {
            self
        }
    }
}

/// Get file length. Original: m_misc.c M_FileLength
pub fn m_file_length(file: &mut File) -> std::io::Result<u64> {
    let pos = file.stream_position()?;
    let len = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(pos))?;
    Ok(len)
}
