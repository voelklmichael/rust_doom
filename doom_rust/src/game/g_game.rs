//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Game logic.
//
// Original: g_game.h + g_game.c

use super::d_event::{buttoncode, Event, EvType};
use super::d_main::{GAMEACTION, SAVEGAMEDESCRIPTION, SAVEGAMESLOT};
use super::d_loop::GAMETIC;
use super::d_ticcmd::Ticcmd;
use super::f_finale;
use crate::deh::misc::DEH_DEFAULT_INITIAL_HEALTH;
use crate::doomdef::{Gameaction, Gamestate, MAXPLAYERS};
use crate::doomstat::{
    DISPLAYPLAYER, GAMEEPISODE, GAMEMAP, GAMEMODE, GAMESKILL, GAMESTATE, LEVELSTARTTIC,
    PLAYERINGAME, PLAYERS, Player, PlayerState, RESPAWNMONSTERS, USERGAME, VIEWACTIVE,
};
use crate::game::d_mode::{GameMode, Skill};
use crate::player::{p_saveg, p_setup, p_tick};
use crate::rendering::r_data::r_texture_num_for_name;
use crate::rendering::{SKYFLATNAME, SKYFLATNUM, SKYTEXTURE};
use crate::ui_hud::controls;

/// Deferred new-game parameters. Set by G_DeferedInitNew, read by G_DoNewGame.
/// Original: d_skill, d_episode, d_map in g_game.c
static mut D_SKILL: Skill = Skill::Medium;
static mut D_EPISODE: i32 = 1;
static mut D_MAP: i32 = 1;

/// Key state for G_BuildTiccmd. Set by G_Responder from ev_keydown/ev_keyup.
const NUMKEYS: usize = 256;
static mut GAMEKEYDOWN: [bool; NUMKEYS] = [false; NUMKEYS];

/// Mouse movement (accumulated, consumed by G_BuildTiccmd). Set by G_Responder from ev_mouse.
static mut MOUSEX: i32 = 0;
static mut MOUSEY: i32 = 0;

/// For accelerative turning. Original: turnheld
static mut TURNHELD: i32 = 0;

/// Pause/save requested by menu. Original: sendpause, sendsave
static mut SENDPAUSE: bool = false;
static mut SENDSAVE: bool = false;

/// Movement lookup tables. Original: forwardmove, sidemove, angleturn
const FORWARDMOVE: [i32; 2] = [0x19, 0x32]; // normal, speed
const SIDEMOVE: [i32; 2] = [0x18, 0x28];
const ANGLETURN: [i32; 3] = [640, 1280, 320]; // slow, normal, speed
const SLOWTURNTICS: i32 = 6;
const MAXPLMOVE: i32 = 0x32; // forwardmove[1]

/// Advance game one tic. Processes gameaction, then P_Ticker, ST_Ticker, etc.
/// Original: G_Ticker
pub fn g_ticker() {
    // Process game actions (C: while gameaction != ga_nothing)
    loop {
        let ga = unsafe { GAMEACTION };
        if ga == Gameaction::Nothing {
            break;
        }
        match ga {
            Gameaction::LoadLevel => g_do_load_level(),
            Gameaction::NewGame => g_do_new_game(),
            Gameaction::LoadGame => g_do_load_game(),
            Gameaction::SaveGame => g_do_save_game(),
            Gameaction::PlayDemo | Gameaction::Completed | Gameaction::Victory
            | Gameaction::WorldDone | Gameaction::Screenshot => {
                // Stub for now
                unsafe { GAMEACTION = Gameaction::Nothing };
            }
            Gameaction::Nothing => break,
        }
    }

    // Do player reborns if needed (C: for i in playeringame, if PST_REBORN -> G_DoReborn)
    for i in 0..MAXPLAYERS {
        if unsafe { PLAYERINGAME[i] } && unsafe { PLAYERS[i].playerstate == PlayerState::Reborn } {
            g_do_reborn(i);
        }
    }

    // Check for special buttons in player cmds (C: after netcmds copy)
    for i in 0..MAXPLAYERS {
        if !unsafe { PLAYERINGAME[i] } {
            continue;
        }
        let buttons = unsafe { PLAYERS[i].cmd.buttons as i32 };
        if (buttons & buttoncode::BT_SPECIAL) != 0 {
            match buttons & buttoncode::BT_SPECIALMASK {
                buttoncode::BTS_PAUSE => {
                    unsafe { crate::doomstat::PAUSED = !crate::doomstat::PAUSED };
                }
                buttoncode::BTS_SAVEGAME => {
                    unsafe {
                        SAVEGAMESLOT = (buttons & buttoncode::BTS_SAVEMASK) >> buttoncode::BTS_SAVESHIFT;
                        GAMEACTION = Gameaction::SaveGame;
                    }
                }
                _ => {}
            }
        }
    }

    if unsafe { GAMESTATE == Gamestate::Finale } {
        f_finale::f_ticker();
        return;
    }
    p_tick::p_ticker();
    crate::ui_hud::st_ticker();
    crate::ui_hud::am_ticker();
    crate::ui_hud::hu_ticker();
}

/// Respawn a reborn player. Original: G_DoReborn
fn g_do_reborn(player: usize) {
    g_player_reborn(player);
    // C also calls P_SpawnPlayer - we may need that when p_mobj is ready
}

/// Load a level. Sets sky, gamestate, resets dead players, calls P_SetupLevel.
/// Original: G_DoLoadLevel
fn g_do_load_level() {
    unsafe {
        SKYFLATNUM = crate::rendering::r_data::r_flat_num_for_name(SKYFLATNAME);
        // Vanilla: sky texture set in G_InitNew; G_DoLoadLevel only sets skyflatnum
        LEVELSTARTTIC = GAMETIC;
        GAMESTATE = Gamestate::Level;
        crate::doomstat::WIPEGAMESTATE = Gamestate::Level;
        // Clear input state (C: memset gamekeydown, mousex, mousey, etc.)
        GAMEKEYDOWN = [false; NUMKEYS];
        MOUSEX = 0;
        MOUSEY = 0;
        SENDPAUSE = false;
        SENDSAVE = false;
        crate::doomstat::PAUSED = false;
    }
    for i in 0..MAXPLAYERS {
        if unsafe { PLAYERINGAME[i] } && unsafe { PLAYERS[i].playerstate == PlayerState::Dead } {
            unsafe { PLAYERS[i].playerstate = PlayerState::Reborn };
        }
        unsafe { PLAYERS[i].frags = [0; MAXPLAYERS] };
    }
    let map_name = p_setup::p_map_name_from_episode_map(
        unsafe { GAMEEPISODE },
        unsafe { GAMEMAP },
    );
    if let Err(e) = p_setup::p_load_level(&map_name) {
        crate::i_system::i_error(&format!("p_load_level failed: {}", e));
    }
    unsafe {
        DISPLAYPLAYER = crate::doomstat::CONSOLEPLAYER;
        GAMEACTION = Gameaction::Nothing;
    }
}

/// Start new game. Resets playeringame, calls G_InitNew. Original: G_DoNewGame
fn g_do_new_game() {
    unsafe {
        PLAYERINGAME[1] = false;
        PLAYERINGAME[2] = false;
        PLAYERINGAME[3] = false;
        crate::doomstat::CONSOLEPLAYER = 0;
        g_init_new(D_SKILL, D_EPISODE, D_MAP);
        GAMEACTION = Gameaction::Nothing;
    }
}

/// Initialize new game. Validates episode/map, sets globals, resets players, sets sky, loads level.
/// Original: G_InitNew
fn g_init_new(skill: Skill, episode: i32, map: i32) {
    let (mut skill, mut episode, mut map) = (skill, episode, map);
    if skill > Skill::Nightmare {
        skill = Skill::Nightmare;
    }
    let (gameversion, gamemode) = unsafe { (crate::doomstat::GAMEVERSION, GAMEMODE) };
    if gameversion >= crate::game::d_mode::GameVersion::ExeUltimate {
        if episode == 0 {
            episode = 4;
        }
    } else {
        if episode < 1 {
            episode = 1;
        }
        if episode > 3 {
            episode = 3;
        }
    }
    if episode > 1 && gamemode == GameMode::Shareware {
        episode = 1;
    }
    if map < 1 {
        map = 1;
    }
    if map > 9 && gamemode != GameMode::Commercial {
        map = 9;
    }
    unsafe {
        RESPAWNMONSTERS = skill == Skill::Nightmare;
        for i in 0..MAXPLAYERS {
            PLAYERS[i].playerstate = PlayerState::Reborn;
        }
        USERGAME = true;
        crate::doomstat::PAUSED = false;
        crate::doomstat::DEMOPLAYBACK = false;
        crate::doomstat::AUTOMAPACTIVE = false;
        VIEWACTIVE = true;
        GAMEEPISODE = episode;
        GAMEMAP = map;
        GAMESKILL = skill;
    }
    // Set sky texture (C: skytexturename by episode/map)
    let skytex = if gamemode == GameMode::Commercial {
        if map < 12 {
            "SKY1"
        } else if map < 21 {
            "SKY2"
        } else {
            "SKY3"
        }
    } else {
        match episode {
            2 => "SKY2",
            3 => "SKY3",
            4 => "SKY4",
            _ => "SKY1",
        }
    };
    unsafe {
        SKYTEXTURE = r_texture_num_for_name(crate::deh::deh_string(skytex));
    }
    g_do_load_level();
}

/// Load game from savefile. Original: G_DoLoadGame
fn g_do_load_game() {
    let slot = unsafe { SAVEGAMESLOT };
    let path = p_saveg::p_save_game_file(slot);
    let mut file = match std::fs::File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            unsafe { GAMEACTION = Gameaction::Nothing };
            return;
        }
    };
    if !p_saveg::p_read_save_game_header(&mut file).unwrap_or(false) {
        unsafe { GAMEACTION = Gameaction::Nothing };
        return;
    }
    let saved_leveltime = unsafe { crate::doomstat::LEVELTIME };
    g_init_new(unsafe { GAMESKILL }, unsafe { GAMEEPISODE }, unsafe { GAMEMAP });
    unsafe { crate::doomstat::LEVELTIME = saved_leveltime };
    let _ = p_saveg::p_unarchive_players(&mut file);
    let _ = p_saveg::p_unarchive_world(&mut file);
    let _ = p_saveg::p_unarchive_thinkers(&mut file);
    let _ = p_saveg::p_unarchive_specials(&mut file);
    if !p_saveg::p_read_save_game_eof(&mut file).unwrap_or(false) {
        crate::i_system::i_error("Bad savegame");
    }
    unsafe { GAMEACTION = Gameaction::Nothing };
}

/// Save game to file. Original: G_DoSaveGame
fn g_do_save_game() {
    let slot = unsafe { SAVEGAMESLOT };
    let temp_path = p_saveg::p_temp_save_game_file();
    let save_path = p_saveg::p_save_game_file(slot);
    let mut file = match std::fs::File::create(&temp_path) {
        Ok(f) => f,
        Err(e) => {
            crate::i_system::i_error(&format!("Failed to create savegame: {}", e));
            unreachable!()
        }
    };
    let desc = unsafe { &SAVEGAMEDESCRIPTION };
    if let Err(e) = p_saveg::p_write_save_game_header(&mut file, desc) {
        crate::i_system::i_error(&format!("Failed to write savegame header: {}", e));
    }
    let _ = p_saveg::p_archive_players(&mut file);
    let _ = p_saveg::p_archive_world(&mut file);
    let _ = p_saveg::p_archive_thinkers(&mut file);
    let _ = p_saveg::p_archive_specials(&mut file);
    if let Err(e) = p_saveg::p_write_save_game_eof(&mut file) {
        crate::i_system::i_error(&format!("Failed to write savegame EOF: {}", e));
    }
    drop(file);
    let _ = std::fs::remove_file(&save_path);
    if std::fs::rename(&temp_path, &save_path).is_err() {
        crate::i_system::i_error("Failed to rename savegame file");
    }
    unsafe {
        GAMEACTION = Gameaction::Nothing;
        for i in 0..24 {
            SAVEGAMEDESCRIPTION[i] = 0;
        }
    }
}

/// Reset player for respawn. Original: G_PlayerReborn
pub fn g_player_reborn(player: usize) {
    if player >= crate::doomdef::MAXPLAYERS {
        return;
    }
    unsafe {
        let p = &mut PLAYERS[player];
        let mut pl = Player::default();
        pl.mo = std::ptr::null_mut();
        pl.playerstate = PlayerState::Live;
        pl.viewheight = crate::player::VIEWHEIGHT;
        pl.health = DEH_DEFAULT_INITIAL_HEALTH;
        *p = pl;
    }
}

/// Handle input event. Sets gamekeydown/mouse from input, dispatches to F/AM/ST responder.
/// Original: G_Responder
pub fn g_responder(ev: &Event) -> bool {
    // Update input state from events (C: switch ev->type)
    match ev.ev_type {
        EvType::KeyDown => {
            if ev.data1 == unsafe { controls::KEY_PAUSE } {
                unsafe { SENDPAUSE = true };
            } else if ev.data1 >= 0 && (ev.data1 as usize) < NUMKEYS {
                unsafe { GAMEKEYDOWN[ev.data1 as usize] = true };
            }
            return true;
        }
        EvType::KeyUp => {
            if ev.data1 >= 0 && (ev.data1 as usize) < NUMKEYS {
                unsafe { GAMEKEYDOWN[ev.data1 as usize] = false };
            }
            return false;
        }
        EvType::Mouse => {
            unsafe {
                MOUSEX += ev.data2 * (crate::doomstat::MOUSESENSITIVITY + 5) / 10;
                MOUSEY += ev.data3 * (crate::doomstat::MOUSESENSITIVITY + 5) / 10;
            }
            return true;
        }
        EvType::Joystick => {
            // Stub: no joystick
            return true;
        }
        EvType::Quit => {}
    }

    if unsafe { crate::doomstat::GAMESTATE == Gamestate::Finale } {
        return f_finale::f_responder(ev);
    }
    crate::ui_hud::am_responder(ev) || crate::ui_hud::st_responder(ev) as bool
}

/// Build ticcmd from current input state. Original: G_BuildTiccmd
pub fn g_build_ticcmd(cmd: &mut Ticcmd, maketic: i32) {
    *cmd = Ticcmd::default();

    let (key_right, key_left, key_up, key_down, key_strafe, key_speed, key_strafeleft, key_straferight, key_fire, key_use) = unsafe {
        (controls::KEY_RIGHT, controls::KEY_LEFT, controls::KEY_UP, controls::KEY_DOWN,
         controls::KEY_STRAFE, controls::KEY_SPEED, controls::KEY_STRAFELEFT, controls::KEY_STRAFERIGHT,
         controls::KEY_FIRE, controls::KEY_USE)
    };

    let gamekeydown = |k: i32| -> bool {
        if k >= 0 && (k as usize) < NUMKEYS {
            unsafe { GAMEKEYDOWN[k as usize] }
        } else {
            false
        }
    };

    let strafe = gamekeydown(key_strafe);
    let speed = key_speed >= NUMKEYS as i32 || gamekeydown(key_speed);

    let speed_idx = if speed { 1 } else { 0 };
    let mut forward: i32 = 0;
    let mut side: i32 = 0;

    // Accelerative turning
    let turnheld = if gamekeydown(key_right) || gamekeydown(key_left) {
        unsafe { TURNHELD += 1; TURNHELD }
    } else {
        unsafe { TURNHELD = 0; 0 }
    };
    let tspeed = if turnheld < SLOWTURNTICS { 2 } else { speed_idx };

    if strafe {
        if gamekeydown(key_right) { side += SIDEMOVE[speed_idx]; }
        if gamekeydown(key_left) { side -= SIDEMOVE[speed_idx]; }
    } else {
        if gamekeydown(key_right) { cmd.angleturn -= ANGLETURN[tspeed] as i16; }
        if gamekeydown(key_left) { cmd.angleturn += ANGLETURN[tspeed] as i16; }
    }

    if gamekeydown(key_up) { forward += FORWARDMOVE[speed_idx]; }
    if gamekeydown(key_down) { forward -= FORWARDMOVE[speed_idx]; }
    if gamekeydown(key_strafeleft) { side -= SIDEMOVE[speed_idx]; }
    if gamekeydown(key_straferight) { side += SIDEMOVE[speed_idx]; }

    if gamekeydown(key_fire) { cmd.buttons |= buttoncode::BT_ATTACK as u8; }
    if gamekeydown(key_use) { cmd.buttons |= buttoncode::BT_USE as u8; }

    cmd.chatchar = crate::ui_hud::hu_dequeue_chat_char();

    let (mousex, mousey) = unsafe { (MOUSEX, MOUSEY) };
    forward += mousey;
    if strafe {
        side += mousex * 2;
    } else {
        cmd.angleturn -= (mousex * 0x8) as i16;
    }
    unsafe { MOUSEX = 0; MOUSEY = 0; }

    forward = forward.clamp(-MAXPLMOVE, MAXPLMOVE);
    side = side.clamp(-MAXPLMOVE, MAXPLMOVE);

    cmd.forwardmove = forward as i8;
    cmd.sidemove = side as i8;

    if unsafe { SENDPAUSE } {
        unsafe { SENDPAUSE = false };
        cmd.buttons = (buttoncode::BT_SPECIAL | buttoncode::BTS_PAUSE) as u8;
    }
    if unsafe { SENDSAVE } {
        unsafe {
            SENDSAVE = false;
            cmd.buttons = (buttoncode::BT_SPECIAL | buttoncode::BTS_SAVEGAME | (SAVEGAMESLOT << buttoncode::BTS_SAVESHIFT)) as u8;
        }
    }
}

/// Defer new game start. Used by idclev cheat and menu.
/// Original: G_DeferedInitNew
pub fn g_defered_init_new(skill: Skill, episode: i32, map: i32) {
    unsafe {
        D_SKILL = skill;
        D_EPISODE = episode;
        D_MAP = map;
        GAMEACTION = Gameaction::NewGame;
    }
}

/// Defer load game. Used by menu Load slot selection.
/// Original: G_LoadGame (deferred via gameaction)
pub fn g_defered_load_game(slot: i32) {
    unsafe {
        SAVEGAMESLOT = slot;
        GAMEACTION = Gameaction::LoadGame;
    }
}

/// Defer save game. Used by menu Save slot selection.
/// Original: G_SaveGame (sets sendsave, next tic G_BuildTiccmd adds button, RunTic sets gameaction)
pub fn g_defered_save_game(slot: i32) {
    unsafe {
        SAVEGAMESLOT = slot;
        SENDSAVE = true;
    }
}

/// Defer level load. Used by G_WorldDone (intermission → next level).
/// Caller must set GAMEMAP, GAMEEPISODE before. Original: gameaction = ga_loadlevel
pub fn g_defered_load_level() {
    unsafe {
        GAMEACTION = Gameaction::LoadLevel;
    }
}
