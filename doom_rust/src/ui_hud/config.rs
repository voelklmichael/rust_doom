//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Configuration file interface.
// Original: m_config.h + m_config.c

use crate::doomtype::Boolean;
use crate::m_argv::m_check_parm_with_args;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::{Mutex, OnceLock};

// =============================================================================
// Public API (from m_config.h)
// =============================================================================

static CONFIG_DIR: Mutex<Option<String>> = Mutex::new(None);
const DEFAULT_CONFIG: &str = "default.cfg";
static MAIN_CONFIG: Mutex<Option<String>> = Mutex::new(None);
static EXTRA_CONFIG: Mutex<Option<String>> = Mutex::new(None);

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
    ints.insert("detaillevel".to_string(), 0);
    ints.insert("mouse_sensitivity".to_string(), 5);
    ints.insert("sfx_volume".to_string(), 8);
    ints.insert("music_volume".to_string(), 8);
    ints.insert("show_messages".to_string(), 1);
}

fn config_path(filename: &str) -> String {
    let dir = CONFIG_DIR.lock().unwrap();
    let base = dir.as_deref().unwrap_or(".");
    if base.is_empty() || base == "." {
        filename.to_string()
    } else {
        format!("{}/{}", base, filename)
    }
}

/// Parse config file: "name value" per line. Skips empty lines and # comments.
fn load_config_file(path: &Path) {
    let Ok(file) = fs::File::open(path) else {
        return;
    };
    let reader = BufReader::new(file);
    for line in reader.lines().filter_map(Result::ok) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((name, value)) = line.split_once(|c: char| c.is_ascii_whitespace()) {
            let value = value.trim();
            if !value.is_empty() {
                m_set_variable(name, value);
            }
        }
    }
}

/// Write config variables to file.
fn save_config_file(path: &Path) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(mut file) = fs::File::create(path) {
        let ints = int_vars().lock().unwrap();
        let floats = float_vars().lock().unwrap();
        let strs = str_vars().lock().unwrap();
        for (name, val) in ints.iter() {
            let _ = writeln!(file, "{} {}", name, val);
        }
        for (name, val) in floats.iter() {
            let _ = writeln!(file, "{} {}", name, val);
        }
        for (name, val) in strs.iter() {
            let s = val.replace('"', "\\\"");
            let _ = writeln!(file, "{} \"{}\"", name, s);
        }
    }
}

pub fn m_load_defaults() {
    set_defaults();

    // -config <file> overrides main config path
    let main_path = if m_check_parm_with_args("-config", 1) != 0 {
        let i = m_check_parm_with_args("-config", 1);
        crate::m_argv::myargv()
            .get(i + 1)
            .cloned()
            .unwrap_or_else(|| config_path(MAIN_CONFIG.lock().unwrap().as_deref().unwrap_or(DEFAULT_CONFIG)))
    } else {
        config_path(MAIN_CONFIG.lock().unwrap().as_deref().unwrap_or(DEFAULT_CONFIG))
    };

    load_config_file(Path::new(&main_path));

    // -extraconfig <file> for extra config
    let extra_path = if m_check_parm_with_args("-extraconfig", 1) != 0 {
        let i = m_check_parm_with_args("-extraconfig", 1);
        crate::m_argv::myargv().get(i + 1).cloned()
    } else {
        Some(config_path(EXTRA_CONFIG.lock().unwrap().as_deref().unwrap_or(DEFAULT_CONFIG)))
    };
    if let Some(ep) = extra_path {
        load_config_file(Path::new(&ep));
    }
}

pub fn m_save_defaults() {
    let main_path = config_path(MAIN_CONFIG.lock().unwrap().as_deref().unwrap_or(DEFAULT_CONFIG));
    save_config_file(Path::new(&main_path));
    let extra_path = config_path(EXTRA_CONFIG.lock().unwrap().as_deref().unwrap_or(DEFAULT_CONFIG));
    if main_path != extra_path {
        save_config_file(Path::new(&extra_path));
    }
}

pub fn m_save_defaults_alternate(main: &str, extra: &str) {
    save_config_file(Path::new(main));
    if main != extra {
        save_config_file(Path::new(extra));
    }
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

pub fn m_set_config_filenames(main_config: &str, extra_config: &str) {
    *MAIN_CONFIG.lock().unwrap() = Some(main_config.to_string());
    *EXTRA_CONFIG.lock().unwrap() = Some(extra_config.to_string());
}

pub fn m_get_save_game_dir(iwadname: &str) -> String {
    let dir = CONFIG_DIR.lock().unwrap();
    let base = dir.as_deref().unwrap_or(".");
    format!("{}/{}", base, iwadname)
}

pub fn configdir() -> Option<String> {
    CONFIG_DIR.lock().unwrap().clone()
}
