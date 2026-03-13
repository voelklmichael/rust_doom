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
use crate::doomdef::{Gamestate, MAXPLAYERS, NUMAMMO, NUMCARDS, NUMWEAPONS, Weapontype};
use crate::doomtype::Boolean;
use crate::game::d_ticcmd::Ticcmd;
use crate::m_fixed::Fixed;

/// Cheat flags (cheat_t in d_player.h).
pub const CF_NOCLIP: i32 = 1;
pub const CF_GODMODE: i32 = 2;
pub const CF_NOMOMENTUM: i32 = 4;

/// Player state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum PlayerState {
    /// Playing or camping.
    Live,
    /// Dead on the ground.
    Dead,
    /// Ready to restart/respawn.
    #[default]
    Reborn,
}

/// Player internal state. Full player_t in d_player.h.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Player {
    /// Player mobj (Mobj*).
    pub mo: *mut std::ffi::c_void,
    pub playerstate: PlayerState,
    pub viewz: Fixed,
    pub viewheight: Fixed,
    pub extralight: i32,
    pub fixedcolormap: i32,
    pub health: i32,
    /// Armor points (0–200).
    pub armorpoints: i32,
    /// Armor type: 0=green, 1=blue, 2=blue.
    pub armortype: i32,
    /// Power-up tic counters (invuln, strength, invis, etc.).
    pub powers: [i32; crate::doomdef::NUMPOWERS],
    /// Key cards owned.
    pub cards: [bool; NUMCARDS],
    pub backpack: bool,
    /// Frags in deathmatch.
    pub frags: [i32; MAXPLAYERS],
    /// Currently equipped weapon.
    pub readyweapon: Weapontype,
    /// Pending weapon change (WP_NOCHANGE if none).
    pub pendingweapon: Weapontype,
    /// Weapons owned.
    pub weaponowned: [bool; NUMWEAPONS],
    /// Ammo counts per type.
    pub ammo: [i32; NUMAMMO],
    /// Max ammo per type.
    pub maxammo: [i32; NUMAMMO],
    /// Cheat flags (CF_*).
    pub cheats: i32,
    /// Hint message (e.g. "Picked up the armor").
    pub message: Option<String>,
    /// Tics of damage flash (red palette). Decremented each tic.
    pub damagecount: i32,
    /// Tics of bonus/pickup flash. Decremented each tic.
    pub bonuscount: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            mo: std::ptr::null_mut(),
            playerstate: PlayerState::Reborn,
            viewz: 0,
            viewheight: 0,
            extralight: 0,
            fixedcolormap: 0,
            health: 100,
            armorpoints: 0,
            armortype: 0,
            powers: [0; crate::doomdef::NUMPOWERS],
            cards: [false; NUMCARDS],
            backpack: false,
            frags: [0; MAXPLAYERS],
            readyweapon: Weapontype::Pistol,
            pendingweapon: Weapontype::Pistol,
            weaponowned: [
                true,  // fist
                true,  // pistol
                false, // shotgun
                false, // chaingun
                false, // missile
                false, // plasma
                false, // bfg
                false, // chainsaw
                false, // supershotgun
            ],
            ammo: [50, 0, 0, 0],  // 50 bullets, no shells/cells/missiles
            maxammo: [200, 50, 300, 50],
            cheats: 0,
            message: None,
            damagecount: 0,
            bonuscount: 0,
        }
    }
}

/// Intermission player stats (wbplayerstruct_t in C).
#[derive(Debug, Clone, Copy, Default)]
pub struct WbPlayerStruct {
    pub in_game: i32,
    pub kills: i32,
    pub items: i32,
    pub secret: i32,
    pub time: i32,
    /// Frags per opponent (deathmatch). frags[i] = frags on player i.
    pub frags: [i32; crate::doomdef::MAXPLAYERS],
}

/// Intermission parameters (wbstartstruct_t in C).
#[derive(Debug, Clone, Default)]
pub struct WbStartStruct {
    /// Episode (0-based).
    pub epsd: i32,
    /// Last level completed (0-based).
    pub last: i32,
    /// Next level to enter (0-based).
    pub next: i32,
    /// Number of players.
    pub pnum: i32,
    /// Max kills in level (for percentage).
    pub maxkills: i32,
    /// Max items in level (for percentage).
    pub maxitems: i32,
    /// Max secrets in level (for percentage).
    pub maxsecret: i32,
    /// Par time in tics.
    pub partime: i32,
    /// Per-player stats.
    pub plyr: [WbPlayerStruct; crate::doomdef::MAXPLAYERS],
}

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

const DEFAULT_PLAYER: Player = Player {
    mo: std::ptr::null_mut(),
    playerstate: PlayerState::Reborn,
    viewz: 0,
    viewheight: 0,
    extralight: 0,
    fixedcolormap: 0,
    health: 100,
    armorpoints: 0,
    armortype: 0,
    powers: [0; crate::doomdef::NUMPOWERS],
    cards: [false; NUMCARDS],
    backpack: false,
    frags: [0; MAXPLAYERS],
    readyweapon: Weapontype::Pistol,
    pendingweapon: Weapontype::Pistol,
    weaponowned: [
        true, false, false, false, false, false, false, false, false,
    ],
    ammo: [50, 0, 0, 0],
    maxammo: [200, 50, 300, 50],
    cheats: 0,
    message: None,
    damagecount: 0,
    bonuscount: 0,
};

pub static mut PLAYERS: [Player; MAXPLAYERS] = [DEFAULT_PLAYER; MAXPLAYERS];
pub static mut PLAYERINGAME: [Boolean; MAXPLAYERS] = [false; MAXPLAYERS];
pub static mut DEATHMATCHSTARTS: [MapThing; MAX_DM_STARTS] =
    [MapThing { x: 0, y: 0, angle: 0, type_: 0, options: 0 }; MAX_DM_STARTS];
pub static mut DEATHMATCH_P: *const MapThing = std::ptr::null();
pub static mut PLAYERSTARTS: [MapThing; MAXPLAYERS] =
    [MapThing { x: 0, y: 0, angle: 0, type_: 0, options: 0 }; MAXPLAYERS];
const DEFAULT_WBPLAYER: WbPlayerStruct = WbPlayerStruct {
    in_game: 0,
    kills: 0,
    items: 0,
    secret: 0,
    time: 0,
    frags: [0; crate::doomdef::MAXPLAYERS],
};

pub static mut WMINFO: WbStartStruct = WbStartStruct {
    epsd: 0,
    last: 0,
    next: 0,
    pnum: 0,
    maxkills: 0,
    maxitems: 0,
    maxsecret: 0,
    partime: 0,
    plyr: [DEFAULT_WBPLAYER; crate::doomdef::MAXPLAYERS],
};

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
