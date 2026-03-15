//! Rust translation of doomgeneric/deh_str.h
//! Dehacked string replacements.

#[cfg(feature = "dehacked")]
/// C function: DEH_String
pub fn deh_string(s: &str) -> String {
    todo!("original: DEH_String")
}

#[cfg(not(feature = "dehacked"))]
/// C macro: DEH_String(x) -> (x) when dehacked disabled
pub fn deh_string(s: &str) -> &str {
    s
}

#[cfg(feature = "dehacked")]
/// C function: DEH_printf
pub fn deh_printf(_fmt: &str) {
    todo!("original: DEH_printf")
}

#[cfg(feature = "dehacked")]
/// C function: DEH_fprintf
pub fn deh_fprintf(_fstream: &mut [u8], _fmt: &str) {
    todo!("original: DEH_fprintf")
}

#[cfg(feature = "dehacked")]
/// C function: DEH_snprintf
pub fn deh_snprintf(_buffer: &mut [u8], _len: usize, _fmt: &str) {
    todo!("original: DEH_snprintf")
}

#[cfg(feature = "dehacked")]
/// C function: DEH_AddStringReplacement
pub fn deh_add_string_replacement(_from_text: &str, _to_text: &str) {
    todo!("original: DEH_AddStringReplacement")
}
