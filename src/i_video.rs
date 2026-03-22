//! Video interface (i_video.h, i_video.c)
//! Original: i_video.h, i_video.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;

pub const SCREENWIDTH: i32 = 320;
pub const SCREENHEIGHT: i32 = 200;
pub const SCREENWIDTH_4_3: i32 = 256;
pub const SCREENHEIGHT_4_3: i32 = 240;
pub const MAX_MOUSE_BUTTONS: i32 = 8;

pub struct ScreenModeT {
    pub width: i32,
    pub height: i32,
    pub poor_quality: Boolean,
}

pub struct I_VideoState;

impl I_VideoState {
    /// Original: void I_InitGraphics(void)
    pub fn i_init_graphics(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_GraphicsCheckCommandLine(void)
    pub fn i_graphics_check_command_line(&self) {
        todo!("Basic stage-0 stub")
    }
}
