// doomgeneric/w_checksum.h

pub use crate::doomtype::*;
pub use crate::sha1::*;

#[allow(non_camel_case_types)]
pub struct W_ChecksumState;

impl W_ChecksumState {
    pub fn new() -> Self {
        Self
    }

    // Original: void W_Checksum(sha1_digest_t digest)
    pub fn w_checksum(&self, _digest: &mut Sha1DigestT) {
        todo!("W_Checksum");
    }
}
