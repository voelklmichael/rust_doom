//! Sky rendering (r_sky.h)
//! Original: r_sky.h

use std::sync::Arc;
use std::sync::Mutex;

// #define SKYFLATNAME "F_SKY1"
pub const SKYFLATNAME: &str = "F_SKY1";
// #define ANGLETOSKYSHIFT 22
pub const ANGLETOSKYSHIFT: i32 = 22;

pub struct R_SkyState {
    // extern int skytexture
    pub skytexture: Arc<Mutex<i32>>,
    // extern int skytexturemid
    pub skytexturemid: Arc<Mutex<i32>>,
}

impl R_SkyState {
    /// Original: void R_InitSkyMap(void)
    pub fn r_init_sky_map(&self) {
        todo!("Basic stage-0 stub")
    }
}
