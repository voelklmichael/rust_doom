//! Intermission (wi_stuff.h, wi_stuff.c)
//! Original: wi_stuff.h, wi_stuff.c

use std::sync::Arc;
use std::sync::Mutex;

#[repr(i32)]
pub enum StateenumT {
    NoState = -1,
    StatCount = 0,
    ShowNextLoc = 1,
}

pub struct WbstartstructT;

pub struct Wi_StuffState;

impl Wi_StuffState {
    /// Original: void WI_Ticker(void)
    pub fn wi_ticker(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void WI_Drawer(void)
    pub fn wi_drawer(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void WI_Start(wbstartstruct_t *wbstartstruct)
    pub fn wi_start(&self, _wbstartstruct: &WbstartstructT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void WI_End(void)
    pub fn wi_end(&self) {
        todo!("Basic stage-0 stub")
    }
}
