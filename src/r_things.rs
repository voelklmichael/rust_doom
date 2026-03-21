// doomgeneric/r_things.h

pub use crate::r_defs::*;

use std::cell::RefCell;

// Original: #define MAXVISSPRITES 128
pub const MAXVISSPRITES: usize = 128;

#[allow(non_camel_case_types)]
pub struct R_ThingsState {
    pub _placeholder: RefCell<()>,
}

impl R_ThingsState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    pub fn r_draw_masked_column(&self, _column: *mut ColumnT) {
        todo!("R_DrawMaskedColumn");
    }

    pub fn r_sort_vis_sprites(&self) {
        todo!("R_SortVisSprites");
    }

    pub fn r_add_sprites(&self, _sec: *mut SectorT) {
        todo!("R_AddSprites");
    }

    pub fn r_add_psprites(&self) {
        todo!("R_AddPSprites");
    }

    pub fn r_draw_sprites(&self) {
        todo!("R_DrawSprites");
    }

    pub fn r_init_sprites(&self, _namelist: *mut *mut std::ffi::c_char) {
        todo!("R_InitSprites");
    }

    pub fn r_clear_sprites(&self) {
        todo!("R_ClearSprites");
    }

    pub fn r_draw_masked(&self) {
        todo!("R_DrawMasked");
    }
}
