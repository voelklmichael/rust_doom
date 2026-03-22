//! Player movement, bobbing, weapons (p_user.c)
//! Original: p_user.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::m_fixed::FixedT;
use crate::doomtype::Boolean;

pub struct P_UserState {
    pub onground: Arc<Mutex<Boolean>>,
}

impl P_UserState {
    /// Original: void P_Thrust(player_t *player, angle_t angle, fixed_t move)
    pub fn p_thrust(&self, _player: &(), _angle: u32, _move_amt: FixedT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_CalcHeight(player_t *player)
    pub fn p_calc_height(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_MovePlayer(player_t *player)
    pub fn p_move_player(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_DeathThink(player_t *player)
    pub fn p_death_think(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_PlayerThink(player_t *player)
    pub fn p_player_think(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }
}
