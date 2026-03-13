//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//     Common code to parse command line, identifying WAD files to load.
//
// Original: w_main.h (public) + w_main.c (private)

use crate::doomfeatures;
use crate::d_iwad;
use crate::m_argv;
use crate::wad::w_merge;
use crate::wad::w_wad;

/// Parse the command line, merging WAD files that are specified.
/// Returns true if at least one file was added.
/// Original: W_ParseCommandLine
pub fn w_parse_command_line() -> bool {
    let mut modifiedgame = false;

    if doomfeatures::FEATURE_WAD_MERGE {
        // -merge, -nwtmerge, -af, -as, -aa - all panic when called
        let p = m_argv::m_check_parm_with_args("-merge", 1);
        if p > 0 {
            let mut idx = p + 1;
            while idx < m_argv::myargc() && !m_argv::myargv()[idx].starts_with('-') {
                modifiedgame = true;
                let filename = d_iwad::d_try_find_wad_by_name(&m_argv::myargv()[idx]);
                println!(" merging {}", filename);
                w_merge::w_merge_file(&filename);
                idx += 1;
            }
        }
        // ... similar for other merge options
    }

    // -file: Load the specified PWAD files.
    let p = m_argv::m_check_parm_with_args("-file", 1);
    if p > 0 {
        let mut idx = p + 1;
        while idx < m_argv::myargc() && !m_argv::myargv()[idx].starts_with('-') {
            modifiedgame = true;
            let filename = d_iwad::d_try_find_wad_by_name(&m_argv::myargv()[idx]);
            println!(" adding {}", filename);
            w_wad::w_add_file(&filename);
            idx += 1;
        }
    }

    modifiedgame
}
