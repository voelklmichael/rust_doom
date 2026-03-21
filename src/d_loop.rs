// doomgeneric/d_loop.h

use std::cell::RefCell;

pub use crate::doomtype::*;
pub use crate::d_ticcmd::*;
pub use crate::net_defs::*;

/// Original: typedef boolean (*netgame_startup_callback_t)(int, int)
pub type NetgameStartupCallbackT = fn(i32, i32) -> Boolean;

/// Original: typedef struct { ... } loop_interface_t
#[derive(Clone, Copy)]
pub struct LoopInterfaceT {
    pub process_events: Option<fn()>,
    pub build_ticcmd: Option<fn(*mut TiccmdT, i32)>,
    pub run_tic: Option<fn(*mut TiccmdT, *mut Boolean)>,
    pub run_menu: Option<fn()>,
}

#[allow(non_camel_case_types)]
pub struct D_LoopState {
    /// Original: extern boolean singletics
    pub singletics: RefCell<Boolean>,
    /// Original: extern int gametic, ticdup
    pub gametic: RefCell<i32>,
    pub ticdup: RefCell<i32>,
    pub loop_interface: RefCell<Option<LoopInterfaceT>>,
}

impl D_LoopState {
    pub fn new() -> Self {
        Self {
            singletics: RefCell::new(Boolean::False),
            gametic: RefCell::new(0),
            ticdup: RefCell::new(0),
            loop_interface: RefCell::new(None),
        }
    }

    // Original: D_RegisterLoopCallbacks
    pub fn d_register_loop_callbacks(&self, _i: *mut LoopInterfaceT) {
        todo!("D_RegisterLoopCallbacks");
    }

    // Original: NetUpdate
    pub fn net_update(&self) {
        todo!("NetUpdate");
    }

    // Original: D_QuitNetGame
    pub fn d_quit_net_game(&self) {
        todo!("D_QuitNetGame");
    }

    // Original: TryRunTics
    pub fn try_run_tics(&self) {
        todo!("TryRunTics");
    }

    // Original: D_StartGameLoop
    pub fn d_start_game_loop(&self) {
        todo!("D_StartGameLoop");
    }

    // Original: D_InitNetGame
    pub fn d_init_net_game(&self, _connect_data: *mut NetConnectDataT) -> Boolean {
        todo!("D_InitNetGame");
    }

    // Original: D_StartNetGame
    pub fn d_start_net_game(&self, _settings: *mut NetGamesettingsT, _callback: NetgameStartupCallbackT) {
        todo!("D_StartNetGame");
    }
}
