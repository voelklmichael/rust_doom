//! Finale screen (f_finale.h, f_finale.c)
//! Original: f_finale.h, f_finale.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_event::EventT;
use crate::doomtype::Boolean;

pub struct F_FinaleState;

impl F_FinaleState {
    /// Original: boolean F_Responder(event_t *ev)
    pub fn f_responder(&self, _ev: &EventT) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void F_Ticker(void)
    pub fn f_ticker(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void F_Drawer(void)
    pub fn f_drawer(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void F_StartFinale(void)
    pub fn f_start_finale(&self) {
        todo!("Basic stage-0 stub")
    }
}
