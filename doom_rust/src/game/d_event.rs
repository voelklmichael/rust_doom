//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Event handling.
//
// Original: d_event.h + d_event.c

use std::sync::Mutex;

// =============================================================================
// Constants
// =============================================================================

pub const MAXEVENTS: usize = 64;

// =============================================================================
// Types (from d_event.h)
// =============================================================================

/// Input event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum EvType {
    KeyDown,
    KeyUp,
    Mouse,
    Joystick,
    #[default]
    Quit,
}

/// Event structure.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Event {
    pub ev_type: EvType,
    pub data1: i32,
    pub data2: i32,
    pub data3: i32,
    pub data4: i32,
}

/// Button/action code definitions (C: buttoncode_t).
pub mod buttoncode {
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
}

/// Strife-specific buttons (stub).
pub mod buttoncode2 {
    pub const BT2_LOOKUP: i32 = 1;
    pub const BT2_LOOKDOWN: i32 = 2;
    pub const BT2_CENTERVIEW: i32 = 4;
    pub const BT2_INVUSE: i32 = 8;
    pub const BT2_INVDROP: i32 = 16;
    pub const BT2_JUMP: i32 = 32;
    pub const BT2_HEALTH: i32 = 128;
}

// =============================================================================
// Event queue (from d_event.c)
// =============================================================================

struct EventQueue {
    events: [Event; MAXEVENTS],
    head: usize,
    tail: usize,
}

impl EventQueue {
    const fn new() -> Self {
        Self {
            events: [Event {
                ev_type: EvType::Quit,
                data1: 0,
                data2: 0,
                data3: 0,
                data4: 0,
            }; MAXEVENTS],
            head: 0,
            tail: 0,
        }
    }

    fn post(&mut self, ev: &Event) {
        self.events[self.head] = *ev;
        self.head = (self.head + 1) % MAXEVENTS;
    }

    fn pop(&mut self) -> Option<Event> {
        if self.tail == self.head {
            return None;
        }
        let result = self.events[self.tail];
        self.tail = (self.tail + 1) % MAXEVENTS;
        Some(result)
    }
}

static EVENT_QUEUE: Mutex<EventQueue> = Mutex::new(EventQueue::new());

// =============================================================================
// API
// =============================================================================

/// Called by I/O functions when input is detected.
/// Original: D_PostEvent
pub fn d_post_event(ev: &Event) {
    EVENT_QUEUE.lock().unwrap().post(ev);
}

/// Read an event from the queue.
/// Original: D_PopEvent
pub fn d_pop_event() -> Option<Event> {
    EVENT_QUEUE.lock().unwrap().pop()
}
