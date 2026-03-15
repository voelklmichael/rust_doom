//! Rust translation of doomgeneric/sha1.h

use crate::doomtype::*;

/// C typedef: sha1_context_t
#[repr(C)]
pub struct Sha1ContextT {
    pub h0: u32,
    pub h1: u32,
    pub h2: u32,
    pub h3: u32,
    pub h4: u32,
    pub nblocks: u32,
    pub buf: [u8; 64],
    pub count: i32,
}

/// C typedef: sha1_digest_t
pub type Sha1DigestT = [u8; 20];

/// C function: SHA1_Init
pub fn sha1_init(context: &mut Sha1ContextT) {
    todo!("original: SHA1_Init")
}

/// C function: SHA1_Update
pub fn sha1_update(context: &mut Sha1ContextT, buf: &mut [byte], len: usize) {
    todo!("original: SHA1_Update")
}

/// C function: SHA1_Final
pub fn sha1_final(digest: &mut Sha1DigestT, context: &mut Sha1ContextT) {
    todo!("original: SHA1_Final")
}

/// C function: SHA1_UpdateInt32
pub fn sha1_update_int32(context: &mut Sha1ContextT, val: u32) {
    todo!("original: SHA1_UpdateInt32")
}

/// C function: SHA1_UpdateString
pub fn sha1_update_string(context: &mut Sha1ContextT, str: &str) {
    todo!("original: SHA1_UpdateString")
}
