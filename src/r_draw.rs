// doomgeneric/r_draw.h

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct R_DrawState {
    pub _placeholder: RefCell<()>,
}

impl R_DrawState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    pub fn r_draw_column(&self) {
        todo!("R_DrawColumn");
    }

    pub fn r_draw_column_low(&self) {
        todo!("R_DrawColumnLow");
    }

    pub fn r_draw_fuzz_column(&self) {
        todo!("R_DrawFuzzColumn");
    }

    pub fn r_draw_fuzz_column_low(&self) {
        todo!("R_DrawFuzzColumnLow");
    }

    pub fn r_draw_span(&self) {
        todo!("R_DrawSpan");
    }

    pub fn r_draw_span_low(&self) {
        todo!("R_DrawSpanLow");
    }

    pub fn r_fill_back_screen(&self) {
        todo!("R_FillBackScreen");
    }

    pub fn r_draw_view_border(&self) {
        todo!("R_DrawViewBorder");
    }
}
