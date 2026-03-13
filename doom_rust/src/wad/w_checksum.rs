//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//       Generate a checksum of the WAD directory.
//
// Original: w_checksum.h (public) + w_checksum.c (private)

use crate::m_misc;
use crate::sha1_mod::{Sha1Context, Sha1Digest};
use crate::wad::w_wad::{with_lumpinfo, LumpInfo};

/// Generate a checksum of the WAD directory.
/// Original: W_Checksum
pub fn w_checksum(digest: &mut Sha1Digest) {
    let mut ctx = Sha1Context::new();
    ctx.init();

    with_lumpinfo(|lumpinfo| {
        for lump in lumpinfo.iter() {
            checksum_add_lump(&mut ctx, lump);
        }
    });

    ctx.finalize(digest);
}

/// Original: ChecksumAddLump (private)
fn checksum_add_lump(ctx: &mut Sha1Context, lump: &LumpInfo) {
    // M_StringCopy(buf, lump->name, sizeof(buf)); SHA1_UpdateString(sha1_context, buf);
    let mut buf = [0u8; 9];
    let name_str = std::str::from_utf8(&lump.name).unwrap_or("").trim_end_matches('\0');
    m_misc::m_string_copy(&mut buf, name_str);
    ctx.update(&buf[..=buf.iter().position(|&b| b == 0).unwrap_or(8)]);
    ctx.update_int32(lump.wad_file_index as u32);
    ctx.update_int32(lump.position as u32);
    ctx.update_int32(lump.size as u32);
}
