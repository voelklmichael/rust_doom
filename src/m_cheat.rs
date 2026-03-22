//! Cheat sequence (m_cheat.h, m_cheat.c)
//! Original: m_cheat.h, m_cheat.c

use std::sync::Arc;
use std::sync::Mutex;

// #define MAX_CHEAT_LEN 25
pub const MAX_CHEAT_LEN: usize = 25;
// #define MAX_CHEAT_PARAMS 5
pub const MAX_CHEAT_PARAMS: usize = 5;

// typedef struct cheatseq_t
pub struct CheatseqT {
    pub sequence: [u8; MAX_CHEAT_LEN],
    pub sequence_len: usize,
    pub parameter_chars: i32,
    pub chars_read: usize,
    pub param_chars_read: i32,
    pub parameter_buf: [u8; MAX_CHEAT_PARAMS],
}

pub struct M_CheatState;

impl M_CheatState {
    /// Original: int cht_CheckCheat(cheatseq_t *cht, char key)
    pub fn cht_check_cheat(&self, _cht: &mut CheatseqT, _key: u8) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void cht_GetParam(cheatseq_t *cht, char *buffer)
    pub fn cht_get_param(&self, _cht: &CheatseqT, _buffer: &mut [u8]) {
        todo!("Basic stage-0 stub")
    }
}
