//! Rust translation of doomgeneric/d_loop.h
//! Main loop stuff.

use crate::d_ticcmd::*;
use crate::doomtype::*;
use crate::net_defs::*;

/// C typedef: netgame_startup_callback_t
pub type NetgameStartupCallbackT = Option<extern "C" fn(i32, i32) -> Boolean>;

/// C typedef: loop_interface_t
#[repr(C)]
/// C typedef: loop_interface_t
pub struct LoopInterfaceT {
    pub process_events: Option<extern "C" fn()>,
    pub build_ticcmd: Option<extern "C" fn(*mut TiccmdT, i32)>,
    pub run_tic: Option<extern "C" fn(*mut TiccmdT, *mut Boolean)>,
    pub run_menu: Option<extern "C" fn()>,
}

/// C function: D_RegisterLoopCallbacks
pub fn d_register_loop_callbacks(i: &mut LoopInterfaceT) {
    todo!("original: D_RegisterLoopCallbacks")
}

/// C function: NetUpdate
pub fn net_update() {
    todo!("original: NetUpdate")
}

/// C function: D_QuitNetGame
pub fn d_quit_net_game() {
    todo!("original: D_QuitNetGame")
}

/// C function: TryRunTics
pub fn try_run_tics() {
    todo!("original: TryRunTics")
}

/// C function: D_StartGameLoop
pub fn d_start_game_loop() {
    todo!("original: D_StartGameLoop")
}

/// C function: D_InitNetGame
pub fn d_init_net_game(connect_data: &mut NetConnectDataT) -> Boolean {
    todo!("original: D_InitNetGame")
}

/// C function: D_StartNetGame
pub fn d_start_net_game(
    settings: *mut NetGamesettingsT,
    callback: NetgameStartupCallbackT,
) {
    todo!("original: D_StartNetGame")
}

pub static mut singletics: Boolean = Boolean::False;
pub static mut gametic: i32 = 0;
pub static mut ticdup: i32 = 0;
