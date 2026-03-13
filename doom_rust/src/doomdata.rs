//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  All external data is defined here.
//
// Original: doomdata.h (partial - mapthing_t for doomstat)

/// Thing definition, position, orientation and type,
/// plus skill/visibility flags and attributes.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MapThing {
    pub x: i16,
    pub y: i16,
    pub angle: i16,
    pub type_: i16,
    pub options: i16,
}
