//! SHA-1 digest (sha1.h, sha1.c)
//! Original: sha1.h, sha1.c

use crate::doomtype::Byte;

pub type Sha1DigestT = [Byte; 20];

pub struct Sha1ContextT {
    pub h0: u32,
    pub h1: u32,
    pub h2: u32,
    pub h3: u32,
    pub h4: u32,
    pub nblocks: u32,
    pub buf: [Byte; 64],
    pub count: i32,
}

pub struct Sha1State;

impl Sha1State {
    /// Original: void SHA1_Init(sha1_context_t *context)
    pub fn sha1_init(&self, _context: &mut Sha1ContextT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void SHA1_Update(sha1_context_t *context, byte *buf, size_t len)
    pub fn sha1_update(&self, _context: &mut Sha1ContextT, _buf: &[Byte]) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void SHA1_Final(sha1_digest_t digest, sha1_context_t *context)
    pub fn sha1_final(&self, _context: &mut Sha1ContextT, _digest: &mut Sha1DigestT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void SHA1_UpdateInt32(sha1_context_t *context, unsigned int val)
    pub fn sha1_update_int32(&self, _context: &mut Sha1ContextT, _val: u32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void SHA1_UpdateString(sha1_context_t *context, char *str)
    pub fn sha1_update_string(&self, _context: &mut Sha1ContextT, _str: &str) {
        todo!("Basic stage-0 stub")
    }
}
