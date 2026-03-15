//! Rust translation of doomgeneric/d_player.h

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_items::*;
use crate::d_ticcmd::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: playerstate_t
pub enum PlayerstateT {
    PstLive,
    PstDead,
    PstReborn,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// C typedef: cheat_t
pub enum CheatT {
    CfNoclip = 1,
    CfGodmode = 2,
    CfNomomentum = 4,
}

/// player_t
#[repr(C)]
#[derive(Clone)]
/// C typedef: player_t
pub struct PlayerT {
    pub mo: Option<Arc<Mutex<MobjT>>>,
    pub playerstate: PlayerstateT,
    pub cmd: TiccmdT,
    pub viewz: FixedT,
    pub viewheight: FixedT,
    pub deltaviewheight: FixedT,
    pub bob: FixedT,
    pub health: i32,
    pub armorpoints: i32,
    pub armortype: i32,
    pub powers: [i32; NUMPOWERS],
    pub cards: [boolean; NUMCARDS],
    pub backpack: boolean,
    pub frags: [i32; MAXPLAYERS as usize],
    pub readyweapon: WeapontypeT,
    pub pendingweapon: WeapontypeT,
    pub weaponowned: [boolean; NUMWEAPONS],
    pub ammo: [i32; NUMAMMO],
    pub maxammo: [i32; NUMAMMO],
    pub attackdown: i32,
    pub usedown: i32,
    pub cheats: i32,
    pub refire: i32,
    pub killcount: i32,
    pub itemcount: i32,
    pub secretcount: i32,
    pub message: String,
    pub damagecount: i32,
    pub bonuscount: i32,
    pub attacker: Option<Arc<Mutex<MobjT>>>,
    pub extralight: i32,
    pub fixedcolormap: i32,
    pub colormap: i32,
    pub psprites: [PspdefT; NUMPSPRITES],
    pub didsecret: boolean,
}
impl PlayerT {
    pub(crate) const fn new() -> Self {
        Self {
            mo: None,
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
            message: String::new(),
            damagecount: 0,
            bonuscount: 0,
            attacker: None,
            extralight: 0,
            fixedcolormap: 0,
            colormap: 0,
            psprites: [
                PspdefT {
                    state: None,
                    tics: 0,
                    sx: 0,
                    sy: 0,
                },
                PspdefT {
                    state: None,
                    tics: 0,
                    sx: 0,
                    sy: 0,
                },
            ],
            didsecret: Boolean::False,
        }
    }
}

/// wbplayerstruct_t
#[repr(C)]
#[derive(Clone, Copy)]
/// C typedef: wbplayerstruct_t
pub struct WbplayerstructT {
    pub in_: boolean,
    pub skills: i32,
    pub sitems: i32,
    pub ssecret: i32,
    pub stime: i32,
    pub frags: [i32; 4],
    pub score: i32,
}

/// wbstartstruct_t
#[repr(C)]
/// C typedef: wbstartstruct_t
pub struct WbstartstructT {
    pub epsd: i32,
    pub didsecret: boolean,
    pub last: i32,
    pub next: i32,
    pub maxkills: i32,
    pub maxitems: i32,
    pub maxsecret: i32,
    pub maxfrags: i32,
    pub partime: i32,
    pub pnum: i32,
    pub plyr: [WbplayerstructT; MAXPLAYERS as usize],
}
