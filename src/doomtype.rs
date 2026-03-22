//! Basic type definitions (doomtype.h)
//! Original: doomtype.h

// typedef uint8_t byte
pub type Byte = u8;

// typedef enum { false = 0, true = 1, undef = 0xFFFFFFFF } boolean
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Boolean {
    // false
    False = 0,
    // true
    True = 1,
    // undef
    Undef = 0xFFFFFFFF,
}

// #define DIR_SEPARATOR '/'
pub const DIR_SEPARATOR: char = '/';
// #define DIR_SEPARATOR_S "/"
pub const DIR_SEPARATOR_S: &str = "/";
// #define PATH_SEPARATOR ':'
pub const PATH_SEPARATOR: char = ':';
