//! Rust translation of doomgeneric/gusconf.h

use crate::doomtype::*;

/// C extern: gus_patch_path
pub static mut gus_patch_path: *mut i8 = std::ptr::null_mut();

/// C extern: gus_ram_kb
pub static mut gus_ram_kb: u32 = 0;

/// C function: GUS_WriteConfig
pub fn gus_write_config(path: &str) -> boolean {
    todo!("original: GUS_WriteConfig")
}
