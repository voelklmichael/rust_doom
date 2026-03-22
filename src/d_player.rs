//! Player structure (d_player.h)
//! Original: d_player.h

use crate::d_ticcmd::TiccmdT;
use crate::doomdef::CheatT;
use crate::m_fixed::FixedT;

// typedef enum playerstate_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerstateT {
    PstLive,
    PstDead,
    PstReborn,
}

// typedef struct player_s (player_t)
pub struct PlayerT {
    // mobj_t* mo - stub, use Option<Arc<Mutex<()>>>
    pub viewz: FixedT,
    pub viewheight: FixedT,
    pub deltaviewheight: FixedT,
    pub bob: FixedT,
    pub health: i32,
    pub armorpoints: i32,
    pub armortype: i32,
    pub cmd: TiccmdT,
    pub playerstate: PlayerstateT,
}
