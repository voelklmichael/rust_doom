//
// Copyright(C) 2005-2014 Simon Howard
//
// Dehacked string replacements (stub - identity when FEATURE_DEHACKED false).
//
// Original: deh_str.h

/// Return string with DeHackEd substitutions applied. Stub: returns input unchanged.
#[inline]
pub fn deh_string(s: &str) -> &str {
    s
}

/// Add string replacement. Stub: no-op.
pub fn deh_add_string_replacement(_from_text: &str, _to_text: &str) {}
