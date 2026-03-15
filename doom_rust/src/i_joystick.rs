//! Rust translation of doomgeneric/i_joystick.h
//! System-specific joystick interface.

/// C #define: NUM_VIRTUAL_BUTTONS
pub const NUM_VIRTUAL_BUTTONS: i32 = 10;
/// C #define: BUTTON_AXIS
pub const BUTTON_AXIS: i32 = 0x10000;
/// C #define: HAT_AXIS
pub const HAT_AXIS: i32 = 0x20000;
/// C #define: HAT_AXIS_HORIZONTAL
pub const HAT_AXIS_HORIZONTAL: i32 = 1;
/// C #define: HAT_AXIS_VERTICAL
pub const HAT_AXIS_VERTICAL: i32 = 2;

/// C macro: IS_BUTTON_AXIS
#[inline]
pub fn is_button_axis(axis: i32) -> bool {
    axis >= 0 && (axis & BUTTON_AXIS) != 0
}

/// C macro: BUTTON_AXIS_NEG
#[inline]
pub fn button_axis_neg(axis: i32) -> i32 {
    axis & 0xff
}

/// C macro: BUTTON_AXIS_POS
#[inline]
pub fn button_axis_pos(axis: i32) -> i32 {
    (axis >> 8) & 0xff
}

/// C macro: CREATE_BUTTON_AXIS
#[inline]
pub fn create_button_axis(neg: i32, pos: i32) -> i32 {
    BUTTON_AXIS | neg | (pos << 8)
}

/// C macro: IS_HAT_AXIS
#[inline]
pub fn is_hat_axis(axis: i32) -> bool {
    axis >= 0 && (axis & HAT_AXIS) != 0
}

/// C macro: HAT_AXIS_HAT
#[inline]
pub fn hat_axis_hat(axis: i32) -> i32 {
    axis & 0xff
}

/// C macro: HAT_AXIS_DIRECTION
#[inline]
pub fn hat_axis_direction(axis: i32) -> i32 {
    (axis >> 8) & 0xff
}

/// C macro: CREATE_HAT_AXIS
#[inline]
pub fn create_hat_axis(hat: i32, direction: i32) -> i32 {
    HAT_AXIS | hat | (direction << 8)
}

/// C function: I_InitJoystick
pub fn i_init_joystick() {
    todo!("original: I_InitJoystick")
}

/// C function: I_ShutdownJoystick
pub fn i_shutdown_joystick() {
    todo!("original: I_ShutdownJoystick")
}

/// C function: I_UpdateJoystick
pub fn i_update_joystick() {
    todo!("original: I_UpdateJoystick")
}

/// C function: I_BindJoystickVariables
pub fn i_bind_joystick_variables() {
    todo!("original: I_BindJoystickVariables")
}
