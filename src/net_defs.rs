// net_defs.h - networking definitions

pub use crate::doomtype::*;
pub use crate::d_ticcmd::*;
pub use crate::sha1::*;

// Original: #define MAXNETNODES 16
pub const MAXNETNODES: i32 = 16;

// Original: #define NET_MAXPLAYERS 8
pub const NET_MAXPLAYERS: i32 = 8;

// Original: #define MAXPLAYERNAME 30
pub const MAXPLAYERNAME: usize = 30;

// Original: #define BACKUPTICS 128
pub const BACKUPTICS: i32 = 128;

// Original: #define NET_MAGIC_NUMBER 3436803284U
pub const NET_MAGIC_NUMBER: u32 = 3436803284;

// Original: #define NET_RELIABLE_PACKET (1 << 15)
pub const NET_RELIABLE_PACKET: u32 = 1 << 15;

// Original: #define NET_TICDIFF_*
pub const NET_TICDIFF_FORWARD: u32 = 1 << 0;
pub const NET_TICDIFF_SIDE: u32 = 1 << 1;
pub const NET_TICDIFF_TURN: u32 = 1 << 2;
pub const NET_TICDIFF_BUTTONS: u32 = 1 << 3;
pub const NET_TICDIFF_CONSISTANCY: u32 = 1 << 4;
pub const NET_TICDIFF_CHATCHAR: u32 = 1 << 5;
pub const NET_TICDIFF_RAVEN: u32 = 1 << 6;
pub const NET_TICDIFF_STRIFE: u32 = 1 << 7;

/// Original: struct _net_packet_s
#[repr(C)]
pub struct NetPacketT {
    pub data: *mut Byte,
    pub len: usize,
    pub alloced: usize,
    pub pos: u32,
}

/// Original: net_addr_t - opaque handle
#[repr(C)]
pub struct NetAddrT {
    pub module: *mut NetModuleT,
    pub handle: *mut std::ffi::c_void,
}

/// Original: net_context_t - opaque
#[repr(C)]
pub struct NetContextT {
    _opaque: [u8; 0],
}

pub type NetInitClientFn = fn() -> Boolean;
pub type NetInitServerFn = fn() -> Boolean;
pub type NetSendPacketFn = fn(*mut NetAddrT, *mut NetPacketT);
pub type NetRecvPacketFn = fn(*mut *mut NetAddrT, *mut *mut NetPacketT) -> Boolean;
pub type NetAddrToStringFn = fn(*mut NetAddrT, *mut i8, i32);
pub type NetFreeAddressFn = fn(*mut NetAddrT);
pub type NetResolveAddressFn = fn(*mut i8) -> *mut NetAddrT;

/// Original: struct _net_module_s
#[repr(C)]
pub struct NetModuleT {
    pub init_client: NetInitClientFn,
    pub init_server: NetInitServerFn,
    pub send_packet: NetSendPacketFn,
    pub recv_packet: NetRecvPacketFn,
    pub addr_to_string: NetAddrToStringFn,
    pub free_address: NetFreeAddressFn,
    pub resolve_address: NetResolveAddressFn,
}

/// Original: net_packet_type_t
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NetPacketTypeT {
    Syn,
    Ack,
    Rejected,
    Keepalive,
    WaitingData,
    Gamestart,
    Gamedata,
    GamedataAck,
    Disconnect,
    DisconnectAck,
    ReliableAck,
    GamedataResend,
    ConsoleMessage,
    Query,
    QueryResponse,
    Launch,
}

/// Original: net_master_packet_type_t
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NetMasterPacketTypeT {
    Add,
    AddResponse,
    Query,
    QueryResponse,
    GetMetadata,
    GetMetadataResponse,
    SignStart,
    SignStartResponse,
    SignEnd,
    SignEndResponse,
}

/// Original: net_connect_data_t
#[repr(C)]
pub struct NetConnectDataT {
    pub gamemode: i32,
    pub gamemission: i32,
    pub lowres_turn: i32,
    pub drone: i32,
    pub max_players: i32,
    pub is_freedoom: i32,
    pub wad_sha1sum: Sha1DigestT,
    pub deh_sha1sum: Sha1DigestT,
    pub player_class: i32,
}

/// Original: net_gamesettings_t
#[repr(C)]
pub struct NetGamesettingsT {
    pub ticdup: i32,
    pub extratics: i32,
    pub deathmatch: i32,
    pub episode: i32,
    pub nomonsters: i32,
    pub fast_monsters: i32,
    pub respawn_monsters: i32,
    pub map: i32,
    pub skill: i32,
    pub gameversion: i32,
    pub lowres_turn: i32,
    pub new_sync: i32,
    pub timelimit: i32,
    pub loadgame: i32,
    pub random: i32,
    pub num_players: i32,
    pub consoleplayer: i32,
    pub player_classes: [i32; NET_MAXPLAYERS as usize],
}

/// Original: net_ticdiff_t
#[repr(C)]
pub struct NetTicdiffT {
    pub diff: u32,
    pub cmd: TiccmdT,
}

/// Original: net_full_ticcmd_t
#[repr(C)]
pub struct NetFullTiccmdT {
    pub latency: i32,
    pub seq: u32,
    pub playeringame: [Boolean; NET_MAXPLAYERS as usize],
    pub cmds: [NetTicdiffT; NET_MAXPLAYERS as usize],
}

/// Original: net_querydata_t
#[repr(C)]
pub struct NetQuerydataT {
    pub version: *mut i8,
    pub server_state: i32,
    pub num_players: i32,
    pub max_players: i32,
    pub gamemode: i32,
    pub gamemission: i32,
    pub description: *mut i8,
}

/// Original: net_waitdata_t
#[repr(C)]
pub struct NetWaitdataT {
    pub num_players: i32,
    pub num_drones: i32,
    pub ready_players: i32,
    pub max_players: i32,
    pub is_controller: i32,
    pub consoleplayer: i32,
    pub player_names: [[i8; MAXPLAYERNAME]; NET_MAXPLAYERS as usize],
    pub player_addrs: [[i8; MAXPLAYERNAME]; NET_MAXPLAYERS as usize],
    pub wad_sha1sum: Sha1DigestT,
    pub deh_sha1sum: Sha1DigestT,
    pub is_freedoom: i32,
}

#[allow(non_camel_case_types)]
pub struct NetDefsState;

impl NetDefsState {
    pub fn new() -> Self {
        Self
    }
}
