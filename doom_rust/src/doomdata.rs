//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  All external data is defined here.
//  Raw WAD map lump formats and map thing definition.
//
// Original: doomdata.h

// =============================================================================
// Map thing (THINGS lump)
// =============================================================================

/// Thing definition, position, orientation and type,
/// plus skill/visibility flags and attributes.
/// Original: mapthing_t
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MapThing {
    pub x: i16,
    pub y: i16,
    pub angle: i16,
    pub type_: i16,
    pub options: i16,
}

/// Map thing options: don't spawn in single player (multiplayer only).
pub const MTF_NOTSINGLE: i16 = 16;

// =============================================================================
// Raw WAD map lump formats (for parsing THINGS, LINEDEFS, SIDEDEFS, etc.)
// =============================================================================

/// Raw vertex from VERTEXES lump. Original: mapvertex_t
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapVertex {
    pub x: i16,
    pub y: i16,
}

/// Raw linedef from LINEDEFS lump. Original: maplinedef_t
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapLineDef {
    pub v1: i16,
    pub v2: i16,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
}

/// Raw sidedef from SIDEDEFS lump. Original: mapsidedef_t
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapSideDef {
    pub textureoffset: i16,
    pub rowoffset: i16,
    pub toptexture: [u8; 8],
    pub bottomtexture: [u8; 8],
    pub midtexture: [u8; 8],
    pub sector: i16,
}

/// Raw sector from SECTORS lump. Original: mapsector_t
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapSector {
    pub floorheight: i16,
    pub ceilingheight: i16,
    pub floorpic: [u8; 8],
    pub ceilingpic: [u8; 8],
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
}

/// Raw seg from SEGS lump. Original: mapseg_t
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapSeg {
    pub v1: i16,
    pub v2: i16,
    pub angle: i16,
    pub linedef: i16,
    pub side: i16,
    pub offset: i16,
}

/// Raw subsector from SSECTORS lump. Original: mapsubsector_t
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapSubsector {
    pub numlines: i16,
    pub firstline: i16,
}

/// Raw node from NODES lump. Original: mapnode_t
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MapNode {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub bbox: [[i16; 4]; 2],
    pub children: [u16; 2],
}
