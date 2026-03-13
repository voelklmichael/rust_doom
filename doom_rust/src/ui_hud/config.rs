//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Configuration file interface.
// Original: m_config.h + m_config.c

use crate::doomkeys;
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

    // Key bindings (Doom base)
    ints.insert("key_right".to_string(), doomkeys::KEY_RIGHTARROW);
    ints.insert("key_left".to_string(), doomkeys::KEY_LEFTARROW);
    ints.insert("key_up".to_string(), doomkeys::KEY_UPARROW);
    ints.insert("key_down".to_string(), doomkeys::KEY_DOWNARROW);
    ints.insert("key_strafeleft".to_string(), doomkeys::KEY_STRAFE_L);
    ints.insert("key_straferight".to_string(), doomkeys::KEY_STRAFE_R);
    ints.insert("key_fire".to_string(), doomkeys::KEY_FIRE);
    ints.insert("key_use".to_string(), doomkeys::KEY_USE);
    ints.insert("key_strafe".to_string(), doomkeys::KEY_RALT);
    ints.insert("key_speed".to_string(), doomkeys::KEY_RSHIFT);
    ints.insert("key_pause".to_string(), doomkeys::KEY_PAUSE);
    ints.insert("key_message_refresh".to_string(), doomkeys::KEY_ENTER);
    ints.insert("key_demo_quit".to_string(), b'q' as i32);
    ints.insert("key_spy".to_string(), doomkeys::KEY_F12);
    ints.insert("key_multi_msg".to_string(), b't' as i32);
    ints.insert("key_weapon1".to_string(), b'1' as i32);
    ints.insert("key_weapon2".to_string(), b'2' as i32);
    ints.insert("key_weapon3".to_string(), b'3' as i32);
    ints.insert("key_weapon4".to_string(), b'4' as i32);
    ints.insert("key_weapon5".to_string(), b'5' as i32);
    ints.insert("key_weapon6".to_string(), b'6' as i32);
    ints.insert("key_weapon7".to_string(), b'7' as i32);
    ints.insert("key_weapon8".to_string(), b'8' as i32);
    ints.insert("key_menu_activate".to_string(), doomkeys::KEY_ESCAPE);
    ints.insert("key_menu_up".to_string(), doomkeys::KEY_UPARROW);
    ints.insert("key_menu_down".to_string(), doomkeys::KEY_DOWNARROW);
    ints.insert("key_menu_left".to_string(), doomkeys::KEY_LEFTARROW);
    ints.insert("key_menu_right".to_string(), doomkeys::KEY_RIGHTARROW);
    ints.insert("key_menu_back".to_string(), doomkeys::KEY_BACKSPACE);
    ints.insert("key_menu_forward".to_string(), doomkeys::KEY_ENTER);
    ints.insert("key_menu_confirm".to_string(), b'y' as i32);
    ints.insert("key_menu_abort".to_string(), b'n' as i32);
    ints.insert("key_menu_help".to_string(), doomkeys::KEY_F1);
    ints.insert("key_menu_save".to_string(), doomkeys::KEY_F2);
    ints.insert("key_menu_load".to_string(), doomkeys::KEY_F3);
    ints.insert("key_menu_volume".to_string(), doomkeys::KEY_F4);
    ints.insert("key_menu_detail".to_string(), doomkeys::KEY_F5);
    ints.insert("key_menu_qsave".to_string(), doomkeys::KEY_F6);
    ints.insert("key_menu_endgame".to_string(), doomkeys::KEY_F7);
    ints.insert("key_menu_messages".to_string(), doomkeys::KEY_F8);
    ints.insert("key_menu_qload".to_string(), doomkeys::KEY_F9);
    ints.insert("key_menu_quit".to_string(), doomkeys::KEY_F10);
    ints.insert("key_menu_gamma".to_string(), doomkeys::KEY_F11);
    ints.insert("key_menu_incscreen".to_string(), doomkeys::KEY_EQUALS);
    ints.insert("key_menu_decscreen".to_string(), doomkeys::KEY_MINUS);
    ints.insert("key_menu_screenshot".to_string(), 0);
    ints.insert("key_demo_quit".to_string(), b'q' as i32);
    ints.insert("key_spy".to_string(), doomkeys::KEY_F12);

    // Mouse/joy (base)
    ints.insert("mouseb_fire".to_string(), 0);
    ints.insert("mouseb_strafe".to_string(), 1);
    ints.insert("mouseb_forward".to_string(), 2);
    ints.insert("joyb_fire".to_string(), 0);
    ints.insert("joyb_strafe".to_string(), 1);
    ints.insert("joyb_use".to_string(), 2);
    ints.insert("joyb_speed".to_string(), 3);
    ints.insert("key_map_north".to_string(), doomkeys::KEY_UPARROW);
    ints.insert("key_map_south".to_string(), doomkeys::KEY_DOWNARROW);
    ints.insert("key_map_east".to_string(), doomkeys::KEY_RIGHTARROW);
    ints.insert("key_map_west".to_string(), doomkeys::KEY_LEFTARROW);
    ints.insert("key_map_zoomin".to_string(), doomkeys::KEY_EQUALS);
    ints.insert("key_map_zoomout".to_string(), doomkeys::KEY_MINUS);
    ints.insert("key_map_toggle".to_string(), doomkeys::KEY_TAB);
    ints.insert("key_map_maxzoom".to_string(), b'0' as i32);
    ints.insert("key_map_follow".to_string(), b'f' as i32);
    ints.insert("key_map_grid".to_string(), b'g' as i32);
    ints.insert("key_map_mark".to_string(), b'm' as i32);
    ints.insert("key_map_clearmark".to_string(), b'c' as i32);
    ints.insert("key_prevweapon".to_string(), 0);
    ints.insert("key_nextweapon".to_string(), 0);
    ints.insert("mouseb_prevweapon".to_string(), -1);
    ints.insert("mouseb_nextweapon".to_string(), -1);
    ints.insert("joyb_prevweapon".to_string(), -1);
    ints.insert("joyb_nextweapon".to_string(), -1);
    ints.insert("joyb_menu_activate".to_string(), -1);
    ints.insert("joyb_strafeleft".to_string(), -1);
    ints.insert("joyb_straferight".to_string(), -1);
    ints.insert("mouseb_strafeleft".to_string(), -1);
    ints.insert("mouseb_straferight".to_string(), -1);
    ints.insert("mouseb_use".to_string(), -1);
    ints.insert("mouseb_backward".to_string(), -1);
    ints.insert("mouseb_jump".to_string(), -1);
    ints.insert("joyb_jump".to_string(), -1);
    ints.insert("dclick_use".to_string(), 1);
    ints.insert("key_flyup".to_string(), doomkeys::KEY_PGUP);
    ints.insert("key_flydown".to_string(), doomkeys::KEY_INS);
    ints.insert("key_flycenter".to_string(), doomkeys::KEY_HOME);
    ints.insert("key_lookup".to_string(), doomkeys::KEY_PGDN);
    ints.insert("key_lookdown".to_string(), doomkeys::KEY_DEL);
    ints.insert("key_lookcenter".to_string(), doomkeys::KEY_END);
    ints.insert("key_invleft".to_string(), b'[' as i32);
    ints.insert("key_invright".to_string(), b']' as i32);
    ints.insert("key_useartifact".to_string(), doomkeys::KEY_ENTER);
    ints.insert("key_jump".to_string(), b'/' as i32);
    ints.insert("key_arti_all".to_string(), doomkeys::KEY_BACKSPACE);
    ints.insert("key_arti_health".to_string(), b'\\' as i32);
    ints.insert("key_arti_poisonbag".to_string(), b'0' as i32);
    ints.insert("key_arti_blastradius".to_string(), b'9' as i32);
    ints.insert("key_arti_teleport".to_string(), b'8' as i32);
    ints.insert("key_arti_teleportother".to_string(), b'7' as i32);
    ints.insert("key_arti_egg".to_string(), b'6' as i32);
    ints.insert("key_arti_invulnerability".to_string(), b'5' as i32);
    ints.insert("key_usehealth".to_string(), b'h' as i32);
    ints.insert("key_invquery".to_string(), b'q' as i32);
    ints.insert("key_mission".to_string(), b'w' as i32);
    ints.insert("key_invpop".to_string(), b'z' as i32);
    ints.insert("key_invkey".to_string(), b'k' as i32);
    ints.insert("key_invhome".to_string(), doomkeys::KEY_HOME);
    ints.insert("key_invend".to_string(), doomkeys::KEY_END);
    ints.insert("key_invuse".to_string(), doomkeys::KEY_ENTER);
    ints.insert("key_invdrop".to_string(), doomkeys::KEY_BACKSPACE);
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
    crate::ui_hud::controls::m_sync_controls_to_config();
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

/// Register variable for config read/write. In C, passes a pointer; config writes to it on load.
/// In Rust, key/control sync is handled by m_set_variable -> m_update_control_from_config
/// for known control names. Custom bindings can use m_get_int_variable/m_set_variable.
pub fn m_bind_variable(_name: &str, _variable: *mut std::ffi::c_void) {
    // No-op: control vars are bound implicitly via m_update_control_from_config in m_set_variable
}

pub fn m_set_variable(name: &str, value: &str) -> Boolean {
    let name = name.to_lowercase();
    if let Ok(i) = value.parse::<i32>() {
        int_vars().lock().unwrap().insert(name.clone(), i);
        crate::ui_hud::controls::m_update_control_from_config(&name, i);
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
