// doomgeneric/p_setup.h

pub use crate::d_mode::SkillT;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct P_SetupState {
    pub _placeholder: RefCell<()>,
}

impl P_SetupState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    pub fn p_setup_level(
        &self,
        _episode: i32,
        _map: i32,
        _playermask: i32,
        _skill: SkillT,
    ) {
        todo!("P_SetupLevel");
    }

    pub fn p_init(&self) {
        todo!("P_Init");
    }
}
