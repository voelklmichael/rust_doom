//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  System specific interface stuff.
//
// Original: d_main.h + d_main.c (stub)

use crate::doomdef::{Gameaction, Gamestate, MAXPLAYERS, SCREENHEIGHT};
use crate::doomstat::{AUTOMAPACTIVE, CONSOLEPLAYER, GAMEMISSION, GAMEMODE, GAMEVERSION, PLAYERINGAME, PLAYERS};
use crate::game::d_iwad;
use crate::game::d_mode::{GameMission, GameMode, GameVersion, Skill};
use crate::input::i_endoom::i_endoom_from_wad;
use crate::input::i_video::i_finish_update;
use crate::m_argv;
use crate::rendering::state::VIEWHEIGHT;
use crate::rendering::{r_init, v_init, v_restore_buffer};
use crate::ui_hud::config::{m_load_defaults, m_set_config_dir, m_set_config_filenames};
use crate::ui_hud::controls::m_bind_base_controls;
use crate::ui_hud::{m_init, m_ticker, st_init};
use crate::wad::{w_add_file, w_check_correct_iwad, w_parse_command_line};
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
    use crate::ui_hud::m_responder;
    while let Some(ev) = d_pop_event() {
        if m_responder(&ev) {
            continue; // menu ate the event
        }
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
/// Simplified: start game at E1M1/MAP01 (full demo sequence would use D_AdvanceDemo).
pub fn d_start_title() {
    super::g_game::g_defered_init_new(Skill::Medium, 1, 1);
}

/// Set GAMEMISSION, GAMEMODE, GAMEVERSION from IWAD. Called after loading IWAD.
/// Original: D_IdentifyVersion + InitGameVersion
fn d_identify_version(mission: GameMission) {
    unsafe {
        GAMEMISSION = mission;
        GAMEMODE = match mission {
            GameMission::Doom2 | GameMission::PackTnt | GameMission::PackPlut | GameMission::PackHacx => {
                GameMode::Commercial
            }
            GameMission::Doom => GameMode::Retail,
            GameMission::PackChex => GameMode::Shareware,
            _ => GameMode::Indetermined,
        };
        GAMEVERSION = match mission {
            GameMission::Doom2 | GameMission::PackTnt | GameMission::PackPlut => GameVersion::ExeFinal2,
            GameMission::Doom | GameMission::PackChex => GameVersion::ExeUltimate,
            _ => GameVersion::ExeFinal2,
        };
    }
}

/// Full startup: config, WAD, video, rendering, menu, then start game or title.
/// Original: D_DoomMain
pub fn d_doom_main() {
    // E.1: Zone allocator (caller must have done m_argv_init first)
    crate::z_zone::z_init();

    // E.2: Config
    m_set_config_dir(".");
    m_set_config_filenames("default.cfg", "doom.cfg");
    m_bind_base_controls();
    m_load_defaults();

    // E.3: Find and load IWAD
    let (iwad_path, mission) = match d_iwad::d_find_iwad() {
        Some(p) => p,
        None => {
            crate::i_system::i_error(
                "Game mode indeterminate. No IWAD file was found. Try\n\
                 specifying one with the '-iwad' or '-file' command line parameter.\n",
            );
            unreachable!()
        }
    };

    w_add_file(&iwad_path).expect("Failed to load IWAD");
    w_check_correct_iwad(GameMission::Doom); // doomgeneric supports Doom only
    d_identify_version(mission);

    // E.5: PWADs from -file
    w_parse_command_line();

    // E.6: Video and rendering
    v_init();
    r_init();

    // E.7: Menu, status bar, etc.
    m_init();
    st_init();

    // E.8: Start game or title
    let autostart = m_argv::m_check_parm("-autostart") != 0;
    if autostart {
        let skill = Skill::Medium;
        let episode = 1;
        let map = 1;
        super::g_game::g_defered_init_new(skill, episode, map);
    } else {
        d_start_title();
    }

    // Register default loop interface (single-player)
    d_register_default_loop();

    // One-time setup for game loop
    v_restore_buffer();
    super::d_loop::d_start_game_loop();
}

fn build_ticcmd_wrapper(cmd: *mut super::d_ticcmd::Ticcmd, maketic: i32) {
    if !cmd.is_null() {
        super::g_game::g_build_ticcmd(unsafe { &mut *cmd }, maketic);
    }
}

/// Register the default single-player loop interface.
fn d_register_default_loop() {
    use super::d_loop::{d_register_loop_callbacks, LoopInterface};
    use super::d_ticcmd::Ticcmd;
    use super::g_game::g_ticker;

    fn run_tic(cmds: *const Ticcmd, _ingame: *const bool) {
        unsafe {
            let cp = CONSOLEPLAYER as usize;
            if cp < MAXPLAYERS && !cmds.is_null() {
                PLAYERS[cp].cmd = *cmds;
            }
            for i in 0..MAXPLAYERS {
                PLAYERINGAME[i] = i == cp;
            }
        }
        g_ticker();
    }

    static LOOP: LoopInterface = LoopInterface {
        process_events: d_process_events,
        build_ticcmd: build_ticcmd_wrapper,
        run_tic,
        run_menu: m_ticker,
    };
    d_register_loop_callbacks(&LOOP);
}

/// One frame: process events, run tics, display. Call from platform loop.
/// Original: doomgeneric_Tick
pub fn d_doom_tick() {
    super::d_loop::try_run_tics();
    d_display();
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
