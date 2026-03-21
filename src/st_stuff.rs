// doomgeneric/st_stuff.h

pub use crate::am_map::*;
pub use crate::d_event::*;
pub use crate::deh_main::*;
pub use crate::deh_misc::*;
pub use crate::doomdef::*;
pub use crate::doomkeys::*;
pub use crate::doomstat::*;
pub use crate::doomtype::*;
pub use crate::dstrings::*;
// g_game omitted (circular with g_game → st_stuff)
pub use crate::i_system::*;
pub use crate::i_video::*;
pub use crate::m_cheat::*;
pub use crate::m_misc::*;
pub use crate::m_random::*;
pub use crate::p_inter::*;
pub use crate::p_local::*;
pub use crate::r_local::*;
pub use crate::s_sound::*;
pub use crate::sounds::*;
pub use crate::st_lib::*;
pub use crate::v_video::*;
pub use crate::w_wad::*;
pub use crate::z_zone::*;

use std::cell::RefCell;

// Original: ST_HEIGHT, ST_WIDTH, ST_Y
pub const ST_HEIGHT: i32 = 32;
pub const ST_WIDTH: i32 = crate::i_video::SCREENWIDTH;
pub const ST_Y: i32 = crate::i_video::SCREENHEIGHT - ST_HEIGHT;

// Original: (zero-init cheatseq_t)
fn empty_cheat() -> CheatseqT {
    CheatseqT {
        sequence: [0; MAX_CHEAT_LEN],
        sequence_len: 0,
        parameter_chars: 0,
        chars_read: 0,
        param_chars_read: 0,
        parameter_buf: [0; MAX_CHEAT_PARAMS],
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StStateenumT {
    AutomapState = 0,
    FirstPersonState = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StChatstateenumT {
    StartChatState = 0,
    WaitDestState = 1,
    GetChatState = 2,
}

#[allow(non_camel_case_types)]
pub struct St_StuffState {
    // Original: st_backing_screen
    pub st_backing_screen: RefCell<*mut Byte>,
    // Original: cheat_mus
    pub cheat_mus: RefCell<CheatseqT>,
    // Original: cheat_god
    pub cheat_god: RefCell<CheatseqT>,
    // Original: cheat_ammo
    pub cheat_ammo: RefCell<CheatseqT>,
    // Original: cheat_ammonokey
    pub cheat_ammonokey: RefCell<CheatseqT>,
    // Original: cheat_noclip
    pub cheat_noclip: RefCell<CheatseqT>,
    // Original: cheat_commercial_noclip
    pub cheat_commercial_noclip: RefCell<CheatseqT>,
    // Original: cheat_powerup
    pub cheat_powerup: RefCell<[CheatseqT; 7]>,
    // Original: cheat_choppers
    pub cheat_choppers: RefCell<CheatseqT>,
    // Original: cheat_clev
    pub cheat_clev: RefCell<CheatseqT>,
    // Original: cheat_mypos
    pub cheat_mypos: RefCell<CheatseqT>,
}

impl St_StuffState {
    pub fn new() -> Self {
        Self {
            st_backing_screen: RefCell::new(std::ptr::null_mut()),
            cheat_mus: RefCell::new(empty_cheat()),
            cheat_god: RefCell::new(empty_cheat()),
            cheat_ammo: RefCell::new(empty_cheat()),
            cheat_ammonokey: RefCell::new(empty_cheat()),
            cheat_noclip: RefCell::new(empty_cheat()),
            cheat_commercial_noclip: RefCell::new(empty_cheat()),
            cheat_powerup: RefCell::new(std::array::from_fn(|_| empty_cheat())),
            cheat_choppers: RefCell::new(empty_cheat()),
            cheat_clev: RefCell::new(empty_cheat()),
            cheat_mypos: RefCell::new(empty_cheat()),
        }
    }

    // Original: ST_Responder
    pub fn st_responder(&self, _ev: *mut EventT) -> Boolean {
        todo!("ST_Responder");
    }

    // Original: ST_Ticker
    pub fn st_ticker(&self) {
        todo!("ST_Ticker");
    }

    // Original: ST_Drawer
    pub fn st_drawer(&self, _fullscreen: Boolean, _refresh: Boolean) {
        todo!("ST_Drawer");
    }

    // Original: ST_Start
    pub fn st_start(&self) {
        todo!("ST_Start");
    }

    // Original: ST_Init
    pub fn st_init(&self) {
        todo!("ST_Init");
    }
}
