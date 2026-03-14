//! Platform I/O: timer, video, input, joystick, scaling, ENDOOM, CD music.
//!
//! Original: i_timer, i_video, i_input, i_joystick, i_scale, i_endoom, i_cdmus
//!
//! See `docs/INPUT_TRANSLATION_PLAN.md` for porting plan.

pub mod i_cdmus;
pub mod i_endoom;
pub mod i_input;
pub mod i_joystick;
pub mod i_scale;
pub mod i_video;
