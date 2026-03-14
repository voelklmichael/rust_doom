//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Exit text-mode ENDOOM screen. Display 80×25 text on shutdown.
//
// Original: i_endoom.h + i_endoom.c

use crate::deh::deh_string;
use crate::wad::{w_check_num_for_name, w_cache_lump_name};
use crate::z_zone::PU_STATIC;

const ENDOOM_W: usize = 80;
const ENDOOM_H: usize = 25;
const ENDOOM_SIZE: usize = ENDOOM_W * ENDOOM_H * 2; // char + attribute per cell

/// Display the ENDOOM screen on shutdown. Pass a pointer to the ENDOOM lump data.
/// If data is null or invalid, no-op.
/// Original: I_Endoom
pub fn i_endoom(data: *const u8) {
    if data.is_null() {
        return;
    }
    unsafe {
        let slice = std::slice::from_raw_parts(data, ENDOOM_SIZE);
        for row in 0..ENDOOM_H {
            let mut line = String::with_capacity(ENDOOM_W);
            for col in 0..ENDOOM_W {
                let idx = (row * ENDOOM_W + col) * 2;
                if idx + 1 < slice.len() {
                    let ch = slice[idx];
                    let c = if ch >= 32 && ch < 127 {
                        ch as char
                    } else {
                        ' '
                    };
                    line.push(c);
                }
            }
            println!("{}", line);
        }
    }
}

/// Display ENDOOM from WAD if present. Call on shutdown.
/// Looks up ENDOOM lump and prints to stdout.
pub fn i_endoom_from_wad() {
    if w_check_num_for_name(deh_string("ENDOOM")) < 0 {
        return;
    }
    let data = w_cache_lump_name(deh_string("ENDOOM"), PU_STATIC);
    if !data.is_null() {
        i_endoom(data.as_ptr());
    }
}
