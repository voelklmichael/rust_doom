//! Global state variables (doomstat.h, doomstat.c)
//! Original: doomstat.h, doomstat.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;
use crate::d_mode::{GameMissionT, GameModeT, GameVersionT, SkillT};
use crate::doomdef::GamestateT;

pub struct DoomstatState {
    // Command line
    pub nomonsters: Arc<Mutex<Boolean>>,
    pub respawnparm: Arc<Mutex<Boolean>>,
    pub fastparm: Arc<Mutex<Boolean>>,
    pub devparm: Arc<Mutex<Boolean>>,
    // Game mode
    pub gamemode: Arc<Mutex<GameModeT>>,
    pub gamemission: Arc<Mutex<GameMissionT>>,
    pub gameversion: Arc<Mutex<GameVersionT>>,
    pub gamedescription: Arc<Mutex<Option<String>>>,
    pub bfgedition: Arc<Mutex<Boolean>>,
    pub modifiedgame: Arc<Mutex<Boolean>>,
    // Menu/start
    pub startskill: Arc<Mutex<SkillT>>,
    pub startepisode: Arc<Mutex<i32>>,
    pub startmap: Arc<Mutex<i32>>,
    pub startloadgame: Arc<Mutex<i32>>,
    pub autostart: Arc<Mutex<Boolean>>,
    pub gameskill: Arc<Mutex<SkillT>>,
    pub gameepisode: Arc<Mutex<i32>>,
    pub gamemap: Arc<Mutex<i32>>,
    pub timelimit: Arc<Mutex<i32>>,
    pub respawnmonsters: Arc<Mutex<Boolean>>,
    pub netgame: Arc<Mutex<Boolean>>,
    pub deathmatch: Arc<Mutex<i32>>,
    // Sound (stub)
    pub sfx_volume: Arc<Mutex<i32>>,
    pub music_volume: Arc<Mutex<i32>>,
    pub snd_music_device: Arc<Mutex<i32>>,
    pub snd_sfx_device: Arc<Mutex<i32>>,
    pub snd_desired_music_device: Arc<Mutex<i32>>,
    pub snd_desired_sfx_device: Arc<Mutex<i32>>,
    // Refresh
    pub statusbaractive: Arc<Mutex<Boolean>>,
    pub automapactive: Arc<Mutex<Boolean>>,
    pub menuactive: Arc<Mutex<Boolean>>,
    pub paused: Arc<Mutex<Boolean>>,
    pub viewactive: Arc<Mutex<Boolean>>,
    pub nodrawers: Arc<Mutex<Boolean>>,
    pub testcontrols: Arc<Mutex<Boolean>>,
    pub testcontrols_mousespeed: Arc<Mutex<i32>>,
    pub viewangleoffset: Arc<Mutex<i32>>,
    pub consoleplayer: Arc<Mutex<i32>>,
    pub displayplayer: Arc<Mutex<i32>>,
    pub totalkills: Arc<Mutex<i32>>,
    pub totalitems: Arc<Mutex<i32>>,
    pub totalsecret: Arc<Mutex<i32>>,
    pub levelstarttic: Arc<Mutex<i32>>,
    pub leveltime: Arc<Mutex<i32>>,
    pub usergame: Arc<Mutex<Boolean>>,
    pub demoplayback: Arc<Mutex<Boolean>>,
    pub demorecording: Arc<Mutex<Boolean>>,
    pub lowres_turn: Arc<Mutex<Boolean>>,
    pub singledemo: Arc<Mutex<Boolean>>,
    pub gamestate: Arc<Mutex<GamestateT>>,
    pub precache: Arc<Mutex<Boolean>>,
    pub wipegamestate: Arc<Mutex<GamestateT>>,
    pub mouse_sensitivity: Arc<Mutex<i32>>,
    pub bodyqueslot: Arc<Mutex<i32>>,
    pub skyflatnum: Arc<Mutex<i32>>,
    pub rndindex: Arc<Mutex<i32>>,
}

impl DoomstatState {
    /// Original: logical_gamemission macro
    pub fn logical_gamemission(&self) -> GameMissionT {
        todo!("Basic stage-0 stub")
    }
}
