//! Heads-up display (hu_stuff.h, hu_stuff.c)
//! Original: hu_stuff.h, hu_stuff.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_event::EventT;
use crate::doomtype::Boolean;

pub const HU_FONTSTART: u8 = b'!';
pub const HU_FONTEND: u8 = b'_';
pub const HU_FONTSIZE: usize = (HU_FONTEND - HU_FONTSTART + 1) as usize;
pub const HU_BROADCAST: i32 = 5;
pub const HU_MSGX: i32 = 0;
pub const HU_MSGY: i32 = 0;
pub const HU_MSGWIDTH: i32 = 64;
pub const HU_MSGHEIGHT: i32 = 1;
// HU_MSGTIMEOUT = 4*TICRATE

pub struct HuStuffState {
    pub chat_macros: Arc<Mutex<[String; 10]>>,
}

impl HuStuffState {
    /// Original: void HU_Init(void)
    pub fn hu_init(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HU_Start(void)
    pub fn hu_start(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean HU_Responder(event_t *ev)
    pub fn hu_responder(&self, _ev: &EventT) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HU_Ticker(void)
    pub fn hu_ticker(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HU_Drawer(void)
    pub fn hu_drawer(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: char HU_dequeueChatChar(void)
    pub fn hu_dequeue_chat_char(&self) -> u8 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HU_Erase(void)
    pub fn hu_erase(&self) {
        todo!("Basic stage-0 stub")
    }
}
