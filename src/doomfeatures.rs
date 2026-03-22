//! Feature switches (doomfeatures.h)
//! Original: doomfeatures.h
//! Unused features per plan §4: not migrated as code paths; flags for reference.

// #undef FEATURE_WAD_MERGE
/// Unused per plan §4.2 - do not migrate.
pub const FEATURE_WAD_MERGE: bool = false;

// #undef FEATURE_DEHACKED
/// Unused per plan §4.2 - do not migrate.
pub const FEATURE_DEHACKED: bool = false;

// #undef FEATURE_MULTIPLAYER
/// Unused per plan §4.2 - do not migrate.
pub const FEATURE_MULTIPLAYER: bool = false;

// #undef FEATURE_SOUND (commented out in source = may be enabled)
/// Per doomfeatures.h: //#undef - may be enabled in some builds.
pub const FEATURE_SOUND: bool = false;
