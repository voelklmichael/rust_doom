// doomgeneric/d_player.h

use std::cell::RefCell;
use std::ffi::c_char;

pub use crate::d_items::*;
pub use crate::d_ticcmd::*;
pub use crate::net_defs::*;
pub use crate::p_mobj::*;
pub use crate::p_pspr::*;

/// Original: typedef enum { PST_LIVE, PST_DEAD, PST_REBORN } playerstate_t
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerstateT {
    PstLive = 0,
    PstDead = 1,
    PstReborn = 2,
}

/// Original: typedef enum { CF_NOCLIP = 1, ... } cheat_t
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CheatT {
    CfNoclip = 1,
    CfGodmode = 2,
    CfNomomentum = 4,
}

/// Original: typedef struct player_s { ... } player_t
#[repr(C)]
pub struct PlayerT {
    pub mo: *mut MobjT,
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
    pub cards: [Boolean; NUMCARDS],
    pub backpack: Boolean,
    pub frags: [i32; MAXPLAYERS],
    pub readyweapon: WeapontypeT,
    pub pendingweapon: WeapontypeT,
    pub weaponowned: [Boolean; NUMWEAPONS],
    pub ammo: [i32; NUMAMMO],
    pub maxammo: [i32; NUMAMMO],
    pub attackdown: i32,
    pub usedown: i32,
    pub cheats: i32,
    pub refire: i32,
    pub killcount: i32,
    pub itemcount: i32,
    pub secretcount: i32,
    pub message: *mut c_char,
    pub damagecount: i32,
    pub bonuscount: i32,
    pub attacker: *mut MobjT,
    pub extralight: i32,
    pub fixedcolormap: i32,
    pub colormap: i32,
    pub psprites: [PspdefT; NUMPSPRITES],
    pub didsecret: Boolean,
}

/// Original: wbplayerstruct_t
#[repr(C)]
pub struct WbplayerstructT {
    pub in_: Boolean,
    pub skills: i32,
    pub sitems: i32,
    pub ssecret: i32,
    pub stime: i32,
    pub frags: [i32; 4],
    pub score: i32,
}

/// Original: wbstartstruct_t
#[repr(C)]
pub struct WbstartstructT {
    pub epsd: i32,
    pub didsecret: Boolean,
    pub last: i32,
    pub next: i32,
    pub maxkills: i32,
    pub maxitems: i32,
    pub maxsecret: i32,
    pub maxfrags: i32,
    pub partime: i32,
    pub pnum: i32,
    pub plyr: [WbplayerstructT; MAXPLAYERS],
}

#[allow(non_camel_case_types)]
pub struct D_PlayerState {
    /// Scratch for future globals from d_player.c if any
    pub _placeholder: RefCell<i32>,
}

impl D_PlayerState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(0),
        }
    }
}
