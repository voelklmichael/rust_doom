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
    key_menu_abort, key_menu_activate, key_menu_back, key_menu_confirm, key_menu_down,
    key_menu_forward, key_menu_left, key_menu_right, key_menu_up,
};
use crate::game::dstrings::EMPTYSTRING;
use crate::player::p_saveg::{p_save_game_file, SAVESTRINGSIZE};
use crate::sound::{s_set_music_volume, s_set_sfx_volume};
use crate::ui_hud::hu_stuff::{hu_draw_save_string, hu_save_string_active, hu_save_string_key, hu_save_string_slot, hu_start_save_string, hu_write_text, SaveStringKeyResult};
use crate::wad::w_cache_lump_name;
use crate::z_zone::PU_CACHE;
use std::sync::{Mutex, OnceLock};

// =============================================================================
// MenuState - thread-safe via OnceLock + Mutex
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

// Load/Save: 6 slots (needed for MenuState)
const LOAD_SLOTS: usize = 6;

static MENU_STATE: OnceLock<Mutex<MenuState>> = OnceLock::new();

pub struct MenuState {
    pub current_menu: MenuKind,
    pub item_on: usize,
    pub which_skull: usize,
    pub skull_anim_counter: i32,
    pub epi: i32,
    pub show_messages: i32,
    pub mouse_sensitivity: i32,
    pub screen_size: i32,
    pub sfx_volume: i32,
    pub music_volume: i32,
    pub savegame_strings: [[u8; SAVESTRINGSIZE]; LOAD_SLOTS],
    pub detail_level: i32,
    pub screenblocks: i32,
}

fn get_menu_state() -> &'static Mutex<MenuState> {
    MENU_STATE.get_or_init(|| {
        Mutex::new(MenuState {
            current_menu: MenuKind::Main,
            item_on: 0,
            which_skull: 0,
            skull_anim_counter: 10,
            epi: 0,
            show_messages: 1,
            mouse_sensitivity: 5,
            screen_size: 7,
            sfx_volume: 8,
            music_volume: 8,
            savegame_strings: [[0; SAVESTRINGSIZE]; LOAD_SLOTS],
            detail_level: 0,
            screenblocks: 10,
        })
    })
}

/// Access MenuState.
pub fn with_menu_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut MenuState) -> R,
{
    let mut guard = get_menu_state().lock().unwrap();
    f(&mut guard)
}

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

const LOAD_X: i32 = 80;
const LOAD_Y: i32 = 54;

// =============================================================================
// Implementation (from m_menu.c)
// =============================================================================

fn slot_is_empty(st: &MenuState, slot: usize) -> bool {
    if slot >= LOAD_SLOTS {
        return true;
    }
    let s = unsafe { std::str::from_utf8_unchecked(&st.savegame_strings[slot]) };
    let trimmed = s.trim_matches('\0').trim();
    trimmed.is_empty() || trimmed.eq_ignore_ascii_case(EMPTYSTRING)
}

fn m_read_save_strings(st: &mut MenuState) {
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
                    st.savegame_strings[i][..len].copy_from_slice(&bytes[..len]);
                    for j in len..SAVESTRINGSIZE {
                        st.savegame_strings[i][j] = 0;
                    }
                    continue;
                }
            }
        }
        let empty = EMPTYSTRING.as_bytes();
        let len = empty.len().min(SAVESTRINGSIZE);
        st.savegame_strings[i][..len].copy_from_slice(&empty[..len]);
        for j in len..SAVESTRINGSIZE {
            st.savegame_strings[i][j] = 0;
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
        v_draw_patch_direct(xx, y + 7, left.as_ptr() as *const _);
        xx = x;
        for _ in 0..24 {
            v_draw_patch_direct(xx, y + 7, mid.as_ptr() as *const _);
            xx += 8;
        }
        v_draw_patch_direct(xx, y + 7, right.as_ptr() as *const _);
    }
}

fn m_draw_load_menu(st: &mut MenuState) {
    let title = w_cache_lump_name(deh_string("M_LOADG"), PU_CACHE);
    if !title.is_null() {
        v_draw_patch_direct(72, 28, title.as_ptr() as *const _);
    }
    m_read_save_strings(st);
    for i in 0..LOAD_SLOTS {
        m_draw_save_load_border(LOAD_X, LOAD_Y + LINEHEIGHT * (i as i32));
        let s = std::str::from_utf8_unchecked(&st.savegame_strings[i]).trim_matches('\0');
        let s = if s.is_empty() { EMPTYSTRING } else { s };
        hu_write_text(LOAD_X, LOAD_Y + LINEHEIGHT * (i as i32), s);
    }
    let skull_name = if st.which_skull == 0 { "M_SKULL1" } else { "M_SKULL2" };
    let skull = w_cache_lump_name(deh_string(skull_name), PU_CACHE);
    if !skull.is_null() {
        v_draw_patch_direct(
            LOAD_X + SKULLXOFF,
            LOAD_Y - 5 + (st.item_on as i32) * LINEHEIGHT,
            skull.as_ptr() as *const _,
        );
    }
}

fn m_draw_save_menu(st: &mut MenuState) {
    let title = w_cache_lump_name(deh_string("M_SAVEG"), PU_CACHE);
    if !title.is_null() {
        v_draw_patch_direct(72, 28, title.as_ptr() as *const _);
    }
    m_read_save_strings(st);
    let editing_slot = hu_save_string_active().then(|| hu_save_string_slot());
    for i in 0..LOAD_SLOTS {
        m_draw_save_load_border(LOAD_X, LOAD_Y + LINEHEIGHT * (i as i32));
        if editing_slot == Some(i) {
            // Slot text drawn by hu_draw_save_string
        } else {
            let s = std::str::from_utf8_unchecked(&st.savegame_strings[i]).trim_matches('\0');
            let s = if s.is_empty() { EMPTYSTRING } else { s };
            hu_write_text(LOAD_X, LOAD_Y + LINEHEIGHT * (i as i32), s);
        }
    }
    let skull_name = if st.which_skull == 0 { "M_SKULL1" } else { "M_SKULL2" };
    let skull = w_cache_lump_name(deh_string(skull_name), PU_CACHE);
    if !skull.is_null() {
        v_draw_patch_direct(
            LOAD_X + SKULLXOFF,
            LOAD_Y - 5 + (st.item_on as i32) * LINEHEIGHT,
            skull.as_ptr() as *const _,
        );
    }
    if hu_save_string_active() {
        hu_draw_save_string();
    }
}

pub fn m_init() {
    crate::ui_hud::hu_stuff::hu_init();
    m_load_defaults();
    m_bind_base_controls();
    m_bind_weapon_controls();
    m_bind_map_controls();
    m_bind_menu_controls();
    with_menu_state(|st| {
        st.screenblocks = m_get_int_variable("screenblocks");
        st.detail_level = m_get_int_variable("detailLevel");
        st.show_messages = m_get_int_variable("show_messages");
        if st.show_messages != 0 {
            st.show_messages = 1;
        }
        st.screen_size = (st.screenblocks - 3).max(0).min(8);
        st.mouse_sensitivity = m_get_int_variable("mouse_sensitivity");
        if st.mouse_sensitivity < 0 || st.mouse_sensitivity > 9 {
            st.mouse_sensitivity = 5;
        }
        st.sfx_volume = m_get_int_variable("sfx_volume");
        st.music_volume = m_get_int_variable("music_volume");
        if st.sfx_volume < 0 || st.sfx_volume > 15 {
            st.sfx_volume = 8;
        }
        if st.music_volume < 0 || st.music_volume > 15 {
            st.music_volume = 8;
        }
        s_set_sfx_volume(st.sfx_volume * 8);
        s_set_music_volume(st.music_volume * 8);
    });
}

/// Handle menu key input. Returns true if event was consumed.
pub fn m_responder(ev: &Event) -> Boolean {
    use crate::game::d_event::EvType;
    if ev.ev_type != EvType::KeyDown {
        return false;
    }
    let key = ev.data2;
    let key_abort = key_menu_abort();
    let key_activate = key_menu_activate();
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
                    with_menu_state(|st| {
                        st.current_menu = MenuKind::Main;
                        st.item_on = 3;
                    });
                    unsafe { MENUACTIVE = false };
                    return true;
                }
                SaveStringKeyResult::Cancel => {
                    crate::ui_hud::hu_stuff::hu_cancel_save_string();
                    return true;
                }
            }
        }
    }
    if unsafe { MENUACTIVE } {
        return with_menu_state(|st| {
            if st.current_menu == MenuKind::ReadThis {
                m_go_back(st);
                return true;
            }
            if key == key_abort {
                unsafe { MENUACTIVE = false };
                return true;
            }
            if key == key_menu_up() {
                m_move_up(st);
                return true;
            }
            if key == key_menu_down() {
                m_move_down(st);
                return true;
            }
            if key == key_menu_left() {
                m_slider_left(st);
                return true;
            }
            if key == key_menu_right() {
                m_slider_right(st);
                return true;
            }
            if key == key_menu_confirm() || key == key_menu_forward() {
                m_activate_item(st);
                return true;
            }
            if key == key_menu_back() {
                m_go_back(st);
                return true;
            }
            false
        });
    } else if key == key_activate {
        m_start_control_panel();
        true
    } else {
        false
    }
}

fn m_item_status(menu: MenuKind, idx: usize) -> i32 {
    match menu {
        MenuKind::Options => OPTIONS_ITEMS.get(idx).map(|(_, s)| *s).unwrap_or(1),
        MenuKind::Sound => SOUND_ITEMS.get(idx).map(|(_, s)| *s).unwrap_or(1),
        _ => 1,
    }
}

fn m_move_up(st: &mut MenuState) {
    let n = m_current_num_items(st);
    let mut idx = st.item_on;
    loop {
        if idx == 0 {
            idx = n - 1;
        } else {
            idx -= 1;
        }
        if m_item_status(st.current_menu, idx) != -1 {
            st.item_on = idx;
            break;
        }
    }
}

fn m_move_down(st: &mut MenuState) {
    let n = m_current_num_items(st);
    let mut idx = st.item_on;
    loop {
        idx = (idx + 1) % n;
        if m_item_status(st.current_menu, idx) != -1 {
            st.item_on = idx;
            break;
        }
    }
}

fn m_slider_left(st: &mut MenuState) {
    if m_item_status(st.current_menu, st.item_on) == 2 {
        match st.current_menu {
            MenuKind::Options => match st.item_on {
                3 => m_size_display(st, 0),
                5 => m_change_sensitivity(st, 0),
                _ => {}
            },
            MenuKind::Sound => match st.item_on {
                0 => m_sfx_vol(st, 0),
                2 => m_music_vol(st, 0),
                _ => {}
            },
            _ => {}
        }
    }
}

fn m_slider_right(st: &mut MenuState) {
    if m_item_status(st.current_menu, st.item_on) == 2 {
        match st.current_menu {
            MenuKind::Options => match st.item_on {
                3 => m_size_display(st, 1),
                5 => m_change_sensitivity(st, 1),
                _ => {}
            },
            MenuKind::Sound => match st.item_on {
                0 => m_sfx_vol(st, 1),
                2 => m_music_vol(st, 1),
                _ => {}
            },
            _ => {}
        }
    }
}

fn m_sfx_vol(st: &mut MenuState, delta: i32) {
    if delta == 0 && st.sfx_volume > 0 {
        st.sfx_volume -= 1;
    } else if delta == 1 && st.sfx_volume < 15 {
        st.sfx_volume += 1;
    }
    m_set_variable("sfx_volume", &st.sfx_volume.to_string());
    s_set_sfx_volume(st.sfx_volume * 8);
}

fn m_music_vol(st: &mut MenuState, delta: i32) {
    if delta == 0 && st.music_volume > 0 {
        st.music_volume -= 1;
    } else if delta == 1 && st.music_volume < 15 {
        st.music_volume += 1;
    }
    m_set_variable("music_volume", &st.music_volume.to_string());
    s_set_music_volume(st.music_volume * 8);
}

fn m_change_sensitivity(st: &mut MenuState, delta: i32) {
    if delta == 0 && st.mouse_sensitivity > 0 {
        st.mouse_sensitivity -= 1;
    } else if delta == 1 && st.mouse_sensitivity < 9 {
        st.mouse_sensitivity += 1;
    }
    m_set_variable("mouse_sensitivity", &st.mouse_sensitivity.to_string());
}

fn m_change_detail() {
    with_menu_state(|st| {
        st.detail_level = 1 - st.detail_level;
        m_set_detail_level(st.detail_level);
        r_set_view_size(st.screenblocks, st.detail_level);
    });
}

fn m_change_messages(st: &mut MenuState) {
    st.show_messages = 1 - st.show_messages;
    m_set_variable("show_messages", &st.show_messages.to_string());
}

fn m_size_display(st: &mut MenuState, delta: i32) {
    if delta == 0 && st.screen_size > 0 {
        st.screenblocks -= 1;
        st.screen_size -= 1;
    } else if delta == 1 && st.screen_size < 8 {
        st.screenblocks += 1;
        st.screen_size += 1;
    }
    m_set_screenblocks(st.screenblocks);
    r_set_view_size(st.screenblocks, st.detail_level);
}

fn m_current_num_items(st: &MenuState) -> usize {
    match st.current_menu {
        MenuKind::Main => {
            if unsafe { GAMEMODE } == GameMode::Commercial {
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

fn m_activate_item(st: &mut MenuState) {
    match st.current_menu {
        MenuKind::Main => match st.item_on {
            0 => {
                st.current_menu = if unsafe { GAMEMODE } == GameMode::Commercial {
                    MenuKind::NewGame
                } else {
                    MenuKind::Episode
                };
                st.item_on = 0;
            }
            1 => {
                st.current_menu = MenuKind::Options;
                st.item_on = 0;
            }
            2 => {
                st.current_menu = MenuKind::Load;
                st.item_on = 0;
            }
            3 => {
                st.current_menu = MenuKind::Save;
                st.item_on = 0;
            }
            4 => {
                if unsafe { GAMEMODE } != GameMode::Commercial {
                    st.current_menu = MenuKind::ReadThis;
                }
            }
            5 => unsafe { MENUACTIVE = false },
            _ => {}
        },
        MenuKind::Episode => {
            st.epi = st.item_on as i32;
            st.current_menu = MenuKind::NewGame;
            st.item_on = 2; // default to "Hurt me plenty"
        }
        MenuKind::NewGame => {
            use crate::game::d_mode::Skill;
            let skill = match st.item_on {
                0 => Skill::Baby,
                1 => Skill::Easy,
                2 => Skill::Medium,
                3 => Skill::Hard,
                _ => Skill::Nightmare,
            };
            crate::game::g_game::g_defered_init_new(skill, st.epi + 1, 1);
            unsafe { MENUACTIVE = false };
        }
        MenuKind::Options => match st.item_on {
            0 => {} // M_EndGame - stub
            1 => m_change_messages(st),
            2 => m_change_detail(),
            3 => m_size_display(st, 1),
            5 => m_change_sensitivity(st, 1),
            7 => {
                st.current_menu = MenuKind::Sound;
                st.item_on = 0;
            }
            _ => {}
        },
        MenuKind::Sound => {
            if st.item_on == 0 {
                m_sfx_vol(st, 1);
            } else if st.item_on == 2 {
                m_music_vol(st, 1);
            }
        }
        MenuKind::Load => {
            if !slot_is_empty(st, st.item_on) {
                crate::game::g_game::g_defered_load_game(st.item_on as i32);
                st.current_menu = MenuKind::Main;
                st.item_on = 2;
                unsafe { MENUACTIVE = false };
            }
        }
        MenuKind::Save => {
            let slot = st.item_on;
            if slot_is_empty(st, slot) {
                hu_start_save_string(slot, LOAD_X, LOAD_Y + LINEHEIGHT * (slot as i32));
            } else {
                let desc = &st.savegame_strings[slot];
                crate::game::d_main::set_savegame_description(desc);
                crate::game::g_game::g_defered_save_game(slot as i32);
                st.current_menu = MenuKind::Main;
                st.item_on = 3;
                unsafe { MENUACTIVE = false };
            }
        }
        MenuKind::ReadThis => {} // any key handled in m_responder; go_back closes
    }
}

fn m_go_back(st: &mut MenuState) {
    match st.current_menu {
        MenuKind::Main => unsafe { MENUACTIVE = false },
        MenuKind::Episode | MenuKind::NewGame => {
            st.current_menu = MenuKind::Main;
            st.item_on = 0;
        }
        MenuKind::Options => {
            st.current_menu = MenuKind::Main;
            st.item_on = 1;
        }
        MenuKind::Sound => {
            st.current_menu = MenuKind::Options;
            st.item_on = 7;
        }
        MenuKind::Load => {
            st.current_menu = MenuKind::Main;
            st.item_on = 2;
        }
        MenuKind::Save => {
            st.current_menu = MenuKind::Main;
            st.item_on = 3;
        }
        MenuKind::ReadThis => {
            st.current_menu = MenuKind::Main;
            st.item_on = 4;
        }
    }
}

pub fn m_ticker() {
    if !unsafe { MENUACTIVE } {
        return;
    }
    with_menu_state(|st| {
        st.skull_anim_counter -= 1;
        if st.skull_anim_counter <= 0 {
            st.which_skull ^= 1;
            st.skull_anim_counter = 8;
        }
    });
}

fn m_draw_read_this() {
    unsafe {
        // Draw HELP1 lump full-screen (Read This / message screen)
        let p = w_cache_lump_name(deh_string("HELP1"), PU_CACHE);
        if !p.is_null() {
            v_draw_patch_direct(0, 0, p.as_ptr() as *const _);
        }
    }
}

fn m_draw_main_menu() {
    unsafe {
        let p = w_cache_lump_name(deh_string("M_DOOM"), PU_CACHE);
        if !p.is_null() {
            v_draw_patch_direct(94, 2, p.as_ptr() as *const _);
        }
    }
}

fn m_draw_episode_menu() {
    unsafe {
        let p = w_cache_lump_name(deh_string("M_EPISOD"), PU_CACHE);
        if !p.is_null() {
            v_draw_patch_direct(54, 38, p.as_ptr() as *const _);
        }
    }
}

fn m_draw_newgame_menu() {
    unsafe {
        let p1 = w_cache_lump_name(deh_string("M_NEWG"), PU_CACHE);
        let p2 = w_cache_lump_name(deh_string("M_SKILL"), PU_CACHE);
        if !p1.is_null() {
            v_draw_patch_direct(96, 14, p1.as_ptr() as *const _);
        }
        if !p2.is_null() {
            v_draw_patch_direct(54, 38, p2.as_ptr() as *const _);
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
        v_draw_patch_direct(xx, y, left.as_ptr() as *const _);
        xx += 8;
        for _ in 0..therm_width {
            v_draw_patch_direct(xx, y, mid.as_ptr() as *const _);
            xx += 8;
        }
        v_draw_patch_direct(xx, y, right.as_ptr() as *const _);
        let dot_x = (x + 8) + therm_dot * 8;
        v_draw_patch_direct(dot_x, y, dot.as_ptr() as *const _);
    }
}

fn m_draw_options_menu(st: &MenuState) {
    let title = w_cache_lump_name(deh_string("M_OPTTTL"), PU_CACHE);
    if !title.is_null() {
        v_draw_patch_direct(108, 15, title.as_ptr() as *const _);
    }
    let detail_name = if st.detail_level == 0 {
        "M_GDHIGH"
    } else {
        "M_GDLOW"
    };
    let detail_p = w_cache_lump_name(deh_string(detail_name), PU_CACHE);
    if !detail_p.is_null() {
        v_draw_patch_direct(
            OPTIONS_X + 175,
            OPTIONS_Y + LINEHEIGHT * 2,
            detail_p.as_ptr() as *const _,
        );
    }
    let msg_name = if st.show_messages == 0 {
        "M_MSGOFF"
    } else {
        "M_MSGON"
    };
    let msg_p = w_cache_lump_name(deh_string(msg_name), PU_CACHE);
    if !msg_p.is_null() {
        v_draw_patch_direct(
            OPTIONS_X + 120,
            OPTIONS_Y + LINEHEIGHT * 1,
            msg_p.as_ptr() as *const _,
        );
    }
    m_draw_thermo(OPTIONS_X, OPTIONS_Y + LINEHEIGHT * 4, 9, st.screen_size);
    m_draw_thermo(OPTIONS_X, OPTIONS_Y + LINEHEIGHT * 6, 10, st.mouse_sensitivity);
}

fn m_draw_sound_menu(st: &MenuState) {
    let title = w_cache_lump_name(deh_string("M_SVOL"), PU_CACHE);
    if !title.is_null() {
        v_draw_patch_direct(60, 38, title.as_ptr() as *const _);
    }
    m_draw_thermo(SOUND_X, SOUND_Y + LINEHEIGHT * 1, 16, st.sfx_volume);
    m_draw_thermo(SOUND_X, SOUND_Y + LINEHEIGHT * 3, 16, st.music_volume);
}

fn m_draw_sound_items(st: &MenuState) {
    let mut yy = SOUND_Y;
    for (name, status) in SOUND_ITEMS.iter() {
        if *status != -1 && !name.is_empty() {
            let p = w_cache_lump_name(deh_string(name), PU_CACHE);
            if !p.is_null() {
                v_draw_patch_direct(SOUND_X, yy, p.as_ptr() as *const _);
            }
        }
        yy += LINEHEIGHT;
    }
    let skull_name = if st.which_skull == 0 { "M_SKULL1" } else { "M_SKULL2" };
    let skull = w_cache_lump_name(deh_string(skull_name), PU_CACHE);
    if !skull.is_null() {
        v_draw_patch_direct(
            SOUND_X + SKULLXOFF,
            SOUND_Y - 5 + (st.item_on as i32) * LINEHEIGHT,
            skull.as_ptr() as *const _,
        );
    }
}

fn m_draw_options_items(st: &MenuState) {
    let mut yy = OPTIONS_Y;
    for (name, status) in OPTIONS_ITEMS.iter() {
        if *status != -1 && !name.is_empty() {
            let p = w_cache_lump_name(deh_string(name), PU_CACHE);
            if !p.is_null() {
                v_draw_patch_direct(OPTIONS_X, yy, p.as_ptr() as *const _);
            }
        }
        yy += LINEHEIGHT;
    }
    let skull_name = if st.which_skull == 0 { "M_SKULL1" } else { "M_SKULL2" };
    let skull = w_cache_lump_name(deh_string(skull_name), PU_CACHE);
    if !skull.is_null() {
        v_draw_patch_direct(
            OPTIONS_X + SKULLXOFF,
            OPTIONS_Y - 5 + (st.item_on as i32) * LINEHEIGHT,
            skull.as_ptr() as *const _,
        );
    }
}

fn m_draw_menu_items(st: &MenuState, _menu: MenuKind, x: i32, y: i32, num_items: usize, items: &[&str]) {
    let mut yy = y;
    for i in 0..num_items {
        let name = deh_string(items[i]);
        let p = w_cache_lump_name(name, PU_CACHE);
        if !p.is_null() {
            v_draw_patch_direct(x, yy, p.as_ptr() as *const _);
        }
        yy += LINEHEIGHT;
    }
    let skull_name = if st.which_skull == 0 { "M_SKULL1" } else { "M_SKULL2" };
    let skull = w_cache_lump_name(deh_string(skull_name), PU_CACHE);
    if !skull.is_null() {
        v_draw_patch_direct(
            x + SKULLXOFF,
            y - 5 + (st.item_on as i32) * LINEHEIGHT,
            skull.as_ptr() as *const _,
        );
    }
}

pub fn m_drawer() {
    if !unsafe { MENUACTIVE } {
        return;
    }
    with_menu_state(|st| {
        let (main_num, main_items): (usize, &[&str]) =
            if unsafe { GAMEMODE } == GameMode::Commercial {
                (5, &["M_NGAME", "M_OPTION", "M_LOADG", "M_SAVEG", "M_QUITG"])
            } else {
                (MAIN_NUM_ITEMS, &MAIN_ITEMS)
            };
        match st.current_menu {
            MenuKind::Main => {
                m_draw_main_menu();
                m_draw_menu_items(st, MenuKind::Main, 97, 64, main_num, main_items);
            }
            MenuKind::Episode => {
                m_draw_episode_menu();
                m_draw_menu_items(st, MenuKind::Episode, 48, 63, EPI_NUM_ITEMS, &EPI_ITEMS);
            }
            MenuKind::NewGame => {
                m_draw_newgame_menu();
                m_draw_menu_items(st, MenuKind::NewGame, 48, 63, NEWG_NUM_ITEMS, &NEWG_ITEMS);
            }
            MenuKind::Options => {
                m_draw_options_menu(st);
                m_draw_options_items(st);
            }
            MenuKind::Sound => {
                m_draw_sound_menu(st);
                m_draw_sound_items(st);
            }
            MenuKind::Load => m_draw_load_menu(st),
            MenuKind::Save => m_draw_save_menu(st),
            MenuKind::ReadThis => m_draw_read_this(),
        }
    });
}

/// Force menu up on keypress. Does nothing if menu already active.
pub fn m_start_control_panel() {
    if unsafe { MENUACTIVE } {
        return;
    }
    unsafe { MENUACTIVE = true };
}

/// Sync screenblocks to config and optionally trigger view size update.
pub fn m_set_screenblocks(val: i32) {
    with_menu_state(|st| st.screenblocks = val);
    m_set_variable("screenblocks", &val.to_string());
}

/// Sync detail level to config.
pub fn m_set_detail_level(val: i32) {
    with_menu_state(|st| st.detail_level = val);
    m_set_variable("detailLevel", &val.to_string());
}
