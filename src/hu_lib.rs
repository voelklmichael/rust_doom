//! HU widget library (hu_lib.h, hu_lib.c)
//! Original: hu_lib.h, hu_lib.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;

pub const HU_CHARERASE: i32 = 0x7f; // KEY_BACKSPACE
pub const HU_MAXLINES: usize = 4;
pub const HU_MAXLINELENGTH: usize = 80;

pub struct HuTextlineT {
    pub x: i32,
    pub y: i32,
    pub len: i32,
    pub needsupdate: i32,
}

pub struct HuStextT {
    pub h: i32,
    pub cl: i32,
}

pub struct HuItextT {
    pub lm: i32,
}

pub struct HuLibState;

impl HuLibState {
    /// Original: void HUlib_init(void)
    pub fn hu_init(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_clearTextLine(hu_textline_t *t)
    pub fn hu_clear_text_line(&self, _t: &mut HuTextlineT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_initTextLine(hu_textline_t *t, int x, int y, patch_t **f, int sc)
    pub fn hu_init_text_line(&self, _t: &mut HuTextlineT, _x: i32, _y: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean HUlib_addCharToTextLine(hu_textline_t *t, char ch)
    pub fn hu_add_char_to_text_line(&self, _t: &mut HuTextlineT, _ch: u8) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean HUlib_delCharFromTextLine(hu_textline_t *t)
    pub fn hu_del_char_from_text_line(&self, _t: &mut HuTextlineT) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_drawTextLine(hu_textline_t *l, boolean drawcursor)
    pub fn hu_draw_text_line(&self, _l: &HuTextlineT, _drawcursor: Boolean) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_initSText(hu_stext_t *s, int x, int y, int h, patch_t **f, int sc, ...)
    pub fn hu_init_stext(&self, _s: &mut HuStextT, _x: i32, _y: i32, _h: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_addLineToSText(hu_stext_t *s)
    pub fn hu_add_line_to_stext(&self, _s: &mut HuStextT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_addMessageToSText(hu_stext_t *s, char *prefix, char *msg)
    pub fn hu_add_message_to_stext(&self, _s: &mut HuStextT, _prefix: &str, _msg: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_drawSText(hu_stext_t *s)
    pub fn hu_draw_stext(&self, _s: &HuStextT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_eraseSText(hu_stext_t *s)
    pub fn hu_erase_stext(&self, _s: &HuStextT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_initIText(hu_itext_t *it)
    pub fn hu_init_itext(&self, _it: &mut HuItextT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_addCharToIText(hu_itext_t *it, char ch)
    pub fn hu_add_char_to_itext(&self, _it: &mut HuItextT, _ch: u8) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean HUlib_delCharFromIText(hu_itext_t *it)
    pub fn hu_del_char_from_itext(&self, _it: &mut HuItextT) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void HUlib_drawIText(hu_itext_t *it)
    pub fn hu_draw_itext(&self, _it: &HuItextT) {
        todo!("Basic stage-0 stub")
    }
}
