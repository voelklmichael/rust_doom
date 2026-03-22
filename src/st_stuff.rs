//! Status bar (st_stuff.h, st_stuff.c)
//! Original: st_stuff.h, st_stuff.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_event::EventT;
use crate::doomtype::Boolean;

pub const ST_HEIGHT: i32 = 32;
pub const ST_WIDTH: i32 = 320;  // SCREENWIDTH
pub const ST_Y: i32 = 168;     // SCREENHEIGHT - ST_HEIGHT

#[repr(i32)]
pub enum StStateenumT {
    AutomapState = 0,
    FirstPersonState = 1,
}

#[repr(i32)]
pub enum StChatstateenumT {
    StartChatState = 0,
    WaitDestState = 1,
    GetChatState = 2,
}

pub struct St_StuffState {
    pub st_backing_screen: Arc<Mutex<Option<Vec<u8>>>>,
}

impl St_StuffState {
    /// Original: boolean ST_Responder(event_t *ev)
    pub fn st_responder(&self, _ev: &EventT) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void ST_Ticker(void)
    pub fn st_ticker(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void ST_Drawer(boolean fullscreen, boolean refresh)
    pub fn st_drawer(&self, _fullscreen: Boolean, _refresh: Boolean) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void ST_Start(void)
    pub fn st_start(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void ST_Init(void)
    pub fn st_init(&self) {
        todo!("Basic stage-0 stub")
    }
}
