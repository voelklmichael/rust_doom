//! Thinker/ticker (p_tick.h, p_tick.c)
//! Original: p_tick.h, p_tick.c

use std::sync::Arc;
use std::sync::Mutex;

pub struct P_TickState {
    // int leveltime
    pub leveltime: Arc<Mutex<i32>>,
}

impl P_TickState {
    /// Original: void P_Ticker(void)
    pub fn p_ticker(&self) {
        // C body: runs player think, P_RunThinkers, P_UpdateSpecials, etc.
        todo!("Basic stage-0 stub")
    }
}
