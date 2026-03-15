//! Rust translation of doomgeneric/deh_misc.h
//! Parses "Misc" sections in dehacked files.

use crate::doomfeatures::*;

/// C #define: DEH_DEFAULT_INITIAL_HEALTH
pub const DEH_DEFAULT_INITIAL_HEALTH: i32 = 100;
/// C #define: DEH_DEFAULT_INITIAL_BULLETS
pub const DEH_DEFAULT_INITIAL_BULLETS: i32 = 50;
/// C #define: DEH_DEFAULT_MAX_HEALTH
pub const DEH_DEFAULT_MAX_HEALTH: i32 = 200;
/// C #define: DEH_DEFAULT_MAX_ARMOR
pub const DEH_DEFAULT_MAX_ARMOR: i32 = 200;
/// C #define: DEH_DEFAULT_GREEN_ARMOR_CLASS
pub const DEH_DEFAULT_GREEN_ARMOR_CLASS: i32 = 1;
/// C #define: DEH_DEFAULT_BLUE_ARMOR_CLASS
pub const DEH_DEFAULT_BLUE_ARMOR_CLASS: i32 = 2;
/// C #define: DEH_DEFAULT_MAX_SOULSPHERE
pub const DEH_DEFAULT_MAX_SOULSPHERE: i32 = 200;
/// C #define: DEH_DEFAULT_SOULSPHERE_HEALTH
pub const DEH_DEFAULT_SOULSPHERE_HEALTH: i32 = 100;
/// C #define: DEH_DEFAULT_MEGASPHERE_HEALTH
pub const DEH_DEFAULT_MEGASPHERE_HEALTH: i32 = 200;
/// C #define: DEH_DEFAULT_GOD_MODE_HEALTH
pub const DEH_DEFAULT_GOD_MODE_HEALTH: i32 = 100;
/// C #define: DEH_DEFAULT_IDFA_ARMOR
pub const DEH_DEFAULT_IDFA_ARMOR: i32 = 200;
/// C #define: DEH_DEFAULT_IDFA_ARMOR_CLASS
pub const DEH_DEFAULT_IDFA_ARMOR_CLASS: i32 = 2;
/// C #define: DEH_DEFAULT_IDKFA_ARMOR
pub const DEH_DEFAULT_IDKFA_ARMOR: i32 = 200;
/// C #define: DEH_DEFAULT_IDKFA_ARMOR_CLASS
pub const DEH_DEFAULT_IDKFA_ARMOR_CLASS: i32 = 2;
/// C #define: DEH_DEFAULT_BFG_CELLS_PER_SHOT
pub const DEH_DEFAULT_BFG_CELLS_PER_SHOT: i32 = 40;
/// C #define: DEH_DEFAULT_SPECIES_INFIGHTING
pub const DEH_DEFAULT_SPECIES_INFIGHTING: i32 = 0;

#[cfg(feature = "dehacked")]
pub static mut deh_initial_health: i32 = DEH_DEFAULT_INITIAL_HEALTH;
#[cfg(feature = "dehacked")]
pub static mut deh_initial_bullets: i32 = DEH_DEFAULT_INITIAL_BULLETS;
#[cfg(feature = "dehacked")]
pub static mut deh_max_health: i32 = DEH_DEFAULT_MAX_HEALTH;
#[cfg(feature = "dehacked")]
pub static mut deh_max_armor: i32 = DEH_DEFAULT_MAX_ARMOR;
#[cfg(feature = "dehacked")]
pub static mut deh_green_armor_class: i32 = DEH_DEFAULT_GREEN_ARMOR_CLASS;
#[cfg(feature = "dehacked")]
pub static mut deh_blue_armor_class: i32 = DEH_DEFAULT_BLUE_ARMOR_CLASS;
#[cfg(feature = "dehacked")]
pub static mut deh_max_soulsphere: i32 = DEH_DEFAULT_MAX_SOULSPHERE;
#[cfg(feature = "dehacked")]
pub static mut deh_soulsphere_health: i32 = DEH_DEFAULT_SOULSPHERE_HEALTH;
#[cfg(feature = "dehacked")]
pub static mut deh_megasphere_health: i32 = DEH_DEFAULT_MEGASPHERE_HEALTH;
#[cfg(feature = "dehacked")]
pub static mut deh_god_mode_health: i32 = DEH_DEFAULT_GOD_MODE_HEALTH;
#[cfg(feature = "dehacked")]
pub static mut deh_idfa_armor: i32 = DEH_DEFAULT_IDFA_ARMOR;
#[cfg(feature = "dehacked")]
pub static mut deh_idfa_armor_class: i32 = DEH_DEFAULT_IDFA_ARMOR_CLASS;
#[cfg(feature = "dehacked")]
pub static mut deh_idkfa_armor: i32 = DEH_DEFAULT_IDKFA_ARMOR;
#[cfg(feature = "dehacked")]
pub static mut deh_idkfa_armor_class: i32 = DEH_DEFAULT_IDKFA_ARMOR_CLASS;
#[cfg(feature = "dehacked")]
pub static mut deh_bfg_cells_per_shot: i32 = DEH_DEFAULT_BFG_CELLS_PER_SHOT;
#[cfg(feature = "dehacked")]
pub static mut deh_species_infighting: i32 = DEH_DEFAULT_SPECIES_INFIGHTING;
