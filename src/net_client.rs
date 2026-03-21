// net_client.h - network client

pub use crate::doomtype::*;
pub use crate::d_ticcmd::*;
pub use crate::sha1::*;
pub use crate::net_defs::*;

// Original: NET_CL_Connect
pub fn net_cl_connect(_addr: *mut NetAddrT, _data: *mut NetConnectDataT) -> Boolean {
    todo!("NET_CL_Connect")
}

// Original: NET_CL_Disconnect
pub fn net_cl_disconnect() {
    todo!("NET_CL_Disconnect")
}

// Original: NET_CL_Run
pub fn net_cl_run() {
    todo!("NET_CL_Run")
}

// Original: NET_CL_Init
pub fn net_cl_init() {
    todo!("NET_CL_Init")
}

// Original: NET_CL_LaunchGame
pub fn net_cl_launch_game() {
    todo!("NET_CL_LaunchGame")
}

// Original: NET_CL_StartGame
pub fn net_cl_start_game(_settings: *mut NetGamesettingsT) {
    todo!("NET_CL_StartGame")
}

// Original: NET_CL_SendTiccmd
pub fn net_cl_send_ticcmd(_ticcmd: *mut TiccmdT, _maketic: i32) {
    todo!("NET_CL_SendTiccmd")
}

// Original: NET_CL_GetSettings
pub fn net_cl_get_settings(_settings: *mut NetGamesettingsT) -> Boolean {
    todo!("NET_CL_GetSettings")
}

// Original: NET_Init
pub fn net_init() {
    todo!("NET_Init")
}

// Original: NET_BindVariables
pub fn net_bind_variables() {
    todo!("NET_BindVariables")
}

// Original: extern boolean net_client_connected
pub fn net_client_connected() -> Boolean {
    todo!("net_client_connected: extern variable")
}

// Original: extern boolean net_client_received_wait_data
pub fn net_client_received_wait_data() -> Boolean {
    todo!("net_client_received_wait_data: extern variable")
}

// Original: extern net_waitdata_t net_client_wait_data
pub fn net_client_wait_data() -> NetWaitdataT {
    todo!("net_client_wait_data: extern variable")
}

// Original: extern boolean net_waiting_for_launch
pub fn net_waiting_for_launch() -> Boolean {
    todo!("net_waiting_for_launch: extern variable")
}

// Original: extern char *net_player_name
pub fn net_player_name() -> *mut i8 {
    todo!("net_player_name: extern variable")
}

// Original: extern sha1_digest_t net_server_wad_sha1sum
pub fn net_server_wad_sha1sum() -> Sha1DigestT {
    todo!("net_server_wad_sha1sum: extern variable")
}

// Original: extern sha1_digest_t net_server_deh_sha1sum
pub fn net_server_deh_sha1sum() -> Sha1DigestT {
    todo!("net_server_deh_sha1sum: extern variable")
}

// Original: extern unsigned int net_server_is_freedoom
pub fn net_server_is_freedoom() -> u32 {
    todo!("net_server_is_freedoom: extern variable")
}

// Original: extern sha1_digest_t net_local_wad_sha1sum
pub fn net_local_wad_sha1sum() -> Sha1DigestT {
    todo!("net_local_wad_sha1sum: extern variable")
}

// Original: extern sha1_digest_t net_local_deh_sha1sum
pub fn net_local_deh_sha1sum() -> Sha1DigestT {
    todo!("net_local_deh_sha1sum: extern variable")
}

// Original: extern unsigned int net_local_is_freedoom
pub fn net_local_is_freedoom() -> u32 {
    todo!("net_local_is_freedoom: extern variable")
}

// Original: extern boolean drone
pub fn drone() -> Boolean {
    todo!("drone: extern variable")
}
