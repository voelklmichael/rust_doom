// doomgeneric/p_inter.h

pub use crate::d_player::*;
pub use crate::doomdef::NUMAMMO;
pub use crate::doomtype::*;
pub use crate::p_mobj::*;

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct P_InterState {
    // Original: maxammo
    pub maxammo: RefCell<[i32; NUMAMMO]>,
    // Original: clipammo
    pub clipammo: RefCell<[i32; NUMAMMO]>,
}

impl P_InterState {
    pub fn new() -> Self {
        Self {
            maxammo: RefCell::new([0; NUMAMMO]),
            clipammo: RefCell::new([0; NUMAMMO]),
        }
    }

    // Original: P_GivePower
    pub fn p_give_power(&self, _player: *mut PlayerT, _power: i32) -> Boolean {
        todo!("P_GivePower");
    }

    // Original: P_TouchSpecialThing
    pub fn p_touch_special_thing(&self, _special: *mut MobjT, _toucher: *mut MobjT) {
        todo!("P_TouchSpecialThing");
    }

    // Original: P_DamageMobj
    pub fn p_damage_mobj(
        &self,
        _target: *mut MobjT,
        _inflictor: *mut MobjT,
        _source: *mut MobjT,
        _damage: i32,
    ) {
        todo!("P_DamageMobj");
    }
}
