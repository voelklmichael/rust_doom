//! Endianness handling (i_swap.h)
//! Original: i_swap.h
//! Used: SYS_LITTLE_ENDIAN path only

// static inline unsigned short swapLE16(unsigned short val)
#[inline]
pub fn swap_le16(val: u16) -> u16 {
    // C body:
    // return ((val << 8) | (val >> 8));
    (val << 8) | (val >> 8)
}

// static inline unsigned long swapLE32(unsigned long val)
#[inline]
pub fn swap_le32(val: u32) -> u32 {
    // C body:
    // return ((val << 24) | ((val << 8) & 0x00FF0000) | ((val >> 8) & 0x0000FF00) | (val >> 24));
    (val << 24) | ((val << 8) & 0x00FF0000) | ((val >> 8) & 0x0000FF00) | (val >> 24)
}
