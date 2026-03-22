//! Basic typedefs (doomtype.h)
//! Original: doomtype.h
//! Plan §4: Platform conditionals removed; single code path.

// typedef enum { false = 0, true = 1, undef = 0xFFFFFFFF } boolean
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Boolean {
    #[default]
    False = 0,
    True = 1,
    Undef = -1, // 0xFFFFFFFF in C
}

// typedef uint8_t byte
pub type Byte = u8;

// PACKEDATTR -> #[repr(C, packed)] on structs that need it
// Used via derive or manual repr on structs.

// #define DIR_SEPARATOR '/'  (non-Windows path)
pub const DIR_SEPARATOR: char = '/';
// #define DIR_SEPARATOR_S "/"
pub const DIR_SEPARATOR_S: &str = "/";
// #define PATH_SEPARATOR ':'
pub const PATH_SEPARATOR: char = ':';

// #define arrlen(array) (sizeof(array) / sizeof(*array))
#[macro_export]
macro_rules! arrlen {
    ($arr:expr) => {
        std::mem::size_of_val(&$arr) / std::mem::size_of_val(&$arr[0])
    };
}
