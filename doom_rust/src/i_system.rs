//! Rust translation of doomgeneric/i_system.h

use crate::d_event::*;
use crate::d_ticcmd::*;
use crate::doomtype::*;

/// C typedef: atexit_func_t
pub type AtexitFuncT = extern "C" fn();

/// C function: I_Init
pub fn i_init() {
    todo!("original: I_Init")
}

/// C function: I_ZoneBase
pub fn i_zone_base(size: &mut i32) -> *mut byte {
    todo!("original: I_ZoneBase")
}

/// C function: I_ConsoleStdout
pub fn i_console_stdout() -> boolean {
    todo!("original: I_ConsoleStdout")
}

/// C function: I_BaseTiccmd
pub fn i_base_ticcmd() -> *mut TiccmdT {
    todo!("original: I_BaseTiccmd")
}

/// C function: I_Quit
pub fn i_quit() {
    todo!("original: I_Quit")
}

/// C function: I_Error
pub fn i_error(error: &str) {
    todo!("original: I_Error")
}

/// C function: I_Tactile
pub fn i_tactile(on: i32, off: i32, total: i32) {
    todo!("original: I_Tactile")
}

/// C function: I_GetMemoryValue
pub fn i_get_memory_value(offset: u32, value: &mut [u8], size: i32) -> boolean {
    todo!("original: I_GetMemoryValue")
}

/// C function: I_AtExit
pub fn i_at_exit(func: AtexitFuncT, run_if_error: boolean) {
    todo!("original: I_AtExit")
}

/// C function: I_BindVariables
pub fn i_bind_variables() {
    todo!("original: I_BindVariables")
}

/// C function: I_PrintStartupBanner
pub fn i_print_startup_banner(gamedescription: &str) {
    todo!("original: I_PrintStartupBanner")
}

/// C function: I_PrintBanner
pub fn i_print_banner(text: &str) {
    todo!("original: I_PrintBanner")
}

/// C function: I_PrintDivider
pub fn i_print_divider() {
    todo!("original: I_PrintDivider")
}
