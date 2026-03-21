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
    // Original: mo
    pub mo: *mut MobjT,
    // Original: playerstate
    pub playerstate: PlayerstateT,
    // Original: cmd
    pub cmd: TiccmdT,
    // Original: viewz
    pub viewz: FixedT,
    // Original: viewheight
    pub viewheight: FixedT,
    // Original: deltaviewheight
    pub deltaviewheight: FixedT,
    // Original: bob
    pub bob: FixedT,
    // Original: health
    pub health: i32,
    // Original: armorpoints
    pub armorpoints: i32,
    // Original: armortype
    pub armortype: i32,
    // Original: powers
    pub powers: [i32; NUMPOWERS],
    // Original: cards
    pub cards: [Boolean; NUMCARDS],
    // Original: backpack
    pub backpack: Boolean,
    // Original: frags
    pub frags: [i32; MAXPLAYERS],
    // Original: readyweapon
    pub readyweapon: WeapontypeT,
    // Original: pendingweapon
    pub pendingweapon: WeapontypeT,
    // Original: weaponowned
    pub weaponowned: [Boolean; NUMWEAPONS],
    // Original: ammo
    pub ammo: [i32; NUMAMMO],
    // Original: maxammo
    pub maxammo: [i32; NUMAMMO],
    // Original: attackdown
    pub attackdown: i32,
    // Original: usedown
    pub usedown: i32,
    // Original: cheats
    pub cheats: i32,
    // Original: refire
    pub refire: i32,
    // Original: killcount
    pub killcount: i32,
    // Original: itemcount
    pub itemcount: i32,
    // Original: secretcount
    pub secretcount: i32,
    // Original: message
    pub message: *mut c_char,
    // Original: damagecount
    pub damagecount: i32,
    // Original: bonuscount
    pub bonuscount: i32,
    // Original: attacker
    pub attacker: *mut MobjT,
    // Original: extralight
    pub extralight: i32,
    // Original: fixedcolormap
    pub fixedcolormap: i32,
    // Original: colormap
    pub colormap: i32,
    // Original: psprites
    pub psprites: [PspdefT; NUMPSPRITES],
    // Original: didsecret
    pub didsecret: Boolean,
}

/// Original: wbplayerstruct_t
#[repr(C)]
pub struct WbplayerstructT {
    // Original: in_
    pub in_: Boolean,
    // Original: skills
    pub skills: i32,
    // Original: sitems
    pub sitems: i32,
    // Original: ssecret
    pub ssecret: i32,
    // Original: stime
    pub stime: i32,
    // Original: frags
    pub frags: [i32; 4],
    // Original: score
    pub score: i32,
}

/// Original: wbstartstruct_t
#[repr(C)]
pub struct WbstartstructT {
    // Original: epsd
    pub epsd: i32,
    // Original: didsecret
    pub didsecret: Boolean,
    // Original: last
    pub last: i32,
    // Original: next
    pub next: i32,
    // Original: maxkills
    pub maxkills: i32,
    // Original: maxitems
    pub maxitems: i32,
    // Original: maxsecret
    pub maxsecret: i32,
    // Original: maxfrags
    pub maxfrags: i32,
    // Original: partime
    pub partime: i32,
    // Original: pnum
    pub pnum: i32,
    // Original: plyr
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
