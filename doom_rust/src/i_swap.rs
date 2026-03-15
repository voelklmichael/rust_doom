//! Rust translation of doomgeneric/i_swap.h
//! Endianness handling, swapping 16bit and 32bit.

/// C macro: swapLE16
#[inline]
pub fn swap_le16(val: u16) -> u16 {
    (val << 8) | (val >> 8)
}

/// C macro: swapLE32
#[inline]
pub fn swap_le32(val: u32) -> u32 {
    (val << 24) | ((val << 8) & 0x00FF0000) | ((val >> 8) & 0x0000FF00) | (val >> 24)
}

/// C macro: doom_swap_s
#[inline]
pub fn doom_swap_s(x: u16) -> i16 {
    (((x & 0x00ff) << 8) | ((x & 0xff00) >> 8)) as i16
}

/// C macro: doom_wtohs - host to short (no-op on little endian)
#[cfg(target_endian = "little")]
#[inline]
pub fn doom_wtohs(x: i16) -> i16 {
    x
}

#[cfg(target_endian = "big")]
#[inline]
pub fn doom_wtohs(x: i16) -> i16 {
    doom_swap_s(x as u16)
}
