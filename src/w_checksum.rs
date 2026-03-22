//! WAD checksum (w_checksum.h, w_checksum.c)
//! Original: w_checksum.h, w_checksum.c

use crate::sha1::Sha1DigestT;

pub struct W_ChecksumState;

impl W_ChecksumState {
    /// Original: void W_Checksum(sha1_digest_t digest)
    pub fn w_checksum(&self, _digest: &mut Sha1DigestT) {
        todo!("Basic stage-0 stub")
    }
}
