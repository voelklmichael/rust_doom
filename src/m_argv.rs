// Stub for m_argv.h - minimal interface for doomgeneric.
// Full migration in Phase 6.

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct M_ArgvState {
    // Original: myargc
    pub myargc: RefCell<i32>,
    // Original: myargv
    pub myargv: RefCell<*mut *mut std::ffi::c_char>,
}

impl M_ArgvState {
    pub fn new() -> Self {
        Self {
            myargc: RefCell::new(0),
            myargv: RefCell::new(std::ptr::null_mut()),
        }
    }

    // Original: M_FindResponseFile
    pub fn m_find_response_file(&self) {
        todo!("M_FindResponseFile")
    }

    // Original: M_CheckParm
    pub fn m_check_parm(&self, _check: &str) -> i32 {
        let _ = _check;
        0
    }
}
