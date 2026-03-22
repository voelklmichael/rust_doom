//! Render data (r_data.h, r_data.c)
//! Original: r_data.h, r_data.c

use crate::doomtype::Byte;

pub struct R_DataState;

impl R_DataState {
    /// Original: byte *R_GetColumn(int tex, int col)
    pub fn r_get_column(&self, _tex: i32, _col: i32) -> Option<Vec<Byte>> {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_InitData(void)
    pub fn r_init_data(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void R_PrecacheLevel(void)
    pub fn r_precache_level(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: int R_FlatNumForName(char *name)
    pub fn r_flat_num_for_name(&self, _name: &str) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int R_TextureNumForName(char *name)
    pub fn r_texture_num_for_name(&self, _name: &str) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int R_CheckTextureNumForName(char *name)
    pub fn r_check_texture_num_for_name(&self, _name: &str) -> i32 {
        todo!("Basic stage-0 stub")
    }
}
