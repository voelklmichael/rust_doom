//! Rust translation of doomgeneric/net_query.h
//! Querying servers to find their current status.

use crate::doomtype::*;
use crate::net_defs::*;

/// C typedef: net_query_callback_t
pub type NetQueryCallbackT =
    Option<extern "C" fn(*mut NetAddrT, *mut NetQuerydataT, u32, *mut core::ffi::c_void)>;

/// C function: NET_StartLANQuery
pub fn net_start_lan_query() -> i32 {
    todo!("original: NET_StartLANQuery")
}

/// C function: NET_StartMasterQuery
pub fn net_start_master_query() -> i32 {
    todo!("original: NET_StartMasterQuery")
}

/// C function: NET_LANQuery
pub fn net_lan_query() {
    todo!("original: NET_LANQuery")
}

/// C function: NET_MasterQuery
pub fn net_master_query() {
    todo!("original: NET_MasterQuery")
}

/// C function: NET_QueryAddress
pub fn net_query_address(addr: &str) {
    todo!("original: NET_QueryAddress")
}

/// C function: NET_FindLANServer
pub fn net_find_lan_server() -> *mut NetAddrT {
    todo!("original: NET_FindLANServer")
}

/// C function: NET_Query_Poll
pub fn net_query_poll(callback: NetQueryCallbackT, user_data: &mut [u8]) -> i32 {
    todo!("original: NET_Query_Poll")
}

/// C function: NET_Query_ResolveMaster
pub fn net_query_resolve_master(context: &mut NetContextT) -> *mut NetAddrT {
    todo!("original: NET_Query_ResolveMaster")
}

/// C function: NET_Query_AddToMaster
pub fn net_query_add_to_master(master_addr: &mut NetAddrT) {
    todo!("original: NET_Query_AddToMaster")
}

/// C function: NET_Query_CheckAddedToMaster
pub fn net_query_check_added_to_master(result: &mut Boolean) -> Boolean {
    todo!("original: NET_Query_CheckAddedToMaster")
}

/// C function: NET_Query_MasterResponse
pub fn net_query_master_response(packet: &mut NetPacketT) {
    todo!("original: NET_Query_MasterResponse")
}
