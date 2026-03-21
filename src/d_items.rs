// doomgeneric/d_items.h + weaponinfo[] from d_items.c (table stubbed)

use std::cell::RefCell;

pub use crate::doomdef::*;

/// Original: typedef struct { ... } weaponinfo_t
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WeaponinfoT {
    // Original: ammo
    pub ammo: AmmotypeT,
    // Original: upstate
    pub upstate: i32,
    // Original: downstate
    pub downstate: i32,
    // Original: readystate
    pub readystate: i32,
    // Original: atkstate
    pub atkstate: i32,
    // Original: flashstate
    pub flashstate: i32,
}

/// Original: weaponinfo_t weaponinfo[NUMWEAPONS] (defined in d_items.c)
#[allow(non_camel_case_types)]
pub struct D_ItemsState {
    // Original: weaponinfo
    pub weaponinfo: RefCell<Vec<WeaponinfoT>>,
}

impl D_ItemsState {
    pub fn new() -> Self {
        Self {
            weaponinfo: RefCell::new(Vec::new()),
        }
    }

    // Original: static initializer in d_items.c
    pub fn init_weaponinfo_table(&self) {
        todo!("weaponinfo[] from d_items.c");
    }
}
