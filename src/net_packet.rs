// net_packet.h - packet manipulation

pub use crate::net_defs::*;

/// Original: NET_NewPacket
pub fn net_new_packet(_initial_size: i32) -> *mut NetPacketT {
    todo!("NET_NewPacket")
}

/// Original: NET_PacketDup
pub fn net_packet_dup(_packet: *mut NetPacketT) -> *mut NetPacketT {
    todo!("NET_PacketDup")
}

/// Original: NET_FreePacket
pub fn net_free_packet(_packet: *mut NetPacketT) {
    todo!("NET_FreePacket")
}

/// Original: NET_ReadInt8
pub fn net_read_int8(_packet: *mut NetPacketT, _data: *mut u32) -> Boolean {
    todo!("NET_ReadInt8")
}

/// Original: NET_ReadInt16
pub fn net_read_int16(_packet: *mut NetPacketT, _data: *mut u32) -> Boolean {
    todo!("NET_ReadInt16")
}

/// Original: NET_ReadInt32
pub fn net_read_int32(_packet: *mut NetPacketT, _data: *mut u32) -> Boolean {
    todo!("NET_ReadInt32")
}

/// Original: NET_ReadSInt8
pub fn net_read_sint8(_packet: *mut NetPacketT, _data: *mut i32) -> Boolean {
    todo!("NET_ReadSInt8")
}

/// Original: NET_ReadSInt16
pub fn net_read_sint16(_packet: *mut NetPacketT, _data: *mut i32) -> Boolean {
    todo!("NET_ReadSInt16")
}

/// Original: NET_ReadSInt32
pub fn net_read_sint32(_packet: *mut NetPacketT, _data: *mut i32) -> Boolean {
    todo!("NET_ReadSInt32")
}

/// Original: NET_ReadString
pub fn net_read_string(_packet: *mut NetPacketT) -> *mut i8 {
    todo!("NET_ReadString")
}

/// Original: NET_WriteInt8
pub fn net_write_int8(_packet: *mut NetPacketT, _i: u32) {
    todo!("NET_WriteInt8")
}

/// Original: NET_WriteInt16
pub fn net_write_int16(_packet: *mut NetPacketT, _i: u32) {
    todo!("NET_WriteInt16")
}

/// Original: NET_WriteInt32
pub fn net_write_int32(_packet: *mut NetPacketT, _i: u32) {
    todo!("NET_WriteInt32")
}

/// Original: NET_WriteString
pub fn net_write_string(_packet: *mut NetPacketT, _string: *mut i8) {
    todo!("NET_WriteString")
}
