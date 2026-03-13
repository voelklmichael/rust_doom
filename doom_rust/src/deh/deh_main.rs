//
// Copyright(C) 2005-2014 Simon Howard
//
// Dehacked entrypoint and common code (stub - FEATURE_DEHACKED false).
//
// Original: deh_main.h

use crate::sha1_mod::Sha1Digest;

/// Vanilla Doom state count limit.
pub const DEH_VANILLA_NUMSTATES: usize = 966;
/// Vanilla Doom sfx count limit.
pub const DEH_VANILLA_NUMSFX: usize = 107;

/// Parse command line for -deh. Stub: no-op.
pub fn deh_parse_command_line() {}

/// Load DeHackEd file. Stub: returns 0 (failure).
pub fn deh_load_file(_filename: &str) -> i32 {
    0
}

/// Load DeHackEd from lump. Stub: returns 0 (failure).
pub fn deh_load_lump(_lumpnum: i32, _allow_long: bool, _allow_error: bool) -> i32 {
    0
}

/// Load DeHackEd by lump name. Stub: returns 0 (failure).
pub fn deh_load_lump_by_name(_name: &str, _allow_long: bool, _allow_error: bool) -> i32 {
    0
}

/// Parse assignment line. Stub: returns false.
pub fn deh_parse_assignment(_line: &str, _variable_name: &mut Option<String>, _value: &mut Option<String>) -> bool {
    false
}

/// Compute DeHackEd checksum. Stub: no-op (digest unchanged).
pub fn deh_checksum(_digest: &mut Sha1Digest) {}
