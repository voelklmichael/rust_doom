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
    // Original: data
    pub data: *mut Byte,
    // Original: len
    pub len: usize,
    // Original: alloced
    pub alloced: usize,
    // Original: pos
    pub pos: u32,
}

/// Original: net_addr_t - opaque handle
#[repr(C)]
pub struct NetAddrT {
    // Original: module
    pub module: *mut NetModuleT,
    // Original: handle
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
    // Original: InitClient
    pub init_client: NetInitClientFn,
    // Original: InitServer
    pub init_server: NetInitServerFn,
    // Original: SendPacket
    pub send_packet: NetSendPacketFn,
    // Original: RecvPacket
    pub recv_packet: NetRecvPacketFn,
    // Original: AddrToString
    pub addr_to_string: NetAddrToStringFn,
    // Original: FreeAddress
    pub free_address: NetFreeAddressFn,
    // Original: ResolveAddress
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
    // Original: gamemode
    pub gamemode: i32,
    // Original: gamemission
    pub gamemission: i32,
    // Original: lowres_turn
    pub lowres_turn: i32,
    // Original: drone
    pub drone: i32,
    // Original: max_players
    pub max_players: i32,
    // Original: is_freedoom
    pub is_freedoom: i32,
    // Original: wad_sha1sum
    pub wad_sha1sum: Sha1DigestT,
    // Original: deh_sha1sum
    pub deh_sha1sum: Sha1DigestT,
    // Original: player_class
    pub player_class: i32,
}

/// Original: net_gamesettings_t
#[repr(C)]
pub struct NetGamesettingsT {
    // Original: ticdup
    pub ticdup: i32,
    // Original: extratics
    pub extratics: i32,
    // Original: deathmatch
    pub deathmatch: i32,
    // Original: episode
    pub episode: i32,
    // Original: nomonsters
    pub nomonsters: i32,
    // Original: fast_monsters
    pub fast_monsters: i32,
    // Original: respawn_monsters
    pub respawn_monsters: i32,
    // Original: map
    pub map: i32,
    // Original: skill
    pub skill: i32,
    // Original: gameversion
    pub gameversion: i32,
    // Original: lowres_turn
    pub lowres_turn: i32,
    // Original: new_sync
    pub new_sync: i32,
    // Original: timelimit
    pub timelimit: i32,
    // Original: loadgame
    pub loadgame: i32,
    // Original: random
    pub random: i32,
    // Original: num_players
    pub num_players: i32,
    // Original: consoleplayer
    pub consoleplayer: i32,
    // Original: player_classes
    pub player_classes: [i32; NET_MAXPLAYERS as usize],
}

/// Original: net_ticdiff_t
#[repr(C)]
pub struct NetTicdiffT {
    // Original: diff
    pub diff: u32,
    // Original: cmd
    pub cmd: TiccmdT,
}

/// Original: net_full_ticcmd_t
#[repr(C)]
pub struct NetFullTiccmdT {
    // Original: latency
    pub latency: i32,
    // Original: seq
    pub seq: u32,
    // Original: playeringame
    pub playeringame: [Boolean; NET_MAXPLAYERS as usize],
    // Original: cmds
    pub cmds: [NetTicdiffT; NET_MAXPLAYERS as usize],
}

/// Original: net_querydata_t
#[repr(C)]
pub struct NetQuerydataT {
    // Original: version
    pub version: *mut i8,
    // Original: server_state
    pub server_state: i32,
    // Original: num_players
    pub num_players: i32,
    // Original: max_players
    pub max_players: i32,
    // Original: gamemode
    pub gamemode: i32,
    // Original: gamemission
    pub gamemission: i32,
    // Original: description
    pub description: *mut i8,
}

/// Original: net_waitdata_t
#[repr(C)]
pub struct NetWaitdataT {
    // Original: num_players
    pub num_players: i32,
    // Original: num_drones
    pub num_drones: i32,
    // Original: ready_players
    pub ready_players: i32,
    // Original: max_players
    pub max_players: i32,
    // Original: is_controller
    pub is_controller: i32,
    // Original: consoleplayer
    pub consoleplayer: i32,
    // Original: player_names
    pub player_names: [[i8; MAXPLAYERNAME]; NET_MAXPLAYERS as usize],
    // Original: player_addrs
    pub player_addrs: [[i8; MAXPLAYERNAME]; NET_MAXPLAYERS as usize],
    // Original: wad_sha1sum
    pub wad_sha1sum: Sha1DigestT,
    // Original: deh_sha1sum
    pub deh_sha1sum: Sha1DigestT,
    // Original: is_freedoom
    pub is_freedoom: i32,
}

#[allow(non_camel_case_types)]
pub struct NetDefsState;

impl NetDefsState {
    pub fn new() -> Self {
        Self
    }
}
