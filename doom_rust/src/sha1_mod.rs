//
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//     SHA-1 digest.
//
// Original: sha1.h - wraps sha1 crate

use crate::doomtype::Byte;
use sha1::Digest;

pub type Sha1Digest = [Byte; 20];

/// SHA1 context - wraps the sha1 crate's state.
pub struct Sha1Context {
    hasher: sha1::Sha1,
}

impl Sha1Context {
    pub fn new() -> Self {
        Self {
            hasher: sha1::Sha1::new(),
        }
    }

    /// Original: SHA1_Init
    pub fn init(&mut self) {
        self.hasher = sha1::Sha1::new();
    }

    /// Original: SHA1_Update
    pub fn update(&mut self, buf: &[Byte]) {
        self.hasher.update(buf);
    }

    /// Original: SHA1_Final
    pub fn finalize(&mut self, digest: &mut Sha1Digest) {
        let result = std::mem::replace(&mut self.hasher, sha1::Sha1::new()).finalize();
        digest.copy_from_slice(result.as_slice());
    }

    /// Original: SHA1_UpdateInt32
    pub fn update_int32(&mut self, val: u32) {
        self.hasher.update(&val.to_le_bytes());
    }

    /// Original: SHA1_UpdateString
    pub fn update_string(&mut self, s: &str) {
        self.hasher.update(s.as_bytes());
        self.hasher.update(&[0u8]); // null terminator
    }
}
