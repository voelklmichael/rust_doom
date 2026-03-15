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
use crate::rendering::{v_draw_block, v_mark_rect, v_read_screen, v_video};
use crate::z_zone::{z_free, z_malloc, PU_STATIC};
use std::ptr::null_mut;
use std::sync::{Mutex, OnceLock};

/// Wipe types (wipe_ColorXForm, wipe_Melt).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WipeType {
    ColorXForm = 0,
    Melt = 1,
}

pub const WIPE_NUMWIPES: i32 = 2;

const SCREEN_SIZE: usize = (SCREENWIDTH * SCREENHEIGHT) as usize;

// =============================================================================
// FWipeState - thread-safe via OnceLock + Mutex
// =============================================================================

static F_WIPE_STATE: OnceLock<Mutex<FWipeState>> = OnceLock::new();

/// Safety: Raw pointers in FWipeState are only used while holding the Mutex lock.
unsafe impl Send for FWipeState {}

pub struct FWipeState {
    pub wipe_go: bool,
    pub wipe_scr_start: *mut u8,
    pub wipe_scr_end: *mut u8,
    pub wipe_init_done: bool,
    pub wipe_melt_y: *mut i32,
}

fn get_f_wipe_state() -> &'static Mutex<FWipeState> {
    F_WIPE_STATE.get_or_init(|| {
        Mutex::new(FWipeState {
            wipe_go: false,
            wipe_scr_start: std::ptr::null_mut(),
            wipe_scr_end: std::ptr::null_mut(),
            wipe_init_done: false,
            wipe_melt_y: std::ptr::null_mut(),
        })
    })
}

/// Access FWipeState.
pub fn with_f_wipe_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut FWipeState) -> R,
{
    let mut guard = get_f_wipe_state().lock().unwrap();
    f(&mut guard)
}

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
    v_video::with_v_video_state(|vv| {
        with_f_wipe_state(|st| {
            if !st.wipe_scr_start.is_null() && !vv.viewimage.is_null() {
                unsafe {
                    std::ptr::copy_nonoverlapping(st.wipe_scr_start, vv.viewimage, SCREEN_SIZE);
                }
            }
        })
    });
}

fn wipe_do_color_xform(width: i32, height: i32, ticks: i32) -> bool {
    v_video::with_v_video_state(|vv| {
        with_f_wipe_state(|st| {
            if vv.viewimage.is_null() || st.wipe_scr_start.is_null() || st.wipe_scr_end.is_null() {
                return true;
            }
            let size = (width * height) as usize;
            let mut changed = false;
            let w = vv.viewimage;
        let e = st.wipe_scr_end;
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
        })
    })
}

fn wipe_init_melt(width: i32, height: i32, _ticks: i32) {
    v_video::with_v_video_state(|vv| {
        with_f_wipe_state(|st| {
            if !st.wipe_scr_start.is_null() && !vv.viewimage.is_null() {
                unsafe {
                    std::ptr::copy_nonoverlapping(st.wipe_scr_start, vv.viewimage, SCREEN_SIZE);
                }
            }
        let w = width / 2;
        wipe_col_major_xform(st.wipe_scr_start, w, height);
        wipe_col_major_xform(st.wipe_scr_end, w, height);

        st.wipe_melt_y = z_malloc((width as usize) * 4, PU_STATIC, null_mut()) as *mut i32;
        let y = st.wipe_melt_y;
        if !y.is_null() {
            unsafe {
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
        });
    });
}

fn wipe_do_melt(width: i32, height: i32, mut ticks: i32) -> bool {
    v_video::with_v_video_state(|vv| {
        with_f_wipe_state(|st| {
            if vv.viewimage.is_null() || st.wipe_scr_start.is_null() || st.wipe_scr_end.is_null()
            || st.wipe_melt_y.is_null()
        {
            return true;
        }
        let w = width / 2;
        let s_start = st.wipe_scr_start as *const u16;
        let s_end = st.wipe_scr_end as *const u16;
        let d = vv.viewimage as *mut u16;
        let y_arr = st.wipe_melt_y;
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
    })
}

fn wipe_exit_melt(st: &mut FWipeState, _width: i32, _height: i32, _ticks: i32) {
    if !st.wipe_melt_y.is_null() {
        z_free(st.wipe_melt_y as *mut u8);
        st.wipe_melt_y = std::ptr::null_mut();
    }
    if !st.wipe_scr_start.is_null() {
        z_free(st.wipe_scr_start);
        st.wipe_scr_start = std::ptr::null_mut();
    }
    if !st.wipe_scr_end.is_null() {
        z_free(st.wipe_scr_end);
        st.wipe_scr_end = std::ptr::null_mut();
    }
}

/// Capture the "before" screen. Call before drawing the new screen.
/// Original: wipe_StartScreen
pub fn wipe_start_screen(_x: i32, _y: i32, _width: i32, _height: i32) -> i32 {
    with_f_wipe_state(|st| {
        st.wipe_scr_start = z_malloc(SCREEN_SIZE, PU_STATIC, null_mut());
        v_read_screen(st.wipe_scr_start);
    });
    0
}

/// Capture the "after" screen and restore start screen. Call after drawing new screen.
/// Original: wipe_EndScreen
pub fn wipe_end_screen(x: i32, y: i32, width: i32, height: i32) -> i32 {
    with_f_wipe_state(|st| {
        st.wipe_scr_end = z_malloc(SCREEN_SIZE, PU_STATIC, null_mut());
        v_read_screen(st.wipe_scr_end);
        if !st.wipe_scr_start.is_null() {
            v_draw_block(x, y, width, height, st.wipe_scr_start);
        }
    });
    0
}

/// Perform one step of the wipe. Returns true when wipe is complete.
/// Original: wipe_ScreenWipe
#[allow(unused_variables)]
pub fn wipe_screen_wipe(wipeno: i32, x: i32, y: i32, width: i32, height: i32, ticks: i32) -> bool {
    let _ = (x, y);
    with_f_wipe_state(|st| {
        if !st.wipe_go {
            st.wipe_go = true;
            st.wipe_init_done = false;
        }
        if !st.wipe_init_done {
            match wipeno {
                0 => wipe_init_color_xform(width, height, ticks),
                1 => wipe_init_melt(width, height, ticks),
                _ => wipe_init_color_xform(width, height, ticks),
            }
            st.wipe_init_done = true;
        }
        v_mark_rect(0, 0, width, height);
        let done = match wipeno {
            0 => wipe_do_color_xform(width, height, ticks),
            1 => wipe_do_melt(width, height, ticks),
            _ => wipe_do_color_xform(width, height, ticks),
        };
        if done {
            st.wipe_go = false;
            match wipeno {
                0 => {
                    if !st.wipe_scr_start.is_null() {
                        z_free(st.wipe_scr_start);
                        st.wipe_scr_start = std::ptr::null_mut();
                    }
                    if !st.wipe_scr_end.is_null() {
                        z_free(st.wipe_scr_end);
                        st.wipe_scr_end = std::ptr::null_mut();
                    }
                }
                1 => wipe_exit_melt(st, width, height, ticks),
                _ => {
                    if !st.wipe_scr_start.is_null() {
                        z_free(st.wipe_scr_start);
                        st.wipe_scr_start = std::ptr::null_mut();
                    }
                    if !st.wipe_scr_end.is_null() {
                        z_free(st.wipe_scr_end);
                        st.wipe_scr_end = std::ptr::null_mut();
                    }
                }
            }
        }
        done
    })
}
