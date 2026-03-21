// deh_str.h — no .c; behavior depends on FEATURE_DEHACKED

pub use crate::doomfeatures::*;

use std::cell::RefCell;
use std::os::raw::c_char;

#[allow(non_camel_case_types)]
pub struct Deh_StrState {
    _ph: RefCell<()>,
}

impl Deh_StrState {
    pub fn new() -> Self {
        Self {
            _ph: RefCell::new(()),
        }
    }

    // Original: DEH_String
    pub fn deh_string(&self, s: *mut c_char) -> *mut c_char {
        if FEATURE_DEHACKED {
            todo!("DEH_String when FEATURE_DEHACKED")
        } else {
            s
        }
    }

    // Original: DEH_printf
    pub fn deh_printf(&self, _fmt: *mut c_char) {
        let _ = _fmt;
        if FEATURE_DEHACKED {
            todo!("DEH_printf when FEATURE_DEHACKED")
        } else {
            todo!("DEH_printf: variadic (was printf)")
        }
    }

    // Original: DEH_fprintf
    pub fn deh_fprintf(&self, _fstream: *mut core::ffi::c_void, _fmt: *mut c_char) {
        let _ = (_fstream, _fmt);
        if FEATURE_DEHACKED {
            todo!("DEH_fprintf when FEATURE_DEHACKED")
        } else {
            todo!("DEH_fprintf: variadic (was fprintf)")
        }
    }

    // Original: DEH_snprintf
    pub fn deh_snprintf(&self, _buffer: *mut c_char, _len: usize, _fmt: *mut c_char) {
        let _ = (_buffer, _len, _fmt);
        if FEATURE_DEHACKED {
            todo!("DEH_snprintf when FEATURE_DEHACKED")
        } else {
            todo!("DEH_snprintf: variadic (was snprintf)")
        }
    }

    // Original: DEH_AddStringReplacement
    pub fn deh_add_string_replacement(&self, _from: *mut c_char, _to: *mut c_char) {
        if FEATURE_DEHACKED {
            todo!("DEH_AddStringReplacement")
        }
    }
}
