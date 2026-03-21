// v_patch.h - patch/column types for sprites and textures
// No dependencies (leaf module)
// Uses byte from doomtype - we use u8

/// Original: patch_t
#[repr(C)]
pub struct PatchT {
    // Original: width
    pub width: i16,
    // Original: height
    pub height: i16,
    // Original: leftoffset
    pub leftoffset: i16,
    // Original: topoffset
    pub topoffset: i16,
    // Original: columnofs
    pub columnofs: [i32; 8],
}

/// Original: post_t
#[repr(C)]
pub struct PostT {
    // Original: topdelta
    pub topdelta: u8,
    // Original: length
    pub length: u8,
}

/// Original: column_t = post_t
pub type ColumnT = PostT;
