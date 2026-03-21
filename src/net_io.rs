// net_io.h - network I/O

pub use crate::net_defs::*;

// Original: extern net_addr_t net_broadcast_addr
pub fn net_broadcast_addr() -> NetAddrT {
    todo!("net_broadcast_addr: extern variable")
}

// Original: NET_NewContext
pub fn net_new_context() -> *mut NetContextT {
    todo!("NET_NewContext")
}

// Original: NET_AddModule
pub fn net_add_module(_context: *mut NetContextT, _module: *mut NetModuleT) {
    todo!("NET_AddModule")
}

// Original: NET_SendPacket
pub fn net_send_packet(_addr: *mut NetAddrT, _packet: *mut NetPacketT) {
    todo!("NET_SendPacket")
}

// Original: NET_SendBroadcast
pub fn net_send_broadcast(_context: *mut NetContextT, _packet: *mut NetPacketT) {
    todo!("NET_SendBroadcast")
}

// Original: NET_RecvPacket
pub fn net_recv_packet(
    _context: *mut NetContextT,
    _addr: *mut *mut NetAddrT,
    _packet: *mut *mut NetPacketT,
) -> Boolean {
    todo!("NET_RecvPacket")
}

// Original: NET_AddrToString
pub fn net_addr_to_string(_addr: *mut NetAddrT) -> *mut i8 {
    todo!("NET_AddrToString")
}

// Original: NET_FreeAddress
pub fn net_free_address(_addr: *mut NetAddrT) {
    todo!("NET_FreeAddress")
}

// Original: NET_ResolveAddress
pub fn net_resolve_address(_context: *mut NetContextT, _address: *mut i8) -> *mut NetAddrT {
    todo!("NET_ResolveAddress")
}
