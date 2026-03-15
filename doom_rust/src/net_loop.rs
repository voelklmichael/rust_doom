//! Rust translation of doomgeneric/net_loop.h
//! Loopback network module for server compiled into the client.

use crate::net_defs::*;

pub static mut net_loop_client_module: NetModuleT = NetModuleT::default();
pub static mut net_loop_server_module: NetModuleT = NetModuleT::default();
