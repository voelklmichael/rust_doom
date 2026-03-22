//! Level setup (p_setup.h, p_setup.c)
//! Original: p_setup.h, p_setup.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_mode::SkillT;

pub struct P_SetupState;

impl P_SetupState {
    /// Original: void P_SetupLevel(int episode, int map, int playermask, skill_t skill)
    pub fn p_setup_level(&self, _episode: i32, _map: i32, _playermask: i32, _skill: SkillT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_Init(void)
    pub fn p_init(&self) {
        todo!("Basic stage-0 stub")
    }
}
