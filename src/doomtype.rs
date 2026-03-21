// doomtype.h - basic typedefs and macros
// No dependencies (leaf module)

// Original: typedef uint8_t byte
pub type Byte = u8;

// Original: typedef enum { false, true, undef } boolean
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Boolean {
    False = 0,
    True = 1,
    Undef = 0xFFFF_FFFF,
}

impl From<bool> for Boolean {
    // Original: (Rust From<bool> for Boolean)
    fn from(b: bool) -> Self {
        if b { Boolean::True } else { Boolean::False }
    }
}

impl From<Boolean> for bool {
    // Original: (Rust From<Boolean> for bool)
    fn from(b: Boolean) -> Self {
        matches!(b, Boolean::True)
    }
}

// Original: #define DIR_SEPARATOR '/' (non-Windows)
pub const DIR_SEPARATOR: char = if cfg!(windows) { '\\' } else { '/' };
pub const DIR_SEPARATOR_S: &str = if cfg!(windows) { "\\" } else { "/" };

// Original: #define PATH_SEPARATOR ':'
pub const PATH_SEPARATOR: char = if cfg!(windows) { ';' } else { ':' };

// Original: #define arrlen(array) (sizeof(array) / sizeof(*array))
// This was a macro.
#[inline]
pub fn arrlen<T>(array: &[T]) -> usize {
    array.len()
}

#[allow(non_camel_case_types)]
pub struct DoomtypeState {
    // No globals in doomtype - placeholder for plan consistency
}

impl DoomtypeState {
    pub fn new() -> Self {
        Self {}
    }
}
