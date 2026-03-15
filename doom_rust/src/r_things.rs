//! Rust translation of doomgeneric/r_things.h
//! Rendering of moving objects, sprites.

use crate::i_video::*;
use crate::r_defs::*;
use crate::v_patch::*;

pub const MAXVISSPRITES: usize = 128;

pub static mut vissprites: [VisspriteT; MAXVISSPRITES] = [VisspriteT {
    prev: std::ptr::null_mut(),
    next: std::ptr::null_mut(),
    x1: 0,
    x2: 0,
    gx: 0,
    gy: 0,
    gz: 0,
    gzt: 0,
    startfrac: 0,
    scale: 0,
    xiscale: 0,
    texturemid: 0,
    patch: 0,
    colormap: std::ptr::null_mut(),
    mobjflags: 0,
}; MAXVISSPRITES];
pub static mut vissprite_p: *mut VisspriteT = std::ptr::null_mut();
pub static mut vsprsortedhead: VisspriteT = VisspriteT {
    prev: std::ptr::null_mut(),
    next: std::ptr::null_mut(),
    x1: 0,
    x2: 0,
    gx: 0,
    gy: 0,
    gz: 0,
    gzt: 0,
    startfrac: 0,
    scale: 0,
    xiscale: 0,
    texturemid: 0,
    patch: 0,
    colormap: std::ptr::null_mut(),
    mobjflags: 0,
};

pub static mut negonearray: [i16; SCREENWIDTH as usize] = [0; SCREENWIDTH as usize];
pub static mut screenheightarray: [i16; SCREENWIDTH as usize] = [0; SCREENWIDTH as usize];

pub static mut mfloorclip: *mut i16 = std::ptr::null_mut();
pub static mut mceilingclip: *mut i16 = std::ptr::null_mut();
pub static mut spryscale: crate::m_fixed::FixedT = 0;
pub static mut sprtopscreen: crate::m_fixed::FixedT = 0;
pub static mut pspritescale: crate::m_fixed::FixedT = 0;
pub static mut pspriteiscale: crate::m_fixed::FixedT = 0;

pub fn r_draw_masked_column(column: *mut ColumnT) {
    todo!("original: R_DrawMaskedColumn")
}

pub fn r_sort_vis_sprites() {
    todo!("original: R_SortVisSprites")
}

pub fn r_add_sprites(sec: *mut SectorT) {
    todo!("original: R_AddSprites")
}

pub fn r_add_psprites() {
    todo!("original: R_AddPSprites")
}

pub fn r_draw_sprites() {
    todo!("original: R_DrawSprites")
}

pub fn r_init_sprites(namelist: *mut *mut i8) {
    todo!("original: R_InitSprites")
}

pub fn r_clear_sprites() {
    todo!("original: R_ClearSprites")
}

pub fn r_draw_masked() {
    todo!("original: R_DrawMasked")
}

pub fn r_clip_vis_sprite(vis: *mut VisspriteT, xl: i32, xh: i32) {
    todo!("original: R_ClipVisSprite")
}
