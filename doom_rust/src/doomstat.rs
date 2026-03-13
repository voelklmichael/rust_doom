//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//   All the global variables that store the internal state.
//
// Original: doomstat.h + doomstat.c

use std::sync::atomic::AtomicI32;

use crate::game::d_mode::{GameMission, GameMode, GameVersion, Skill};
use crate::doomdata::MapThing;
use crate::doomdef::{Gamestate, MAXPLAYERS};
use crate::doomtype::Boolean;
use crate::game::d_ticcmd::Ticcmd;

// Stub for player_t - full definition in p_mobj/d_player when ported.
#[derive(Debug, Clone, Copy, Default)]
pub struct Player;

// Stub for wbstartstruct_t - intermission parameters.
#[derive(Debug, Clone, Copy, Default)]
pub struct WbStartStruct;

// Command line parameters.
pub static NOMONSTERS: AtomicI32 = AtomicI32::new(0);
pub static RESPAWNPARM: AtomicI32 = AtomicI32::new(0);
pub static FASTPARM: AtomicI32 = AtomicI32::new(0);
pub static DEVPARM: AtomicI32 = AtomicI32::new(0);

// Game Mode - identify IWAD as shareware, retail etc.
pub static mut GAMEMODE: GameMode = GameMode::Indetermined;
pub static mut GAMEMISSION: GameMission = GameMission::Doom;
pub static mut GAMEVERSION: GameVersion = GameVersion::ExeFinal2;
pub static mut GAMEDESCRIPTION: Option<String> = None;

// If true, we're using one of the mangled BFG edition IWADs.
pub static mut BFGEDITION: Boolean = false;

// Set if homebrew PWAD stuff has been added.
pub static mut MODIFIEDGAME: Boolean = false;

// Selected skill type, map etc.
pub static mut STARTSKILL: Skill = Skill::Medium;
pub static mut STARTEPISODE: i32 = 1;
pub static mut STARTMAP: i32 = 1;
pub static mut STARTLOADGAME: i32 = -1;
pub static mut AUTOSTART: Boolean = false;
pub static mut GAMESKILL: Skill = Skill::Medium;
pub static mut GAMEEPISODE: i32 = 1;
pub static mut GAMEMAP: i32 = 1;
pub static mut TIMELIMIT: i32 = 0;
pub static mut RESPAWNMONSTERS: Boolean = false;
pub static mut NETGAME: Boolean = false;
pub static mut DEATHMATCH: i32 = 0;

// Sound
pub static mut SFXVOLUME: i32 = 8;
pub static mut MUSICVOLUME: i32 = 8;
pub static mut SND_MUSICDEVICE: i32 = 0;
pub static mut SND_SFXDEVICE: i32 = 0;
pub static mut SND_DESIREDMUSICDEVICE: i32 = 0;
pub static mut SND_DESIREDSFXDEVICE: i32 = 0;

// Status flags for refresh
pub static mut STATUSBARACTIVE: Boolean = true;
pub static mut AUTOMAPACTIVE: Boolean = false;
pub static mut MENUACTIVE: Boolean = false;
pub static mut PAUSED: Boolean = false;
pub static mut VIEWACTIVE: Boolean = true;
pub static mut NODRAWERS: Boolean = false;
pub static mut TESTCONTROLS: Boolean = false;
pub static mut TESTCONTROLS_MOUSESPEED: i32 = 0;
pub static mut VIEWANGLEOFFSET: i32 = 0;
pub static mut CONSOLEPLAYER: i32 = 0;
pub static mut DISPLAYPLAYER: i32 = 0;

// Scores, rating
pub static mut TOTALKILLS: i32 = 0;
pub static mut TOTALITEMS: i32 = 0;
pub static mut TOTALSECRET: i32 = 0;
pub static mut LEVELSTARTTIC: i32 = 0;
pub static mut LEVELTIME: i32 = 0;

// Demo playback/recording
pub static mut USERGAME: Boolean = true;
pub static mut DEMOPLAYBACK: Boolean = false;
pub static mut DEMORECORDING: Boolean = false;
pub static mut LOWRES_TURN: Boolean = false;
pub static mut SINGLEDEMO: Boolean = false;
pub static mut GAMESTATE: Gamestate = Gamestate::Level;

// Internal parameters
pub const MAX_DM_STARTS: usize = 10;

pub static mut PLAYERS: [Player; MAXPLAYERS] = [Player; MAXPLAYERS];
pub static mut PLAYERINGAME: [Boolean; MAXPLAYERS] = [false; MAXPLAYERS];
pub static mut DEATHMATCHSTARTS: [MapThing; MAX_DM_STARTS] =
    [MapThing { x: 0, y: 0, angle: 0, type_: 0, options: 0 }; MAX_DM_STARTS];
pub static mut DEATHMATCH_P: *const MapThing = std::ptr::null();
pub static mut PLAYERSTARTS: [MapThing; MAXPLAYERS] =
    [MapThing { x: 0, y: 0, angle: 0, type_: 0, options: 0 }; MAXPLAYERS];
pub static mut WMINFO: WbStartStruct = WbStartStruct;

pub static mut SAVEGAMEDIR: Option<String> = None;
pub static mut BASEDEFAULT: [u8; 1024] = [0; 1024];
pub static mut PRECACHE: Boolean = false;
pub static mut WIPEGAMESTATE: Gamestate = Gamestate::Level;
pub static mut MOUSESENSITIVITY: i32 = 5;
pub static mut BODYQUESLOT: i32 = 0;
pub static mut SKYFLATNUM: i32 = 0;
// rndindex defined in m_random
pub static mut NETCMDS: *mut Ticcmd = std::ptr::null_mut();

/// logical_gamemission: pack_chex -> doom, pack_hacx -> doom2, else gamemission
#[inline]
pub fn logical_gamemission() -> GameMission {
    unsafe {
        match GAMEMISSION {
            GameMission::PackChex => GameMission::Doom,
            GameMission::PackHacx => GameMission::Doom2,
            m => m,
        }
    }
}
