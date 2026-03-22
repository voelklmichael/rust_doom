//! Dehacked string replacements (deh_str.h)
//! Original: deh_str.h
//! Plan §4: FEATURE_DEHACKED unused - pass-through macros only.

// When FEATURE_DEHACKED disabled:
// #define DEH_String(x) (x)
// #define DEH_printf printf
// #define DEH_fprintf fprintf
// #define DEH_snprintf snprintf
// #define DEH_AddStringReplacement(x, y)

/// Pass-through: DEH_String(x) -> x when DEHACKED disabled
#[inline(always)]
pub fn deh_string(s: &str) -> &str {
    s
}
