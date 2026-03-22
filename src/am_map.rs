//! AutoMap (am_map.h, am_map.c)
//! Original: am_map.h, am_map.c

use crate::d_event::EventT;
use crate::doomtype::Boolean;

pub const AM_MSGHEADER: u32 = 0x61000000 | 0x006d0000;
pub const AM_MSGENTERED: u32 = AM_MSGHEADER | 0x00006500;
pub const AM_MSGEXITED: u32 = AM_MSGHEADER | 0x00007800;

pub struct Am_MapState;

impl Am_MapState {
    pub fn am_responder(&self, _ev: &EventT) -> Boolean {
        todo!("Basic stage-0 stub")
    }
    pub fn am_ticker(&self) {
        todo!("Basic stage-0 stub")
    }
    pub fn am_drawer(&self) {
        todo!("Basic stage-0 stub")
    }
    pub fn am_stop(&self) {
        todo!("Basic stage-0 stub")
    }
}
