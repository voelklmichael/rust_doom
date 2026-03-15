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
use crate::rendering::v_patch::{patch_t, ColumnT};
use crate::wad::{
    w_cache_lump_name, w_cache_lump_num, w_check_num_for_name, w_get_num_for_name, w_lump_length,
    w_lump_name_hash, w_release_lump_name, with_lumpinfo,
};
use crate::z_zone::{z_free, z_malloc, PU_CACHE, PU_STATIC};
use std::ptr;
use std::sync::{Mutex, OnceLock};

// =============================================================================
// RDataState - thread-safe via OnceLock + Mutex
// =============================================================================

static R_DATA_STATE: OnceLock<Mutex<RDataState>> = OnceLock::new();

pub struct RDataState {
    pub firstflat: i32,
    pub lastflat: i32,
    pub numflats: i32,
    pub firstspritelump: i32,
    pub lastspritelump: i32,
    pub numspritelumps: i32,
    pub numtextures: i32,
    pub textures: *mut *mut Texture,
    pub textures_hashtable: *mut *mut Texture,
    pub texturewidthmask: *mut i32,
    pub texturecompositesize: *mut i32,
    pub texturecolumnlump: *mut *mut i16,
    pub texturecolumnofs: *mut *mut u16,
    pub texturecomposite: *mut *mut u8,
    pub flattranslation: *mut i32,
    pub texturetranslation: *mut i32,
    pub textureheight: *mut i32,
    pub spritewidth: *mut i32,
    pub spriteoffset: *mut i32,
    pub spritetopoffset: *mut i32,
    pub colormaps: *mut u8,
}

impl Default for RDataState {
    fn default() -> Self {
        Self {
            firstflat: 0,
            lastflat: 0,
            numflats: 0,
            firstspritelump: 0,
            lastspritelump: 0,
            numspritelumps: 0,
            numtextures: 0,
            textures: ptr::null_mut(),
            textures_hashtable: ptr::null_mut(),
            texturewidthmask: ptr::null_mut(),
            texturecompositesize: ptr::null_mut(),
            texturecolumnlump: ptr::null_mut(),
            texturecolumnofs: ptr::null_mut(),
            texturecomposite: ptr::null_mut(),
            flattranslation: ptr::null_mut(),
            texturetranslation: ptr::null_mut(),
            textureheight: ptr::null_mut(),
            spritewidth: ptr::null_mut(),
            spriteoffset: ptr::null_mut(),
            spritetopoffset: ptr::null_mut(),
            colormaps: ptr::null_mut(),
        }
    }
}

/// Access RDataState. Panics if not yet initialized (call r_init_data first).
pub fn with_r_data_state<F, R>(f: F) -> R
where
    F: FnOnce(&RDataState) -> R,
{
    let guard = R_DATA_STATE
        .get()
        .expect("r_data not initialized")
        .lock()
        .unwrap();
    f(&guard)
}

/// Mutably access RDataState. Panics if not yet initialized.
pub fn with_r_data_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut RDataState) -> R,
{
    let mut guard = R_DATA_STATE
        .get()
        .expect("r_data not initialized")
        .lock()
        .unwrap();
    f(&mut guard)
}

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

fn r_draw_column_in_cache(patch: *const ColumnT, cache: *mut u8, originy: i32, cacheheight: i32) {
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
                ptr::copy_nonoverlapping(source, cache.add(position as usize), count as usize);
            }
        }
        patch = unsafe { (patch as *const u8).add(length as usize + 4) as *const ColumnT };
    }
}

// =============================================================================
// R_GenerateComposite (private)
// =============================================================================

fn r_generate_composite(s: &mut RDataState, texnum: i32) {
    unsafe {
        let texture = *s.textures.add(texnum as usize);
        let block = z_malloc(
            *s.texturecompositesize.add(texnum as usize) as usize,
            PU_STATIC,
            s.texturecomposite.add(texnum as usize),
        );
        *s.texturecomposite.add(texnum as usize) = block;

        let collump = *s.texturecolumnlump.add(texnum as usize);
        let colofs = *s.texturecolumnofs.add(texnum as usize);

        let patch_count = (*texture).patchcount as usize;
        let patches_ptr = &(*texture).patches as *const Texpatch;

        for i in 0..patch_count {
            let patch = &*patches_ptr.add(i);
            let lump = patch.patch;
            let realpatch = w_cache_lump_num(lump, PU_CACHE).as_ptr();
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
                    as *const ColumnT;
                r_draw_column_in_cache(
                    patchcol,
                    block.add(*colofs.add(x as usize) as usize),
                    patch.originy as i32,
                    (*texture).height as i32,
                );
                x += 1;
            }
        }
    }
}

// =============================================================================
// R_GenerateLookup (private)
// =============================================================================

fn r_generate_lookup(s: &mut RDataState, texnum: i32) {
    unsafe {
        let texture = *s.textures.add(texnum as usize);
        *s.texturecomposite.add(texnum as usize) = ptr::null_mut();
        *s.texturecompositesize.add(texnum as usize) = 0;

        let collump = *s.texturecolumnlump.add(texnum as usize);
        let colofs = *s.texturecolumnofs.add(texnum as usize);

        let width = (*texture).width as usize;
        let patchcount = z_malloc(width, PU_STATIC, ptr::null_mut()) as *mut u8;
        ptr::write_bytes(patchcount, 0, width);

        let patch_count = (*texture).patchcount as usize;
        let patches_ptr = &(*texture).patches as *const Texpatch;

        for i in 0..patch_count {
            let patch = &*patches_ptr.add(i);
            let realpatch = w_cache_lump_num(patch.patch, PU_CACHE).as_ptr();
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
                *colofs.add(x as usize) = (patch_columnofs(realpatch, col_idx) + 3) as u16;
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
                *colofs.add(x) = *s.texturecompositesize.add(texnum as usize) as u16;
                let new_size =
                    *s.texturecompositesize.add(texnum as usize) + (*texture).height as i32;
                if new_size > 0x10000 - (*texture).height as i32 {
                    i_system::i_error("R_GenerateLookup: texture >64k");
                }
                *s.texturecompositesize.add(texnum as usize) = new_size;
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
    with_r_data_state_mut(|s| unsafe {
        let mask = *s.texturewidthmask.add(tex as usize);
        let col = col & mask;
        let lump = *(*s.texturecolumnlump.add(tex as usize)).add(col as usize);
        let ofs = *(*s.texturecolumnofs.add(tex as usize)).add(col as usize);

        if lump > 0 {
            return w_cache_lump_num(lump as i32, PU_CACHE)
                .as_ptr_mut()
                .add(ofs as usize);
        }
        if (*s.texturecomposite.add(tex as usize)).is_null() {
            r_generate_composite(s, tex);
        }
        (*s.texturecomposite.add(tex as usize)).add(ofs as usize)
    })
}

/// I/O, setting up the stuff. Must be called after W_Init.
pub fn r_init_data() {
    let _ = R_DATA_STATE.set(Mutex::new(RDataState::default()));

    with_r_data_state_mut(|s| {
        r_init_textures(s);
        let InitFlats {
            firstflat,
            flattranslation,
        } = r_init_flats();
        s.firstflat = firstflat;
        s.flattranslation = Box::leak(flattranslation.into_boxed_slice()).as_mut_ptr();
        r_init_sprite_lumps(s);
        r_init_colormaps(s);
    });

    // Sync state to r_state
    with_r_data_state(|s| {
        state::with_state_mut(|rs| {
            rs.firstflat = s.firstflat;
            rs.flattranslation = s.flattranslation;
            rs.texturetranslation = s.texturetranslation;
            rs.textureheight = s.textureheight;
            rs.firstspritelump = s.firstspritelump;
            rs.lastspritelump = s.lastspritelump;
            rs.numspritelumps = s.numspritelumps;
            rs.spritewidth = s.spritewidth;
            rs.spriteoffset = s.spriteoffset;
            rs.spritetopoffset = s.spritetopoffset;
            rs.colormaps = s.colormaps;
        });
    });
    crate::rendering::r_things::r_init_sprites();
}

// /// Preloads all relevant graphics for the level.
// pub fn r_precache_level() {
//     use crate::doomstat;
//     use crate::rendering::r_sky;

//     unsafe {
//         if doomstat::DEMOPLAYBACK {
//             return;
//         }

//         let numsectors = state::with_state(|s| s.numsectors);
//         let sectors = state::with_state(|s| s.sectors);
//         let numsides = state::with_state(|s| s.numsides);
//         let sides = state::with_state(|s| s.sides);
//         let numsprites = state::with_state(|s| s.numsprites);
//         let sprites = state::with_state(|s| s.sprites);

//         // Precache flats
//         if numsectors > 0 && !sectors.is_null() {
//             let mut flatpresent = vec![0u8; NUMFLATS as usize];
//             for i in 0..numsectors {
//                 let sec = &*sectors.add(i as usize);
//                 flatpresent[(*sec).floorpic as usize] = 1;
//                 flatpresent[(*sec).ceilingpic as usize] = 1;
//             }
//             with_lumpinfo(|lumpinfo| {
//                 for i in 0..NUMFLATS {
//                     if flatpresent[i as usize] != 0 {
//                         let lump = (FIRSTFLAT + i) as usize;
//                         if lump < lumpinfo.len() {
//                             w_cache_lump_num((FIRSTFLAT + i) as i32, PU_CACHE);
//                         }
//                     }
//                 }
//             });
//         }

//         // Precache textures
//         if numsides > 0 && !sides.is_null() {
//             let mut texturepresent = vec![0u8; NUMTEXTURES as usize];
//             for i in 0..numsides {
//                 let side = &*sides.add(i as usize);
//                 let top = (*side).toptexture as usize;
//                 let mid = (*side).midtexture as usize;
//                 let bottom = (*side).bottomtexture as usize;
//                 if top < texturepresent.len() {
//                     texturepresent[top] = 1;
//                 }
//                 if mid < texturepresent.len() {
//                     texturepresent[mid] = 1;
//                 }
//                 if bottom < texturepresent.len() {
//                     texturepresent[bottom] = 1;
//                 }
//             }
//             let skytex = r_sky::SKYTEXTURE as usize;
//             if skytex < texturepresent.len() {
//                 texturepresent[skytex] = 1;
//             }
//             for i in 0..NUMTEXTURES {
//                 if texturepresent[i as usize] == 0 {
//                     continue;
//                 }
//                 let texture = *TEXTURES.add(i as usize);
//                 for j in 0..(*texture).patchcount {
//                     w_cache_lump_num(
//                         (*texture).patches.as_ptr().add(j as usize).read().patch,
//                         PU_CACHE,
//                     );
//                 }
//             }
//         }

//         // Precache sprites - requires thinker list (p_local), skip for now
//         if numsprites > 0 && !sprites.is_null() {
//             // TODO: iterate thinkercap when p_local is ported
//         }
//     }
// }

/// Check whether flat is available. Returns -1 if not found.
pub fn r_check_flat_num_for_name(name: &str) -> i32 {
    let i = w_check_num_for_name(name);
    if i < 0 {
        return -1;
    }
    with_r_data_state(|s| i - s.firstflat)
}

/// Get flat number for a flat name.
pub fn r_flat_num_for_name(name: &str) -> i32 {
    let i = r_check_flat_num_for_name(name);
    if i < 0 {
        let namet = if name.len() >= 8 { &name[..8] } else { name };
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
    with_r_data_state(|s| {
        unsafe {
            let key = (w_lump_name_hash(name) as i32) % s.numtextures;
            let mut texture = *s.textures_hashtable.add(key as usize);
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
    })
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

fn generate_texture_hash_table(s: &mut RDataState) {
    unsafe {
        s.textures_hashtable = z_malloc(
            (s.numtextures as usize) * std::mem::size_of::<*mut Texture>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut Texture;
        ptr::write_bytes(
            s.textures_hashtable,
            0,
            (s.numtextures as usize) * std::mem::size_of::<*mut Texture>(),
        );

        for i in 0..s.numtextures {
            let tex = *s.textures.add(i as usize);
            (*tex).index = i;

            let name = std::str::from_utf8(&(*tex).name)
                .unwrap_or("")
                .trim_end_matches('\0');
            let key = (w_lump_name_hash(name) as usize) % (s.numtextures as usize);

            let mut rover = s.textures_hashtable.add(key);
            while !(*rover).is_null() {
                rover = &mut (*(*rover)).next;
            }
            (*tex).next = ptr::null_mut();
            *rover = tex;
        }
    }
}

fn r_init_textures(s: &mut RDataState) {
    let pnames = w_cache_lump_name(deh_string("PNAMES"), PU_STATIC).as_ptr();
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

    let maptex1 = w_cache_lump_name(deh_string("TEXTURE1"), PU_STATIC).as_ptr();
    let numtextures1 = read_i32_le(maptex1);
    let maxoff = w_lump_length(w_get_num_for_name(deh_string("TEXTURE1")) as usize);

    let (maptex2, numtextures2, maxoff2) = if w_check_num_for_name(deh_string("TEXTURE2")) >= 0 {
        let m2 = w_cache_lump_name(deh_string("TEXTURE2"), PU_STATIC).as_ptr();
        let n2 = read_i32_le(m2);
        let o2 = w_lump_length(w_get_num_for_name(deh_string("TEXTURE2")) as usize);
        (Some(m2), n2, o2)
    } else {
        (None, 0, 0)
    };

    unsafe {
        s.numtextures = numtextures1 + numtextures2;
        s.textures = z_malloc(
            (s.numtextures as usize) * std::mem::size_of::<*mut Texture>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut Texture;
        s.texturecolumnlump = z_malloc(
            (s.numtextures as usize) * std::mem::size_of::<*mut i16>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut i16;
        s.texturecolumnofs = z_malloc(
            (s.numtextures as usize) * std::mem::size_of::<*mut u16>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut u16;
        s.texturecomposite = z_malloc(
            (s.numtextures as usize) * std::mem::size_of::<*mut u8>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut *mut u8;
        s.texturecompositesize = z_malloc(
            (s.numtextures as usize) * std::mem::size_of::<i32>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut i32;
        s.texturewidthmask = z_malloc(
            (s.numtextures as usize) * std::mem::size_of::<i32>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut i32;
        s.textureheight = z_malloc(
            (s.numtextures as usize) * std::mem::size_of::<i32>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut i32;

        let mut directory = maptex1.add(4);
        let mut maxoff = maxoff;
        let mut maptex = maptex1;

        for i in 0..s.numtextures {
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
            *s.textures.add(i as usize) = texture;

            (*texture).width = mwidth;
            (*texture).height = mheight;
            (*texture).patchcount = mpatchcount as i16;
            ptr::copy_nonoverlapping((*mtexture).name.as_ptr(), (*texture).name.as_mut_ptr(), 8);

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
                    let name = std::str::from_utf8(&(*texture).name)
                        .unwrap_or("")
                        .trim_end_matches('\0');
                    i_system::i_error(&format!(
                        "R_InitTextures: Missing patch in texture {}",
                        name
                    ));
                }
            }

            *s.texturecolumnlump.add(i as usize) =
                z_malloc((mwidth as usize) * 2, PU_STATIC, ptr::null_mut()) as *mut i16;
            *s.texturecolumnofs.add(i as usize) =
                z_malloc((mwidth as usize) * 2, PU_STATIC, ptr::null_mut()) as *mut u16;

            let mut j = 1i32;
            while j * 2 <= mwidth as i32 {
                j <<= 1;
            }
            *s.texturewidthmask.add(i as usize) = j - 1;
            *s.textureheight.add(i as usize) = (mheight as i32) << FRACBITS;

            directory = directory.add(4);
        }

        z_free(patchlookup as *mut u8);
    }

    w_release_lump_name(deh_string("TEXTURE1"));
    if maptex2.is_some() {
        w_release_lump_name(deh_string("TEXTURE2"));
    }

    unsafe {
        for i in 0..s.numtextures {
            r_generate_lookup(s, i);
        }
    }

    unsafe {
        s.texturetranslation = z_malloc(
            ((s.numtextures + 1) as usize) * std::mem::size_of::<i32>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut i32;
        for i in 0..s.numtextures {
            *s.texturetranslation.add(i as usize) = i;
        }
        generate_texture_hash_table(s);
    }
}

struct InitFlats {
    firstflat: i32,
    flattranslation: Vec<i32>,
}

#[must_use]
fn r_init_flats() -> InitFlats {
    let firstflat = w_get_num_for_name(deh_string("F_START")) + 1;
    let lastflat = w_get_num_for_name(deh_string("F_END")) - 1;
    let numflats = lastflat - firstflat + 1;
    let flattranslation = (0..numflats).collect::<Vec<i32>>();

    InitFlats {
        firstflat,
        flattranslation,
    }
}

fn r_init_sprite_lumps(s: &mut RDataState) {
    unsafe {
        s.firstspritelump = w_get_num_for_name(deh_string("S_START")) + 1;
        s.lastspritelump = w_get_num_for_name(deh_string("S_END")) - 1;
        s.numspritelumps = s.lastspritelump - s.firstspritelump + 1;
    }

    unsafe {
        s.spritewidth =
            z_malloc((s.numspritelumps as usize) * 4, PU_STATIC, ptr::null_mut()) as *mut i32;
        s.spriteoffset =
            z_malloc((s.numspritelumps as usize) * 4, PU_STATIC, ptr::null_mut()) as *mut i32;
        s.spritetopoffset =
            z_malloc((s.numspritelumps as usize) * 4, PU_STATIC, ptr::null_mut()) as *mut i32;

        for i in 0..s.numspritelumps {
            let patch =
                w_cache_lump_num(s.firstspritelump + i, PU_CACHE).as_ptr() as *const patch_t;
            *s.spritewidth.add(i as usize) = ((*patch).width as i32) << FRACBITS;
            *s.spriteoffset.add(i as usize) = ((*patch).leftoffset as i32) << FRACBITS;
            *s.spritetopoffset.add(i as usize) = ((*patch).topoffset as i32) << FRACBITS;
        }
    }
}

fn r_init_colormaps(s: &mut RDataState) {
    let lump = w_get_num_for_name(deh_string("COLORMAP"));
    unsafe {
        s.colormaps = w_cache_lump_num(lump, PU_STATIC).as_ptr_mut();
    }
}
