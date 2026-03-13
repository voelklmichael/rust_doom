//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Patches - used for sprites and masked pictures.
//  Textures are composed from TEXTURE1/2 patch lists.
//
// Original: v_patch.h

// =============================================================================
// Public API (from .h)
// =============================================================================

/// Patch - holds one or more columns.
/// Used for sprites and all masked pictures.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct patch_t {
    /// Bounding box width
    pub width: i16,
    /// Bounding box height
    pub height: i16,
    /// Pixels to the left of origin
    pub leftoffset: i16,
    /// Pixels below the origin
    pub topoffset: i16,
    /// Column offsets; only [width] used. [0] = &columnofs[width]
    pub columnofs: [i32; 8],
}

/// Post - run of non-masked source pixels in a column.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct post_t {
    /// -1 is the last post in a column
    pub topdelta: u8,
    /// Length; data bytes follow
    pub length: u8,
}

/// Column is a list of 0 or more post_t, (byte)-1 terminated.
pub type column_t = post_t;
