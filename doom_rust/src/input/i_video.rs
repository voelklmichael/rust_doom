//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Video init, palette, screen blit. Platform-specific.
//
// Original: i_video.h + i_video.c

use crate::rendering::VIEWIMAGE;

/// Screen dimensions for aspect-ratio modes.
pub const SCREENWIDTH_4_3: i32 = 256;
pub const SCREENHEIGHT_4_3: i32 = 240;

/// Max mouse buttons.
pub const MAX_MOUSE_BUTTONS: i32 = 8;

/// Palette color (8-bit RGBA). Original: struct color
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Screen mode descriptor. Original: screen_mode_t
#[derive(Clone, Debug)]
pub struct ScreenMode {
    pub width: i32,
    pub height: i32,
    /// Called with palette when switching to this mode. None = no init.
    pub init_mode: Option<fn(*const u8)>,
    /// Draw screen region. Returns true if successful.
    pub draw_screen: Option<fn(i32, i32, i32, i32) -> bool>,
    /// If true, autoadjust prefers other modes in fullscreen (poor quality).
    pub poor_quality: bool,
}

/// Callback for mouse grab. Original: grabmouse_callback_t
pub type GrabmouseCallback = fn() -> bool;

/// Set callback for mouse grab. Original: I_SetGrabMouseCallback
pub fn i_set_grab_mouse_callback(_callback: Option<GrabmouseCallback>) {
    // Stub
}

/// Initialize graphics. Platform: create window, alloc buffer.
/// Original: I_InitGraphics
pub fn i_init_graphics() {
    // Stub: v_video already provides SCREENS/VIEWIMAGE
}

/// Check command line for video options.
/// Original: I_GraphicsCheckCommandLine
pub fn i_graphics_check_command_line() {
    // Stub
}

/// Shutdown graphics.
/// Original: I_ShutdownGraphics
pub fn i_shutdown_graphics() {
    // Stub
}

/// Apply 256-color palette to display.
/// Original: I_SetPalette
pub fn i_set_palette(_palette: *const u8) {
    // Stub: platform-specific
}

/// Find palette index closest to given RGB. Original: I_GetPaletteIndex
pub fn i_get_palette_index(r: u8, g: u8, b: u8) -> i32 {
    // Stub: would search palette (768 bytes: 256 * 3) for closest match
    let _ = (r, g, b);
    0
}

/// Copy buffer to screen (no vsync).
/// Original: I_UpdateNoBlit
pub fn i_update_no_blit() {
    // Stub
}

/// Present frame, vsync.
/// Original: I_FinishUpdate
pub fn i_finish_update() {
    // Stub
}

/// Set window title.
/// Original: I_SetWindowTitle
pub fn i_set_window_title(_title: &str) {
    // Stub
}

/// Check if running as screensaver.
/// Original: I_CheckIsScreensaver
pub fn i_check_is_screensaver() {
    // Stub
}

/// Called before processing tics in a frame.
/// Original: I_StartFrame
pub fn i_start_frame() {
    // Stub
}

/// Called before each tic in a frame.
/// Original: I_StartTic
pub fn i_start_tic() {
    // Stub
}

/// Enable loading disk icon.
/// Original: I_EnableLoadingDisk
pub fn i_enable_loading_disk() {
    // Stub
}

/// Get video buffer. Points to VIEWIMAGE.
/// Original: I_VideoBuffer
pub fn i_video_buffer() -> *mut u8 {
    unsafe { VIEWIMAGE }
}

/// Copy screen buffer to destination. Original: I_ReadScreen
pub fn i_read_screen(dest: *mut u8) {
    crate::rendering::v_read_screen(dest);
}
