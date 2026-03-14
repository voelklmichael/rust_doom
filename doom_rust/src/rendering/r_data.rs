//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Refresh module, data I/O, caching, retrieval of graphics by name.
//
// Original: r_data.h + r_data.c

use crate::deh::deh_string;
use crate::i_swap;
use crate::i_system;
use crate::m_fixed::FRACBITS;
use crate::rendering::state;
use crate::rendering::v_patch::{column_t, patch_t};
use crate::wad::{
    w_cache_lump_name, w_cache_lump_num, w_check_num_for_name, w_get_num_for_name,
    w_lump_length, w_lump_name_hash, w_release_lump_name, with_lumpinfo,
};
use crate::z_zone::{z_free, z_malloc, PU_CACHE, PU_STATIC};
use std::ptr;

// =============================================================================
// Internal types (from r_data.c)
// =============================================================================

#[repr(C)]
struct Mappatch {
    originx: i16,
    originy: i16,
    patch: i16,
    stepdir: i16,
    colormap: i16,
}

#[repr(C)]
struct Maptexture {
    name: [u8; 8],
    masked: i32,
    width: i16,
    height: i16,
    obsolete: i32,
    patchcount: i16,
    patches: [Mappatch; 1],
}

#[repr(C)]
pub struct Texpatch {
    pub originx: i16,
    pub originy: i16,
    pub patch: i32,
}

#[repr(C)]
pub struct Texture {
    pub name: [u8; 8],
    pub width: i16,
    pub height: i16,
    pub index: i32,
    pub next: *mut Texture,
    pub patchcount: i16,
    pub patches: [Texpatch; 1],
}

// =============================================================================
// Internal state (from r_data.c)
// =============================================================================

static mut FIRSTFLAT: i32 = 0;
static mut LASTFLAT: i32 = 0;
static mut NUMFLATS: i32 = 0;

static mut FIRSTPATCH: i32 = 0;
static mut LASTPATCH: i32 = 0;
static mut NUMPATCHES: i32 = 0;

static mut FIRSTSPRITELUMP: i32 = 0;
static mut LASTSPRITELUMP: i32 = 0;
static mut NUMSPRITELUMPS: i32 = 0;

static mut NUMTEXTURES: i32 = 0;
static mut TEXTURES: *mut *mut Texture = ptr::null_mut();
static mut TEXTURES_HASHTABLE: *mut *mut Texture = ptr::null_mut();

static mut TEXTUREWIDTHMASK: *mut i32 = ptr::null_mut();
static mut TEXTURECOMPOSITESIZE: *mut i32 = ptr::null_mut();
static mut TEXTURECOLUMNLUMP: *mut *mut i16 = ptr::null_mut();
static mut TEXTURECOLUMNOFS: *mut *mut u16 = ptr::null_mut();
static mut TEXTURECOMPOSITE: *mut *mut u8 = ptr::null_mut();

// =============================================================================
// Helpers for reading from lump data
// =============================================================================

#[inline]
fn read_i16_le(ptr: *const u8) -> i16 {
    i_swap::short(unsafe { *(ptr as *const [u8; 2]) })
}

#[inline]
fn read_i32_le(ptr: *const u8) -> i32 {
    i_swap::long(unsafe { *(ptr as *const [u8; 4]) })
}

fn patch_width(patch_ptr: *const u8) -> i16 {
    read_i16_le(patch_ptr)
}

fn patch_columnofs(patch_ptr: *const u8, col: usize) -> i32 {
    let ofs = 8 + col * 4; // 8 = width+height+leftoffset+topoffset
    read_i32_le(unsafe { patch_ptr.add(ofs) })
}

// =============================================================================
// R_DrawColumnInCache (private)
// =============================================================================

fn r_draw_column_in_cache(
    patch: *const column_t,
    cache: *mut u8,
    originy: i32,
    cacheheight: i32,
) {
    let mut patch = patch;
    loop {
        let topdelta = unsafe { (*patch).topdelta };
        if topdelta == 0xff {
            break;
        }
        let length = unsafe { (*patch).length } as i32;
        let source = unsafe { (patch as *const u8).add(3) };
        let mut position = originy + topdelta as i32;

        let mut count = length;
        if position < 0 {
            count += position;
            position = 0;
        }
        if position + count > cacheheight {
            count = cacheheight - position;
        }
        if count > 0 {
            unsafe {
                ptr::copy_nonoverlapping(
                    source,
                    cache.add(position as usize),
                    count as usize,
                );
            }
        }
        patch = unsafe { (patch as *const u8).add(length as usize + 4) as *const column_t };
    }
}

// =============================================================================
// R_GenerateComposite (private)
// =============================================================================

fn r_generate_composite(texnum: i32) {
    unsafe {
        let texture = *TEXTURES.add(texnum as usize);
        let block = z_malloc(
            *TEXTURECOMPOSITESIZE.add(texnum as usize) as usize,
            PU_STATIC,
            TEXTURECOMPOSITE.add(texnum as usize),
        );
        *TEXTURECOMPOSITE.add(texnum as usize) = block;

        let collump = *TEXTURECOLUMNLUMP.add(texnum as usize);
        let colofs = *TEXTURECOLUMNOFS.add(texnum as usize);

        let patch_count = (*texture).patchcount as usize;
        let patches_ptr = &(*texture).patches as *const Texpatch;

        for i in 0..patch_count {
            let patch = &*patches_ptr.add(i);
            let lump = patch.patch;
            let realpatch = w_cache_lump_num(lump, PU_CACHE) as *const u8;
            let x1 = patch.originx as i32;
            let x2 = x1 + patch_width(realpatch) as i32;

            let mut x = if x1 < 0 { 0 } else { x1 };
            let mut x2 = x2;
            if x2 > (*texture).width as i32 {
                x2 = (*texture).width as i32;
            }

            while x < x2 {
                if *collump.add(x as usize) >= 0 {
                    x += 1;
                    continue;
                }
                let col_idx = (x - x1) as usize;
                let patchcol = (realpatch as *const u8)
                    .add(patch_columnofs(realpatch, col_idx) as usize)
                    as *const column_t;
                r_draw_column_in_cache(
                    patchcol,
                    block.add(*colofs.add(x as usize) as usize),
                    patch.originy as i32,
                    (*texture).height as i32,
                );
                x += 1;
            }
        }

        {
            let ptr = block;
            let tag = PU_CACHE; };
    }
}

// =============================================================================
// R_GenerateLookup (private)
// =============================================================================

fn r_generate_lookup(texnum: i32) {
    unsafe {
        let texture = *TEXTURES.add(texnum as usize);
        *TEXTURECOMPOSITE.add(texnum as usize) = ptr::null_mut();
        *TEXTURECOMPOSITESIZE.add(texnum as usize) = 0;

        let collump = *TEXTURECOLUMNLUMP.add(texnum as usize);
        let colofs = *TEXTURECOLUMNOFS.add(texnum as usize);

        let width = (*texture).width as usize;
        let patchcount = z_malloc(width, PU_STATIC, ptr::null_mut()) as *mut u8;
        ptr::write_bytes(patchcount, 0, width);

        let patch_count = (*texture).patchcount as usize;
        let patches_ptr = &(*texture).patches as *const Texpatch;

        for i in 0..patch_count {
            let patch = &*patches_ptr.add(i);
            let realpatch = w_cache_lump_num(patch.patch, PU_CACHE) as *const u8;
            let x1 = patch.originx as i32;
            let x2 = x1 + patch_width(realpatch) as i32;

            let mut x = if x1 < 0 { 0 } else { x1 };
            let mut x2 = x2;
            if x2 > (*texture).width as i32 {
                x2 = (*texture).width as i32;
            }

            while x < x2 {
                *patchcount.add(x as usize) += 1;
                *collump.add(x as usize) = patch.patch as i16;
                let col_idx = (x - x1) as usize;
                *colofs.add(x as usize) =
                    (patch_columnofs(realpatch, col_idx) + 3) as u16;
                x += 1;
            }
        }

        for x in 0..width {
            if *patchcount.add(x) == 0 {
                let name = std::str::from_utf8(&(*texture).name)
                    .unwrap_or("")
                    .trim_end_matches('\0');
                eprintln!("R_GenerateLookup: column without a patch ({})", name);
                z_free(patchcount);
                return;
            }
            if *patchcount.add(x) > 1 {
                *collump.add(x) = -1;
                *colofs.add(x) =
                    *TEXTURECOMPOSITESIZE.add(texnum as usize) as u16;
                let new_size =
                    *TEXTURECOMPOSITESIZE.add(texnum as usize) + (*texture).height as i32;
                if new_size > 0x10000 - (*texture).height as i32 {
                    i_system::i_error("R_GenerateLookup: texture >64k");
                }
                *TEXTURECOMPOSITESIZE.add(texnum as usize) = new_size;
            }
        }

        z_free(patchcount);
    }
}

// =============================================================================
// Public API
// =============================================================================

/// Retrieve column data for span blitting.
pub fn r_get_column(tex: i32, col: i32) -> *mut u8 {
    unsafe {
        let mask = *TEXTUREWIDTHMASK.add(tex as usize);
        let col = col & mask;
        let lump = *(*TEXTURECOLUMNLUMP.add(tex as usize)).add(col as usize);
        let ofs = *(*TEXTURECOLUMNOFS.add(tex as usize)).add(col as usize);

        if lump > 0 {
            return w_cache_lump_num(lump as i32, PU_CACHE).add(ofs as usize);
        }
        if (*TEXTURECOMPOSITE.add(tex as usize)).is_null() {
            r_generate_composite(tex);
        }
        (*TEXTURECOMPOSITE.add(tex as usize)).add(ofs as usize)
    }
}

/// I/O, setting up the stuff. Must be called after W_Init.
pub fn r_init_data() {
    r_init_textures();
    r_init_flats();
    r_init_sprite_lumps();
    r_init_colormaps();

    // Sync state to r_state
    unsafe {
        state::FIRSTFLAT = FIRSTFLAT;
        state::FLATTRANSLATION = FLATTRANSLATION;
        state::TEXTURETRANSLATION = TEXTURETRANSLATION;
        state::TEXTUREHEIGHT = TEXTUREHEIGHT;
        state::FIRSTSPRITELUMP = FIRSTSPRITELUMP;
        state::LASTSPRITELUMP = LASTSPRITELUMP;
        state::NUMSPRITELUMPS = NUMSPRITELUMPS;
        state::SPRITEWIDTH = SPRITEWIDTH;
        state::SPRITEOFFSET = SPRITEOFFSET;
        state::SPRITETOPOFFSET = SPRITETOPOFFSET;
        state::COLORMAPS = COLORMAPS;
    }
    crate::rendering::r_things::r_init_sprites();
}

/// Preloads all relevant graphics for the level.
pub fn r_precache_level() {
    use crate::doomstat;
    use crate::rendering::r_sky;

    unsafe {
        if doomstat::DEMOPLAYBACK {
            return;
        }

        let numsectors = state::NUMSECTORS;
        let sectors = state::SECTORS;
        let numsides = state::NUMSIDES;
        let sides = state::SIDES;
        let numsprites = state::NUMSPRITES;
        let sprites = state::SPRITES;

        // Precache flats
        if numsectors > 0 && !sectors.is_null() {
            let mut flatpresent = vec![0u8; NUMFLATS as usize];
            for i in 0..numsectors {
                let sec = &*sectors.add(i as usize);
                flatpresent[(*sec).floorpic as usize] = 1;
                flatpresent[(*sec).ceilingpic as usize] = 1;
            }
            with_lumpinfo(|lumpinfo| {
                for i in 0..NUMFLATS {
                    if flatpresent[i as usize] != 0 {
                        let lump = (FIRSTFLAT + i) as usize;
                        if lump < lumpinfo.len() {
                            w_cache_lump_num((FIRSTFLAT + i) as i32, PU_CACHE);
                        }
                    }
                }
            });
        }

        // Precache textures
        if numsides > 0 && !sides.is_null() {
            let mut texturepresent = vec![0u8; NUMTEXTURES as usize];
            for i in 0..numsides {
                let side = &*sides.add(i as usize);
                let top = (*side).toptexture as usize;
                let mid = (*side).midtexture as usize;
                let bottom = (*side).bottomtexture as usize;
                if top < texturepresent.len() {
                    texturepresent[top] = 1;
                }
                if mid < texturepresent.len() {
                    texturepresent[mid] = 1;
                }
                if bottom < texturepresent.len() {
                    texturepresent[bottom] = 1;
                }
            }
            let skytex = r_sky::SKYTEXTURE as usize;
            if skytex < texturepresent.len() {
                texturepresent[skytex] = 1;
            }
            for i in 0..NUMTEXTURES {
                if texturepresent[i as usize] == 0 {
                    continue;
                }
                let texture = *TEXTURES.add(i as usize);
                for j in 0..(*texture).patchcount {
                    w_cache_lump_num((*texture).patches.as_ptr().add(j as usize).read().patch, PU_CACHE);
                }
            }
        }

        // Precache sprites - requires thinker list (p_local), skip for now
        if numsprites > 0 && !sprites.is_null() {
            // TODO: iterate thinkercap when p_local is ported
        }
    }
}

/// Check whether flat is available. Returns -1 if not found.
pub fn r_check_flat_num_for_name(name: &str) -> i32 {
    let i = w_check_num_for_name(name);
    if i < 0 {
        return -1;
    }
    unsafe { i - FIRSTFLAT }
}

/// Get flat number for a flat name.
pub fn r_flat_num_for_name(name: &str) -> i32 {
    let i = r_check_flat_num_for_name(name);
    if i < 0 {
        let namet = if name.len() >= 8 {
            &name[..8]
        } else {
            name
        };
        i_system::i_error(&format!("R_FlatNumForName: {} not found", namet));
    }
    i
}

/// Check whether texture is available. Returns -1 if not found.
pub fn r_check_texture_num_for_name(name: &str) -> i32 {
    if name.is_empty() {
        return 0;
    }
    if name.starts_with('-') {
        return 0; // "NoTexture" marker
    }
    unsafe {
        let key = (w_lump_name_hash(name) as i32) % NUMTEXTURES;
        let mut texture = *TEXTURES_HASHTABLE.add(key as usize);
        while !texture.is_null() {
            let tex_name = std::str::from_utf8(&(*texture).name)
                .unwrap_or("")
                .trim_end_matches('\0');
            if name.as_bytes().len() >= 8 && tex_name.as_bytes().len() >= 8 {
                if name.as_bytes()[..8].eq_ignore_ascii_case(&tex_name.as_bytes()[..8]) {
                    return (*texture).index;
                }
            } else if name.eq_ignore_ascii_case(tex_name) {
                return (*texture).index;
            }
            texture = (*texture).next;
        }
    }
    -1
}

/// Get texture number for name. Aborts with error if not found.
pub fn r_texture_num_for_name(name: &str) -> i32 {
    let i = r_check_texture_num_for_name(name);
    if i < 0 {
        i_system::i_error(&format!("R_TextureNumForName: {} not found", name));
    }
    i
}

// =============================================================================
// Private init functions
// =============================================================================

static mut FLATTRANSLATION: *mut i32 = ptr::null_mut();
static mut TEXTURETRANSLATION: *mut i32 = ptr::null_mut();
static mut TEXTUREHEIGHT: *mut i32 = ptr::null_mut();
static mut SPRITEWIDTH: *mut i32 = ptr::null_mut();
static mut SPRITEOFFSET: *mut i32 = ptr::null_mut();
static mut SPRITETOPOFFSET: *mut i32 = ptr::null_mut();
static mut COLORMAPS: *mut u8 = ptr::null_mut();

fn generate_texture_hash_table() {
    unsafe {
        TEXTURES_HASHTABLE =
            z_malloc((NUMTEXTURES as usize) * std::mem::size_of::<*mut Texture>(), PU_STATIC, ptr::null_mut())
                as *mut *mut Texture;
        ptr::write_bytes(
            TEXTURES_HASHTABLE,
            0,
            (NUMTEXTURES as usize) * std::mem::size_of::<*mut Texture>(),
        );

        for i in 0..NUMTEXTURES {
            let tex = *TEXTURES.add(i as usize);
            (*tex).index = i;

            let name = std::str::from_utf8(&(*tex).name)
                .unwrap_or("")
                .trim_end_matches('\0');
            let key = (w_lump_name_hash(name) as usize) % (NUMTEXTURES as usize);

            let mut rover = TEXTURES_HASHTABLE.add(key);
            while !(*rover).is_null() {
                rover = &mut (*(*rover)).next;
            }
            (*tex).next = ptr::null_mut();
            *rover = tex;
        }
    }
}

fn r_init_textures() {
    let pnames = w_cache_lump_name(deh_string("PNAMES"), PU_STATIC);
    let nummappatches = read_i32_le(pnames);
    let name_p = unsafe { pnames.add(4) };

    let patchlookup = unsafe {
        z_malloc(
            (nummappatches as usize) * std::mem::size_of::<i32>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut i32
    };
    for i in 0..nummappatches {
        let src = unsafe { std::slice::from_raw_parts(name_p.add((i * 8) as usize), 8) };
        let name_str = String::from_utf8_lossy(src);
        let name_trimmed = name_str.as_ref().trim_end_matches('\0');
        let lump = w_check_num_for_name(name_trimmed);
        unsafe {
            *patchlookup.add(i as usize) = lump;
        }
    }
    w_release_lump_name(deh_string("PNAMES"));

    let maptex1 = w_cache_lump_name(deh_string("TEXTURE1"), PU_STATIC);
    let numtextures1 = read_i32_le(maptex1);
    let maxoff = w_lump_length(w_get_num_for_name(deh_string("TEXTURE1")) as usize);

    let (maptex2, numtextures2, maxoff2) = if w_check_num_for_name(deh_string("TEXTURE2")) >= 0 {
        let m2 = w_cache_lump_name(deh_string("TEXTURE2"), PU_STATIC);
        let n2 = read_i32_le(m2);
        let o2 = w_lump_length(w_get_num_for_name(deh_string("TEXTURE2")) as usize);
        (Some(m2), n2, o2)
    } else {
        (None, 0, 0)
    };

    unsafe {
        NUMTEXTURES = numtextures1 + numtextures2;
        TEXTURES = z_malloc(
            (NUMTEXTURES as usize) * std::mem::size_of::<*mut Texture>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut Texture;
        TEXTURECOLUMNLUMP = z_malloc(
            (NUMTEXTURES as usize) * std::mem::size_of::<*mut i16>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut i16;
        TEXTURECOLUMNOFS = z_malloc(
            (NUMTEXTURES as usize) * std::mem::size_of::<*mut u16>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut u16;
        TEXTURECOMPOSITE = z_malloc(
            (NUMTEXTURES as usize) * std::mem::size_of::<*mut u8>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut u8;
        TEXTURECOMPOSITESIZE = z_malloc(
            (NUMTEXTURES as usize) * std::mem::size_of::<i32>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut i32;
        TEXTUREWIDTHMASK = z_malloc(
            (NUMTEXTURES as usize) * std::mem::size_of::<i32>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut i32;
        TEXTUREHEIGHT = z_malloc(
            (NUMTEXTURES as usize) * std::mem::size_of::<i32>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut i32;

        let mut directory = maptex1.add(4);
        let mut maxoff = maxoff;
        let mut maptex = maptex1;

        for i in 0..NUMTEXTURES {
            if i == numtextures1 {
                if let Some(m2) = maptex2 {
                    maptex = m2;
                    maxoff = maxoff2;
                    directory = m2.add(4);
                }
            }

            let offset = read_i32_le(directory);
            if offset > maxoff as i32 {
                i_system::i_error("R_InitTextures: bad texture directory");
            }

            let mtexture = maptex.add(offset as usize) as *const Maptexture;
            let mtexture_ptr = mtexture as *const u8;
            let mpatchcount = read_i16_le(unsafe { mtexture_ptr.add(20) }) as usize;
            let mwidth = read_i16_le(unsafe { mtexture_ptr.add(12) });
            let mheight = read_i16_le(unsafe { mtexture_ptr.add(14) });

            let tex_size = std::mem::size_of::<Texture>()
                + (mpatchcount.saturating_sub(1)) * std::mem::size_of::<Texpatch>();
            let texture = z_malloc(tex_size, PU_STATIC, ptr::null_mut()) as *mut Texture;
            *TEXTURES.add(i as usize) = texture;

            (*texture).width = mwidth;
            (*texture).height = mheight;
            (*texture).patchcount = mpatchcount as i16;
            ptr::copy_nonoverlapping(
                (*mtexture).name.as_ptr(),
                (*texture).name.as_mut_ptr(),
                8,
            );

            let mpatch_base = &(*mtexture).patches as *const Mappatch;
            let patch_base = &(*texture).patches as *const Texpatch;
            for j in 0..mpatchcount {
                let mpatch = &*mpatch_base.add(j);
                let patch = &mut *((patch_base as *mut Texpatch).add(j));
                patch.originx = read_i16_le(&mpatch.originx as *const i16 as *const u8);
                patch.originy = read_i16_le(&mpatch.originy as *const i16 as *const u8);
                let patch_idx = read_i16_le(&mpatch.patch as *const i16 as *const u8) as usize;
                patch.patch = unsafe { *patchlookup.add(patch_idx) };
                if patch.patch < 0 {
                    let name =
                        std::str::from_utf8(&(*texture).name).unwrap_or("").trim_end_matches('\0');
                    i_system::i_error(&format!(
                        "R_InitTextures: Missing patch in texture {}",
                        name
                    ));
                }
            }

            *TEXTURECOLUMNLUMP.add(i as usize) =
                z_malloc((mwidth as usize) * 2, PU_STATIC, ptr::null_mut()) as *mut i16;
            *TEXTURECOLUMNOFS.add(i as usize) =
                z_malloc((mwidth as usize) * 2, PU_STATIC, ptr::null_mut()) as *mut u16;

            let mut j = 1i32;
            while j * 2 <= mwidth as i32 {
                j <<= 1;
            }
            *TEXTUREWIDTHMASK.add(i as usize) = j - 1;
            *TEXTUREHEIGHT.add(i as usize) = (mheight as i32) << FRACBITS;

            directory = directory.add(4);
        }

        z_free(patchlookup as *mut u8);
    }

    w_release_lump_name(deh_string("TEXTURE1"));
    if maptex2.is_some() {
        w_release_lump_name(deh_string("TEXTURE2"));
    }

    unsafe {
        for i in 0..NUMTEXTURES {
            r_generate_lookup(i);
        }
    }

    unsafe {
        TEXTURETRANSLATION =
            z_malloc(((NUMTEXTURES + 1) as usize) * std::mem::size_of::<i32>(), PU_STATIC, ptr::null_mut())
                as *mut i32;
        for i in 0..NUMTEXTURES {
            *TEXTURETRANSLATION.add(i as usize) = i;
        }
        generate_texture_hash_table();
    }
}

fn r_init_flats() {
    unsafe {
        FIRSTFLAT = w_get_num_for_name(deh_string("F_START")) + 1;
        LASTFLAT = w_get_num_for_name(deh_string("F_END")) - 1;
        NUMFLATS = LASTFLAT - FIRSTFLAT + 1;
    }

    unsafe {
        FLATTRANSLATION =
            z_malloc(((NUMFLATS + 1) as usize) * std::mem::size_of::<i32>(), PU_STATIC, ptr::null_mut())
                as *mut i32;
        for i in 0..NUMFLATS {
            *FLATTRANSLATION.add(i as usize) = i;
        }
    }
}

fn r_init_sprite_lumps() {
    unsafe {
        FIRSTSPRITELUMP = w_get_num_for_name(deh_string("S_START")) + 1;
        LASTSPRITELUMP = w_get_num_for_name(deh_string("S_END")) - 1;
        NUMSPRITELUMPS = LASTSPRITELUMP - FIRSTSPRITELUMP + 1;
    }

    unsafe {
        SPRITEWIDTH =
            z_malloc((NUMSPRITELUMPS as usize) * 4, PU_STATIC, ptr::null_mut()) as *mut i32;
        SPRITEOFFSET =
            z_malloc((NUMSPRITELUMPS as usize) * 4, PU_STATIC, ptr::null_mut()) as *mut i32;
        SPRITETOPOFFSET =
            z_malloc((NUMSPRITELUMPS as usize) * 4, PU_STATIC, ptr::null_mut()) as *mut i32;

        for i in 0..NUMSPRITELUMPS {
            let patch =
                w_cache_lump_num(FIRSTSPRITELUMP + i, PU_CACHE) as *const patch_t;
            *SPRITEWIDTH.add(i as usize) = ((*patch).width as i32) << FRACBITS;
            *SPRITEOFFSET.add(i as usize) = ((*patch).leftoffset as i32) << FRACBITS;
            *SPRITETOPOFFSET.add(i as usize) = ((*patch).topoffset as i32) << FRACBITS;
        }
    }
}

fn r_init_colormaps() {
    let lump = w_get_num_for_name(deh_string("COLORMAP"));
    unsafe {
        COLORMAPS = w_cache_lump_num(lump, PU_STATIC);
    }
}
