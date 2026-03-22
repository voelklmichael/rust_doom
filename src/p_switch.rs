//! Switch texture logic (p_switch.c) - from p_spec
//! Original: p_switch.c

use std::sync::Arc;
use std::sync::Mutex;

pub struct P_SwitchState;

impl P_SwitchState {
    /// Original: boolean P_UseSpecialLine(mobj_t *thing, line_t *line, int side)
    pub fn p_use_special_line(&self, _thing: &(), _line: &(), _side: i32) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_ChangeSwitchTexture(line_t *line, int useAgain)
    pub fn p_change_switch_texture(&self, _line: &(), _use_again: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_InitSwitchList(void)
    pub fn p_init_switch_list(&self) {
        todo!("Basic stage-0 stub")
    }
}
