// doomgeneric/m_config.h

use std::cell::RefCell;
use std::ffi::{c_char, c_void, CString};
use std::os::raw::c_float;

pub use crate::doomtype::*;

/// Original: extern char *configdir
#[allow(non_camel_case_types)]
pub struct M_ConfigState {
    // Original: configdir
    pub configdir: RefCell<Option<CString>>,
}

impl M_ConfigState {
    pub fn new() -> Self {
        Self {
            configdir: RefCell::new(None),
        }
    }

    // Original: M_LoadDefaults
    pub fn m_load_defaults(&self) {
        todo!("M_LoadDefaults");
    }

    // Original: M_SaveDefaults
    pub fn m_save_defaults(&self) {
        todo!("M_SaveDefaults");
    }

    // Original: M_SaveDefaultsAlternate
    pub fn m_save_defaults_alternate(&self, _main: *mut c_char, _extra: *mut c_char) {
        todo!("M_SaveDefaultsAlternate");
    }

    // Original: M_SetConfigDir
    pub fn m_set_config_dir(&self, _dir: *mut c_char) {
        todo!("M_SetConfigDir");
    }

    // Original: M_BindVariable
    pub fn m_bind_variable(&self, _name: *mut c_char, _variable: *mut c_void) {
        todo!("M_BindVariable");
    }

    // Original: M_SetVariable
    pub fn m_set_variable(&self, _name: *mut c_char, _value: *mut c_char) -> Boolean {
        todo!("M_SetVariable");
    }

    // Original: M_GetIntVariable
    pub fn m_get_int_variable(&self, _name: *mut c_char) -> i32 {
        todo!("M_GetIntVariable");
    }

    // Original: M_GetStrVariable
    pub fn m_get_str_variable(&self, _name: *mut c_char) -> *const c_char {
        todo!("M_GetStrVariable");
    }

    // Original: M_GetFloatVariable
    pub fn m_get_float_variable(&self, _name: *mut c_char) -> c_float {
        todo!("M_GetFloatVariable");
    }

    // Original: M_SetConfigFilenames
    pub fn m_set_config_filenames(&self, _main_config: *mut c_char, _extra_config: *mut c_char) {
        todo!("M_SetConfigFilenames");
    }

    // Original: M_GetSaveGameDir
    pub fn m_get_save_game_dir(&self, _iwadname: *mut c_char) -> *mut c_char {
        todo!("M_GetSaveGameDir");
    }
}
