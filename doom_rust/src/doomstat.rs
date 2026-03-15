//! Rust translation of doomgeneric/doomstat.h
//! All the global variables that store the internal state.

use crate::d_items::NUMWEAPONS;
use crate::d_mode::*;
use crate::d_player::*;
use crate::d_ticcmd::*;
use crate::p_pspr::{NUMPSPRITES, PspdefT};
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::net_defs::*;

pub static mut nomonsters: boolean = Boolean::False;
pub static mut respawnparm: boolean = Boolean::False;
pub static mut fastparm: boolean = Boolean::False;
pub static mut devparm: boolean = Boolean::False;

pub static mut gamemode: GameModeT = GameModeT::Indetermined;
pub static mut gamemission: GameMissionT = GameMissionT::None;
pub static mut gameversion: GameVersionT = GameVersionT::ExeFinal;
pub static mut gamedescription: *mut i8 = std::ptr::null_mut();
pub static mut bfgedition: boolean = Boolean::False;
pub static mut modifiedgame: boolean = Boolean::False;

pub static mut startskill: SkillT = SkillT::Medium;
pub static mut startepisode: i32 = 1;
pub static mut startmap: i32 = 1;
pub static mut startloadgame: i32 = -1;
pub static mut autostart: boolean = Boolean::False;
pub static mut gameskill: SkillT = SkillT::Medium;
pub static mut gameepisode: i32 = 1;
pub static mut gamemap: i32 = 1;
pub static mut timelimit: i32 = 0;
pub static mut respawnmonsters: boolean = Boolean::False;
pub static mut netgame: boolean = Boolean::False;
pub static mut deathmatch: i32 = 0;

pub static mut sfx_volume: i32 = 0;
pub static mut music_volume: i32 = 0;
pub static mut snd_music_device: i32 = 0;
pub static mut snd_sfx_device: i32 = 0;
pub static mut snd_desired_music_device: i32 = 0;
pub static mut snd_desired_sfx_device: i32 = 0;

pub static mut statusbaractive: boolean = Boolean::True;
pub static mut automapactive: boolean = Boolean::False;
pub static mut menuactive: boolean = Boolean::False;
pub static mut paused: boolean = Boolean::False;
pub static mut viewactive: boolean = Boolean::True;
pub static mut nodrawers: boolean = Boolean::False;
pub static mut testcontrols: boolean = Boolean::False;
pub static mut testcontrols_mousespeed: i32 = 0;
pub static mut viewangleoffset: i32 = 0;
pub static mut consoleplayer: i32 = 0;
pub static mut displayplayer: i32 = 0;

pub static mut totalkills: i32 = 0;
pub static mut totalitems: i32 = 0;
pub static mut totalsecret: i32 = 0;
pub static mut levelstarttic: i32 = 0;
pub static mut leveltime: i32 = 0;

pub static mut usergame: boolean = Boolean::True;
pub static mut demoplayback: boolean = Boolean::False;
pub static mut demorecording: boolean = Boolean::False;
pub static mut lowres_turn: boolean = Boolean::False;
pub static mut singledemo: boolean = Boolean::False;

pub static mut gamestate: GamestateT = GamestateT::Level;

pub static mut players: [PlayerT; MAXPLAYERS as usize] = [PlayerT {
    mo: std::ptr::null_mut(),
    playerstate: PlayerstateT::PstLive,
    cmd: TiccmdT {
        forwardmove: 0,
        sidemove: 0,
        angleturn: 0,
        chatchar: 0,
        buttons: 0,
        consistancy: 0,
        buttons2: 0,
        inventory: 0,
        lookfly: 0,
        arti: 0,
    },
    viewz: 0,
    viewheight: 0,
    deltaviewheight: 0,
    bob: 0,
    health: 0,
    armorpoints: 0,
    armortype: 0,
    powers: [0; NUMPOWERS],
    cards: [Boolean::False; NUMCARDS],
    backpack: Boolean::False,
    frags: [0; MAXPLAYERS as usize],
    readyweapon: WeapontypeT::Fist,
    pendingweapon: WeapontypeT::Nochange,
    weaponowned: [Boolean::False; NUMWEAPONS],
    ammo: [0; NUMAMMO],
    maxammo: [0; NUMAMMO],
    attackdown: 0,
    usedown: 0,
    cheats: 0,
    refire: 0,
    killcount: 0,
    itemcount: 0,
    secretcount: 0,
    message: std::ptr::null_mut(),
    damagecount: 0,
    bonuscount: 0,
    attacker: std::ptr::null_mut(),
    extralight: 0,
    fixedcolormap: 0,
    colormap: 0,
    psprites: [PspdefT {
        state: std::ptr::null_mut(),
        tics: 0,
        sx: 0,
        sy: 0,
    }; NUMPSPRITES],
    didsecret: Boolean::False,
}; MAXPLAYERS as usize];
pub static mut playeringame: [boolean; MAXPLAYERS as usize] = [Boolean::False; MAXPLAYERS as usize];

pub const MAX_DM_STARTS: usize = 10;
pub static mut deathmatchstarts: [MapthingT; MAX_DM_STARTS] = [MapthingT {
    x: 0,
    y: 0,
    angle: 0,
    type_: 0,
    options: 0,
}; MAX_DM_STARTS];
pub static mut deathmatch_p: *mut MapthingT = std::ptr::null_mut();
pub static mut playerstarts: [MapthingT; MAXPLAYERS as usize] = [MapthingT {
    x: 0,
    y: 0,
    angle: 0,
    type_: 0,
    options: 0,
}; MAXPLAYERS as usize];

pub static mut wminfo: WbstartstructT = WbstartstructT {
    epsd: 0,
    didsecret: Boolean::False,
    last: 0,
    next: 0,
    maxkills: 0,
    maxitems: 0,
    maxsecret: 0,
    maxfrags: 0,
    partime: 0,
    pnum: 0,
    plyr: [WbplayerstructT {
        in_: Boolean::False,
        skills: 0,
        sitems: 0,
        ssecret: 0,
        stime: 0,
        frags: [0; 4],
        score: 0,
    }; MAXPLAYERS as usize],
};

pub static mut savegamedir: *mut i8 = std::ptr::null_mut();
pub static mut basedefault: [i8; 1024] = [0; 1024];
pub static mut precache: boolean = Boolean::False;
pub static mut wipegamestate: GamestateT = GamestateT::Level;
pub static mut mouse_sensitivity: i32 = 0;
pub static mut bodyqueslot: i32 = 0;
pub static mut skyflatnum: i32 = 0;
pub static mut rndindex: i32 = 0;
pub static mut netcmds: *mut TiccmdT = std::ptr::null_mut();
