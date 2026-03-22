//! Player sprite/weapon (p_pspr.h, p_pspr.c)
//! Original: p_pspr.h, p_pspr.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::m_fixed::FixedT;

pub const FF_FULLBRIGHT: i32 = 0x8000;
pub const FF_FRAMEMASK: i32 = 0x7fff;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PsprnumT {
    PsWeapon = 0,
    PsFlash = 1,
    Numpsprites = 2,
}

pub struct PspdefT {
    pub state: Option<()>,
    pub tics: i32,
    pub sx: FixedT,
    pub sy: FixedT,
}

pub struct P_PsprState;

impl P_PsprState {
    /// Original: void P_SetupPsprites(player_t *curplayer)
    pub fn p_setup_psprites(&self, _curplayer: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_MovePsprites(player_t *curplayer)
    pub fn p_move_psprites(&self, _curplayer: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_DropWeapon(player_t *player)
    pub fn p_drop_weapon(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_BringUpWeapon(player_t *player)
    pub fn p_bring_up_weapon(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_CheckAmmo(player_t *player)
    pub fn p_check_ammo(&self, _player: &()) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_FireWeapon(player_t *player)
    pub fn p_fire_weapon(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }
}
