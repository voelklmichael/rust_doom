// deh_main.h - Dehacked entrypoint

pub use crate::doomtype::*;
pub use crate::doomfeatures::*;
pub use crate::deh_str::*;
pub use crate::sha1::*;

// Original: #define DEH_VANILLA_NUMSTATES 966
pub const DEH_VANILLA_NUMSTATES: i32 = 966;

// Original: #define DEH_VANILLA_NUMSFX 107
pub const DEH_VANILLA_NUMSFX: i32 = 107;

// Original: DEH_ParseCommandLine
pub fn deh_parse_command_line() {
    todo!("DEH_ParseCommandLine")
}

// Original: DEH_LoadFile
pub fn deh_load_file(_filename: *mut i8) -> i32 {
    todo!("DEH_LoadFile")
}

// Original: DEH_LoadLump
pub fn deh_load_lump(_lumpnum: i32, _allow_long: Boolean, _allow_error: Boolean) -> i32 {
    todo!("DEH_LoadLump")
}

// Original: DEH_LoadLumpByName
pub fn deh_load_lump_by_name(
    _name: *mut i8,
    _allow_long: Boolean,
    _allow_error: Boolean,
) -> i32 {
    todo!("DEH_LoadLumpByName")
}

// Original: DEH_ParseAssignment
pub fn deh_parse_assignment(
    _line: *mut i8,
    _variable_name: *mut *mut i8,
    _value: *mut *mut i8,
) -> Boolean {
    todo!("DEH_ParseAssignment")
}

// Original: DEH_Checksum
pub fn deh_checksum(_digest: &mut Sha1DigestT) {
    todo!("DEH_Checksum")
}

// Original: extern boolean deh_allow_extended_strings
pub fn deh_allow_extended_strings() -> Boolean {
    todo!("deh_allow_extended_strings: extern variable")
}

// Original: extern boolean deh_allow_long_strings
pub fn deh_allow_long_strings() -> Boolean {
    todo!("deh_allow_long_strings: extern variable")
}

// Original: extern boolean deh_allow_long_cheats
pub fn deh_allow_long_cheats() -> Boolean {
    todo!("deh_allow_long_cheats: extern variable")
}

// Original: extern boolean deh_apply_cheats
pub fn deh_apply_cheats() -> Boolean {
    todo!("deh_apply_cheats: extern variable")
}
