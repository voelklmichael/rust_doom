//! Movement/collision (p_map.c)
//! Original: p_map.c

use crate::m_fixed::FixedT;
use crate::doomtype::Boolean;

pub struct P_MapState;

impl P_MapState {
    /// Original: boolean P_CheckPosition(mobj_t *thing, fixed_t x, fixed_t y)
    pub fn p_check_position(&self, _thing: &(), _x: FixedT, _y: FixedT) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_TryMove(mobj_t *thing, fixed_t x, fixed_t y)
    pub fn p_try_move(&self, _thing: &(), _x: FixedT, _y: FixedT) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_UseLines(player_t *player)
    pub fn p_use_lines(&self, _player: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_SlideMove(mobj_t *mo)
    pub fn p_slide_move(&self, _mo: &()) {
        todo!("Basic stage-0 stub")
    }
}
