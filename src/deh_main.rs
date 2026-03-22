//! Dehacked entrypoint (deh_main.h)
//! Original: deh_main.h

use crate::doomtype::Boolean;
use crate::sha1::Sha1DigestT;

// #define DEH_VANILLA_NUMSTATES 966
pub const DEH_VANILLA_NUMSTATES: i32 = 966;
// #define DEH_VANILLA_NUMSFX 107
pub const DEH_VANILLA_NUMSFX: i32 = 107;

pub struct DehMainState {
    // extern boolean deh_allow_extended_strings
    pub deh_allow_extended_strings: std::sync::Arc<std::sync::Mutex<Boolean>>,
    // extern boolean deh_allow_long_strings
    pub deh_allow_long_strings: std::sync::Arc<std::sync::Mutex<Boolean>>,
    // extern boolean deh_allow_long_cheats
    pub deh_allow_long_cheats: std::sync::Arc<std::sync::Mutex<Boolean>>,
    // extern boolean deh_apply_cheats
    pub deh_apply_cheats: std::sync::Arc<std::sync::Mutex<Boolean>>,
}

impl DehMainState {
    /// Original: void DEH_ParseCommandLine(void)
    pub fn deh_parse_command_line(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: int DEH_LoadFile(char *filename)
    pub fn deh_load_file(&self, _filename: &str) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int DEH_LoadLump(int lumpnum, boolean allow_long, boolean allow_error)
    pub fn deh_load_lump(&self, _lumpnum: i32, _allow_long: Boolean, _allow_error: Boolean) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int DEH_LoadLumpByName(char *name, ...)
    pub fn deh_load_lump_by_name(&self, _name: &str, _allow_long: Boolean, _allow_error: Boolean) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean DEH_ParseAssignment(...)
    pub fn deh_parse_assignment(&self, _line: &str, _variable_name: &mut Option<String>, _value: &mut Option<String>) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void DEH_Checksum(sha1_digest_t digest)
    pub fn deh_checksum(&self, _digest: &mut Sha1DigestT) {
        todo!("Basic stage-0 stub")
    }
}
