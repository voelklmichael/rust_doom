//! Random number LUT (m_random.h, m_random.c)
//! Original: m_random.h, m_random.c

use std::sync::Arc;
use std::sync::Mutex;

pub struct M_RandomState {
    // int rndindex
    rndindex: Arc<Mutex<i32>>,
    // int prndindex
    prndindex: Arc<Mutex<i32>>,
}

impl M_RandomState {
    /// Original: int M_Random(void)
    pub fn m_random(&self) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int P_Random(void)
    pub fn p_random(&self) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_ClearRandom(void)
    pub fn m_clear_random(&self) {
        todo!("Basic stage-0 stub")
    }
}
