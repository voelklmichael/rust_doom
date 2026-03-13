//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Cheat code checking.
// Original: m_cheat.h + m_cheat.c

// =============================================================================
// Public API (from m_cheat.h)
// =============================================================================

pub const MAX_CHEAT_LEN: usize = 25;
pub const MAX_CHEAT_PARAMS: usize = 5;

/// Cheat sequence state. CHEAT(value, parameters) in C becomes
/// CheatSeq::new(value, parameters).
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CheatSeq {
    /// Cheat sequence string (null-terminated in C)
    pub sequence: [u8; MAX_CHEAT_LEN],
    pub sequence_len: usize,
    pub parameter_chars: i32,

    pub chars_read: usize,
    pub param_chars_read: i32,
    pub parameter_buf: [u8; MAX_CHEAT_PARAMS],
}

impl Default for CheatSeq {
    fn default() -> Self {
        CheatSeq::EMPTY
    }
}

impl CheatSeq {
    pub const EMPTY: CheatSeq = CheatSeq {
        sequence: [0; MAX_CHEAT_LEN],
        sequence_len: 0,
        parameter_chars: 0,
        chars_read: 0,
        param_chars_read: 0,
        parameter_buf: [0; MAX_CHEAT_PARAMS],
    };
    /// Create a cheat sequence. Equivalent to CHEAT(value, parameters).
    pub fn new(sequence: &str, parameter_chars: i32) -> Self {
        let seq_len = sequence.len().min(MAX_CHEAT_LEN);
        let mut seq_bytes = [0u8; MAX_CHEAT_LEN];
        seq_bytes[..seq_len].copy_from_slice(sequence.as_bytes());
        Self {
            sequence: seq_bytes,
            sequence_len: seq_len,
            parameter_chars: parameter_chars.min(MAX_CHEAT_PARAMS as i32),
            chars_read: 0,
            param_chars_read: 0,
            parameter_buf: [0; MAX_CHEAT_PARAMS],
        }
    }

    fn sequence_str(&self) -> &[u8] {
        &self.sequence[..self.sequence_len]
    }
}

// =============================================================================
// Implementation (from m_cheat.c)
// =============================================================================

/// Called in st_stuff module, which handles the input.
/// Returns true if the cheat was successful, false if failed.
pub fn cht_check_cheat(cht: &mut CheatSeq, key: u8) -> bool {
    if cht.parameter_chars > 0 && cht.sequence_str().len() < cht.sequence_len {
        return false;
    }

    if cht.chars_read < cht.sequence_str().len() {
        if key == cht.sequence[cht.chars_read] {
            cht.chars_read += 1;
        } else {
            cht.chars_read = 0;
        }
        cht.param_chars_read = 0;
    } else if (cht.param_chars_read as usize) < cht.parameter_chars as usize {
        cht.parameter_buf[cht.param_chars_read as usize] = key;
        cht.param_chars_read += 1;
    }

    if cht.chars_read >= cht.sequence_str().len()
        && cht.param_chars_read >= cht.parameter_chars
    {
        cht.chars_read = 0;
        cht.param_chars_read = 0;
        return true;
    }

    false
}

/// Copy parameter buffer into output. Caller must ensure buffer has at least
/// parameter_chars bytes.
pub fn cht_get_param(cht: &CheatSeq, buffer: &mut [u8]) {
    let n = cht.parameter_chars as usize;
    buffer[..n].copy_from_slice(&cht.parameter_buf[..n]);
}
