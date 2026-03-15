//! Rust translation of doomgeneric/deh_main.h
//! Dehacked entrypoint and common code.

use crate::doomtype::*;
use crate::sha1::*;

/// C #define: DEH_VANILLA_NUMSTATES
pub const DEH_VANILLA_NUMSTATES: i32 = 966;
/// C #define: DEH_VANILLA_NUMSFX
pub const DEH_VANILLA_NUMSFX: i32 = 107;

/// C function: DEH_ParseCommandLine
pub fn deh_parse_command_line() {
    todo!("original: DEH_ParseCommandLine")
}

/// C function: DEH_LoadFile
pub fn deh_load_file(filename: &str) -> i32 {
    todo!("original: DEH_LoadFile")
}

/// C function: DEH_LoadLump
pub fn deh_load_lump(lumpnum: i32, allow_long: Boolean, allow_error: Boolean) -> i32 {
    todo!("original: DEH_LoadLump")
}

/// C function: DEH_LoadLumpByName
pub fn deh_load_lump_by_name(name: &str, allow_long: Boolean, allow_error: Boolean) -> i32 {
    todo!("original: DEH_LoadLumpByName")
}

/* Function is never used
/// C function: DEH_ParseAssignment
pub fn deh_parse_assignment(
    line: * mut i8,
    variable_name: * mut * mut i8,
    value: * mut * mut i8,
) -> Boolean {
    todo!("original: DEH_ParseAssignment")
}
*/

/// C function: DEH_Checksum
pub fn deh_checksum(digest: &mut Sha1DigestT) {
    todo!("original: DEH_Checksum")
}

pub static mut deh_allow_extended_strings: Boolean = Boolean::False;
pub static mut deh_allow_long_strings: Boolean = Boolean::False;
pub static mut deh_allow_long_cheats: Boolean = Boolean::False;
pub static mut deh_apply_cheats: Boolean = Boolean::False;
