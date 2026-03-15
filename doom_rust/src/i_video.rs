//! Rust translation of doomgeneric/i_video.h

use crate::doomtype::*;

/// C #define: SCREENWIDTH
pub const SCREENWIDTH: i32 = 320;
/// C #define: SCREENHEIGHT
pub const SCREENHEIGHT: i32 = 200;
/// C #define: SCREENWIDTH_4_3
pub const SCREENWIDTH_4_3: i32 = 256;
/// C #define: SCREENHEIGHT_4_3
pub const SCREENHEIGHT_4_3: i32 = 240;
/// C #define: MAX_MOUSE_BUTTONS
pub const MAX_MOUSE_BUTTONS: i32 = 8;

/// C typedef: screen_mode_t
#[repr(C)]
/// C typedef: screen_mode_t
pub struct ScreenModeT {
    pub width: i32,
    pub height: i32,
    pub init_mode: Option<extern "C" fn(&mut byte)>,
    pub draw_screen: Option<extern "C" fn(i32, i32, i32, i32) -> boolean>,
    pub poor_quality: boolean,
}

/// C typedef: grabmouse_callback_t
pub type GrabmouseCallbackT = extern "C" fn() -> boolean;

/// C function: I_InitGraphics
pub fn i_init_graphics() {
    todo!("original: I_InitGraphics")
}

/// C function: I_GraphicsCheckCommandLine
pub fn i_graphics_check_command_line() {
    todo!("original: I_GraphicsCheckCommandLine")
}

/// C function: I_ShutdownGraphics
pub fn i_shutdown_graphics() {
    todo!("original: I_ShutdownGraphics")
}

/// C function: I_SetPalette
pub fn i_set_palette(palette: &mut [byte]) {
    todo!("original: I_SetPalette")
}

/// C function: I_GetPaletteIndex
pub fn i_get_palette_index(r: i32, g: i32, b: i32) -> i32 {
    todo!("original: I_GetPaletteIndex")
}

/// C function: I_UpdateNoBlit
pub fn i_update_no_blit() {
    todo!("original: I_UpdateNoBlit")
}

/// C function: I_FinishUpdate
pub fn i_finish_update() {
    todo!("original: I_FinishUpdate")
}

/// C function: I_ReadScreen
pub fn i_read_screen(scr: &mut [byte]) {
    todo!("original: I_ReadScreen")
}

/// C function: I_BeginRead
pub fn i_begin_read() {
    todo!("original: I_BeginRead")
}

/// C function: I_SetWindowTitle
pub fn i_set_window_title(title: &str) {
    todo!("original: I_SetWindowTitle")
}

/// C function: I_CheckIsScreensaver
pub fn i_check_is_screensaver() {
    todo!("original: I_CheckIsScreensaver")
}

/// C function: I_SetGrabMouseCallback
pub fn i_set_grab_mouse_callback(func: GrabmouseCallbackT) {
    todo!("original: I_SetGrabMouseCallback")
}

/// C function: I_DisplayFPSDots
pub fn i_display_fps_dots(dots_on: boolean) {
    todo!("original: I_DisplayFPSDots")
}

/// C function: I_BindVideoVariables
pub fn i_bind_video_variables() {
    todo!("original: I_BindVideoVariables")
}

/// C function: I_InitWindowTitle
pub fn i_init_window_title() {
    todo!("original: I_InitWindowTitle")
}

/// C function: I_InitWindowIcon
pub fn i_init_window_icon() {
    todo!("original: I_InitWindowIcon")
}

/// C function: I_StartFrame
pub fn i_start_frame() {
    todo!("original: I_StartFrame")
}

/// C function: I_StartTic
pub fn i_start_tic() {
    todo!("original: I_StartTic")
}

/// C function: I_EnableLoadingDisk
pub fn i_enable_loading_disk() {
    todo!("original: I_EnableLoadingDisk")
}

/// C function: I_EndRead
pub fn i_end_read() {
    todo!("original: I_EndRead")
}

/// C struct: color
#[repr(C)]
/// C typedef: color_t
pub struct Color {
    pub b: u32,
    pub g: u32,
    pub r: u32,
    pub a: u32,
}

/// C extern
pub static mut video_driver: *mut i8 = std::ptr::null_mut();
pub static mut screenvisible: boolean = crate::doomtype::Boolean::False;
pub static mut mouse_acceleration: f32 = 0.0;
pub static mut mouse_threshold: i32 = 0;
pub static mut vanilla_keyboard_mapping: i32 = 0;
pub static mut screensaver_mode: boolean = crate::doomtype::Boolean::False;
pub static mut usegamma: i32 = 0;
pub static mut I_VIDEO_BUFFER: *mut byte = std::ptr::null_mut();
pub static mut screen_width: i32 = 0;
pub static mut screen_height: i32 = 0;
pub static mut screen_bpp: i32 = 0;
pub static mut fullscreen: i32 = 0;
pub static mut aspect_ratio_correct: i32 = 0;
pub static mut show_diskicon: i32 = 0;
pub static mut diskicon_readbytes: i32 = 0;
