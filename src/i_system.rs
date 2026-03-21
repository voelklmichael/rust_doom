// doomgeneric/i_system.h

use std::cell::RefCell;
use std::ffi::{c_char, c_void};
use std::os::raw::c_int;

pub use crate::d_event::*;
pub use crate::d_ticcmd::*;
pub use crate::doomtype::*;

// Original: typedef void (*atexit_func_t)(void)
pub type AtexitFuncT = unsafe extern "C" fn();

#[allow(non_camel_case_types)]
pub struct I_SystemState {
    /// Original: static atexit chain / error state placeholders from i_system.c
    pub quit_in_progress: RefCell<Boolean>,
}

impl I_SystemState {
    pub fn new() -> Self {
        Self {
            quit_in_progress: RefCell::new(Boolean::False),
        }
    }

    // Original: I_Init
    pub fn i_init(&self) {
        todo!("I_Init");
    }

    // Original: I_ZoneBase
    pub fn i_zone_base(&self, _size: *mut c_int) -> *mut Byte {
        todo!("I_ZoneBase");
    }

    // Original: I_ConsoleStdout
    pub fn i_console_stdout(&self) -> Boolean {
        todo!("I_ConsoleStdout");
    }

    // Original: I_BaseTiccmd
    pub fn i_base_ticcmd(&self) -> *mut TiccmdT {
        todo!("I_BaseTiccmd");
    }

    // Original: I_Quit
    pub fn i_quit(&self) {
        todo!("I_Quit");
    }

    // Original: I_Error
    pub fn i_error(&self, _error: *mut c_char) {
        todo!("I_Error");
    }

    // Original: I_Tactile
    pub fn i_tactile(&self, _on: c_int, _off: c_int, _total: c_int) {
        todo!("I_Tactile");
    }

    // Original: I_GetMemoryValue
    pub fn i_get_memory_value(&self, _offset: u32, _value: *mut c_void, _size: c_int) -> Boolean {
        todo!("I_GetMemoryValue");
    }

    // Original: I_AtExit
    pub fn i_at_exit(&self, _func: AtexitFuncT, _run_if_error: Boolean) {
        todo!("I_AtExit");
    }

    // Original: I_BindVariables
    pub fn i_bind_variables(&self) {
        todo!("I_BindVariables");
    }

    // Original: I_PrintStartupBanner
    pub fn i_print_startup_banner(&self, _gamedescription: *mut c_char) {
        todo!("I_PrintStartupBanner");
    }

    // Original: I_PrintBanner
    pub fn i_print_banner(&self, _text: *mut c_char) {
        todo!("I_PrintBanner");
    }

    // Original: I_PrintDivider
    pub fn i_print_divider(&self) {
        todo!("I_PrintDivider");
    }
}
