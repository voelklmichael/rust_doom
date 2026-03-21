// sha1.h / sha1.c

pub use crate::doomtype::*;

use std::cell::RefCell;

// Original: typedef byte sha1_digest_t[20]
pub type Sha1DigestT = [Byte; 20];

// Original: struct sha1_context_s
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Sha1ContextT {
    // Original: h0
    pub h0: u32,
    // Original: h1
    pub h1: u32,
    // Original: h2
    pub h2: u32,
    // Original: h3
    pub h3: u32,
    // Original: h4
    pub h4: u32,
    // Original: nblocks
    pub nblocks: u32,
    // Original: buf
    pub buf: [Byte; 64],
    // Original: count
    pub count: i32,
}

#[allow(non_camel_case_types)]
pub struct Sha1State;

impl Sha1State {
    pub fn new() -> Self {
        Self
    }

    // Original: SHA1_Init
    pub fn sha1_init(&self, context: &RefCell<Sha1ContextT>) {
        let mut c = context.borrow_mut();
        c.h0 = 0x6745_2301;
        c.h1 = 0xefcd_ab89;
        c.h2 = 0x98ba_dcfe;
        c.h3 = 0x1032_5476;
        c.h4 = 0xc3d2_e1f0;
        c.nblocks = 0;
        c.count = 0;
    }

    // Original: SHA1_Update
    pub fn sha1_update(&self, _context: &RefCell<Sha1ContextT>, _buf: &[Byte]) {
        todo!("SHA1_Update")
    }

    // Original: SHA1_Final
    pub fn sha1_final(&self, _digest: &mut Sha1DigestT, _context: &RefCell<Sha1ContextT>) {
        todo!("SHA1_Final")
    }

    // Original: SHA1_UpdateInt32 — depends on SHA1_Update (>10 lines in C)
    pub fn sha1_update_int32(&self, _context: &RefCell<Sha1ContextT>, _val: u32) {
        todo!("SHA1_UpdateInt32")
    }

    // Original: SHA1_UpdateString
    pub fn sha1_update_string(&self, _context: &RefCell<Sha1ContextT>, _s: &str) {
        todo!("SHA1_UpdateString")
    }
}
