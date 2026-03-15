//! Rust translation of doomgeneric/r_draw.h

use crate::doomtype::*;
use crate::r_defs::*;

pub static mut dc_colormap: *mut LighttableT = std::ptr::null_mut();
pub static mut dc_x: i32 = 0;
pub static mut dc_yl: i32 = 0;
pub static mut dc_yh: i32 = 0;
pub static mut dc_iscale: crate::m_fixed::FixedT = 0;
pub static mut dc_texturemid: crate::m_fixed::FixedT = 0;
pub static mut dc_source: *mut byte = std::ptr::null_mut();

pub fn r_draw_column() {
    todo!("original: R_DrawColumn")
}

pub fn r_draw_column_low() {
    todo!("original: R_DrawColumnLow")
}

pub fn r_draw_fuzz_column() {
    todo!("original: R_DrawFuzzColumn")
}

pub fn r_draw_fuzz_column_low() {
    todo!("original: R_DrawFuzzColumnLow")
}

pub fn r_draw_translated_column() {
    todo!("original: R_DrawTranslatedColumn")
}

pub fn r_draw_translated_column_low() {
    todo!("original: R_DrawTranslatedColumnLow")
}

pub fn r_video_erase(ofs: u32, count: i32) {
    todo!("original: R_VideoErase")
}

pub static mut ds_y: i32 = 0;
pub static mut ds_x1: i32 = 0;
pub static mut ds_x2: i32 = 0;
pub static mut ds_colormap: *mut LighttableT = std::ptr::null_mut();
pub static mut ds_xfrac: crate::m_fixed::FixedT = 0;
pub static mut ds_yfrac: crate::m_fixed::FixedT = 0;
pub static mut ds_xstep: crate::m_fixed::FixedT = 0;
pub static mut ds_ystep: crate::m_fixed::FixedT = 0;
pub static mut ds_source: *mut byte = std::ptr::null_mut();

pub static mut translationtables: *mut byte = std::ptr::null_mut();
pub static mut dc_translation: *mut byte = std::ptr::null_mut();

pub fn r_draw_span() {
    todo!("original: R_DrawSpan")
}

pub fn r_draw_span_low() {
    todo!("original: R_DrawSpanLow")
}

pub fn r_init_buffer(width: i32, height: i32) {
    todo!("original: R_InitBuffer")
}

pub fn r_init_translation_tables() {
    todo!("original: R_InitTranslationTables")
}

pub fn r_fill_back_screen() {
    todo!("original: R_FillBackScreen")
}

pub fn r_draw_view_border() {
    todo!("original: R_DrawViewBorder")
}
