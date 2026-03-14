//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  System specific interface stuff.
//
// Original: d_main.h + d_main.c (stub)

use crate::doomdef::{Gameaction, Gamestate, SCREENHEIGHT};
use crate::doomstat::AUTOMAPACTIVE;
use crate::input::i_endoom::i_endoom_from_wad;
use crate::input::i_video::i_finish_update;
use crate::rendering::state::VIEWHEIGHT;
use super::d_event::d_pop_event;
use super::f_finale;

/// Current game action. Original: gameaction
pub static mut GAMEACTION: Gameaction = Gameaction::Nothing;

/// Save/load slot when GAMEACTION is LoadGame or SaveGame. Original: savegameslot
pub static mut SAVEGAMESLOT: i32 = 0;

/// Save description for SaveGame. Original: savegamestrings[slot] or user input
pub static mut SAVEGAMEDESCRIPTION: [u8; 24] = [0; 24];

/// Set save description before G_SaveGame. Called by menu.
pub fn set_savegame_description(desc: &[u8]) {
    unsafe {
        let len = desc.len().min(24);
        SAVEGAMEDESCRIPTION[..len].copy_from_slice(&desc[..len]);
        for i in len..24 {
            SAVEGAMEDESCRIPTION[i] = 0;
        }
    }
}

/// Read events from all input devices.
/// Original: D_ProcessEvents
pub fn d_process_events() {
    use super::g_game::g_responder;
    while let Some(ev) = d_pop_event() {
        g_responder(&ev);
    }
}

/// Original: D_PageTicker
pub fn d_page_ticker() {
    // Stub
}

/// Original: D_PageDrawer
pub fn d_page_drawer() {
    // Stub
}

/// Original: D_AdvanceDemo
pub fn d_advance_demo() {
    // Stub
}

/// Original: D_DoAdvanceDemo
pub fn d_do_advance_demo() {
    // Stub
}

/// Original: D_StartTitle
pub fn d_start_title() {
    // Stub
}

/// Shutdown cleanup. Call before process exit (e.g. from D_DoomMain).
/// Displays ENDOOM lump if present. Original: I_Endoom in D_DoomMain.
pub fn d_shutdown() {
    i_endoom_from_wad();
}

/// Draw current display based on gamestate. Call from game loop after G_Ticker.
/// Original: D_Display (simplified)
pub fn d_display() {
    unsafe {
        let gamestate = crate::doomstat::GAMESTATE;
        match gamestate {
            Gamestate::Level => {
                let fullscreen = VIEWHEIGHT == SCREENHEIGHT;
                crate::ui_hud::st_drawer(fullscreen, true);
                if !AUTOMAPACTIVE {
                    let player = crate::rendering::view_player_from_console()
                        .unwrap_or_default();
                    crate::rendering::r_render_player_view(&player);
                } else {
                    crate::ui_hud::am_drawer();
                }
                crate::ui_hud::hu_drawer();
            }
            Gamestate::Intermission => {
                crate::ui_hud::wi_drawer();
            }
            Gamestate::Finale => {
                f_finale::f_drawer();
            }
            Gamestate::DemoScreen => {
                d_page_drawer();
            }
        }
        crate::ui_hud::m_drawer();
        i_finish_update();
    }
}
