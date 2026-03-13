//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: Menu widget stuff, episode selection.
// Original: m_menu.h + m_menu.c

use crate::deh::deh_string;
use crate::doomstat::{GAMEMODE, MENUACTIVE};
use crate::doomtype::Boolean;
use crate::game::d_event::Event;
use crate::game::d_mode::GameMode;
use crate::rendering::v_draw_patch_direct;
use crate::ui_hud::config::{m_get_int_variable, m_load_defaults, m_set_variable};
use crate::rendering::r_set_view_size;
use crate::ui_hud::controls::{
    m_bind_base_controls, m_bind_map_controls, m_bind_menu_controls, m_bind_weapon_controls,
    KEY_MENU_BACK, KEY_MENU_CONFIRM, KEY_MENU_DOWN, KEY_MENU_FORWARD, KEY_MENU_LEFT,
    KEY_MENU_RIGHT, KEY_MENU_UP,
};
use crate::game::dstrings::EMPTYSTRING;
use crate::player::p_saveg::{p_save_game_file, SAVESTRINGSIZE};
use crate::sound::{s_set_music_volume, s_set_sfx_volume};
use crate::ui_hud::hu_stuff::{hu_draw_save_string, hu_save_string_active, hu_save_string_key, hu_save_string_slot, hu_start_save_string, hu_write_text, SaveStringKeyResult};
use crate::wad::w_cache_lump_name;
use crate::z_zone::PU_CACHE;

// =============================================================================
// Public API (from m_menu.h)
// =============================================================================

pub static mut DETAIL_LEVEL: i32 = 0;
pub static mut SCREENBLOCKS: i32 = 10;

// =============================================================================
// Menu state (from m_menu.c)
// =============================================================================

const LINEHEIGHT: i32 = 16;
const SKULLXOFF: i32 = -32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum MenuKind {
    Main,
    Episode,
    NewGame,
    Options,
    Sound,
    Load,
    Save,
    ReadThis,
}

static mut CURRENT_MENU: MenuKind = MenuKind::Main;
static mut ITEM_ON: usize = 0;
static mut WHICH_SKULL: usize = 0;
static mut SKULL_ANIM_COUNTER: i32 = 10;
static mut EPI: i32 = 0;
static mut SHOW_MESSAGES: i32 = 1;
static mut MOUSE_SENSITIVITY: i32 = 5;
static mut SCREEN_SIZE: i32 = 7; // screenblocks - 3, range 0-8
static mut SFX_VOLUME: i32 = 8;
static mut MUSIC_VOLUME: i32 = 8;

// Main menu items (M_NGAME, M_OPTION, M_LOADG, M_SAVEG, M_RDTHIS, M_QUITG)
const MAIN_ITEMS: [&str; 6] = ["M_NGAME", "M_OPTION", "M_LOADG", "M_SAVEG", "M_RDTHIS", "M_QUITG"];
const MAIN_NUM_ITEMS: usize = 6;

// Episode menu (M_EPI1..4)
const EPI_ITEMS: [&str; 4] = ["M_EPI1", "M_EPI2", "M_EPI3", "M_EPI4"];
const EPI_NUM_ITEMS: usize = 4;

// New game / skill menu (M_JKILL, M_ROUGH, M_HURT, M_ULTRA, M_NMARE)
const NEWG_ITEMS: [&str; 5] = ["M_JKILL", "M_ROUGH", "M_HURT", "M_ULTRA", "M_NMARE"];
const NEWG_NUM_ITEMS: usize = 5;

// Options menu: status -1=empty, 1=toggle, 2=slider (8 items, indices 4 and 6 are empty)
const OPTIONS_ITEMS: [(&str, i32); 8] = [
    ("M_ENDGAM", 1),
    ("M_MESSG", 1),
    ("M_DETAIL", 1),
    ("M_SCRNSZ", 2),
    ("", -1),
    ("M_MSENS", 2),
    ("", -1),
    ("M_SVOL", 1),
];
const OPTIONS_NUM_ITEMS: usize = 8;
const OPTIONS_X: i32 = 60;
const OPTIONS_Y: i32 = 37;

// Sound menu: M_SFXVOL, M_MUSVOL (status 2, with empty slots)
const SOUND_ITEMS: [(&str, i32); 4] = [
    ("M_SFXVOL", 2),
    ("", -1),
    ("M_MUSVOL", 2),
    ("", -1),
];
const SOUND_NUM_ITEMS: usize = 4;
const SOUND_X: i32 = 80;
const SOUND_Y: i32 = 64;

// Load/Save: 6 slots
const LOAD_SLOTS: usize = 6;
const LOAD_X: i32 = 80;
const LOAD_Y: i32 = 54;

static mut SAVEGAME_STRINGS: [[u8; SAVESTRINGSIZE]; LOAD_SLOTS] =
    [[0; SAVESTRINGSIZE]; LOAD_SLOTS];

// =============================================================================
// Implementation (from m_menu.c)
// =============================================================================

fn slot_is_empty(slot: usize) -> bool {
    if slot >= LOAD_SLOTS {
        return true;
    }
    let s = unsafe { std::str::from_utf8_unchecked(&SAVEGAME_STRINGS[slot]) };
    let trimmed = s.trim_matches('\0').trim();
    trimmed.is_empty() || trimmed.eq_ignore_ascii_case(EMPTYSTRING)
}

fn m_read_save_strings() {
    for i in 0..LOAD_SLOTS {
        let path = p_save_game_file(i as i32);
        if let Ok(data) = std::fs::read(&path) {
            if data.len() >= 24 {
                let desc = &data[..SAVESTRINGSIZE.min(data.len())];
                let s = String::from_utf8_lossy(desc);
                let trimmed = s.trim_matches('\0').trim();
                if !trimmed.is_empty() {
                    let bytes = trimmed.as_bytes();
                    let len = bytes.len().min(SAVESTRINGSIZE);
                    unsafe {
                        SAVEGAME_STRINGS[i][..len].copy_from_slice(&bytes[..len]);
                        for j in len..SAVESTRINGSIZE {
                            SAVEGAME_STRINGS[i][j] = 0;
                        }
                    }
                    continue;
                }
            }
        }
        let empty = EMPTYSTRING.as_bytes();
        let len = empty.len().min(SAVESTRINGSIZE);
        unsafe {
            SAVEGAME_STRINGS[i][..len].copy_from_slice(&empty[..len]);
            for j in len..SAVESTRINGSIZE {
                SAVEGAME_STRINGS[i][j] = 0;
            }
        }
    }
}

fn m_draw_save_load_border(x: i32, y: i32) {
    unsafe {
        let left = w_cache_lump_name(deh_string("M_LSLEFT"), PU_CACHE);
        let mid = w_cache_lump_name(deh_string("M_LSCNTR"), PU_CACHE);
        let right = w_cache_lump_name(deh_string("M_LSRGHT"), PU_CACHE);
        if left.is_null() || mid.is_null() || right.is_null() {
            return;
        }
        let mut xx = x - 8;
        v_draw_patch_direct(xx, y + 7, left as *const _);
        xx = x;
        for _ in 0..24 {
            v_draw_patch_direct(xx, y + 7, mid as *const _);
            xx += 8;
        }
        v_draw_patch_direct(xx, y + 7, right as *const _);
    }
}

fn m_draw_load_menu() {
    unsafe {
        let title = w_cache_lump_name(deh_string("M_LOADG"), PU_CACHE);
        if !title.is_null() {
            v_draw_patch_direct(72, 28, title as *const _);
        }
        m_read_save_strings();
        for i in 0..LOAD_SLOTS {
            m_draw_save_load_border(LOAD_X, LOAD_Y + LINEHEIGHT * (i as i32));
            let s = std::str::from_utf8_unchecked(&SAVEGAME_STRINGS[i])
                .trim_matches('\0');
            let s = if s.is_empty() { EMPTYSTRING } else { s };
            hu_write_text(LOAD_X, LOAD_Y + LINEHEIGHT * (i as i32), s);
        }
        let skull_name = if WHICH_SKULL == 0 { "M_SKULL1" } else { "M_SKULL2" };
        let skull = w_cache_lump_name(deh_string(skull_name), PU_CACHE);
        if !skull.is_null() {
            v_draw_patch_direct(
                LOAD_X + SKULLXOFF,
                LOAD_Y - 5 + (ITEM_ON as i32) * LINEHEIGHT,
                skull as *const _,
            );
        }
    }
}

fn m_draw_save_menu() {
    unsafe {
        let title = w_cache_lump_name(deh_string("M_SAVEG"), PU_CACHE);
        if !title.is_null() {
            v_draw_patch_direct(72, 28, title as *const _);
        }
        m_read_save_strings();
        let editing_slot = hu_save_string_active().then(|| hu_save_string_slot());
        for i in 0..LOAD_SLOTS {
            m_draw_save_load_border(LOAD_X, LOAD_Y + LINEHEIGHT * (i as i32));
            if editing_slot == Some(i) {
                // Slot text drawn by hu_draw_save_string
            } else {
                let s = std::str::from_utf8_unchecked(&SAVEGAME_STRINGS[i])
                    .trim_matches('\0');
                let s = if s.is_empty() { EMPTYSTRING } else { s };
                hu_write_text(LOAD_X, LOAD_Y + LINEHEIGHT * (i as i32), s);
            }
        }
        let skull_name = if WHICH_SKULL == 0 { "M_SKULL1" } else { "M_SKULL2" };
        let skull = w_cache_lump_name(deh_string(skull_name), PU_CACHE);
        if !skull.is_null() {
            v_draw_patch_direct(
                LOAD_X + SKULLXOFF,
                LOAD_Y - 5 + (ITEM_ON as i32) * LINEHEIGHT,
                skull as *const _,
            );
        }
        if hu_save_string_active() {
            hu_draw_save_string();
        }
    }
}

pub fn m_init() {
    crate::ui_hud::hu_stuff::hu_init();
    m_load_defaults();
    m_bind_base_controls();
    m_bind_weapon_controls();
    m_bind_map_controls();
    m_bind_menu_controls();
    unsafe {
        SCREENBLOCKS = m_get_int_variable("screenblocks");
        DETAIL_LEVEL = m_get_int_variable("detailLevel");
        SHOW_MESSAGES = m_get_int_variable("show_messages");
        if SHOW_MESSAGES != 0 {
            SHOW_MESSAGES = 1;
        }
        SCREEN_SIZE = (SCREENBLOCKS - 3).max(0).min(8);
        MOUSE_SENSITIVITY = m_get_int_variable("mouse_sensitivity");
        if MOUSE_SENSITIVITY < 0 || MOUSE_SENSITIVITY > 9 {
            MOUSE_SENSITIVITY = 5;
        }
        SFX_VOLUME = m_get_int_variable("sfx_volume");
        MUSIC_VOLUME = m_get_int_variable("music_volume");
        if SFX_VOLUME < 0 || SFX_VOLUME > 15 {
            SFX_VOLUME = 8;
        }
        if MUSIC_VOLUME < 0 || MUSIC_VOLUME > 15 {
            MUSIC_VOLUME = 8;
        }
        s_set_sfx_volume(SFX_VOLUME * 8);
        s_set_music_volume(MUSIC_VOLUME * 8);
    }
}

/// Handle menu key input. Returns true if event was consumed.
pub fn m_responder(ev: &Event) -> Boolean {
    use crate::game::d_event::EvType;
    use crate::ui_hud::controls::{KEY_MENU_ABORT, KEY_MENU_ACTIVATE};
    if ev.ev_type != EvType::KeyDown {
        return false;
    }
    let key = ev.data2;
    if hu_save_string_active() {
        if let Some(result) = hu_save_string_key(key as u8) {
            match result {
                SaveStringKeyResult::Consumed => return true,
                SaveStringKeyResult::Commit { slot, desc } => {
                    let desc_str: &str = if desc.is_empty() {
                        "Doom Save"
                    } else {
                        &desc
                    };
                    crate::game::d_main::set_savegame_description(desc_str.as_bytes());
                    crate::game::g_game::g_defered_save_game(slot as i32);
                    unsafe {
                        CURRENT_MENU = MenuKind::Main;
                        ITEM_ON = 3;
                        MENUACTIVE = false;
                    }
                    return true;
                }
                SaveStringKeyResult::Cancel => {
                    crate::ui_hud::hu_stuff::hu_cancel_save_string();
                    return true;
                }
            }
        }
    }
    unsafe {
        if MENUACTIVE {
            // Read This: any key closes and returns to main menu
            if CURRENT_MENU == MenuKind::ReadThis {
                m_go_back();
                return true;
            }
            if key == KEY_MENU_ABORT {
                MENUACTIVE = false;
                return true;
            }
            if key == KEY_MENU_UP as i32 {
                m_move_up();
                return true;
            }
            if key == KEY_MENU_DOWN as i32 {
                m_move_down();
                return true;
            }
            if key == KEY_MENU_LEFT as i32 {
                m_slider_left();
                return true;
            }
            if key == KEY_MENU_RIGHT as i32 {
                m_slider_right();
                return true;
            }
            if key == KEY_MENU_CONFIRM as i32 || key == KEY_MENU_FORWARD as i32 {
                m_activate_item();
                return true;
            }
            if key == KEY_MENU_BACK as i32 {
                m_go_back();
                return true;
            }
        } else if key == KEY_MENU_ACTIVATE as i32 {
            m_start_control_panel();
            return true;
        }
    }
    false
}

fn m_item_status(menu: MenuKind, idx: usize) -> i32 {
    match menu {
        MenuKind::Options => OPTIONS_ITEMS.get(idx).map(|(_, s)| *s).unwrap_or(1),
        MenuKind::Sound => SOUND_ITEMS.get(idx).map(|(_, s)| *s).unwrap_or(1),
        _ => 1,
    }
}

fn m_move_up() {
    unsafe {
        let n = m_current_num_items();
        let mut idx = ITEM_ON;
        loop {
            if idx == 0 {
                idx = n - 1;
            } else {
                idx -= 1;
            }
            if m_item_status(CURRENT_MENU, idx) != -1 {
                ITEM_ON = idx;
                break;
            }
        }
    }
}

fn m_move_down() {
    unsafe {
        let n = m_current_num_items();
        let mut idx = ITEM_ON;
        loop {
            idx = (idx + 1) % n;
            if m_item_status(CURRENT_MENU, idx) != -1 {
                ITEM_ON = idx;
                break;
            }
        }
    }
}

fn m_slider_left() {
    unsafe {
        if m_item_status(CURRENT_MENU, ITEM_ON) == 2 {
            match CURRENT_MENU {
                MenuKind::Options => match ITEM_ON {
                    3 => m_size_display(0),
                    5 => m_change_sensitivity(0),
                    _ => {}
                },
                MenuKind::Sound => match ITEM_ON {
                    0 => m_sfx_vol(0),
                    2 => m_music_vol(0),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn m_slider_right() {
    unsafe {
        if m_item_status(CURRENT_MENU, ITEM_ON) == 2 {
            match CURRENT_MENU {
                MenuKind::Options => match ITEM_ON {
                    3 => m_size_display(1),
                    5 => m_change_sensitivity(1),
                    _ => {}
                },
                MenuKind::Sound => match ITEM_ON {
                    0 => m_sfx_vol(1),
                    2 => m_music_vol(1),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn m_sfx_vol(delta: i32) {
    unsafe {
        if delta == 0 && SFX_VOLUME > 0 {
            SFX_VOLUME -= 1;
        } else if delta == 1 && SFX_VOLUME < 15 {
            SFX_VOLUME += 1;
        }
        m_set_variable("sfx_volume", &SFX_VOLUME.to_string());
        s_set_sfx_volume(SFX_VOLUME * 8);
    }
}

fn m_music_vol(delta: i32) {
    unsafe {
        if delta == 0 && MUSIC_VOLUME > 0 {
            MUSIC_VOLUME -= 1;
        } else if delta == 1 && MUSIC_VOLUME < 15 {
            MUSIC_VOLUME += 1;
        }
        m_set_variable("music_volume", &MUSIC_VOLUME.to_string());
        s_set_music_volume(MUSIC_VOLUME * 8);
    }
}

fn m_change_sensitivity(delta: i32) {
    unsafe {
        if delta == 0 && MOUSE_SENSITIVITY > 0 {
            MOUSE_SENSITIVITY -= 1;
        } else if delta == 1 && MOUSE_SENSITIVITY < 9 {
            MOUSE_SENSITIVITY += 1;
        }
        m_set_variable("mouse_sensitivity", &MOUSE_SENSITIVITY.to_string());
    }
}

fn m_change_detail() {
    unsafe {
        DETAIL_LEVEL = 1 - DETAIL_LEVEL;
        m_set_detail_level(DETAIL_LEVEL);
        r_set_view_size(SCREENBLOCKS, DETAIL_LEVEL);
    }
}

fn m_change_messages() {
    unsafe {
        SHOW_MESSAGES = 1 - SHOW_MESSAGES;
        m_set_variable("show_messages", &SHOW_MESSAGES.to_string());
    }
}

fn m_size_display(delta: i32) {
    unsafe {
        if delta == 0 && SCREEN_SIZE > 0 {
            SCREENBLOCKS -= 1;
            SCREEN_SIZE -= 1;
        } else if delta == 1 && SCREEN_SIZE < 8 {
            SCREENBLOCKS += 1;
            SCREEN_SIZE += 1;
        }
        m_set_screenblocks(SCREENBLOCKS);
        r_set_view_size(SCREENBLOCKS, DETAIL_LEVEL);
    }
}

fn m_current_num_items() -> usize {
    unsafe {
        match CURRENT_MENU {
            MenuKind::Main => {
                if GAMEMODE == GameMode::Commercial {
                    5
                } else {
                    MAIN_NUM_ITEMS
                }
            }
            MenuKind::Episode => EPI_NUM_ITEMS,
            MenuKind::NewGame => NEWG_NUM_ITEMS,
            MenuKind::Options => OPTIONS_NUM_ITEMS,
            MenuKind::Sound => SOUND_NUM_ITEMS,
            MenuKind::Load | MenuKind::Save => MAIN_NUM_ITEMS,
            MenuKind::ReadThis => 1, // no selectable items; any key closes
        }
    }
}

fn m_activate_item() {
    unsafe {
        match CURRENT_MENU {
            MenuKind::Main => match ITEM_ON {
                0 => {
                    CURRENT_MENU = if GAMEMODE == GameMode::Commercial {
                        MenuKind::NewGame
                    } else {
                        MenuKind::Episode
                    };
                    ITEM_ON = 0;
                }
                1 => {
                    CURRENT_MENU = MenuKind::Options;
                    ITEM_ON = 0;
                }
                2 => {
                    CURRENT_MENU = MenuKind::Load;
                    ITEM_ON = 0;
                }
                3 => {
                    CURRENT_MENU = MenuKind::Save;
                    ITEM_ON = 0;
                }
                4 => {
                    if GAMEMODE != GameMode::Commercial {
                        CURRENT_MENU = MenuKind::ReadThis;
                    }
                }
                5 => MENUACTIVE = false,
                _ => {}
            },
            MenuKind::Episode => {
                EPI = ITEM_ON as i32;
                CURRENT_MENU = MenuKind::NewGame;
                ITEM_ON = 2; // default to "Hurt me plenty"
            }
            MenuKind::NewGame => {
                use crate::game::d_mode::Skill;
                let skill = match ITEM_ON {
                    0 => Skill::Baby,
                    1 => Skill::Easy,
                    2 => Skill::Medium,
                    3 => Skill::Hard,
                    _ => Skill::Nightmare,
                };
                crate::game::g_game::g_defered_init_new(skill, EPI + 1, 1);
                MENUACTIVE = false;
            }
            MenuKind::Options => match ITEM_ON {
                0 => {} // M_EndGame - stub
                1 => m_change_messages(),
                2 => m_change_detail(),
                3 => m_size_display(1),
                5 => m_change_sensitivity(1),
                7 => {
                    CURRENT_MENU = MenuKind::Sound;
                    ITEM_ON = 0;
                }
                _ => {}
            },
            MenuKind::Sound => {
                if ITEM_ON == 0 {
                    m_sfx_vol(1);
                } else if ITEM_ON == 2 {
                    m_music_vol(1);
                }
            }
            MenuKind::Load => {
                if !slot_is_empty(ITEM_ON) {
                    crate::game::g_game::g_defered_load_game(ITEM_ON as i32);
                    CURRENT_MENU = MenuKind::Main;
                    ITEM_ON = 2;
                    MENUACTIVE = false;
                }
            }
            MenuKind::Save => {
                let slot = ITEM_ON;
                if slot_is_empty(slot) {
                    hu_start_save_string(slot, LOAD_X, LOAD_Y + LINEHEIGHT * (slot as i32));
                } else {
                    let desc = &SAVEGAME_STRINGS[slot];
                    crate::game::d_main::set_savegame_description(desc);
                    crate::game::g_game::g_defered_save_game(slot as i32);
                    CURRENT_MENU = MenuKind::Main;
                    ITEM_ON = 3;
                    MENUACTIVE = false;
                }
            }
            MenuKind::ReadThis => {} // any key handled in m_responder; go_back closes
        }
    }
}

fn m_go_back() {
    unsafe {
        match CURRENT_MENU {
            MenuKind::Main => MENUACTIVE = false,
            MenuKind::Episode | MenuKind::NewGame => {
                CURRENT_MENU = MenuKind::Main;
                ITEM_ON = 0;
            }
            MenuKind::Options => {
                CURRENT_MENU = MenuKind::Main;
                ITEM_ON = 1;
            }
            MenuKind::Sound => {
                CURRENT_MENU = MenuKind::Options;
                ITEM_ON = 7;
            }
            MenuKind::Load => {
                CURRENT_MENU = MenuKind::Main;
                ITEM_ON = 2;
            }
            MenuKind::Save => {
                CURRENT_MENU = MenuKind::Main;
                ITEM_ON = 3;
            }
            MenuKind::ReadThis => {
                CURRENT_MENU = MenuKind::Main;
                ITEM_ON = 4;
            }
        }
    }
}

pub fn m_ticker() {
    unsafe {
        if !MENUACTIVE {
            return;
        }
        SKULL_ANIM_COUNTER -= 1;
        if SKULL_ANIM_COUNTER <= 0 {
            WHICH_SKULL ^= 1;
            SKULL_ANIM_COUNTER = 8;
        }
    }
}

fn m_draw_read_this() {
    unsafe {
        // Draw HELP1 lump full-screen (Read This / message screen)
        let p = w_cache_lump_name(deh_string("HELP1"), PU_CACHE);
        if !p.is_null() {
            v_draw_patch_direct(0, 0, p as *const _);
        }
    }
}

fn m_draw_main_menu() {
    unsafe {
        let p = w_cache_lump_name(deh_string("M_DOOM"), PU_CACHE);
        if !p.is_null() {
            v_draw_patch_direct(94, 2, p as *const _);
        }
    }
}

fn m_draw_episode_menu() {
    unsafe {
        let p = w_cache_lump_name(deh_string("M_EPISOD"), PU_CACHE);
        if !p.is_null() {
            v_draw_patch_direct(54, 38, p as *const _);
        }
    }
}

fn m_draw_newgame_menu() {
    unsafe {
        let p1 = w_cache_lump_name(deh_string("M_NEWG"), PU_CACHE);
        let p2 = w_cache_lump_name(deh_string("M_SKILL"), PU_CACHE);
        if !p1.is_null() {
            v_draw_patch_direct(96, 14, p1 as *const _);
        }
        if !p2.is_null() {
            v_draw_patch_direct(54, 38, p2 as *const _);
        }
    }
}

fn m_draw_thermo(x: i32, y: i32, therm_width: i32, therm_dot: i32) {
    unsafe {
        let left = w_cache_lump_name(deh_string("M_THERML"), PU_CACHE);
        let mid = w_cache_lump_name(deh_string("M_THERMM"), PU_CACHE);
        let right = w_cache_lump_name(deh_string("M_THERMR"), PU_CACHE);
        let dot = w_cache_lump_name(deh_string("M_THERMO"), PU_CACHE);
        if left.is_null() || mid.is_null() || right.is_null() || dot.is_null() {
            return;
        }
        let mut xx = x;
        v_draw_patch_direct(xx, y, left as *const _);
        xx += 8;
        for _ in 0..therm_width {
            v_draw_patch_direct(xx, y, mid as *const _);
            xx += 8;
        }
        v_draw_patch_direct(xx, y, right as *const _);
        let dot_x = (x + 8) + therm_dot * 8;
        v_draw_patch_direct(dot_x, y, dot as *const _);
    }
}

fn m_draw_options_menu() {
    unsafe {
        let title = w_cache_lump_name(deh_string("M_OPTTTL"), PU_CACHE);
        if !title.is_null() {
            v_draw_patch_direct(108, 15, title as *const _);
        }
        let detail_name = if DETAIL_LEVEL == 0 { "M_GDHIGH" } else { "M_GDLOW" };
        let detail_p = w_cache_lump_name(deh_string(detail_name), PU_CACHE);
        if !detail_p.is_null() {
            v_draw_patch_direct(
                OPTIONS_X + 175,
                OPTIONS_Y + LINEHEIGHT * 2,
                detail_p as *const _,
            );
        }
        let msg_name = if SHOW_MESSAGES == 0 { "M_MSGOFF" } else { "M_MSGON" };
        let msg_p = w_cache_lump_name(deh_string(msg_name), PU_CACHE);
        if !msg_p.is_null() {
            v_draw_patch_direct(
                OPTIONS_X + 120,
                OPTIONS_Y + LINEHEIGHT * 1,
                msg_p as *const _,
            );
        }
        m_draw_thermo(OPTIONS_X, OPTIONS_Y + LINEHEIGHT * 4, 9, SCREEN_SIZE);
        m_draw_thermo(OPTIONS_X, OPTIONS_Y + LINEHEIGHT * 6, 10, MOUSE_SENSITIVITY);
    }
}

fn m_draw_sound_menu() {
    unsafe {
        let title = w_cache_lump_name(deh_string("M_SVOL"), PU_CACHE);
        if !title.is_null() {
            v_draw_patch_direct(60, 38, title as *const _);
        }
        m_draw_thermo(SOUND_X, SOUND_Y + LINEHEIGHT * 1, 16, SFX_VOLUME);
        m_draw_thermo(SOUND_X, SOUND_Y + LINEHEIGHT * 3, 16, MUSIC_VOLUME);
    }
}

fn m_draw_sound_items() {
    unsafe {
        let mut yy = SOUND_Y;
        for (i, (name, status)) in SOUND_ITEMS.iter().enumerate() {
            if *status != -1 && !name.is_empty() {
                let p = w_cache_lump_name(deh_string(name), PU_CACHE);
                if !p.is_null() {
                    v_draw_patch_direct(SOUND_X, yy, p as *const _);
                }
            }
            yy += LINEHEIGHT;
        }
        let skull_name = if WHICH_SKULL == 0 { "M_SKULL1" } else { "M_SKULL2" };
        let skull = w_cache_lump_name(deh_string(skull_name), PU_CACHE);
        if !skull.is_null() {
            v_draw_patch_direct(
                SOUND_X + SKULLXOFF,
                SOUND_Y - 5 + (ITEM_ON as i32) * LINEHEIGHT,
                skull as *const _,
            );
        }
    }
}

fn m_draw_options_items() {
    unsafe {
        let mut yy = OPTIONS_Y;
        for (i, (name, status)) in OPTIONS_ITEMS.iter().enumerate() {
            if *status != -1 && !name.is_empty() {
                let p = w_cache_lump_name(deh_string(name), PU_CACHE);
                if !p.is_null() {
                    v_draw_patch_direct(OPTIONS_X, yy, p as *const _);
                }
            }
            yy += LINEHEIGHT;
        }
        let skull_name = if WHICH_SKULL == 0 { "M_SKULL1" } else { "M_SKULL2" };
        let skull = w_cache_lump_name(deh_string(skull_name), PU_CACHE);
        if !skull.is_null() {
            v_draw_patch_direct(
                OPTIONS_X + SKULLXOFF,
                OPTIONS_Y - 5 + (ITEM_ON as i32) * LINEHEIGHT,
                skull as *const _,
            );
        }
    }
}

fn m_draw_menu_items(menu: MenuKind, x: i32, y: i32, num_items: usize, items: &[&str]) {
    unsafe {
        let mut yy = y;
        for i in 0..num_items {
            let name = deh_string(items[i]);
            let p = w_cache_lump_name(name, PU_CACHE);
            if !p.is_null() {
                v_draw_patch_direct(x, yy, p as *const _);
            }
            yy += LINEHEIGHT;
        }
        let skull_name = if WHICH_SKULL == 0 { "M_SKULL1" } else { "M_SKULL2" };
        let skull = w_cache_lump_name(deh_string(skull_name), PU_CACHE);
        if !skull.is_null() {
            v_draw_patch_direct(
                x + SKULLXOFF,
                y - 5 + (ITEM_ON as i32) * LINEHEIGHT,
                skull as *const _,
            );
        }
    }
}

pub fn m_drawer() {
    unsafe {
        if !MENUACTIVE {
            return;
        }
        let (main_num, main_items): (usize, &[&str]) = if GAMEMODE == GameMode::Commercial {
            (5, &["M_NGAME", "M_OPTION", "M_LOADG", "M_SAVEG", "M_QUITG"])
        } else {
            (MAIN_NUM_ITEMS, &MAIN_ITEMS)
        };
        match CURRENT_MENU {
            MenuKind::Main => {
                m_draw_main_menu();
                m_draw_menu_items(MenuKind::Main, 97, 64, main_num, main_items);
            }
            MenuKind::Episode => {
                m_draw_episode_menu();
                m_draw_menu_items(MenuKind::Episode, 48, 63, EPI_NUM_ITEMS, &EPI_ITEMS);
            }
            MenuKind::NewGame => {
                m_draw_newgame_menu();
                m_draw_menu_items(MenuKind::NewGame, 48, 63, NEWG_NUM_ITEMS, &NEWG_ITEMS);
            }
            MenuKind::Options => {
                m_draw_options_menu();
                m_draw_options_items();
            }
            MenuKind::Sound => {
                m_draw_sound_menu();
                m_draw_sound_items();
            }
            MenuKind::Load => {
                m_draw_load_menu();
            }
            MenuKind::Save => {
                m_draw_save_menu();
            }
            MenuKind::ReadThis => {
                m_draw_read_this();
            }
        }
    }
}

/// Force menu up on keypress. Does nothing if menu already active.
pub fn m_start_control_panel() {
    unsafe {
        if MENUACTIVE {
            return;
        }
        MENUACTIVE = true;
    }
}

/// Sync screenblocks to config and optionally trigger view size update.
pub fn m_set_screenblocks(val: i32) {
    unsafe {
        SCREENBLOCKS = val;
    }
    m_set_variable("screenblocks", &val.to_string());
}

/// Sync detail level to config.
pub fn m_set_detail_level(val: i32) {
    unsafe {
        DETAIL_LEVEL = val;
    }
    m_set_variable("detailLevel", &val.to_string());
}
