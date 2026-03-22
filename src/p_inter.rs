//! Player interaction (p_inter.h, p_inter.c)
//! Original: p_inter.h, p_inter.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;

pub struct P_InterState;

impl P_InterState {
    /// Original: boolean P_GivePower(player_t *player, int power)
    pub fn p_give_power(&self, _player: &(), _power: i32) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_TouchSpecialThing(mobj_t *special, mobj_t *toucher)
    pub fn p_touch_special_thing(&self, _special: &(), _toucher: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_DamageMobj(mobj_t *target, mobj_t *inflictor, mobj_t *source, int damage)
    pub fn p_damage_mobj(&self, _target: &(), _inflictor: &(), _source: &(), _damage: i32) {
        todo!("Basic stage-0 stub")
    }
}
