//! WAD checksum (w_checksum.h)
//! Original: w_checksum.h

use crate::sha1::Sha1DigestT;

pub struct W_ChecksumState;

impl W_ChecksumState {
    /// Original: void W_Checksum(sha1_digest_t digest)
    pub fn w_checksum(&self, _digest: &mut Sha1DigestT) {
        // C body: (from w_checksum.c - generates checksum of WAD directory)
        todo!("Basic stage-0 stub")
    }
}
