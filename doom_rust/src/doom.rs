//! Rust translation of doomgeneric/doom.h

use crate::d_main;

/// C function: D_DoomMain - entry point, delegates to d_main
pub fn d_doom_main() {
    d_main::d_doom_main();
}
