//! Rust translation of doomgeneric/doomfeatures.h
//! List of features which can be enabled/disabled.

// C #undef FEATURE_WAD_MERGE - use cfg feature
#[cfg(feature = "wad_merge")]
pub const FEATURE_WAD_MERGE: bool = true;

#[cfg(not(feature = "wad_merge"))]
pub const FEATURE_WAD_MERGE: bool = false;

// C #undef FEATURE_DEHACKED
#[cfg(feature = "dehacked")]
pub const FEATURE_DEHACKED: bool = true;

#[cfg(not(feature = "dehacked"))]
pub const FEATURE_DEHACKED: bool = false;

// C #undef FEATURE_MULTIPLAYER
#[cfg(feature = "multiplayer")]
pub const FEATURE_MULTIPLAYER: bool = true;

#[cfg(not(feature = "multiplayer"))]
pub const FEATURE_MULTIPLAYER: bool = false;

// C: FEATURE_SOUND (commented #undef = enabled by default)
#[cfg(feature = "sound")]
pub const FEATURE_SOUND: bool = true;

#[cfg(not(feature = "sound"))]
pub const FEATURE_SOUND: bool = false;
