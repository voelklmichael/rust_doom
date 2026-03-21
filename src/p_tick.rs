// doomgeneric/p_tick.h

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct P_TickState {
    pub _placeholder: RefCell<()>,
}

impl P_TickState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    pub fn p_ticker(&self) {
        todo!("P_Ticker");
    }
}
