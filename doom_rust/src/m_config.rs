//! Rust translation of doomgeneric/m_config.h

use crate::doomtype::*;

/// C function: M_LoadDefaults
pub fn m_load_defaults() {
    todo!("original: M_LoadDefaults")
}

/// C function: M_SaveDefaults
pub fn m_save_defaults() {
    todo!("original: M_SaveDefaults")
}

/// C function: M_SaveDefaultsAlternate
pub fn m_save_defaults_alternate(main: &str, extra: &str) {
    todo!("original: M_SaveDefaultsAlternate")
}

/// C function: M_SetConfigDir
pub fn m_set_config_dir(dir: &str) {
    todo!("original: M_SetConfigDir")
}

/// C function: M_BindVariable
pub fn m_bind_variable(name: &str, variable: &mut [u8]) {
    todo!("original: M_BindVariable")
}

/// C function: M_SetVariable
pub fn m_set_variable(name: &str, value: &str) -> boolean {
    todo!("original: M_SetVariable")
}

/// C function: M_GetIntVariable
pub fn m_get_int_variable(name: &str) -> i32 {
    todo!("original: M_GetIntVariable")
}

/// C function: M_GetStrVariable
pub fn m_get_str_variable(name: *mut i8) -> *const i8 {
    todo!("original: M_GetStrVariable")
}

/// C function: M_GetFloatVariable
pub fn m_get_float_variable(name: &str) -> f32 {
    todo!("original: M_GetFloatVariable")
}

/// C function: M_SetConfigFilenames
pub fn m_set_config_filenames(main_config: &str, extra_config: &str) {
    todo!("original: M_SetConfigFilenames")
}

/// C function: M_GetSaveGameDir
pub fn m_get_save_game_dir(iwadname: &str) -> String {
    todo!("original: M_GetSaveGameDir")
}

/// C extern: configdir
pub static mut configdir: *mut i8 = std::ptr::null_mut();
