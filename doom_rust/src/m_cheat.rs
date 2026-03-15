//! Rust translation of doomgeneric/m_cheat.h
//! Cheat code checking.

/// C #define: MAX_CHEAT_LEN
pub const MAX_CHEAT_LEN: usize = 25;
/// C #define: MAX_CHEAT_PARAMS
pub const MAX_CHEAT_PARAMS: usize = 5;

/// C typedef: cheatseq_t
#[repr(C)]
#[derive(Clone, Copy)]
/// C typedef: cheatseq_t
pub struct CheatseqT {
    pub sequence: [i8; MAX_CHEAT_LEN],
    pub sequence_len: usize,
    pub parameter_chars: i32,
    pub chars_read: usize,
    pub param_chars_read: i32,
    pub parameter_buf: [i8; MAX_CHEAT_PARAMS],
}

/// C macro: CHEAT - struct initializer, use CheatseqT directly
/// C macro: CHEAT(value, parameters) { value, sizeof(value) - 1, parameters, 0, 0, "" }

/// C function: cht_CheckCheat
pub fn cht_check_cheat(cht: *mut CheatseqT, key: i8) -> i32 {
    todo!("original: cht_CheckCheat")
}

/// C function: cht_GetParam
pub fn cht_get_param(cht: *mut CheatseqT, buffer: *mut i8) {
    todo!("original: cht_GetParam")
}
