// deh_misc.h — no .c; externs only when FEATURE_DEHACKED

pub use crate::doomfeatures::*;

use std::cell::RefCell;

// Original: #define DEH_DEFAULT_INITIAL_HEALTH 100
pub const DEH_DEFAULT_INITIAL_HEALTH: i32 = 100;
pub const DEH_DEFAULT_INITIAL_BULLETS: i32 = 50;
pub const DEH_DEFAULT_MAX_HEALTH: i32 = 200;
pub const DEH_DEFAULT_MAX_ARMOR: i32 = 200;
pub const DEH_DEFAULT_GREEN_ARMOR_CLASS: i32 = 1;
pub const DEH_DEFAULT_BLUE_ARMOR_CLASS: i32 = 2;
pub const DEH_DEFAULT_MAX_SOULSPHERE: i32 = 200;
pub const DEH_DEFAULT_SOULSPHERE_HEALTH: i32 = 100;
pub const DEH_DEFAULT_MEGASPHERE_HEALTH: i32 = 200;
pub const DEH_DEFAULT_GOD_MODE_HEALTH: i32 = 100;
pub const DEH_DEFAULT_IDFA_ARMOR: i32 = 200;
pub const DEH_DEFAULT_IDFA_ARMOR_CLASS: i32 = 2;
pub const DEH_DEFAULT_IDKFA_ARMOR: i32 = 200;
pub const DEH_DEFAULT_IDKFA_ARMOR_CLASS: i32 = 2;
pub const DEH_DEFAULT_BFG_CELLS_PER_SHOT: i32 = 40;
pub const DEH_DEFAULT_SPECIES_INFIGHTING: i32 = 0;

// When FEATURE_DEHACKED is disabled, C maps these names to defaults via #define.
#[allow(non_upper_case_globals)]
mod deh_misc_c_names {
    use super::*;

    pub const deh_initial_health: i32 = DEH_DEFAULT_INITIAL_HEALTH;
    pub const deh_initial_bullets: i32 = DEH_DEFAULT_INITIAL_BULLETS;
    pub const deh_max_health: i32 = DEH_DEFAULT_MAX_HEALTH;
    pub const deh_max_armor: i32 = DEH_DEFAULT_MAX_ARMOR;
    pub const deh_green_armor_class: i32 = DEH_DEFAULT_GREEN_ARMOR_CLASS;
    pub const deh_blue_armor_class: i32 = DEH_DEFAULT_BLUE_ARMOR_CLASS;
    pub const deh_max_soulsphere: i32 = DEH_DEFAULT_MAX_SOULSPHERE;
    pub const deh_soulsphere_health: i32 = DEH_DEFAULT_SOULSPHERE_HEALTH;
    pub const deh_megasphere_health: i32 = DEH_DEFAULT_MEGASPHERE_HEALTH;
    pub const deh_god_mode_health: i32 = DEH_DEFAULT_GOD_MODE_HEALTH;
    pub const deh_idfa_armor: i32 = DEH_DEFAULT_IDFA_ARMOR;
    pub const deh_idfa_armor_class: i32 = DEH_DEFAULT_IDFA_ARMOR_CLASS;
    pub const deh_idkfa_armor: i32 = DEH_DEFAULT_IDKFA_ARMOR;
    pub const deh_idkfa_armor_class: i32 = DEH_DEFAULT_IDKFA_ARMOR_CLASS;
    pub const deh_bfg_cells_per_shot: i32 = DEH_DEFAULT_BFG_CELLS_PER_SHOT;
    pub const deh_species_infighting: i32 = DEH_DEFAULT_SPECIES_INFIGHTING;
}

pub use deh_misc_c_names::*;

#[allow(non_camel_case_types)]
pub struct Deh_MiscState {
    _ph: RefCell<()>,
}

impl Deh_MiscState {
    pub fn new() -> Self {
        let _ = FEATURE_DEHACKED;
        Self {
            _ph: RefCell::new(()),
        }
    }
}
