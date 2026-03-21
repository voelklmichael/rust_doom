// i_swap.h - endianness and byte swapping
// No dependencies (leaf module)

// Original: #define SYS_LITTLE_ENDIAN / SYS_BIG_ENDIAN
#[cfg(target_endian = "little")]
pub const SYS_LITTLE_ENDIAN: () = ();

#[cfg(target_endian = "big")]
pub const SYS_BIG_ENDIAN: () = ();

// Original: static inline unsigned short swapLE16
// This was a macro/inline function.
#[inline]
pub fn swap_le16(val: u16) -> u16 {
    val.swap_bytes()
}

// Original: static inline unsigned long swapLE32
#[inline]
pub fn swap_le32(val: u32) -> u32 {
    val.swap_bytes()
}

// Original: #define SHORT(x) - on little endian, cast; on big endian, swap
#[inline]
pub fn short(x: u16) -> i16 {
    #[cfg(target_endian = "little")]
    return x as i16;
    #[cfg(target_endian = "big")]
    return swap_le16(x) as i16;
}

// Original: #define LONG(x)
#[inline]
pub fn long(x: u32) -> i32 {
    #[cfg(target_endian = "little")]
    return x as i32;
    #[cfg(target_endian = "big")]
    return swap_le32(x) as i32;
}

// Original: #define doom_swap_s(x) / doom_wtohs
#[inline]
pub fn doom_swap_s(x: u16) -> i16 {
    (((x & 0x00ff) << 8) | ((x & 0xff00) >> 8)) as i16
}

#[inline]
// Original: doom_wtohs
pub fn doom_wtohs(x: u16) -> i16 {
    #[cfg(target_endian = "little")]
    return x as i16;
    #[cfg(target_endian = "big")]
    return doom_swap_s(x);
}
