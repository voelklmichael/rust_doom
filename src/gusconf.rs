// gusconf.h / gusconf.c

pub use crate::doomtype::*;
pub use crate::w_wad::*;
pub use crate::z_zone::*;

use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;

// Original: #define MAX_INSTRUMENTS 256 (from gusconf.c)
pub const MAX_INSTRUMENTS: usize = 256;

#[allow(non_camel_case_types)]
pub struct GusconfState {
    // Original: char *gus_patch_path
    pub gus_patch_path: RefCell<CString>,
    // Original: unsigned int gus_ram_kb
    pub gus_ram_kb: RefCell<u32>,
}

impl GusconfState {
    pub fn new() -> Self {
        Self {
            gus_patch_path: RefCell::new(CString::new("").unwrap()),
            gus_ram_kb: RefCell::new(1024),
        }
    }

    // Original: GUS_WriteConfig
    pub fn gus_write_config(&self, _path: *mut c_char) -> Boolean {
        let _ = _path;
        todo!("GUS_WriteConfig")
    }
}
