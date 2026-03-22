//! Pixel scaling (i_scale.h, i_scale.c)
//! Original: i_scale.h, i_scale.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Byte;
use crate::i_video::ScreenModeT;

pub struct I_ScaleState {
    pub mode_scale_1x: Arc<Mutex<ScreenModeT>>,
    pub mode_scale_2x: Arc<Mutex<ScreenModeT>>,
    pub mode_scale_3x: Arc<Mutex<ScreenModeT>>,
    pub mode_scale_4x: Arc<Mutex<ScreenModeT>>,
    pub mode_scale_5x: Arc<Mutex<ScreenModeT>>,
    pub mode_stretch_1x: Arc<Mutex<ScreenModeT>>,
    pub mode_stretch_2x: Arc<Mutex<ScreenModeT>>,
    pub mode_stretch_3x: Arc<Mutex<ScreenModeT>>,
    pub mode_stretch_4x: Arc<Mutex<ScreenModeT>>,
    pub mode_stretch_5x: Arc<Mutex<ScreenModeT>>,
    pub mode_squash_1x: Arc<Mutex<ScreenModeT>>,
    pub mode_squash_2x: Arc<Mutex<ScreenModeT>>,
    pub mode_squash_3x: Arc<Mutex<ScreenModeT>>,
    pub mode_squash_4x: Arc<Mutex<ScreenModeT>>,
    pub mode_squash_5x: Arc<Mutex<ScreenModeT>>,
}

impl I_ScaleState {
    /// Original: void I_InitScale(byte *_src_buffer, byte *_dest_buffer, int _dest_pitch)
    pub fn i_init_scale(&self, _src_buffer: &[Byte], _dest_buffer: &mut [Byte], _dest_pitch: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_ResetScaleTables(byte *palette)
    pub fn i_reset_scale_tables(&self, _palette: &[Byte]) {
        todo!("Basic stage-0 stub")
    }
}
