//! Rust translation of doomgeneric/i_scale.h

use crate::doomtype::*;
use crate::i_video::ScreenModeT;

/// C function: I_InitScale
pub fn i_init_scale(_src_buffer: &mut [byte], _dest_buffer: &mut [byte], _dest_pitch: i32) {
    todo!("original: I_InitScale")
}

/// C function: I_ResetScaleTables
pub fn i_reset_scale_tables(palette: &mut [byte]) {
    todo!("original: I_ResetScaleTables")
}

/// C extern: mode_scale_*
pub static mut mode_scale_1x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_scale_2x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_scale_3x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_scale_4x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_scale_5x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};

/// C extern: mode_stretch_*
pub static mut mode_stretch_1x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_stretch_2x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_stretch_3x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_stretch_4x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_stretch_5x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};

/// C extern: mode_squash_*
pub static mut mode_squash_1x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_squash_2x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_squash_3x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_squash_4x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
pub static mut mode_squash_5x: ScreenModeT = ScreenModeT {
    width: 0,
    height: 0,
    init_mode: None,
    draw_screen: None,
    poor_quality: crate::doomtype::Boolean::False,
};
