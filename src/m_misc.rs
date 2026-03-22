//! Misc utilities (m_misc.h, m_misc.c)
//! Original: m_misc.h, m_misc.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;

pub struct M_MiscState;

impl M_MiscState {
    /// Original: boolean M_WriteFile(char *name, void *source, int length)
    pub fn m_write_file(&self, _name: &str, _source: &[u8]) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: int M_ReadFile(char *name, byte **buffer)
    pub fn m_read_file(&self, _name: &str) -> Option<Vec<u8>> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_MakeDirectory(char *dir)
    pub fn m_make_directory(&self, _dir: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *M_TempFile(char *s)
    pub fn m_temp_file(&self, _s: &str) -> String {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean M_FileExists(char *file)
    pub fn m_file_exists(&self, _file: &str) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean M_StrToInt(const char *str, int *result)
    pub fn m_str_to_int(&self, _str: &str) -> Option<i32> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_ExtractFileBase(char *path, char *dest)
    pub fn m_extract_file_base(&self, _path: &str) -> String {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_ForceUppercase(char *text)
    pub fn m_force_uppercase(&self, _text: &str) -> String {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *M_StrCaseStr(char *haystack, char *needle)
    pub fn m_str_case_str(&self, _haystack: &str, _needle: &str) -> Option<usize> {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *M_StringDuplicate(const char *orig)
    pub fn m_string_duplicate(&self, _orig: &str) -> String {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean M_StringCopy(char *dest, const char *src, size_t dest_size)
    pub fn m_string_copy(&self, _dest: &mut [u8], _src: &str) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean M_StringConcat(char *dest, const char *src, size_t dest_size)
    pub fn m_string_concat(&self, _dest: &mut [u8], _src: &str) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *M_StringReplace(const char *haystack, const char *needle, const char *replacement)
    pub fn m_string_replace(&self, _haystack: &str, _needle: &str, _replacement: &str) -> String {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *M_StringJoin(const char *s, ...)
    pub fn m_string_join(&self, _parts: &[&str]) -> String {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean M_StringStartsWith(const char *s, const char *prefix)
    pub fn m_string_starts_with(&self, _s: &str, _prefix: &str) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean M_StringEndsWith(const char *s, const char *suffix)
    pub fn m_string_ends_with(&self, _s: &str, _suffix: &str) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: int M_snprintf(char *buf, size_t buf_len, const char *s, ...)
    pub fn m_snprintf(&self, _buf: &mut [u8], _s: &str) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *M_OEMToUTF8(const char *ansi)
    pub fn m_oem_to_utf8(&self, _ansi: &str) -> String {
        todo!("Basic stage-0 stub")
    }
}
