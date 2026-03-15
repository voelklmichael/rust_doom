//! Rust translation of doomgeneric/net_defs.h
//! Definitions for use in networking code.

use crate::d_ticcmd::*;
use crate::doomtype::*;
use crate::sha1::*;

pub const MAXNETNODES: i32 = 16;
pub const NET_MAXPLAYERS: i32 = 8;
pub const MAXPLAYERNAME: usize = 30;
pub const BACKUPTICS: i32 = 128;
pub const NET_MAGIC_NUMBER: u32 = 3436803284;
pub const NET_RELIABLE_PACKET: u32 = 1 << 15;

/// C typedef: net_packet_type_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// C typedef: net_master_packet_type_t
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// C typedef: net_connect_data_t
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

/// C typedef: net_gamesettings_t
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

pub const NET_TICDIFF_FORWARD: u32 = 1 << 0;
pub const NET_TICDIFF_SIDE: u32 = 1 << 1;
pub const NET_TICDIFF_TURN: u32 = 1 << 2;
pub const NET_TICDIFF_BUTTONS: u32 = 1 << 3;
pub const NET_TICDIFF_CONSISTANCY: u32 = 1 << 4;
pub const NET_TICDIFF_CHATCHAR: u32 = 1 << 5;
pub const NET_TICDIFF_RAVEN: u32 = 1 << 6;
pub const NET_TICDIFF_STRIFE: u32 = 1 << 7;

/// C typedef: net_ticdiff_t
#[repr(C)]
pub struct NetTicdiffT {
    pub diff: u32,
    pub cmd: TiccmdT,
}

/// C typedef: net_full_ticcmd_t
#[repr(C)]
pub struct NetFullTiccmdT {
    pub latency: i32,
    pub seq: u32,
    pub playeringame: [Boolean; NET_MAXPLAYERS as usize],
    pub cmds: [NetTicdiffT; NET_MAXPLAYERS as usize],
}

/// C typedef: net_querydata_t
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

/// C typedef: net_waitdata_t
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

impl NetAddrT {
    pub const fn default() -> Self {
        Self {
            module: core::ptr::null_mut(),
            handle: core::ptr::null_mut(),
        }
    }
}

impl NetWaitdataT {
    pub const fn default() -> Self {
        Self {
            num_players: 0,
            num_drones: 0,
            ready_players: 0,
            max_players: 0,
            is_controller: 0,
            consoleplayer: 0,
            player_names: [[0i8; MAXPLAYERNAME]; NET_MAXPLAYERS as usize],
            player_addrs: [[0i8; MAXPLAYERNAME]; NET_MAXPLAYERS as usize],
            wad_sha1sum: [0u8; 20],
            deh_sha1sum: [0u8; 20],
            is_freedoom: 0,
        }
    }
}

/// C typedef: net_packet_t (struct _net_packet_s)
#[repr(C)]
pub struct NetPacketT {
    pub data: *mut byte,
    pub len: usize,
    pub alloced: usize,
    pub pos: u32,
}

/// C typedef: net_addr_t (struct _net_addr_s) - opaque handle
#[repr(C)]
pub struct NetAddrT {
    pub module: *mut NetModuleT,
    pub handle: *mut core::ffi::c_void,
}

/// C typedef: net_module_t (struct _net_module_s)
#[repr(C)]
pub struct NetModuleT {
    pub init_client: Option<extern "C" fn() -> Boolean>,
    pub init_server: Option<extern "C" fn() -> Boolean>,
    pub send_packet: Option<extern "C" fn(*mut NetAddrT, *mut NetPacketT)>,
    pub recv_packet: Option<
        extern "C" fn(*mut *mut NetAddrT, *mut *mut NetPacketT) -> Boolean,
    >,
    pub addr_to_string: Option<extern "C" fn(*mut NetAddrT, *mut i8, i32)>,
    pub free_address: Option<extern "C" fn(*mut NetAddrT)>,
    pub resolve_address: Option<extern "C" fn(*mut i8) -> *mut NetAddrT>,
}

impl NetModuleT {
    pub const fn default() -> Self {
        Self {
            init_client: None,
            init_server: None,
            send_packet: None,
            recv_packet: None,
            addr_to_string: None,
            free_address: None,
            resolve_address: None,
        }
    }
}

/// Opaque net_context_t
#[repr(C)]
pub struct NetContextT {
    _private: [u8; 0],
}
