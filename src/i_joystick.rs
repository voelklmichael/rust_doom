// doomgeneric/i_joystick.h

pub use crate::d_event::*;
pub use crate::doomtype::*;
pub use crate::i_system::*;
pub use crate::m_config::*;
pub use crate::m_misc::*;

use std::cell::RefCell;

// Original: #define NUM_VIRTUAL_BUTTONS 10
pub const NUM_VIRTUAL_BUTTONS: i32 = 10;
// Original: #define BUTTON_AXIS 0x10000
pub const BUTTON_AXIS: i32 = 0x10000;
// Original: #define HAT_AXIS 0x20000
pub const HAT_AXIS: i32 = 0x20000;
pub const HAT_AXIS_HORIZONTAL: i32 = 1;
pub const HAT_AXIS_VERTICAL: i32 = 2;

// Original: #define IS_BUTTON_AXIS(axis) ...
#[inline]
pub fn is_button_axis(axis: i32) -> bool {
    axis >= 0 && (axis & BUTTON_AXIS) != 0
}

#[inline]
// Original: BUTTON_AXIS_NEG
pub fn button_axis_neg(axis: i32) -> i32 {
    axis & 0xff
}

#[inline]
// Original: BUTTON_AXIS_POS
pub fn button_axis_pos(axis: i32) -> i32 {
    (axis >> 8) & 0xff
}

#[inline]
// Original: CREATE_BUTTON_AXIS
pub fn create_button_axis(neg: i32, pos: i32) -> i32 {
    BUTTON_AXIS | neg | (pos << 8)
}

#[inline]
// Original: IS_HatAxis
pub fn is_hat_axis(axis: i32) -> bool {
    axis >= 0 && (axis & HAT_AXIS) != 0
}

#[inline]
// Original: HAT_AXIS_HAT
pub fn hat_axis_hat(axis: i32) -> i32 {
    axis & 0xff
}

#[inline]
// Original: HAT_AXIS_DIRECTION
pub fn hat_axis_direction(axis: i32) -> i32 {
    (axis >> 8) & 0xff
}

#[inline]
// Original: CREATE_HAT_AXIS
pub fn create_hat_axis(hat: i32, direction: i32) -> i32 {
    HAT_AXIS | hat | (direction << 8)
}

#[allow(non_camel_case_types)]
pub struct I_JoystickState {
    pub _placeholder: RefCell<()>,
}

impl I_JoystickState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: I_InitJoystick
    pub fn i_init_joystick(&self) {
        todo!("I_InitJoystick");
    }

    // Original: I_ShutdownJoystick
    pub fn i_shutdown_joystick(&self) {
        todo!("I_ShutdownJoystick");
    }

    // Original: I_UpdateJoystick
    pub fn i_update_joystick(&self) {
        todo!("I_UpdateJoystick");
    }

    // Original: I_BindJoystickVariables
    pub fn i_bind_joystick_variables(&self) {
        todo!("I_BindJoystickVariables");
    }
}
