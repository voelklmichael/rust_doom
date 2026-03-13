//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 1993-2008 Raven Software
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  System specific interface stuff.
//
// Original: d_ticcmd.h

use crate::doomtype::Byte;

/// The data sampled per tick (single player) and transmitted to other peers
/// (multiplayer). Mainly movements/button commands per game tick.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Ticcmd {
    pub forwardmove: i8,   // *2048 for move
    pub sidemove: i8,      // *2048 for move
    pub angleturn: i16,    // <<16 for angle delta
    pub chatchar: Byte,
    pub buttons: Byte,
    pub consistancy: Byte, // checks for net game
    pub buttons2: Byte,
    pub inventory: i32,
    pub lookfly: Byte,     // look/fly up/down/centering
    pub arti: Byte,       // artitype_t to use
}
