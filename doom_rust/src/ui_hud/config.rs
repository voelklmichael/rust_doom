//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Configuration file interface.
// Original: m_config.h + m_config.c

use crate::doomtype::Boolean;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// =============================================================================
// Public API (from m_config.h)
// =============================================================================

static CONFIG_DIR: Mutex<Option<String>> = Mutex::new(None);

fn int_vars() -> &'static Mutex<HashMap<String, i32>> {
    static INT_VARS: OnceLock<Mutex<HashMap<String, i32>>> = OnceLock::new();
    INT_VARS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn float_vars() -> &'static Mutex<HashMap<String, f32>> {
    static FLOAT_VARS: OnceLock<Mutex<HashMap<String, f32>>> = OnceLock::new();
    FLOAT_VARS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn str_vars() -> &'static Mutex<HashMap<String, String>> {
    static STR_VARS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    STR_VARS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn set_defaults() {
    let mut ints = int_vars().lock().unwrap();
    ints.insert("screenblocks".to_string(), 10);
    ints.insert("detailLevel".to_string(), 0);
    ints.insert("mouse_sensitivity".to_string(), 5);
    ints.insert("sfx_volume".to_string(), 8);
    ints.insert("music_volume".to_string(), 8);
    ints.insert("show_messages".to_string(), 1);
}

pub fn m_load_defaults() {
    set_defaults();
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

pub fn m_set_variable(name: &str, value: &str) -> Boolean {
    let name = name.to_lowercase();
    if let Ok(i) = value.parse::<i32>() {
        int_vars().lock().unwrap().insert(name.clone(), i);
        return true;
    }
    if let Ok(f) = value.parse::<f32>() {
        float_vars().lock().unwrap().insert(name.clone(), f);
        return true;
    }
    str_vars().lock().unwrap().insert(name, value.to_string());
    true
}

pub fn m_get_int_variable(name: &str) -> i32 {
    let name = name.to_lowercase();
    int_vars()
        .lock()
        .unwrap()
        .get(&name)
        .copied()
        .unwrap_or_else(|| match name.as_str() {
            "screenblocks" => 10,
            "detaillevel" => 0,
            _ => 0,
        })
}

pub fn m_get_str_variable(name: &str) -> Option<String> {
    let name = name.to_lowercase();
    str_vars().lock().unwrap().get(&name).cloned()
}

pub fn m_get_float_variable(name: &str) -> f32 {
    let name = name.to_lowercase();
    float_vars()
        .lock()
        .unwrap()
        .get(&name)
        .copied()
        .unwrap_or(0.0)
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
