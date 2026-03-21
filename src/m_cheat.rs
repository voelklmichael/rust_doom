// m_cheat.h - cheat code checking

pub use crate::doomtype::*;

// Original: #define MAX_CHEAT_LEN 25
pub const MAX_CHEAT_LEN: usize = 25;

// Original: #define MAX_CHEAT_PARAMS 5
pub const MAX_CHEAT_PARAMS: usize = 5;

/// Original: cheatseq_t
#[repr(C)]
pub struct CheatseqT {
    pub sequence: [i8; MAX_CHEAT_LEN],
    pub sequence_len: usize,
    pub parameter_chars: i32,
    pub chars_read: usize,
    pub param_chars_read: i32,
    pub parameter_buf: [i8; MAX_CHEAT_PARAMS],
}

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct M_CheatState;

impl M_CheatState {
    pub fn new() -> Self {
        Self
    }

    // Original: cht_CheckCheat - complex (>10 lines), stub
    pub fn cht_check_cheat(&self, _cht: &RefCell<CheatseqT>, _key: i8) -> bool {
        todo!("cht_CheckCheat")
    }

    // Original: cht_GetParam - simple memcpy
    pub fn cht_get_param(&self, cht: &CheatseqT, buffer: &mut [i8]) {
        let n = cht.parameter_chars as usize;
        let n = n.min(buffer.len()).min(MAX_CHEAT_PARAMS);
        buffer[..n].copy_from_slice(&cht.parameter_buf[..n]);
    }
}
