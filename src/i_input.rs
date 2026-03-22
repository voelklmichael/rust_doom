//! Input handling (i_input.c)
//! Original: i_input.c

use std::sync::Arc;
use std::sync::Mutex;

pub struct I_InputState {
    pub vanilla_keyboard_mapping: Arc<Mutex<i32>>,
}

impl I_InputState {
    /// Original: void I_GetEvent(void)
    pub fn i_get_event(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_InitInput(void)
    pub fn i_init_input(&self) {
        todo!("Basic stage-0 stub")
    }
}
