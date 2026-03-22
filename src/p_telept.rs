//! Teleportation (p_telept.c)
//! Original: p_telept.c

use std::sync::Arc;
use std::sync::Mutex;

pub struct P_TeleptState;

impl P_TeleptState {
    /// Original: int EV_Teleport(line_t *line, int side, mobj_t *thing)
    pub fn ev_teleport(&self, _line: &(), _side: i32, _thing: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }
}
