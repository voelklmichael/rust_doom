//! Rust translation of doomgeneric/net_client.h
//! Network client code.

use crate::d_ticcmd::*;
use crate::doomtype::*;
use crate::net_defs::*;

/// C function: NET_CL_Connect
pub fn net_cl_connect(addr: *mut NetAddrT, data: *mut NetConnectDataT) -> Boolean {
    todo!("original: NET_CL_Connect")
}

/// C function: NET_CL_Disconnect
pub fn net_cl_disconnect() {
    todo!("original: NET_CL_Disconnect")
}

/// C function: NET_CL_Run
pub fn net_cl_run() {
    todo!("original: NET_CL_Run")
}

/// C function: NET_CL_Init
pub fn net_cl_init() {
    todo!("original: NET_CL_Init")
}

/// C function: NET_CL_LaunchGame
pub fn net_cl_launch_game() {
    todo!("original: NET_CL_LaunchGame")
}

/// C function: NET_CL_StartGame
pub fn net_cl_start_game(settings: *mut NetGamesettingsT) {
    todo!("original: NET_CL_StartGame")
}

/// C function: NET_CL_SendTiccmd
pub fn net_cl_send_ticcmd(ticcmd: *mut TiccmdT, maketic: i32) {
    todo!("original: NET_CL_SendTiccmd")
}

/// C function: NET_CL_GetSettings
pub fn net_cl_get_settings(settings: *mut NetGamesettingsT) -> Boolean {
    todo!("original: NET_CL_GetSettings")
}

/// C function: NET_Init
pub fn net_init() {
    todo!("original: NET_Init")
}

/// C function: NET_BindVariables
pub fn net_bind_variables() {
    todo!("original: NET_BindVariables")
}

pub static mut net_client_connected: Boolean = Boolean::False;
pub static mut net_client_received_wait_data: Boolean = Boolean::False;
pub static mut net_client_wait_data: NetWaitdataT = NetWaitdataT::default();
pub static mut net_waiting_for_launch: Boolean = Boolean::False;
pub static mut net_player_name: *mut i8 = core::ptr::null_mut();
pub static mut net_server_wad_sha1sum: crate::sha1::Sha1DigestT = [0u8; 20];
pub static mut net_server_deh_sha1sum: crate::sha1::Sha1DigestT = [0u8; 20];
pub static mut net_server_is_freedoom: u32 = 0;
pub static mut net_local_wad_sha1sum: crate::sha1::Sha1DigestT = [0u8; 20];
pub static mut net_local_deh_sha1sum: crate::sha1::Sha1DigestT = [0u8; 20];
pub static mut net_local_is_freedoom: u32 = 0;
pub static mut drone: Boolean = Boolean::False;
