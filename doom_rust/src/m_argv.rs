//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Nil.
//
// Original: m_argv.h / m_argv.c

use crate::doomtype::Boolean;
use std::sync::OnceLock;

static MY_ARGC: OnceLock<usize> = OnceLock::new();
static MY_ARGV: OnceLock<Vec<String>> = OnceLock::new();

/// Initialize command line args. Call from main.
pub fn m_argv_init(args: Vec<String>) {
    let _ = MY_ARGC.set(args.len());
    let _ = MY_ARGV.set(args);
}

/// Get argc.
pub fn myargc() -> usize {
    MY_ARGC.get().copied().unwrap_or(0)
}

/// Get argv as slice of strings.
pub fn myargv() -> &'static [String] {
    MY_ARGV.get().map(|v| v.as_slice()).unwrap_or(&[])
}

/// Returns the position of the given parameter in the arg list (0 if not found).
/// Original: m_argv.c M_CheckParm
pub fn m_check_parm(check: &str) -> usize {
    m_check_parm_with_args(check, 0)
}

/// Same as M_CheckParm, but checks that num_args arguments are available
/// following the specified argument.
/// Original: m_argv.c M_CheckParmWithArgs
pub fn m_check_parm_with_args(check: &str, num_args: usize) -> usize {
    let argv = myargv();
    let argc = myargc();
    if argc <= num_args {
        return 0;
    }
    for (i, arg) in argv.iter().enumerate() {
        if i >= argc - num_args {
            break;
        }
        if arg.eq_ignore_ascii_case(check) {
            return i;
        }
    }
    0
}

/// Parameter has been specified?
pub fn m_parm_exists(check: &str) -> Boolean {
    m_check_parm(check) != 0
}
