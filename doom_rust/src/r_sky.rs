//! Rust translation of doomgeneric/r_sky.h
//! Sky rendering.

/// C #define: SKYFLATNAME
pub const SKYFLATNAME: &str = "F_SKY1";
/// C #define: ANGLETOSKYSHIFT
pub const ANGLETOSKYSHIFT: i32 = 22;

pub static mut skytexture: i32 = 0;
pub static mut skytexturemid: i32 = 0;

/// C function: R_InitSkyMap
pub fn r_init_sky_map() {
    todo!("original: R_InitSkyMap")
}
