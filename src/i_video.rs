// doomgeneric/i_video.h

use std::cell::RefCell;
use std::ffi::c_char;

pub use crate::doomtype::*;

// Original: #define SCREENWIDTH 320
pub const SCREENWIDTH: i32 = 320;
// Original: #define SCREENHEIGHT 200
pub const SCREENHEIGHT: i32 = 200;
// Original: #define SCREENWIDTH_4_3 256
pub const SCREENWIDTH_4_3: i32 = 256;
// Original: #define SCREENHEIGHT_4_3 240
pub const SCREENHEIGHT_4_3: i32 = 240;
// Original: #define MAX_MOUSE_BUTTONS 8
pub const MAX_MOUSE_BUTTONS: i32 = 8;

/// Original: typedef boolean (*grabmouse_callback_t)(void)
pub type GrabmouseCallbackT = unsafe extern "C" fn() -> Boolean;

/// Original: struct color (C bitfields → plain bytes, little-endian layout)
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VideoColor {
    // Original: b
    pub b: u8,
    // Original: g
    pub g: u8,
    // Original: r
    pub r: u8,
    // Original: a
    pub a: u8,
}

/// Original: typedef struct { ... } screen_mode_t
#[repr(C)]
pub struct ScreenModeT {
    // Original: width
    pub width: i32,
    // Original: height
    pub height: i32,
    // Original: init_mode
    pub init_mode: Option<unsafe extern "C" fn(*mut Byte)>,
    // Original: draw_screen
    pub draw_screen: Option<unsafe extern "C" fn(i32, i32, i32, i32) -> Boolean>,
    // Original: poor_quality
    pub poor_quality: Boolean,
}

#[allow(non_camel_case_types)]
pub struct I_VideoState {
    /// Original: char *video_driver
    pub video_driver: RefCell<*mut c_char>,
    /// Original: boolean screenvisible
    pub screenvisible: RefCell<Boolean>,
    /// Original: float mouse_acceleration
    pub mouse_acceleration: RefCell<f32>,
    /// Original: int mouse_threshold
    pub mouse_threshold: RefCell<i32>,
    /// Original: int vanilla_keyboard_mapping
    pub vanilla_keyboard_mapping: RefCell<i32>,
    /// Original: boolean screensaver_mode
    pub screensaver_mode: RefCell<Boolean>,
    /// Original: int usegamma
    pub usegamma: RefCell<i32>,
    /// Original: byte *I_VideoBuffer
    pub i_video_buffer: RefCell<*mut Byte>,
    /// Original: int screen_width, screen_height, screen_bpp, fullscreen, aspect_ratio_correct
    pub screen_width: RefCell<i32>,
    // Original: screen_height
    pub screen_height: RefCell<i32>,
    // Original: screen_bpp
    pub screen_bpp: RefCell<i32>,
    // Original: fullscreen
    pub fullscreen: RefCell<i32>,
    // Original: aspect_ratio_correct
    pub aspect_ratio_correct: RefCell<i32>,
    /// Original: int show_diskicon, diskicon_readbytes
    pub show_diskicon: RefCell<i32>,
    // Original: diskicon_readbytes
    pub diskicon_readbytes: RefCell<i32>,
    /// Original: boolean palette_changed (CMAP256)
    pub palette_changed: RefCell<Boolean>,
    /// Original: struct color colors[256]
    pub colors: RefCell<[VideoColor; 256]>,
}

impl I_VideoState {
    pub fn new() -> Self {
        Self {
            video_driver: RefCell::new(std::ptr::null_mut()),
            screenvisible: RefCell::new(Boolean::False),
            mouse_acceleration: RefCell::new(0.0),
            mouse_threshold: RefCell::new(0),
            vanilla_keyboard_mapping: RefCell::new(0),
            screensaver_mode: RefCell::new(Boolean::False),
            usegamma: RefCell::new(0),
            i_video_buffer: RefCell::new(std::ptr::null_mut()),
            screen_width: RefCell::new(0),
            screen_height: RefCell::new(0),
            screen_bpp: RefCell::new(0),
            fullscreen: RefCell::new(0),
            aspect_ratio_correct: RefCell::new(0),
            show_diskicon: RefCell::new(0),
            diskicon_readbytes: RefCell::new(0),
            palette_changed: RefCell::new(Boolean::False),
            colors: RefCell::new([VideoColor::default(); 256]),
        }
    }

    // Original: I_InitGraphics
    pub fn i_init_graphics(&self) {
        todo!("I_InitGraphics");
    }

    // Original: I_GraphicsCheckCommandLine
    pub fn i_graphics_check_command_line(&self) {
        todo!("I_GraphicsCheckCommandLine");
    }

    // Original: I_ShutdownGraphics
    pub fn i_shutdown_graphics(&self) {
        todo!("I_ShutdownGraphics");
    }

    // Original: I_SetPalette
    pub fn i_set_palette(&self, _palette: *mut Byte) {
        todo!("I_SetPalette");
    }

    // Original: I_GetPaletteIndex
    pub fn i_get_palette_index(&self, _r: i32, _g: i32, _b: i32) -> i32 {
        todo!("I_GetPaletteIndex");
    }

    // Original: I_UpdateNoBlit
    pub fn i_update_no_blit(&self) {
        todo!("I_UpdateNoBlit");
    }

    // Original: I_FinishUpdate
    pub fn i_finish_update(&self) {
        todo!("I_FinishUpdate");
    }

    // Original: I_ReadScreen
    pub fn i_read_screen(&self, _scr: *mut Byte) {
        todo!("I_ReadScreen");
    }

    // Original: I_BeginRead
    pub fn i_begin_read(&self) {
        todo!("I_BeginRead");
    }

    // Original: I_SetWindowTitle
    pub fn i_set_window_title(&self, _title: *mut c_char) {
        todo!("I_SetWindowTitle");
    }

    // Original: I_CheckIsScreensaver
    pub fn i_check_is_screensaver(&self) {
        todo!("I_CheckIsScreensaver");
    }

    // Original: I_SetGrabMouseCallback
    pub fn i_set_grab_mouse_callback(&self, _func: GrabmouseCallbackT) {
        todo!("I_SetGrabMouseCallback");
    }

    // Original: I_DisplayFPSDots
    pub fn i_display_fps_dots(&self, _dots_on: Boolean) {
        todo!("I_DisplayFPSDots");
    }

    // Original: I_BindVideoVariables
    pub fn i_bind_video_variables(&self) {
        todo!("I_BindVideoVariables");
    }

    // Original: I_InitWindowTitle
    pub fn i_init_window_title(&self) {
        todo!("I_InitWindowTitle");
    }

    // Original: I_InitWindowIcon
    pub fn i_init_window_icon(&self) {
        todo!("I_InitWindowIcon");
    }

    // Original: I_StartFrame
    pub fn i_start_frame(&self) {
        todo!("I_StartFrame");
    }

    // Original: I_StartTic
    pub fn i_start_tic(&self) {
        todo!("I_StartTic");
    }

    // Original: I_EnableLoadingDisk
    pub fn i_enable_loading_disk(&self) {
        todo!("I_EnableLoadingDisk");
    }

    // Original: I_EndRead
    pub fn i_end_read(&self) {
        todo!("I_EndRead");
    }
}
