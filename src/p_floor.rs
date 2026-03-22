//! Floor animation (p_floor.c)
//! Original: p_floor.c

use std::sync::Arc;
use std::sync::Mutex;

pub struct P_FloorState;

impl P_FloorState {
    /// Original: void T_MoveFloor(floormove_t *floor)
    pub fn t_move_floor(&self, _floor: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: int EV_DoFloor(line_t *line, floor_e type)
    pub fn ev_do_floor(&self, _line: &(), _floor_type: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int EV_BuildStairs(line_t *line, stair_e type)
    pub fn ev_build_stairs(&self, _line: &(), _stair_type: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }
}
