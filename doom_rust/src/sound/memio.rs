//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// Emulates the IO functions in C stdio.h reading and writing to memory.
//
// Original: memio.h / memio.c

use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemSeek {
    Set,
    Cur,
    End,
}

/// Memory stream for reading.
pub struct MemFileRead<'a> {
    buf: &'a [u8],
    position: usize,
}

/// Memory stream for writing.
pub struct MemFileWrite {
    buf: Vec<u8>,
    position: usize,
}

impl<'a> MemFileRead<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, position: 0 }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.position)
    }
}

impl Read for MemFileRead<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.remaining().min(buf.len());
        buf[..n].copy_from_slice(&self.buf[self.position..self.position + n]);
        self.position += n;
        Ok(n)
    }
}

impl Seek for MemFileRead<'_> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(n) => n as usize,
            SeekFrom::Current(delta) => {
                if delta >= 0 {
                    self.position.saturating_add(delta as usize)
                } else {
                    self.position.saturating_sub((-delta) as usize)
                }
            }
            SeekFrom::End(delta) => {
                if delta >= 0 {
                    self.buf.len().saturating_add(delta as usize)
                } else {
                    self.buf.len().saturating_sub((-delta) as usize)
                }
            }
        };
        self.position = new_pos.min(self.buf.len());
        Ok(self.position as u64)
    }
}

impl MemFileWrite {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
            position: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn into_buf(self) -> Vec<u8> {
        self.buf
    }

    pub fn get_buf(&self) -> &[u8] {
        &self.buf
    }
}

impl Default for MemFileWrite {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for MemFileWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let need = self.position + buf.len();
        if need > self.buf.len() {
            self.buf.resize(need, 0);
        }
        self.buf[self.position..self.position + buf.len()].copy_from_slice(buf);
        self.position += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Seek for MemFileWrite {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(n) => n as usize,
            SeekFrom::Current(delta) => {
                if delta >= 0 {
                    self.position.saturating_add(delta as usize)
                } else {
                    self.position.saturating_sub((-delta) as usize)
                }
            }
            SeekFrom::End(delta) => {
                if delta >= 0 {
                    self.buf.len().saturating_add(delta as usize)
                } else {
                    self.buf.len().saturating_sub((-delta) as usize)
                }
            }
        };
        self.position = new_pos;
        if self.position > self.buf.len() {
            self.buf.resize(self.position, 0);
        }
        Ok(self.position as u64)
    }
}

/// Open a memory area for reading.
pub fn mem_fopen_read(buf: &[u8]) -> MemFileRead<'_> {
    MemFileRead::new(buf)
}

/// Open a memory area for writing.
pub fn mem_fopen_write() -> MemFileWrite {
    MemFileWrite::new()
}
