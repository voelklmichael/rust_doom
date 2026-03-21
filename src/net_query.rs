// net_query.h - server querying

pub use crate::net_defs::*;

/// Original: net_query_callback_t
pub type NetQueryCallbackT =
    fn(*mut NetAddrT, *mut NetQuerydataT, u32, *mut std::ffi::c_void);

// Original: extern int NET_StartLANQuery
pub fn net_start_lan_query() -> i32 {
    todo!("NET_StartLANQuery")
}

// Original: extern int NET_StartMasterQuery
pub fn net_start_master_query() -> i32 {
    todo!("NET_StartMasterQuery")
}

/// Original: NET_LANQuery
pub fn net_lan_query() {
    todo!("NET_LANQuery")
}

/// Original: NET_MasterQuery
pub fn net_master_query() {
    todo!("NET_MasterQuery")
}

/// Original: NET_QueryAddress
pub fn net_query_address(_addr: *mut i8) {
    todo!("NET_QueryAddress")
}

/// Original: NET_FindLANServer
pub fn net_find_lan_server() -> *mut NetAddrT {
    todo!("NET_FindLANServer")
}

/// Original: NET_Query_Poll
pub fn net_query_poll(_callback: NetQueryCallbackT, _user_data: *mut std::ffi::c_void) -> i32 {
    todo!("NET_Query_Poll")
}

/// Original: NET_Query_ResolveMaster
pub fn net_query_resolve_master(_context: *mut NetContextT) -> *mut NetAddrT {
    todo!("NET_Query_ResolveMaster")
}

/// Original: NET_Query_AddToMaster
pub fn net_query_add_to_master(_master_addr: *mut NetAddrT) {
    todo!("NET_Query_AddToMaster")
}

/// Original: NET_Query_CheckAddedToMaster
pub fn net_query_check_added_to_master(_result: *mut Boolean) -> Boolean {
    todo!("NET_Query_CheckAddedToMaster")
}

/// Original: NET_Query_MasterResponse
pub fn net_query_master_response(_packet: *mut NetPacketT) {
    todo!("NET_Query_MasterResponse")
}
