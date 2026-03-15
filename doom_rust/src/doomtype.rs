//! Rust translation of doomgeneric/doomtype.h
//! Simple basic typedefs, isolated here to make it easier separating modules.

// Note: strcasecmp/strncasecmp are C macros (Windows: _stricmp/_strnicmp; else: strings.h).
// Rust uses built-in string comparison. Provided as functions for API compatibility.

// Note: PACKEDATTR (__attribute__((packed))) - use #[repr(C, packed)] on structs
// that need packed layout for disk/network compatibility.

/// C macro: arrlen - sizeof(array) / sizeof(*array)
pub fn arrlen<T>(array: &[T]) -> usize {
    array.len()
}

/// C boolean type with undef state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// C typedef: boolean_t
pub enum Boolean {
    #[default]
    False = 0,
    True = 1,
    Undef = 0xFFFFFFFF,
}

/// C typedef: byte = uint8_t
pub type Byte = u8;
/// C-compatible alias for byte
#[allow(non_camel_case_types)]
/// C typedef: byte
pub type byte = u8;
/// C-compatible alias for boolean
#[allow(non_camel_case_types)]
/// C typedef: boolean
pub type boolean = Boolean;

// Platform-specific path separators (C: _WIN32 || __DJGPP__ vs else)
#[cfg(target_os = "windows")]
/// C #define: DIR_SEPARATOR
pub const DIR_SEPARATOR: char = '\\';

#[cfg(target_os = "windows")]
pub static DIR_SEPARATOR_S: &str = "\\";

#[cfg(target_os = "windows")]
/// C #define: PATH_SEPARATOR
pub const PATH_SEPARATOR: char = ';';

#[cfg(not(target_os = "windows"))]
/// C #define: DIR_SEPARATOR
pub const DIR_SEPARATOR: char = '/';

#[cfg(not(target_os = "windows"))]
pub static DIR_SEPARATOR_S: &str = "/";

#[cfg(not(target_os = "windows"))]
/// C #define: PATH_SEPARATOR
pub const PATH_SEPARATOR: char = ':';
