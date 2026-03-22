//! Door logic (p_doors.c)
//! Original: p_doors.c

use std::sync::Arc;
use std::sync::Mutex;

pub struct P_DoorsState;

impl P_DoorsState {
    /// Original: void T_VerticalDoor(vldoor_t *door)
    pub fn t_vertical_door(&self, _door: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: int EV_DoDoor(line_t *line, vldoor_e type)
    pub fn ev_do_door(&self, _line: &(), _door_type: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int EV_DoLockedDoor(line_t *line, vldoor_e type, mobj_t *thing)
    pub fn ev_do_locked_door(&self, _line: &(), _door_type: i32, _thing: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SpawnDoorCloseIn30(sector_t *sec)
    pub fn p_spawn_door_close_in_30(&self, _sec: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SpawnDoorRaiseIn5Mins(sector_t *sec, int secnum)
    pub fn p_spawn_door_raise_in_5_mins(&self, _sec: &(), _secnum: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_InitSlidingDoorFrames(void)
    pub fn p_init_sliding_door_frames(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: int P_FindSlidingDoorType(line_t *line)
    pub fn p_find_sliding_door_type(&self, _line: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void T_SlidingDoor(slidedoor_t *door)
    pub fn t_sliding_door(&self, _door: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: int EV_SlidingDoor(line_t *line, int type)
    pub fn ev_sliding_door(&self, _line: &(), _door_type: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }
}
