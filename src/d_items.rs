//! Items/weapons (d_items.h, d_items.c)
//! Original: d_items.h, d_items.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomdef::{AmmotypeT, WeapontypeT};

// NUMWEAPONS from doomdef
pub const NUMWEAPONS: usize = 9;

// typedef struct weaponinfo_t
pub struct WeaponinfoT {
    pub ammo: AmmotypeT,
    pub upstate: i32,
    pub downstate: i32,
    pub readystate: i32,
    pub atkstate: i32,
    pub flashstate: i32,
}

pub struct D_ItemsState {
    // extern weaponinfo_t weaponinfo[NUMWEAPONS]
    pub weaponinfo: Arc<Mutex<Vec<WeaponinfoT>>>,
}

impl D_ItemsState {
    pub fn new() -> Self {
        Self {
            weaponinfo: Arc::new(Mutex::new(vec![
                WeaponinfoT {
                    ammo: AmmotypeT::AmNoammo,
                    upstate: 0, downstate: 0, readystate: 0, atkstate: 0, flashstate: 0,
                },
                WeaponinfoT {
                    ammo: AmmotypeT::AmClip,
                    upstate: 0, downstate: 0, readystate: 0, atkstate: 0, flashstate: 0,
                },
                WeaponinfoT {
                    ammo: AmmotypeT::AmShell,
                    upstate: 0, downstate: 0, readystate: 0, atkstate: 0, flashstate: 0,
                },
                WeaponinfoT {
                    ammo: AmmotypeT::AmClip,
                    upstate: 0, downstate: 0, readystate: 0, atkstate: 0, flashstate: 0,
                },
                WeaponinfoT {
                    ammo: AmmotypeT::AmMisl,
                    upstate: 0, downstate: 0, readystate: 0, atkstate: 0, flashstate: 0,
                },
                WeaponinfoT {
                    ammo: AmmotypeT::AmCell,
                    upstate: 0, downstate: 0, readystate: 0, atkstate: 0, flashstate: 0,
                },
                WeaponinfoT {
                    ammo: AmmotypeT::AmCell,
                    upstate: 0, downstate: 0, readystate: 0, atkstate: 0, flashstate: 0,
                },
                WeaponinfoT {
                    ammo: AmmotypeT::AmNoammo,
                    upstate: 0, downstate: 0, readystate: 0, atkstate: 0, flashstate: 0,
                },
                WeaponinfoT {
                    ammo: AmmotypeT::AmShell,
                    upstate: 0, downstate: 0, readystate: 0, atkstate: 0, flashstate: 0,
                },
            ])),
        }
    }
}
