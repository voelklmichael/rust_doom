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
use crate::ui_hud::controls::{
    m_bind_base_controls, m_bind_map_controls, m_bind_menu_controls, m_bind_weapon_controls,
    KEY_MENU_BACK, KEY_MENU_CONFIRM, KEY_MENU_DOWN, KEY_MENU_FORWARD, KEY_MENU_UP,
};
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
    Load,
    Save,
}

static mut CURRENT_MENU: MenuKind = MenuKind::Main;
static mut ITEM_ON: usize = 0;
static mut WHICH_SKULL: usize = 0;
static mut SKULL_ANIM_COUNTER: i32 = 10;
static mut EPI: i32 = 0;

// Main menu items (M_NGAME, M_OPTION, M_LOADG, M_SAVEG, M_RDTHIS, M_QUITG)
const MAIN_ITEMS: [&str; 6] = ["M_NGAME", "M_OPTION", "M_LOADG", "M_SAVEG", "M_RDTHIS", "M_QUITG"];
const MAIN_NUM_ITEMS: usize = 6;

// Episode menu (M_EPI1..4)
const EPI_ITEMS: [&str; 4] = ["M_EPI1", "M_EPI2", "M_EPI3", "M_EPI4"];
const EPI_NUM_ITEMS: usize = 4;

// New game / skill menu (M_JKILL, M_ROUGH, M_HURT, M_ULTRA, M_NMARE)
const NEWG_ITEMS: [&str; 5] = ["M_JKILL", "M_ROUGH", "M_HURT", "M_ULTRA", "M_NMARE"];
const NEWG_NUM_ITEMS: usize = 5;

// =============================================================================
// Implementation (from m_menu.c)
// =============================================================================

pub fn m_init() {
    m_load_defaults();
    m_bind_base_controls();
    m_bind_weapon_controls();
    m_bind_map_controls();
    m_bind_menu_controls();
    unsafe {
        SCREENBLOCKS = m_get_int_variable("screenblocks");
        DETAIL_LEVEL = m_get_int_variable("detailLevel");
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
    unsafe {
        if MENUACTIVE {
            if key == KEY_MENU_ABORT {
                MENUACTIVE = false;
                return true;
            }
            if key == KEY_MENU_UP as i32 {
                if ITEM_ON > 0 {
                    ITEM_ON -= 1;
                } else {
                    ITEM_ON = m_current_num_items() - 1;
                }
                return true;
            }
            if key == KEY_MENU_DOWN as i32 {
                let n = m_current_num_items();
                ITEM_ON = (ITEM_ON + 1) % n;
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
            MenuKind::Options | MenuKind::Load | MenuKind::Save => MAIN_NUM_ITEMS,
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
                        // Read This - stub
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
            MenuKind::Options | MenuKind::Load | MenuKind::Save => {
                CURRENT_MENU = MenuKind::Main;
                ITEM_ON = 0;
            }
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
            MenuKind::Options | MenuKind::Load | MenuKind::Save => {
                CURRENT_MENU = MenuKind::Main;
                ITEM_ON = 0;
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
            MenuKind::Options | MenuKind::Load | MenuKind::Save => {
                m_draw_main_menu();
                m_draw_menu_items(MenuKind::Main, 97, 64, main_num, main_items);
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
