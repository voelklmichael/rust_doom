//! Event handling (d_event.h, d_event.c)
//! Original: d_event.h, d_event.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;

// #define MAXEVENTS 64
const MAXEVENTS: usize = 64;

// typedef enum evtype_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvtypeT {
    EvKeydown,
    EvKeyup,
    EvMouse,
    EvJoystick,
    EvQuit,
}

// typedef struct event_t
pub struct EventT {
    pub type_: EvtypeT,
    pub data1: i32,
    pub data2: i32,
    pub data3: i32,
    pub data4: i32,
}

// typedef enum buttoncode_t
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

pub const BT2_LOOKUP: i32 = 1;
pub const BT2_LOOKDOWN: i32 = 2;
pub const BT2_CENTERVIEW: i32 = 4;
pub const BT2_INVUSE: i32 = 8;
pub const BT2_INVDROP: i32 = 16;
pub const BT2_JUMP: i32 = 32;
pub const BT2_HEALTH: i32 = 128;

pub struct D_EventState {
    // static event_t events[MAXEVENTS]
    events: Arc<Mutex<Vec<EventT>>>,
    // static int eventhead
    eventhead: Arc<Mutex<usize>>,
    // static int eventtail
    eventtail: Arc<Mutex<usize>>,
}

impl D_EventState {
    /// Original: void D_PostEvent(event_t *ev)
    pub fn d_post_event(&self, ev: &EventT) {
        // C body:
        // events[eventhead] = *ev;
        // eventhead = (eventhead + 1) % MAXEVENTS;
        todo!("Basic stage-0 stub")
    }

    /// Original: event_t *D_PopEvent(void)
    pub fn d_pop_event(&self) -> Option<EventT> {
        // C body:
        // if (eventtail == eventhead) return NULL;
        // result = &events[eventtail];
        // eventtail = (eventtail + 1) % MAXEVENTS;
        // return result;
        todo!("Basic stage-0 stub")
    }
}
