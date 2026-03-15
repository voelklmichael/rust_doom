//! Rust translation of doomgeneric/doomfeatures.h
//! List of features which can be enabled/disabled.

// C #undef FEATURE_WAD_MERGE - use cfg feature
#[cfg(feature = "wad_merge")]
/// C #define: FEATURE_WAD_MERGE
pub const FEATURE_WAD_MERGE: bool = true;

#[cfg(not(feature = "wad_merge"))]
/// C #define: FEATURE_WAD_MERGE
pub const FEATURE_WAD_MERGE: bool = false;

// C #undef FEATURE_DEHACKED
#[cfg(feature = "dehacked")]
/// C #define: FEATURE_DEHACKED
pub const FEATURE_DEHACKED: bool = true;

#[cfg(not(feature = "dehacked"))]
/// C #define: FEATURE_DEHACKED
pub const FEATURE_DEHACKED: bool = false;

// C #undef FEATURE_MULTIPLAYER
#[cfg(feature = "multiplayer")]
/// C #define: FEATURE_MULTIPLAYER
pub const FEATURE_MULTIPLAYER: bool = true;

#[cfg(not(feature = "multiplayer"))]
/// C #define: FEATURE_MULTIPLAYER
pub const FEATURE_MULTIPLAYER: bool = false;

// C: FEATURE_SOUND (commented #undef = enabled by default)
#[cfg(feature = "sound")]
/// C #define: FEATURE_SOUND
pub const FEATURE_SOUND: bool = true;

#[cfg(not(feature = "sound"))]
/// C #define: FEATURE_SOUND
pub const FEATURE_SOUND: bool = false;
