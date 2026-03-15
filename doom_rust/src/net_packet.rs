//! Rust translation of doomgeneric/net_packet.h
//! Network packet manipulation.

use crate::doomtype::*;
use crate::net_defs::*;

/// C function: NET_NewPacket
pub fn net_new_packet(initial_size: i32) -> *mut NetPacketT {
    todo!("original: NET_NewPacket")
}

/// C function: NET_PacketDup
pub fn net_packet_dup(packet: &mut NetPacketT) -> *mut NetPacketT {
    todo!("original: NET_PacketDup")
}

/// C function: NET_FreePacket
pub fn net_free_packet(packet: &mut NetPacketT) {
    todo!("original: NET_FreePacket")
}

/// C function: NET_ReadInt8
pub fn net_read_int8(packet: &mut NetPacketT, data: &mut u32) -> Boolean {
    todo!("original: NET_ReadInt8")
}

/// C function: NET_ReadInt16
pub fn net_read_int16(packet: &mut NetPacketT, data: &mut u32) -> Boolean {
    todo!("original: NET_ReadInt16")
}

/// C function: NET_ReadInt32
pub fn net_read_int32(packet: &mut NetPacketT, data: &mut u32) -> Boolean {
    todo!("original: NET_ReadInt32")
}

/// C function: NET_ReadSInt8
pub fn net_read_sint8(packet: &mut NetPacketT, data: &mut i32) -> Boolean {
    todo!("original: NET_ReadSInt8")
}

/// C function: NET_ReadSInt16
pub fn net_read_sint16(packet: &mut NetPacketT, data: &mut i32) -> Boolean {
    todo!("original: NET_ReadSInt16")
}

/// C function: NET_ReadSInt32
pub fn net_read_sint32(packet: &mut NetPacketT, data: &mut i32) -> Boolean {
    todo!("original: NET_ReadSInt32")
}

/// C function: NET_ReadString
pub fn net_read_string(packet: &mut NetPacketT) -> String {
    todo!("original: NET_ReadString")
}

/// C function: NET_WriteInt8
pub fn net_write_int8(packet: &mut NetPacketT, i: u32) {
    todo!("original: NET_WriteInt8")
}

/// C function: NET_WriteInt16
pub fn net_write_int16(packet: &mut NetPacketT, i: u32) {
    todo!("original: NET_WriteInt16")
}

/// C function: NET_WriteInt32
pub fn net_write_int32(packet: &mut NetPacketT, i: u32) {
    todo!("original: NET_WriteInt32")
}

/// C function: NET_WriteString
pub fn net_write_string(packet: &mut NetPacketT, string: &str) {
    todo!("original: NET_WriteString")
}
