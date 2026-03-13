//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Configuration file interface.
// Original: m_config.h + m_config.c

use crate::doomtype::Boolean;
use std::sync::Mutex;

// =============================================================================
// Public API (from m_config.h)
// =============================================================================

static CONFIG_DIR: Mutex<Option<String>> = Mutex::new(None);

pub fn m_load_defaults() {
    // Stub: would load default.cfg
}

pub fn m_save_defaults() {
    // Stub: would save config to file
}

pub fn m_save_defaults_alternate(_main: &str, _extra: &str) {
    // Stub
}

pub fn m_set_config_dir(dir: &str) {
    *CONFIG_DIR.lock().unwrap() = Some(dir.to_string());
}

pub fn m_bind_variable(_name: &str, _variable: *mut std::ffi::c_void) {
    // Stub: would register variable for config
}

pub fn m_set_variable(_name: &str, _value: &str) -> Boolean {
    // Stub
    false
}

pub fn m_get_int_variable(_name: &str) -> i32 {
    // Stub
    0
}

pub fn m_get_str_variable(_name: &str) -> Option<String> {
    // Stub
    None
}

pub fn m_get_float_variable(_name: &str) -> f32 {
    // Stub
    0.0
}

pub fn m_set_config_filenames(_main_config: &str, _extra_config: &str) {
    // Stub
}

pub fn m_get_save_game_dir(iwadname: &str) -> String {
    let dir = CONFIG_DIR.lock().unwrap();
    let base = dir.as_deref().unwrap_or(".");
    format!("{}/{}", base, iwadname)
}

pub fn configdir() -> Option<String> {
    CONFIG_DIR.lock().unwrap().clone()
}
