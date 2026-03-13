//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// DESCRIPTION:
//	Simple basic typedefs, isolated here to make it easier
//	 separating modules.
//

// Original: doomtype.h

// strcasecmp / strncasecmp - case-insensitive string comparison.
// C: On Windows uses _stricmp/_strnicmp; elsewhere strings.h.
// Returns: negative if a < b, 0 if equal, positive if a > b.
#[inline]
pub fn strcasecmp(a: &str, b: &str) -> i32 {
    let a_lower = a.to_ascii_lowercase();
    let b_lower = b.to_ascii_lowercase();
    match a_lower.cmp(&b_lower) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

#[inline]
pub fn strncasecmp(a: &str, b: &str, n: usize) -> i32 {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let len = n.min(a_bytes.len()).min(b_bytes.len());
    for i in 0..len {
        let ca = a_bytes[i].to_ascii_lowercase();
        let cb = b_bytes[i].to_ascii_lowercase();
        match ca.cmp(&cb) {
            std::cmp::Ordering::Less => return -1,
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Equal => {}
        }
    }
    // If we get here, the first len bytes match; shorter string is "less"
    (a_bytes.len() as i32).saturating_sub(b_bytes.len() as i32)
}

// PACKEDATTR - C: __attribute__((packed)) for GCC.
// In Rust use #[repr(C)] or #[repr(packed)] on structs where alignment
// matters (e.g. data read/written to disk).

// boolean - C: typedef enum { false=0, true=1, undef=0xFFFFFFFF } boolean;
pub type Boolean = bool;

/// Undef sentinel value (0xFFFFFFFF) - used in C boolean enum
pub const UNDEF: u32 = 0xFFFFFFFF;

// byte - C: typedef uint8_t byte;
pub type Byte = u8;

// Platform-specific path separators
// C: #if defined(_WIN32) || defined(__DJGPP__) -> '\\', "\\", ';'
//    #else -> '/', "/", ':'
#[cfg(target_os = "windows")]
pub const DIR_SEPARATOR: char = '\\';
#[cfg(target_os = "windows")]
pub const DIR_SEPARATOR_S: &str = "\\";
#[cfg(target_os = "windows")]
pub const PATH_SEPARATOR: char = ';';

#[cfg(not(target_os = "windows"))]
pub const DIR_SEPARATOR: char = '/';
#[cfg(not(target_os = "windows"))]
pub const DIR_SEPARATOR_S: &str = "/";
#[cfg(not(target_os = "windows"))]
pub const PATH_SEPARATOR: char = ':';

// arrlen - C: #define arrlen(array) (sizeof(array) / sizeof(*array))
#[inline]
pub fn arrlen<T>(array: &[T]) -> usize {
    array.len()
}
