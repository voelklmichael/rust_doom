//! Rust translation of doomgeneric/m_argv.h

use crate::doomtype::*;

/// C extern: myargc
pub static mut myargc: i32 = 0;

/// C extern: myargv
pub static mut myargv: *mut *mut i8 = std::ptr::null_mut();

/// C function: M_CheckParm
pub fn m_check_parm(check: &str) -> i32 {
    todo!("original: M_CheckParm")
}

/// C function: M_CheckParmWithArgs
pub fn m_check_parm_with_args(check: &str, num_args: i32) -> i32 {
    todo!("original: M_CheckParmWithArgs")
}

/// C function: M_FindResponseFile
pub fn m_find_response_file() {
    todo!("original: M_FindResponseFile")
}

/// C function: M_ParmExists
pub fn m_parm_exists(check: &str) -> boolean {
    todo!("original: M_ParmExists")
}

/// C function: M_GetExecutableName
pub fn m_get_executable_name() -> *mut i8 {
    todo!("original: M_GetExecutableName")
}
