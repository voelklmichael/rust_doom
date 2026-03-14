//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Mission begin melt/wipe screen special effect.
//
// Original: f_wipe.h + f_wipe.c

use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::m_random::m_random;
use crate::rendering::{v_draw_block, v_mark_rect, v_read_screen, VIEWIMAGE};
use crate::z_zone::{z_free, z_malloc, PU_STATIC};
use std::ptr::null_mut;

/// Wipe types (wipe_ColorXForm, wipe_Melt).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WipeType {
    ColorXForm = 0,
    Melt = 1,
}

pub const WIPE_NUMWIPES: i32 = 2;

const SCREEN_SIZE: usize = (SCREENWIDTH * SCREENHEIGHT) as usize;

static mut WIPE_GO: bool = false;
static mut WIPE_SCR_START: *mut u8 = std::ptr::null_mut();
static mut WIPE_SCR_END: *mut u8 = std::ptr::null_mut();
static mut WIPE_INIT_DONE: bool = false;

/// Melt wipe: per-column Y position (y<0 => delay, y>=height => column done).
static mut WIPE_MELT_Y: *mut i32 = std::ptr::null_mut();

/// Transpose buffer from row-major to column-major (as u16 grid width/2 x height).
fn wipe_col_major_xform(array: *mut u8, width: i32, height: i32) {
    unsafe {
        let count = (width * height) as usize;
        let dest = z_malloc(count * 2, PU_STATIC, null_mut()) as *mut u16;
        let arr = array as *const u16;
        for y in 0..height {
            for x in 0..width {
                let src_idx = (y as usize) * (width as usize) + (x as usize);
                let dst_idx = (x as usize) * (height as usize) + (y as usize);
                *dest.add(dst_idx) = *arr.add(src_idx);
            }
        }
        std::ptr::copy_nonoverlapping(dest as *const u8, array, count * 2);
        z_free(dest as *mut u8);
    }
}

fn wipe_init_color_xform(_width: i32, _height: i32, _ticks: i32) {
    unsafe {
        if !WIPE_SCR_START.is_null() && !VIEWIMAGE.is_null() {
            std::ptr::copy_nonoverlapping(WIPE_SCR_START, VIEWIMAGE, SCREEN_SIZE);
        }
    }
}

fn wipe_do_color_xform(width: i32, height: i32, ticks: i32) -> bool {
    unsafe {
        if VIEWIMAGE.is_null() || WIPE_SCR_START.is_null() || WIPE_SCR_END.is_null() {
            return true;
        }
        let size = (width * height) as usize;
        let mut changed = false;
        let w = VIEWIMAGE;
        let e = WIPE_SCR_END;
        for i in 0..size {
            let w_val = *w.add(i);
            let e_val = *e.add(i);
            if w_val != e_val {
                let newval = if w_val > e_val {
                    let n = (w_val as i32 - ticks).max(e_val as i32);
                    if n <= e_val as i32 {
                        e_val
                    } else {
                        n as u8
                    }
                } else {
                    let n = (w_val as i32 + ticks).min(e_val as i32);
                    if n >= e_val as i32 {
                        e_val
                    } else {
                        n as u8
                    }
                };
                *w.add(i) = newval;
                changed = true;
            }
        }
        !changed
    }
}

fn wipe_init_melt(width: i32, height: i32, _ticks: i32) {
    unsafe {
        if !WIPE_SCR_START.is_null() && !VIEWIMAGE.is_null() {
            std::ptr::copy_nonoverlapping(WIPE_SCR_START, VIEWIMAGE, SCREEN_SIZE);
        }
        let w = width / 2;
        wipe_col_major_xform(WIPE_SCR_START, w, height);
        wipe_col_major_xform(WIPE_SCR_END, w, height);

        WIPE_MELT_Y = z_malloc((width as usize) * 4, PU_STATIC, null_mut()) as *mut i32;
        let y = WIPE_MELT_Y;
        if !y.is_null() {
            *y.add(0) = -(m_random() % 16);
            for i in 1..(width as usize) {
                let r = (m_random() % 3) - 1;
                let mut yi = *y.add(i - 1) + r;
                if yi > 0 {
                    yi = 0;
                } else if yi == -16 {
                    yi = -15;
                }
                *y.add(i) = yi;
            }
        }
    }
}

fn wipe_do_melt(width: i32, height: i32, mut ticks: i32) -> bool {
    unsafe {
        if VIEWIMAGE.is_null() || WIPE_SCR_START.is_null() || WIPE_SCR_END.is_null()
            || WIPE_MELT_Y.is_null()
        {
            return true;
        }
        let w = width / 2;
        let s_start = WIPE_SCR_START as *const u16;
        let s_end = WIPE_SCR_END as *const u16;
        let d = VIEWIMAGE as *mut u16;
        let y_arr = WIPE_MELT_Y;
        let mut done = true;

        while ticks > 0 {
            ticks -= 1;
            for i in 0..(w as usize) {
                let yi = *y_arr.add(i);
                if yi < 0 {
                    *y_arr.add(i) = yi + 1;
                    done = false;
                } else if yi < height {
                    let dy = if yi < 16 { yi + 1 } else { 8 };
                    let dy = if yi + dy >= height {
                        (height - yi) as i32
                    } else {
                        dy
                    };
                    let mut src = s_end.add(i * (height as usize) + yi as usize);
                    let mut dst = d.add((yi as usize) * (w as usize) + i);
                    let mut idx = 0i32;
                    for _ in 0..dy {
                        *dst.add((idx * w) as usize) = *src;
                        src = src.add(1);
                        idx += 1;
                    }
                    let new_yi = yi + dy;
                    *y_arr.add(i) = new_yi;

                    src = s_start.add(i * (height as usize));
                    dst = d.add((new_yi as usize) * (w as usize) + i);
                    idx = 0;
                    let remain = height - new_yi;
                    for _ in 0..remain {
                        *dst.add((idx * w) as usize) = *src;
                        src = src.add(1);
                        idx += 1;
                    }
                    done = false;
                }
            }
        }
        done
    }
}

fn wipe_exit_melt(_width: i32, _height: i32, _ticks: i32) {
    unsafe {
        if !WIPE_MELT_Y.is_null() {
            z_free(WIPE_MELT_Y as *mut u8);
            WIPE_MELT_Y = std::ptr::null_mut();
        }
        if !WIPE_SCR_START.is_null() {
            z_free(WIPE_SCR_START);
            WIPE_SCR_START = std::ptr::null_mut();
        }
        if !WIPE_SCR_END.is_null() {
            z_free(WIPE_SCR_END);
            WIPE_SCR_END = std::ptr::null_mut();
        }
    }
}

/// Capture the "before" screen. Call before drawing the new screen.
/// Original: wipe_StartScreen
pub fn wipe_start_screen(_x: i32, _y: i32, _width: i32, _height: i32) -> i32 {
    unsafe {
        WIPE_SCR_START = z_malloc(SCREEN_SIZE, PU_STATIC, null_mut());
        v_read_screen(WIPE_SCR_START);
    }
    0
}

/// Capture the "after" screen and restore start screen. Call after drawing new screen.
/// Original: wipe_EndScreen
pub fn wipe_end_screen(x: i32, y: i32, width: i32, height: i32) -> i32 {
    unsafe {
        WIPE_SCR_END = z_malloc(SCREEN_SIZE, PU_STATIC, null_mut());
        v_read_screen(WIPE_SCR_END);
        if !WIPE_SCR_START.is_null() {
            v_draw_block(x, y, width, height, WIPE_SCR_START);
        }
    }
    0
}

/// Perform one step of the wipe. Returns true when wipe is complete.
/// Original: wipe_ScreenWipe
#[allow(unused_variables)]
pub fn wipe_screen_wipe(wipeno: i32, x: i32, y: i32, width: i32, height: i32, ticks: i32) -> bool {
    unsafe {
        if !WIPE_GO {
            WIPE_GO = true;
            WIPE_INIT_DONE = false;
        }
        if !WIPE_INIT_DONE {
            match wipeno {
                0 => wipe_init_color_xform(width, height, ticks),
                1 => wipe_init_melt(width, height, ticks),
                _ => wipe_init_color_xform(width, height, ticks),
            }
            WIPE_INIT_DONE = true;
        }
        v_mark_rect(0, 0, width, height);
        let done = match wipeno {
            0 => wipe_do_color_xform(width, height, ticks),
            1 => wipe_do_melt(width, height, ticks),
            _ => wipe_do_color_xform(width, height, ticks),
        };
        if done {
            WIPE_GO = false;
            match wipeno {
                0 => {
                    if !WIPE_SCR_START.is_null() {
                        z_free(WIPE_SCR_START);
                        WIPE_SCR_START = std::ptr::null_mut();
                    }
                    if !WIPE_SCR_END.is_null() {
                        z_free(WIPE_SCR_END);
                        WIPE_SCR_END = std::ptr::null_mut();
                    }
                }
                1 => wipe_exit_melt(width, height, ticks),
                _ => {
                    if !WIPE_SCR_START.is_null() {
                        z_free(WIPE_SCR_START);
                        WIPE_SCR_START = std::ptr::null_mut();
                    }
                    if !WIPE_SCR_END.is_null() {
                        z_free(WIPE_SCR_END);
                        WIPE_SCR_END = std::ptr::null_mut();
                    }
                }
            }
        }
        done
    }
}
