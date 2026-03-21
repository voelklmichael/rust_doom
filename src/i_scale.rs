// doomgeneric/i_scale.h

pub use crate::doomtype::*;
pub use crate::i_video::ScreenModeT;
pub use crate::i_video::*;
pub use crate::m_argv::*;
pub use crate::z_zone::*;

use std::cell::RefCell;

// Original: (zero ScreenMode_t)
fn empty_screen_mode() -> ScreenModeT {
    ScreenModeT {
        width: 0,
        height: 0,
        init_mode: None,
        draw_screen: None,
        poor_quality: Boolean::False,
    }
}

#[allow(non_camel_case_types)]
pub struct I_ScaleState {
    // Original: mode_scale_1x
    pub mode_scale_1x: RefCell<ScreenModeT>,
    // Original: mode_scale_2x
    pub mode_scale_2x: RefCell<ScreenModeT>,
    // Original: mode_scale_3x
    pub mode_scale_3x: RefCell<ScreenModeT>,
    // Original: mode_scale_4x
    pub mode_scale_4x: RefCell<ScreenModeT>,
    // Original: mode_scale_5x
    pub mode_scale_5x: RefCell<ScreenModeT>,
    // Original: mode_stretch_1x
    pub mode_stretch_1x: RefCell<ScreenModeT>,
    // Original: mode_stretch_2x
    pub mode_stretch_2x: RefCell<ScreenModeT>,
    // Original: mode_stretch_3x
    pub mode_stretch_3x: RefCell<ScreenModeT>,
    // Original: mode_stretch_4x
    pub mode_stretch_4x: RefCell<ScreenModeT>,
    // Original: mode_stretch_5x
    pub mode_stretch_5x: RefCell<ScreenModeT>,
    // Original: mode_squash_1x
    pub mode_squash_1x: RefCell<ScreenModeT>,
    // Original: mode_squash_2x
    pub mode_squash_2x: RefCell<ScreenModeT>,
    // Original: mode_squash_3x
    pub mode_squash_3x: RefCell<ScreenModeT>,
    // Original: mode_squash_4x
    pub mode_squash_4x: RefCell<ScreenModeT>,
    // Original: mode_squash_5x
    pub mode_squash_5x: RefCell<ScreenModeT>,
}

impl I_ScaleState {
    pub fn new() -> Self {
        let m = || RefCell::new(empty_screen_mode());
        Self {
            mode_scale_1x: m(),
            mode_scale_2x: m(),
            mode_scale_3x: m(),
            mode_scale_4x: m(),
            mode_scale_5x: m(),
            mode_stretch_1x: m(),
            mode_stretch_2x: m(),
            mode_stretch_3x: m(),
            mode_stretch_4x: m(),
            mode_stretch_5x: m(),
            mode_squash_1x: m(),
            mode_squash_2x: m(),
            mode_squash_3x: m(),
            mode_squash_4x: m(),
            mode_squash_5x: m(),
        }
    }

    // Original: I_InitScale
    pub fn i_init_scale(&self, _src_buffer: *mut Byte, _dest_buffer: *mut Byte, _dest_pitch: i32) {
        todo!("I_InitScale");
    }

    // Original: I_ResetScaleTables
    pub fn i_reset_scale_tables(&self, _palette: *mut Byte) {
        todo!("I_ResetScaleTables");
    }
}
