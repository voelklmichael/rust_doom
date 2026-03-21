// doomgeneric/statdump.h

pub use crate::d_mode::*;
pub use crate::d_player::*;
pub use crate::m_argv::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct StatdumpState {
    pub _placeholder: RefCell<()>,
}

impl StatdumpState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    pub fn stat_copy(&self, _stats: *mut WbstartstructT) {
        todo!("StatCopy");
    }

    pub fn stat_dump(&self) {
        todo!("StatDump");
    }
}
