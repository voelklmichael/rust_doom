//! Menu (m_menu.h, m_menu.c)
//! Original: m_menu.h, m_menu.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_event::EventT;
use crate::doomtype::Boolean;

pub struct M_MenuState {
    pub detail_level: Arc<Mutex<i32>>,
    pub screenblocks: Arc<Mutex<i32>>,
}

impl M_MenuState {
    /// Original: boolean M_Responder(event_t *ev)
    pub fn m_responder(&self, _ev: &EventT) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_Ticker(void)
    pub fn m_ticker(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_Drawer(void)
    pub fn m_drawer(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_Init(void)
    pub fn m_init(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_StartControlPanel(void)
    pub fn m_start_control_panel(&self) {
        todo!("Basic stage-0 stub")
    }
}
