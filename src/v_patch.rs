// v_patch.h - patch/column types for sprites and textures
// No dependencies (leaf module)
// Uses byte from doomtype - we use u8

/// Original: patch_t
#[repr(C)]
pub struct PatchT {
    pub width: i16,
    pub height: i16,
    pub leftoffset: i16,
    pub topoffset: i16,
    pub columnofs: [i32; 8],
}

/// Original: post_t
#[repr(C)]
pub struct PostT {
    pub topdelta: u8,
    pub length: u8,
}

/// Original: column_t = post_t
pub type ColumnT = PostT;
