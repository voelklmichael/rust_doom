//! Rust translation of doomgeneric/net_io.h
//! Network packet manipulation (net_packet_t).

use crate::doomtype::*;
use crate::net_defs::*;
use std::sync::{Arc, Mutex};

pub static mut net_broadcast_addr: NetAddrT = NetAddrT {
    module: None,
    handle: None,
};

/// C function: NET_NewContext
pub fn net_new_context() -> Arc<Mutex<NetContextT>> {
    todo!("original: NET_NewContext")
}

/// C function: NET_AddModule
pub fn net_add_module(context: &mut NetContextT, module: &mut NetModuleT) {
    todo!("original: NET_AddModule")
}

/// C function: NET_SendPacket
pub fn net_send_packet(addr: &mut NetAddrT, packet: &mut NetPacketT) {
    todo!("original: NET_SendPacket")
}

/// C function: NET_SendBroadcast
pub fn net_send_broadcast(context: &mut NetContextT, packet: &mut NetPacketT) {
    todo!("original: NET_SendBroadcast")
}

/// C function: NET_RecvPacket
pub fn net_recv_packet(
    context: &mut NetContextT,
    addr: &mut Vec<NetAddrT>,
    packet: &mut Vec<NetPacketT>,
) -> Boolean {
    todo!("original: NET_RecvPacket")
}

/// C function: NET_AddrToString
pub fn net_addr_to_string(addr: &mut NetAddrT) -> String {
    todo!("original: NET_AddrToString")
}

/// C function: NET_FreeAddress
pub fn net_free_address(addr: &mut NetAddrT) {
    todo!("original: NET_FreeAddress")
}

/// C function: NET_ResolveAddress
pub fn net_resolve_address(context: &mut NetContextT, address: &str) -> Arc<Mutex<NetAddrT>> {
    todo!("original: NET_ResolveAddress")
}
