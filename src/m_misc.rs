// doomgeneric/m_misc.h

use std::ffi::{c_char, c_void};
use std::os::raw::c_int;

pub use crate::doomtype::*;

#[allow(non_camel_case_types)]
pub struct M_MiscState;

impl M_MiscState {
    pub fn new() -> Self {
        Self
    }

    // Original: M_WriteFile
    pub fn m_write_file(&self, _name: *mut c_char, _source: *mut c_void, _length: c_int) -> Boolean {
        todo!("M_WriteFile");
    }

    // Original: M_ReadFile
    pub fn m_read_file(&self, _name: *mut c_char, _buffer: *mut *mut Byte) -> c_int {
        todo!("M_ReadFile");
    }

    // Original: M_MakeDirectory
    pub fn m_make_directory(&self, _dir: *mut c_char) {
        todo!("M_MakeDirectory");
    }

    // Original: M_TempFile
    pub fn m_temp_file(&self, _s: *mut c_char) -> *mut c_char {
        todo!("M_TempFile");
    }

    // Original: M_FileExists
    pub fn m_file_exists(&self, _file: *mut c_char) -> Boolean {
        todo!("M_FileExists");
    }

    // Original: M_FileLength(FILE *handle)
    pub fn m_file_length(&self, _handle: *mut c_void) -> i64 {
        todo!("M_FileLength");
    }

    // Original: M_StrToInt
    pub fn m_str_to_int(&self, _str: *const c_char, _result: *mut c_int) -> Boolean {
        todo!("M_StrToInt");
    }

    // Original: M_ExtractFileBase
    pub fn m_extract_file_base(&self, _path: *mut c_char, _dest: *mut c_char) {
        todo!("M_ExtractFileBase");
    }

    // Original: M_ForceUppercase
    pub fn m_force_uppercase(&self, _text: *mut c_char) {
        todo!("M_ForceUppercase");
    }

    // Original: M_StrCaseStr
    pub fn m_str_case_str(&self, _haystack: *mut c_char, _needle: *mut c_char) -> *mut c_char {
        todo!("M_StrCaseStr");
    }

    // Original: M_StringDuplicate
    pub fn m_string_duplicate(&self, _orig: *const c_char) -> *mut c_char {
        todo!("M_StringDuplicate");
    }

    // Original: M_StringCopy
    pub fn m_string_copy(&self, _dest: *mut c_char, _src: *const c_char, _dest_size: usize) -> Boolean {
        todo!("M_StringCopy");
    }

    // Original: M_StringConcat
    pub fn m_string_concat(&self, _dest: *mut c_char, _src: *const c_char, _dest_size: usize) -> Boolean {
        todo!("M_StringConcat");
    }

    // Original: M_StringReplace
    pub fn m_string_replace(
        &self,
        _haystack: *const c_char,
        _needle: *const c_char,
        _replacement: *const c_char,
    ) -> *mut c_char {
        todo!("M_StringReplace");
    }

    // Original: M_StringJoin
    pub fn m_string_join(&self, _s: *const c_char) -> *mut c_char {
        todo!("M_StringJoin");
    }

    // Original: M_StringStartsWith
    pub fn m_string_starts_with(&self, _s: *const c_char, _prefix: *const c_char) -> Boolean {
        todo!("M_StringStartsWith");
    }

    // Original: M_StringEndsWith
    pub fn m_string_ends_with(&self, _s: *const c_char, _suffix: *const c_char) -> Boolean {
        todo!("M_StringEndsWith");
    }

    // Original: M_vsnprintf
    pub fn m_vsnprintf(&self, _buf: *mut c_char, _buf_len: usize, _s: *const c_char) -> c_int {
        todo!("M_vsnprintf");
    }

    // Original: M_snprintf
    pub fn m_snprintf(&self, _buf: *mut c_char, _buf_len: usize, _s: *const c_char) -> c_int {
        todo!("M_snprintf");
    }

    // Original: M_OEMToUTF8
    pub fn m_oem_to_utf8(&self, _ansi: *const c_char) -> *mut c_char {
        todo!("M_OEMToUTF8");
    }
}
