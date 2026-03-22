//! Main loop (d_main.h, d_main.c)
//! Original: d_main.h, d_main.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomdef::GameactionT;

pub struct D_MainState {
    pub gameaction: Arc<Mutex<GameactionT>>,
}

impl D_MainState {
    /// Original: void D_ProcessEvents(void)
    pub fn d_process_events(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void D_PageTicker(void)
    pub fn d_page_ticker(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void D_PageDrawer(void)
    pub fn d_page_drawer(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void D_AdvanceDemo(void)
    pub fn d_advance_demo(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void D_DoAdvanceDemo(void)
    pub fn d_do_advance_demo(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void D_StartTitle(void)
    pub fn d_start_title(&self) {
        todo!("Basic stage-0 stub")
    }
}
