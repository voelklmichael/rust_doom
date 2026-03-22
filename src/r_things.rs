//! Things renderer (r_things.h, r_things.c)
//! Original: r_things.h, r_things.c

use crate::m_fixed::FixedT;

pub const MAXVISSPRITES: usize = 128;

pub struct VisspriteT;
pub struct ColumnT;

pub struct R_ThingsState;

impl R_ThingsState {
    /// Original: void R_DrawMaskedColumn(column_t *column)
    pub fn r_draw_masked_column(&self, _column: &ColumnT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_SortVisSprites(void)
    pub fn r_sort_vis_sprites(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_AddSprites(sector_t *sec)
    pub fn r_add_sprites(&self, _sec: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_AddPSprites(void)
    pub fn r_add_psprites(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_DrawSprites(void)
    pub fn r_draw_sprites(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_InitSprites(char **namelist)
    pub fn r_init_sprites(&self, _namelist: &[&str]) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_ClearSprites(void)
    pub fn r_clear_sprites(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_DrawMasked(void)
    pub fn r_draw_masked(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_ClipVisSprite(vissprite_t *vis, int xl, int xh)
    pub fn r_clip_vis_sprite(&self, _vis: &VisspriteT, _xl: i32, _xh: i32) {
        todo!("Basic stage-0 stub")
    }
}
