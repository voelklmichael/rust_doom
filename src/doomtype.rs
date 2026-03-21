// Stub for doomtype.h - minimal types needed by doomgeneric's dependencies.
// Full migration in Phase 1.

#[allow(dead_code)]
pub type Byte = u8;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Boolean {
    False = 0,
    True = 1,
    Undef = 0xFFFF_FFFF,
}

impl From<bool> for Boolean {
    fn from(b: bool) -> Self {
        if b { Boolean::True } else { Boolean::False }
    }
}

impl From<Boolean> for bool {
    fn from(b: Boolean) -> Self {
        matches!(b, Boolean::True)
    }
}
