//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Game logic.
//
// Original: g_game.h + g_game.c

use super::d_event::{buttoncode, Event, EvType};
use std::sync::{Mutex, OnceLock};
use super::d_main::with_d_main_state;
use super::d_loop::with_d_loop_state;
use super::d_ticcmd::Ticcmd;
use super::f_finale;
use crate::deh::misc::DEH_DEFAULT_INITIAL_HEALTH;
use crate::doomdef::{Gameaction, Gamestate, MAXPLAYERS, TICRATE};
use crate::doomstat::{with_doomstat_state, Player, PlayerState};
use crate::game::d_mode::{GameMode, Skill};
use crate::player::{p_saveg, p_setup, p_tick};
use crate::rendering::r_data::r_texture_num_for_name;
use crate::rendering::{r_sky, v_screen_shot, SKYFLATNAME};
use crate::ui_hud::controls;
use crate::wad::w_check_num_for_name;

// =============================================================================
// GGameState - thread-safe via OnceLock + Mutex
// =============================================================================

const NUMKEYS: usize = 256;

static G_GAME_STATE: OnceLock<Mutex<GGameState>> = OnceLock::new();

pub struct GGameState {
    pub oldgamestate: Gamestate,
    pub d_skill: Skill,
    pub d_episode: i32,
    pub d_map: i32,
    pub gamekeydown: [bool; NUMKEYS],
    pub mousex: i32,
    pub mousey: i32,
    pub turnheld: i32,
    pub sendpause: bool,
    pub sendsave: bool,
}

fn get_g_game_state() -> &'static Mutex<GGameState> {
    G_GAME_STATE.get_or_init(|| {
        Mutex::new(GGameState {
            oldgamestate: Gamestate::Level,
            d_skill: Skill::Medium,
            d_episode: 1,
            d_map: 1,
            gamekeydown: [false; NUMKEYS],
            mousex: 0,
            mousey: 0,
            turnheld: 0,
            sendpause: false,
            sendsave: false,
        })
    })
}

/// Access GGameState.
pub fn with_g_game_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut GGameState) -> R,
{
    let mut guard = get_g_game_state().lock().unwrap();
    f(&mut guard)
}

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
            Gameaction::PlayDemo => g_do_play_demo(),
            Gameaction::Completed => g_do_completed(),
            Gameaction::Victory => f_finale::f_start_finale(),
            Gameaction::WorldDone => g_do_world_done(),
            Gameaction::Screenshot => {
                v_screen_shot("DOOM%02i.%s");
                unsafe {
                    PLAYERS[crate::doomstat::CONSOLEPLAYER as usize].message =
                        Some(crate::deh::deh_string("screen shot").to_string());
                    GAMEACTION = Gameaction::Nothing;
                }
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

    // Have we just finished displaying an intermission screen?
    with_g_game_state(|st| {
        if st.oldgamestate == Gamestate::Intermission && GAMESTATE != Gamestate::Intermission {
            crate::ui_hud::wi_end();
        }
        st.oldgamestate = GAMESTATE;
    });

    match unsafe { GAMESTATE } {
        Gamestate::Level => {
            p_tick::p_ticker();
            crate::ui_hud::st_ticker();
            crate::ui_hud::am_ticker();
            crate::ui_hud::hu_ticker();
        }
        Gamestate::Intermission => crate::ui_hud::wi_ticker(),
        Gamestate::Finale => f_finale::f_ticker(),
        Gamestate::DemoScreen => super::d_main::d_page_ticker(),
    }
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
        r_sky::with_r_sky_state_mut(|rs| {
            rs.skyflatnum = crate::rendering::r_data::r_flat_num_for_name(SKYFLATNAME);
        });
    }
    let gametic = with_d_loop_state(|st| st.gametic);
    with_doomstat_state(|st| {
        st.levelstarttic = gametic;
        st.gamestate = Gamestate::Level;
        st.wipegamestate = Gamestate::Level;
        st.paused = false;
    });
    with_g_game_state(|st| {
        st.gamekeydown = [false; NUMKEYS];
        st.mousex = 0;
        st.mousey = 0;
        st.sendpause = false;
        st.sendsave = false;
    });
    for i in 0..MAXPLAYERS {
        if with_doomstat_state(|st| st.playeringame[i] && st.players[i].playerstate == PlayerState::Dead) {
            with_doomstat_state(|st| {
                st.players[i].playerstate = PlayerState::Reborn;
                st.players[i].frags = [0; MAXPLAYERS];
            });
        } else if with_doomstat_state(|st| st.playeringame[i]) {
            with_doomstat_state(|st| st.players[i].frags = [0; MAXPLAYERS]);
        }
    }
    let (gameepisode, gamemap, gamemode) =
        with_doomstat_state(|st| (st.gameepisode, st.gamemap, st.gamemode));
    let map_name = p_setup::p_map_name_from_episode_map(gameepisode, gamemap, gamemode);
    if let Err(e) = p_setup::p_load_level(&map_name) {
        crate::i_system::i_error(&format!("p_load_level failed: {}", e));
    }
    with_doomstat_state(|st| st.displayplayer = st.consoleplayer);
    with_d_main_state(|st| st.gameaction = Gameaction::Nothing);
}

/// Start new game. Resets playeringame, calls G_InitNew. Original: G_DoNewGame
fn g_do_new_game() {
    with_doomstat_state(|st| {
        st.playeringame[1] = false;
        st.playeringame[2] = false;
        st.playeringame[3] = false;
        st.consoleplayer = 0;
    });
    g_init_new(
        with_g_game_state(|st| st.d_skill),
        with_g_game_state(|st| st.d_episode),
        with_g_game_state(|st| st.d_map),
    );
    with_d_main_state(|st| st.gameaction = Gameaction::Nothing);
}

/// Initialize new game. Validates episode/map, sets globals, resets players, sets sky, loads level.
/// Original: G_InitNew
fn g_init_new(skill: Skill, episode: i32, map: i32) {
    let (mut skill, mut episode, mut map) = (skill, episode, map);
    if skill > Skill::Nightmare {
        skill = Skill::Nightmare;
    }
    let (gameversion, gamemode) = with_doomstat_state(|st| (st.gameversion, st.gamemode));
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
    with_doomstat_state(|st| {
        st.respawnmonsters = skill == Skill::Nightmare;
        for i in 0..MAXPLAYERS {
            st.players[i].playerstate = PlayerState::Reborn;
        }
        st.usergame = true;
        st.paused = false;
        st.demoplayback = false;
        st.automapactive = false;
        st.viewactive = true;
        st.gameepisode = episode;
        st.gamemap = map;
        st.gameskill = skill;
    });
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
    r_sky::with_r_sky_state_mut(|rs| {
        rs.skytexture = r_texture_num_for_name(crate::deh::deh_string(skytex));
    });
    g_do_load_level();
}

/// Load game from savefile. Original: G_DoLoadGame
fn g_do_load_game() {
    let slot = with_d_main_state(|st| st.savegameslot);
    let path = p_saveg::p_save_game_file(slot);
    let mut file = match std::fs::File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            with_d_main_state(|st| st.gameaction = Gameaction::Nothing);
            return;
        }
    };
    if !p_saveg::p_read_save_game_header(&mut file).unwrap_or(false) {
        with_d_main_state(|st| st.gameaction = Gameaction::Nothing);
        return;
    }
    let saved_leveltime = with_doomstat_state(|st| st.leveltime);
    let (gameskill, gameepisode, gamemap) =
        with_doomstat_state(|st| (st.gameskill, st.gameepisode, st.gamemap));
    g_init_new(gameskill, gameepisode, gamemap);
    with_doomstat_state(|st| st.leveltime = saved_leveltime);
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
    let (slot, desc) = with_d_main_state(|st| (st.savegameslot, st.savegamedescription));
    let temp_path = p_saveg::p_temp_save_game_file();
    let save_path = p_saveg::p_save_game_file(slot);
    let mut file = match std::fs::File::create(&temp_path) {
        Ok(f) => f,
        Err(e) => {
            crate::i_system::i_error(&format!("Failed to create savegame: {}", e));
            unreachable!()
        }
    };
    if let Err(e) = p_saveg::p_write_save_game_header(&mut file, &desc) {
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
    with_d_main_state(|st| {
        st.gameaction = Gameaction::Nothing;
        for i in 0..24 {
            st.savegamedescription[i] = 0;
        }
    });
}

/// Reset player for respawn. Original: G_PlayerReborn
pub fn g_player_reborn(player: usize) {
    if player >= crate::doomdef::MAXPLAYERS {
        return;
    }
    with_doomstat_state(|st| {
        let mut pl = Player::default();
        pl.mo = std::ptr::null_mut();
        pl.playerstate = PlayerState::Live;
        pl.viewheight = crate::player::VIEWHEIGHT;
        pl.health = DEH_DEFAULT_INITIAL_HEALTH;
        st.players[player] = pl;
    });
}

/// Handle input event. Sets gamekeydown/mouse from input, dispatches to F/AM/ST responder.
/// Original: G_Responder
pub fn g_responder(ev: &Event) -> bool {
    // Update input state from events (C: switch ev->type)
    match ev.ev_type {
        EvType::KeyDown => {
            with_g_game_state(|st| {
                if ev.data1 == controls::key_pause() {
                    st.sendpause = true;
                } else if ev.data1 >= 0 && (ev.data1 as usize) < NUMKEYS {
                    st.gamekeydown[ev.data1 as usize] = true;
                }
            });
            return true;
        }
        EvType::KeyUp => {
            with_g_game_state(|st| {
                if ev.data1 >= 0 && (ev.data1 as usize) < NUMKEYS {
                    st.gamekeydown[ev.data1 as usize] = false;
                }
            });
            return false;
        }
        EvType::Mouse => {
            let sens = with_doomstat_state(|st| st.mousesensitivity);
            with_g_game_state(|st| {
                st.mousex += ev.data2 * (sens + 5) / 10;
                st.mousey += ev.data3 * (sens + 5) / 10;
            });
            return true;
        }
        EvType::Joystick => {
            // Stub: no joystick
            return true;
        }
        EvType::Quit => {}
    }

    if with_doomstat_state(|st| st.gamestate == Gamestate::Finale) {
        return f_finale::f_responder(ev);
    }
    crate::ui_hud::am_responder(ev) || crate::ui_hud::st_responder(ev) as bool
}

/// Build ticcmd from current input state. Original: G_BuildTiccmd
pub fn g_build_ticcmd(cmd: &mut Ticcmd, maketic: i32) {
    *cmd = Ticcmd::default();

    let (key_right, key_left, key_up, key_down, key_strafe, key_speed, key_strafeleft, key_straferight, key_fire, key_use) = (
        controls::key_right(),
        controls::key_left(),
        controls::key_up(),
        controls::key_down(),
        controls::key_strafe(),
        controls::key_speed(),
        controls::key_strafeleft(),
        controls::key_straferight(),
        controls::key_fire(),
        controls::key_use(),
    );

    let gamekeydown = |k: i32| -> bool {
        if k >= 0 && (k as usize) < NUMKEYS {
            with_g_game_state(|st| st.gamekeydown[k as usize])
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
    let turnheld = with_g_game_state(|st| {
        if gamekeydown(key_right) || gamekeydown(key_left) {
            st.turnheld += 1;
            st.turnheld
        } else {
            st.turnheld = 0;
            0
        }
    });
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

    let (mousex, mousey) = with_g_game_state(|st| {
        let (mx, my) = (st.mousex, st.mousey);
        st.mousex = 0;
        st.mousey = 0;
        (mx, my)
    });
    forward += mousey;
    if strafe {
        side += mousex * 2;
    } else {
        cmd.angleturn -= (mousex * 0x8) as i16;
    }

    forward = forward.clamp(-MAXPLMOVE, MAXPLMOVE);
    side = side.clamp(-MAXPLMOVE, MAXPLMOVE);

    cmd.forwardmove = forward as i8;
    cmd.sidemove = side as i8;

    if with_g_game_state(|st| std::mem::replace(&mut st.sendpause, false)) {
        cmd.buttons = (buttoncode::BT_SPECIAL | buttoncode::BTS_PAUSE) as u8;
    }
    if with_g_game_state(|st| std::mem::replace(&mut st.sendsave, false)) {
        let slot = with_d_main_state(|st| st.savegameslot);
        cmd.buttons = (buttoncode::BT_SPECIAL
            | buttoncode::BTS_SAVEGAME
            | (slot << buttoncode::BTS_SAVESHIFT)) as u8;
    }
}

/// Defer new game start. Used by idclev cheat and menu.
/// Original: G_DeferedInitNew
pub fn g_defered_init_new(skill: Skill, episode: i32, map: i32) {
    with_g_game_state(|st| {
        st.d_skill = skill;
        st.d_episode = episode;
        st.d_map = map;
    });
    with_d_main_state(|st| st.gameaction = Gameaction::NewGame);
}

/// Defer load game. Used by menu Load slot selection.
/// Original: G_LoadGame (deferred via gameaction)
pub fn g_defered_load_game(slot: i32) {
    with_d_main_state(|st| {
        st.savegameslot = slot;
        st.gameaction = Gameaction::LoadGame;
    });
}

/// Defer save game. Used by menu Save slot selection.
/// Original: G_SaveGame (sets sendsave, next tic G_BuildTiccmd adds button, RunTic sets gameaction)
pub fn g_defered_save_game(slot: i32) {
    with_d_main_state(|st| st.savegameslot = slot);
    with_g_game_state(|st| st.sendsave = true);
}

/// Defer level load. Used by G_WorldDone (intermission → next level).
/// Caller must set GAMEMAP, GAMEEPISODE before. Original: gameaction = ga_loadlevel
pub fn g_defered_load_level() {
    with_d_main_state(|st| st.gameaction = Gameaction::LoadLevel);
}

/// Exit level (normal exit). Sets gameaction to Completed.
/// Original: G_ExitLevel
pub fn g_exit_level() {
    with_doomstat_state(|st| st.secretexit = false);
    with_d_main_state(|st| st.gameaction = Gameaction::Completed);
}

/// Secret exit level. Sets gameaction to Completed, SECRETEXIT = true if map31 exists.
/// Original: G_SecretExitLevel
pub fn g_secret_exit_level() {
    with_doomstat_state(|st| {
        st.secretexit =
            st.gamemode != GameMode::Commercial || w_check_num_for_name("map31") >= 0;
    });
    with_d_main_state(|st| st.gameaction = Gameaction::Completed);
}

/// Initialize player at game start. Original: G_InitPlayer
pub fn g_init_player(player: usize) {
    g_player_reborn(player);
}

/// Called when player completes a level. Clears powers, cards, etc.
/// Original: G_PlayerFinishLevel
pub fn g_player_finish_level(player: usize) {
    if player >= MAXPLAYERS {
        return;
    }
    with_doomstat_state(|st| {
        let p = &mut st.players[player];
        p.powers = [0; crate::doomdef::NUMPOWERS];
        p.cards = [false; crate::doomdef::NUMCARDS];
        if !p.mo.is_null() {
            let mo = p.mo as *mut crate::player::p_mobj::Mobj;
            unsafe { (*mo).flags &= !crate::player::p_mobj::MF_SHADOW };
        }
        p.extralight = 0;
        p.fixedcolormap = 0;
        p.damagecount = 0;
        p.bonuscount = 0;
    });
}

/// Level completed. Fill wminfo, start intermission. Original: G_DoCompleted
fn g_do_completed() {
    with_d_main_state(|st| st.gameaction = Gameaction::Nothing);

    for i in 0..MAXPLAYERS {
        if with_doomstat_state(|st| st.playeringame[i]) {
            g_player_finish_level(i);
        }
    }

    if with_doomstat_state(|st| st.automapactive) {
        crate::ui_hud::am_stop();
    }

    let (victory, wminfo) = with_doomstat_state(|st| {
        let gameversion = st.gameversion;
        let gamemode = st.gamemode;
        let gamemap = st.gamemap;
        let gameepisode = st.gameepisode;
        let secretexit = st.secretexit;
        let consoleplayer = st.consoleplayer;

        // Chex Quest ends after 5 levels
        if gamemode != GameMode::Commercial {
            if gameversion == crate::game::d_mode::GameVersion::ExeChex && gamemap == 5 {
                return (true, None);
            }
            if gamemap == 8 {
                return (true, None);
            }
            if gamemap == 9 {
                for i in 0..MAXPLAYERS {
                    st.players[i].didsecret = true;
                }
            }
        }

        let mut wminfo = st.wminfo.clone();
        wminfo.didsecret = st.players[consoleplayer as usize].didsecret;
        wminfo.epsd = gameepisode - 1;
        wminfo.last = gamemap - 1;

        if gamemode == GameMode::Commercial {
            if secretexit {
                wminfo.next = match gamemap {
                    15 => 30,
                    31 => 31,
                    _ => gamemap - 1,
                };
            } else {
                wminfo.next = match gamemap {
                    31 | 32 => 15,
                    _ => gamemap - 1,
                };
            }
        } else {
            if secretexit {
                wminfo.next = 8;
            } else if gamemap == 9 {
                wminfo.next = match gameepisode {
                    1 => 3,
                    2 => 5,
                    3 => 6,
                    4 => 2,
                    _ => gamemap - 1,
                };
            } else {
                wminfo.next = gamemap - 1;
            }
        }

        wminfo.maxkills = st.totalkills;
        wminfo.maxitems = st.totalitems;
        wminfo.maxsecret = st.totalsecret;
        wminfo.maxfrags = 0;

        if gamemode == GameMode::Commercial {
            wminfo.partime = TICRATE * CPARS[(gamemap - 1) as usize];
        } else if gameepisode < 4 {
            wminfo.partime = TICRATE * PARS[gameepisode as usize][gamemap as usize];
        } else {
            wminfo.partime = TICRATE * CPARS[(gamemap - 1) as usize];
        }

        wminfo.pnum = consoleplayer;

        for i in 0..MAXPLAYERS {
            wminfo.plyr[i].in_game = if st.playeringame[i] { 1 } else { 0 };
            wminfo.plyr[i].kills = st.players[i].killcount;
            wminfo.plyr[i].items = st.players[i].itemcount;
            wminfo.plyr[i].secret = st.players[i].secretcount;
            wminfo.plyr[i].time = st.leveltime;
            wminfo.plyr[i].frags = st.players[i].frags;
        }

        st.gamestate = Gamestate::Intermission;
        st.viewactive = false;
        st.automapactive = false;

        (false, Some(wminfo))
    });

    if victory {
        with_d_main_state(|st| st.gameaction = Gameaction::Victory);
        return;
    }

    if let Some(wminfo) = wminfo {
        crate::game::statdump::stat_copy(&wminfo);
        crate::ui_hud::wi_start(&wminfo);
    }
}

/// Par times: DOOM episodes 1-3 (pars[episode][map]), DOOM II (cpars[map])
const PARS: [[i32; 10]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 30, 75, 120, 90, 165, 180, 180, 30, 165],
    [0, 90, 90, 90, 120, 90, 360, 240, 30, 170],
    [0, 90, 45, 90, 150, 90, 90, 165, 30, 135],
];
const CPARS: [i32; 32] = [
    30, 90, 120, 120, 90, 150, 120, 120, 270, 90,
    210, 150, 150, 150, 210, 150, 420, 150, 210, 150,
    240, 150, 180, 150, 150, 300, 330, 420, 300, 180,
    120, 30,
];

/// Called by WI when intermission ends. Sets gameaction to WorldDone (or Victory for finale maps).
/// Original: G_WorldDone
pub fn g_world_done() {
    with_doomstat_state(|st| {
        if st.secretexit {
            st.players[st.consoleplayer as usize].didsecret = true;
        }
        if st.gamemode == GameMode::Commercial {
            match st.gamemap {
                6 | 11 | 20 | 30 => {
                    f_finale::f_start_finale();
                    return;
                }
                15 | 31 => {
                    if st.secretexit {
                        f_finale::f_start_finale();
                        return;
                    }
                }
                _ => {}
            }
        }
    });
    with_d_main_state(|st| st.gameaction = Gameaction::WorldDone);
}

/// Load next level after intermission. Original: G_DoWorldDone
fn g_do_world_done() {
    with_doomstat_state(|st| {
        st.gamestate = Gamestate::Level;
        st.gamemap = st.wminfo.next + 1;
        st.viewactive = true;
    });
    g_do_load_level();
    with_d_main_state(|st| st.gameaction = Gameaction::Nothing);
}

/// Demo playback. Stub: clears gameaction. Original: G_DoPlayDemo
fn g_do_play_demo() {
    with_d_main_state(|st| st.gameaction = Gameaction::Nothing);
}

/// Request screenshot. Menu calls this; next tick G_Ticker processes it.
/// Original: G_ScreenShot
pub fn g_screen_shot() {
    with_d_main_state(|st| st.gameaction = Gameaction::Screenshot);
}

/// Defer demo playback. Stub: clears gameaction. Original: G_DeferedPlayDemo
pub fn g_defered_play_demo(_name: &str) {
    with_d_main_state(|st| st.gameaction = Gameaction::Nothing);
}

/// Vanilla version code for demo compatibility. Original: G_VanillaVersionCode
pub fn g_vanilla_version_code() -> i32 {
    use crate::game::d_mode::GameVersion;
    match with_doomstat_state(|st| st.gameversion) {
        GameVersion::ExeDoom12 => {
            crate::i_system::i_error("Doom 1.2 does not have a version code!");
            0
        }
        GameVersion::ExeDoom1666 => 106,
        GameVersion::ExeDoom17 => 107,
        GameVersion::ExeDoom18 => 108,
        _ => 109,
    }
}

/// Checksum for network consistency. Original: G_CmdChecksum
pub fn g_cmd_checksum(cmd: &Ticcmd) -> i32 {
    let ptr = cmd as *const Ticcmd as *const i32;
    let len = std::mem::size_of::<Ticcmd>() / 4;
    let mut sum = 0i32;
    for i in 0..len.saturating_sub(1) {
        sum += unsafe { *ptr.add(i) };
    }
    sum
}
