//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Joystick/gamepad input.
//
// Original: i_joystick.h + i_joystick.c

/// Number of virtual joystick buttons.
pub const NUM_VIRTUAL_BUTTONS: i32 = 10;

/// Initialize joystick.
/// Original: I_InitJoystick
pub fn i_init_joystick() {
    // Stub: platform-specific
}

/// Shutdown joystick.
/// Original: I_ShutdownJoystick
pub fn i_shutdown_joystick() {
    // Stub
}

/// Poll joystick, update state.
/// Original: I_UpdateJoystick
pub fn i_update_joystick() {
    // Stub
}

/// Bind joystick config variables.
/// Original: I_BindJoystickVariables
pub fn i_bind_joystick_variables() {
    // Stub
}
