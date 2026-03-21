// d_event.h / d_event.c - Event handling

pub use crate::doomtype::*;

use std::cell::RefCell;

// Original: #define MAXEVENTS 64
const MAXEVENTS: usize = 64;

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum EvtypeT {
    EvKeydown = 0,
    EvKeyup = 1,
    EvMouse = 2,
    EvJoystick = 3,
    EvQuit = 4,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EventT {
    pub ev_type: EvtypeT,
    pub data1: i32,
    pub data2: i32,
    pub data3: i32,
    pub data4: i32,
}

// Original: buttoncode_t - use consts for overlapping values
pub const BT_ATTACK: i32 = 1;
pub const BT_USE: i32 = 2;
pub const BT_SPECIAL: i32 = 128;
pub const BT_SPECIALMASK: i32 = 3;
pub const BT_CHANGE: i32 = 4;
pub const BT_WEAPONMASK: i32 = 8 + 16 + 32;
pub const BT_WEAPONSHIFT: i32 = 3;
pub const BTS_PAUSE: i32 = 1;
pub const BTS_SAVEGAME: i32 = 2;
pub const BTS_SAVEMASK: i32 = 4 + 8 + 16;
pub const BTS_SAVESHIFT: i32 = 2;

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Buttoncode2T {
    LookUp = 1,
    LookDown = 2,
    CenterView = 4,
    InvUse = 8,
    InvDrop = 16,
    Jump = 32,
    Health = 128,
}

#[allow(non_camel_case_types)]
pub struct D_EventState {
    // Original: events[MAXEVENTS]
    events: RefCell<[EventT; MAXEVENTS]>,
    // Original: eventhead
    eventhead: RefCell<usize>,
    // Original: eventtail
    eventtail: RefCell<usize>,
}

impl D_EventState {
    pub fn new() -> Self {
        Self {
            events: RefCell::new([EventT {
                ev_type: EvtypeT::EvKeydown,
                data1: 0,
                data2: 0,
                data3: 0,
                data4: 0,
            }; MAXEVENTS]),
            eventhead: RefCell::new(0),
            eventtail: RefCell::new(0),
        }
    }

    // Original: D_PostEvent
    pub fn d_post_event(&self, ev: &EventT) {
        let mut head = *self.eventhead.borrow();
        self.events.borrow_mut()[head] = EventT {
            ev_type: ev.ev_type,
            data1: ev.data1,
            data2: ev.data2,
            data3: ev.data3,
            data4: ev.data4,
        };
        head = (head + 1) % MAXEVENTS;
        *self.eventhead.borrow_mut() = head;
    }

    // Original: D_PopEvent
    pub fn d_pop_event(&self) -> Option<EventT> {
        let tail = *self.eventtail.borrow();
        let head = *self.eventhead.borrow();
        if tail == head {
            return None;
        }
        let result = self.events.borrow()[tail];
        *self.eventtail.borrow_mut() = (tail + 1) % MAXEVENTS;
        Some(result)
    }
}
