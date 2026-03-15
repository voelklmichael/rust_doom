//! Rust translation of doomgeneric/m_misc.h
//! Miscellaneous utilities.

use crate::doomtype::*;

/// C function: M_WriteFile
pub fn m_write_file(name: &str, source: &mut [u8], length: i32) -> Boolean {
    todo!("original: M_WriteFile")
}

/// C function: M_ReadFile
pub fn m_read_file(name: &str, buffer: &mut Option<Box<[u8]>>) -> i32 {
    todo!("original: M_ReadFile")
}

/// C function: M_MakeDirectory
pub fn m_make_directory(dir: &str) {
    todo!("original: M_MakeDirectory")
}

/// C function: M_TempFile
pub fn m_temp_file(s: &mut [u8]) -> *mut i8 {
    todo!("original: M_TempFile")
}

/// C function: M_FileExists
pub fn m_file_exists(file: &str) -> Boolean {
    todo!("original: M_FileExists")
}

/// C function: M_FileLength
pub fn m_file_length(handle: *mut core::ffi::c_void) -> i64 {
    todo!("original: M_FileLength")
}

/// C function: M_StrToInt
pub fn m_str_to_int(str: &str, result: &mut i32) -> Boolean {
    todo!("original: M_StrToInt")
}

/// C function: M_ExtractFileBase
pub fn m_extract_file_base(path: &str, dest: &mut [u8]) {
    todo!("original: M_ExtractFileBase")
}

/// C function: M_ForceUppercase
pub fn m_force_uppercase(text: &mut [u8]) {
    todo!("original: M_ForceUppercase")
}

/// C function: M_StrCaseStr
pub fn m_str_case_str(haystack: &str, needle: &str) -> *mut i8 {
    todo!("original: M_StrCaseStr")
}

/// C function: M_StringDuplicate
pub fn m_string_duplicate(orig: *const i8) -> *mut i8 {
    todo!("original: M_StringDuplicate")
}

/// C function: M_StringCopy
pub fn m_string_copy(dest: &mut [u8], src: &str, dest_size: usize) -> Boolean {
    todo!("original: M_StringCopy")
}

/// C function: M_StringConcat
pub fn m_string_concat(dest: &mut [u8], src: &str, dest_size: usize) -> Boolean {
    todo!("original: M_StringConcat")
}

/// C function: M_StringReplace
pub fn m_string_replace(
    haystack: *const i8,
    needle: *const i8,
    replacement: *const i8,
) -> *mut i8 {
    todo!("original: M_StringReplace")
}

/// C function: M_StringJoin (variadic - varargs omitted in stub)
pub fn m_string_join(_s: *const i8) -> *mut i8 {
    todo!("original: M_StringJoin")
}

/// C function: M_StringStartsWith
pub fn m_string_starts_with(s: *const i8, prefix: *const i8) -> Boolean {
    todo!("original: M_StringStartsWith")
}

/// C function: M_StringEndsWith
pub fn m_string_ends_with(s: *const i8, suffix: *const i8) -> Boolean {
    todo!("original: M_StringEndsWith")
}

/// C function: M_vsnprintf
pub fn m_vsnprintf(buf: &mut [u8], buf_len: usize, s: &str, args: &mut [u8]) -> i32 {
    todo!("original: M_vsnprintf")
}

/// C function: M_snprintf (variadic - varargs omitted in stub)
pub fn m_snprintf(_buf: &mut [u8], _buf_len: usize, _s: &str) -> i32 {
    todo!("original: M_snprintf")
}

/// C function: M_OEMToUTF8
pub fn m_oem_to_utf8(ansi: *const i8) -> *mut i8 {
    todo!("original: M_OEMToUTF8")
}
