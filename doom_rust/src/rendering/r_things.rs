//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Rendering of moving objects, sprites.
//
// Original: r_things.h + r_things.c

use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::geometry::{finecosine, finesine, ANG45, ANGLETOFINESHIFT};
use crate::m_fixed::{fixed_div, fixed_mul, Fixed, FRACBITS, FRACUNIT};
use crate::player::p_mobj::{Mobj, FF_FRAMEMASK, FF_FULLBRIGHT, MF_SHADOW};
use crate::rendering::defs::{DrawSeg, Spritedef, SpriteFrame, Vissprite};
use crate::rendering::r_data::r_get_column;
use crate::rendering::r_draw::colfunc;
use crate::rendering::v_video::CENTERY;
use crate::rendering::r_main::{
    r_point_to_angle, r_point_on_seg_side, CENTERXFRAC, CENTERYFRAC, EXTRALIGHT, FIXEDCOLORMAP,
    LIGHTLEVELS, LIGHTSCALESHIFT, MAXLIGHTSCALE, PROJECTION, SCALELIGHT, VALIDCOUNT,
};
use crate::rendering::r_segs::r_render_masked_seg_range;
use crate::rendering::state;
use crate::rendering::v_patch::{patch_t, post_t};
use crate::wad::{w_cache_lump_num, with_lumpinfo};
use crate::z_zone::{z_malloc, PU_CACHE, PU_STATIC};
use std::collections::BTreeSet;
use std::ptr;

const MINZ: Fixed = FRACUNIT * 4;
const MAXVISSPRITES: usize = 128;
const LIGHTSEGSHIFT: i32 = 4;

fn sprite_name_upper4(src: &[u8]) -> [u8; 4] {
    let mut out = [0u8; 4];
    for i in 0..4 {
        let b = src.get(i).copied().unwrap_or(0);
        out[i] = if (b'a'..=b'z').contains(&b) { b - 32 } else { b };
    }
    out
}

// =============================================================================
// State
// =============================================================================

static mut VISSPRITES: [Vissprite; MAXVISSPRITES] = unsafe { std::mem::zeroed() };
static mut VISSPRITE_P: *mut Vissprite = ptr::null_mut();
static mut VSPRSORTEDHEAD: Vissprite = unsafe { std::mem::zeroed() };

/// Sprite names in index order (filled by r_init_sprites).
static mut SPRITE_NAMES: Option<Vec<[u8; 4]>> = None;

/// Clipping arrays for sprite drawing - set by R_DrawSprite.
static mut MFLOORCLIP: *const i16 = ptr::null();
static mut MCEILINGCLIP: *const i16 = ptr::null();

// =============================================================================
// Public API
// =============================================================================

/// Clear sprites at start of frame.
pub fn r_clear_sprites() {
    unsafe {
        VISSPRITE_P = VISSPRITES.as_mut_ptr();
    }
}

/// Load sprite definitions from WAD. Call from R_InitData after R_InitSpriteLumps.
pub fn r_init_sprites() {
    let (first, last, numlumps) = state::with_state(|s| {
        (s.firstspritelump, s.lastspritelump, s.numspritelumps)
    });
    if numlumps <= 0 || first < 0 {
        return;
    }

    let sprite_names: BTreeSet<[u8; 4]> = with_lumpinfo(|lumpinfo| {
        let mut names = BTreeSet::new();
        let first_u = first as usize;
        let last_u = last as usize;
        if last_u >= lumpinfo.len() {
            return names;
        }
        for i in first_u..=last_u {
            let lump = &lumpinfo[i];
            if lump.name[4] == 0 {
                continue;
            }
            names.insert(sprite_name_upper4(&lump.name[0..4]));
        }
        names
    });

    let numsprites = sprite_names.len() as i32;
    if numsprites == 0 {
        return;
    }

    let sprites_ptr = unsafe {
        z_malloc(
            numsprites as usize * std::mem::size_of::<Spritedef>(),
            PU_STATIC,
            ptr::null_mut(),
        ) as *mut Spritedef
    };

    let sprite_vec: Vec<[u8; 4]> = sprite_names.into_iter().collect();

    for (sprite_idx, name) in sprite_vec.iter().enumerate() {
        use std::collections::BTreeMap;
        let mut frame_lumps: BTreeMap<u8, (bool, [i16; 8])> = BTreeMap::new();
        with_lumpinfo(|lumpinfo| {
            let first_u = first as usize;
            let last_u = last.min(lumpinfo.len() as i32 - 1) as usize;
            for i in first_u..=last_u {
                let lump = &lumpinfo[i];
                if lump.name[0..4] != name[..] {
                    continue;
                }
                let frame_char = lump.name[4];
                let rot_char = lump.name[5];
                if frame_char == 0 {
                    continue;
                }
                let lump_offset = (i as i32 - first) as i16;
                let entry = frame_lumps.entry(frame_char).or_insert((false, [0i16; 8]));
                if rot_char == b'[' {
                    entry.0 = false;
                    entry.1[0] = lump_offset;
                } else if rot_char >= b'0' && rot_char <= b'7' {
                    let rot = (rot_char - b'0') as usize;
                    entry.0 = true;
                    entry.1[rot] = lump_offset;
                }
            }
        });

        for (_, (_, lumps)) in frame_lumps.iter_mut() {
            let fill = lumps.iter().find(|&&x| x != 0).copied().unwrap_or(0);
            for i in 0..8 {
                if lumps[i] == 0 {
                    lumps[i] = fill;
                }
            }
        }

        let frame_list: Vec<_> = frame_lumps.into_iter().collect();
        let numframes = frame_list.len() as i32;
        let spriteframes_ptr = unsafe {
            z_malloc(
                numframes as usize * std::mem::size_of::<SpriteFrame>(),
                PU_STATIC,
                ptr::null_mut(),
            ) as *mut SpriteFrame
        };
        for (fi, (_, (rotate, lump))) in frame_list.iter().enumerate() {
            unsafe {
                let sf = spriteframes_ptr.add(fi);
                (*sf).rotate = *rotate;
                (*sf).lump = *lump;
                (*sf).flip = [0u8; 8];
            }
        }
        unsafe {
            let spd = sprites_ptr.add(sprite_idx);
            (*spd).numframes = numframes;
            (*spd).spriteframes = spriteframes_ptr;
        }
    }

    state::with_state_mut(|s| {
        s.numsprites = numsprites;
        s.sprites = sprites_ptr;
    });
    unsafe {
        SPRITE_NAMES = Some(sprite_vec.clone());
    }
}

/// Lookup sprite index by 4-char name (e.g. "TROO", "PLAY"). Returns -1 if not found.
/// Case-insensitive to match Doom lump name convention.
pub fn r_sprite_num_for_name(name: &str) -> i32 {
    let name_bytes = name.as_bytes();
    if name_bytes.len() < 4 {
        return -1;
    }
    let key = sprite_name_upper4(name_bytes);
    unsafe {
        SPRITE_NAMES
            .as_ref()
            .and_then(|names| names.iter().position(|n| n[..] == key[..]))
            .map_or(-1, |i| i as i32)
    }
}

/// Add sprites for sector things.
pub fn r_add_sprites(sec: *mut crate::rendering::defs::Sector) {
    unsafe {
        if sec.is_null() {
            return;
        }
        let validcount = VALIDCOUNT;
        if (*sec).validcount == validcount {
            return;
        }
        (*sec).validcount = validcount;

        let sprites = state::with_state(|s| s.sprites);
        let numsprites = state::with_state(|s| s.numsprites);
        if sprites.is_null() || numsprites <= 0 {
            return;
        }

        let mut lightnum = ((*sec).lightlevel as i32 >> LIGHTSEGSHIFT) + EXTRALIGHT;
        if lightnum < 0 {
            lightnum = 0;
        } else if lightnum >= LIGHTLEVELS as i32 {
            lightnum = LIGHTLEVELS as i32 - 1;
        }
        let spritelights = SCALELIGHT[lightnum as usize].as_ptr();

        let mut thing = (*sec).thinglist;
        while !thing.is_null() {
            r_project_sprite(thing, spritelights);
            thing = (*thing).snext;
        }
    }
}

/// Draw masked things: sprites and masked walls. Call after R_DrawPlanes.
pub fn r_draw_masked() {
    unsafe {
        r_sort_vis_sprites();

        if VISSPRITE_P > VISSPRITES.as_mut_ptr() {
            let mut spr = VSPRSORTEDHEAD.next;
            while spr != &mut VSPRSORTEDHEAD as *mut Vissprite {
                r_draw_sprite(spr);
                spr = (*spr).next;
            }
        }

        let ds_p = state::with_state(|s| s.ds_p);
        let drawsegs = state::with_state_mut(|s| s.drawsegs.as_mut_ptr());
        if !ds_p.is_null() && ds_p > drawsegs {
            let mut ds = ds_p.sub(1);
            while ds >= drawsegs {
                if !(*ds).maskedtexturecol.is_null() {
                    r_render_masked_seg_range(ds, (*ds).x1, (*ds).x2);
                }
                if ds == drawsegs {
                    break;
                }
                ds = ds.sub(1);
            }
        }
    }
}

// =============================================================================
// Internal
// =============================================================================

fn r_new_vis_sprite() -> *mut Vissprite {
    unsafe {
        if VISSPRITE_P >= VISSPRITES.as_mut_ptr().add(MAXVISSPRITES) {
            return ptr::null_mut();
        }
        let vis = VISSPRITE_P;
        VISSPRITE_P = VISSPRITE_P.add(1);
        vis
    }
}

fn r_project_sprite(thing: *mut Mobj, spritelights: *const *mut u8) {
    unsafe {
        let viewx = state::with_state(|s| s.viewx);
        let viewy = state::with_state(|s| s.viewy);
        let viewz = state::with_state(|s| s.viewz);
        let viewcos = crate::rendering::r_main::VIEWCOS;
        let viewsin = crate::rendering::r_main::VIEWSIN;
        let viewwidth = state::with_state(|s| s.viewwidth);
        let centerxfrac = CENTERXFRAC;
        let colormaps = state::with_state(|s| s.colormaps);
        let firstspritelump = state::with_state(|s| s.firstspritelump);
        let sprites = state::with_state(|s| s.sprites);
        let spritewidth = state::with_state(|s| s.spritewidth);
        let spriteoffset = state::with_state(|s| s.spriteoffset);
        let spritetopoffset = state::with_state(|s| s.spritetopoffset);

        if sprites.is_null() || spritewidth.is_null() || spriteoffset.is_null() || spritetopoffset.is_null() {
            return;
        }

        let sprite = (*thing).sprite;
        if sprite < 0 || sprite >= state::with_state(|s| s.numsprites) {
            return;
        }

        let sprdef = sprites.add(sprite as usize);
        let frame_idx = (*thing).frame & FF_FRAMEMASK;
        if frame_idx >= (*sprdef).numframes {
            return;
        }

        let sprframes = (*sprdef).spriteframes;
        if sprframes.is_null() {
            return;
        }
        let sprframe = sprframes.add(frame_idx as usize);

        let lump: i32;
        let flip: bool;
        if (*sprframe).rotate {
            let ang = r_point_to_angle((*thing).x, (*thing).y);
            let rot = (ang.wrapping_sub((*thing).angle).wrapping_add(ANG45 / 2 * 9)) >> 29;
            lump = (*sprframe).lump[rot as usize] as i32;
            flip = (*sprframe).flip[rot as usize] != 0;
        } else {
            lump = (*sprframe).lump[0] as i32;
            flip = (*sprframe).flip[0] != 0;
        }

        let tr_x = (*thing).x - viewx;
        let tr_y = (*thing).y - viewy;

        let gxt = fixed_mul(tr_x, viewcos);
        let gyt = -fixed_mul(tr_y, viewsin);
        let tz = gxt - gyt;

        if tz < MINZ {
            return;
        }

        let xscale = fixed_div(PROJECTION, tz);

        let gxt2 = -fixed_mul(tr_x, viewsin);
        let gyt2 = fixed_mul(tr_y, viewcos);
        let mut tx = -(gyt2 + gxt2);

        if tx.abs() > (tz << 2) {
            return;
        }

        let sw = *spritewidth.add(lump as usize);
        let so = *spriteoffset.add(lump as usize);
        let sto = *spritetopoffset.add(lump as usize);

        tx -= so;
        let mut x1 = (centerxfrac + fixed_mul(tx, xscale)) >> FRACBITS;

        if x1 > viewwidth {
            return;
        }

        tx += sw;
        let mut x2 = ((centerxfrac + fixed_mul(tx, xscale)) >> FRACBITS) - 1;

        if x2 < 0 {
            return;
        }

        let vis = r_new_vis_sprite();
        if vis.is_null() {
            return;
        }

        (*vis).mobjflags = (*thing).flags;
        (*vis).scale = xscale;
        (*vis).gx = (*thing).x;
        (*vis).gy = (*thing).y;
        (*vis).gz = (*thing).z;
        (*vis).gzt = (*thing).z + sto;
        (*vis).texturemid = (*vis).gzt - viewz;
        (*vis).x1 = x1.max(0);
        (*vis).x2 = x2.min(viewwidth - 1);

        let iscale = fixed_div(FRACUNIT, xscale);

        if flip {
            (*vis).startfrac = sw - FRACUNIT;
            (*vis).xiscale = -iscale;
        } else {
            (*vis).startfrac = 0;
            (*vis).xiscale = iscale;
        }

        if (*vis).x1 > x1 {
            (*vis).startfrac += fixed_mul((*vis).xiscale, ((*vis).x1 - x1) as Fixed);
        }
        (*vis).patch = lump;

        if (*thing).flags & MF_SHADOW != 0 {
            (*vis).colormap = ptr::null_mut();
        } else if !FIXEDCOLORMAP.is_null() {
            (*vis).colormap = FIXEDCOLORMAP;
        } else if (*thing).frame & FF_FULLBRIGHT != 0 {
            (*vis).colormap = colormaps;
        } else {
            let detailshift = crate::rendering::r_main::DETAILSHIFT;
            let mut index = (xscale >> (LIGHTSCALESHIFT - detailshift)) as usize;
            index = index.min(MAXLIGHTSCALE - 1);
            (*vis).colormap = spritelights.add(index).read();
        }
    }
}

fn r_sort_vis_sprites() {
    unsafe {
        let count = VISSPRITE_P.offset_from(VISSPRITES.as_ptr()) as usize;
        if count == 0 {
            return;
        }

        let mut unsorted: Vissprite = std::mem::zeroed();
        unsorted.next = &mut unsorted as *mut Vissprite;
        unsorted.prev = &mut unsorted as *mut Vissprite;

        for i in 0..count {
            let ds = VISSPRITES.as_mut_ptr().add(i);
            (*ds).next = if i + 1 < count {
                VISSPRITES.as_mut_ptr().add(i + 1)
            } else {
                &mut unsorted as *mut Vissprite
            };
            (*ds).prev = if i > 0 {
                VISSPRITES.as_mut_ptr().add(i - 1)
            } else {
                &mut unsorted as *mut Vissprite
            };
        }

        (*VISSPRITES.as_mut_ptr()).prev = &mut unsorted as *mut Vissprite;
        unsorted.next = VISSPRITES.as_mut_ptr();
        (*VISSPRITES.as_mut_ptr().add(count - 1)).next = &mut unsorted as *mut Vissprite;
        unsorted.prev = VISSPRITES.as_mut_ptr().add(count - 1);

        VSPRSORTEDHEAD.next = &mut VSPRSORTEDHEAD as *mut Vissprite;
        VSPRSORTEDHEAD.prev = &mut VSPRSORTEDHEAD as *mut Vissprite;

        for _ in 0..count {
            let mut bestscale = i32::MIN;
            let mut best = unsorted.next;

            let mut ds = unsorted.next;
            while ds != &mut unsorted as *mut Vissprite {
                if (*ds).scale > bestscale {
                    bestscale = (*ds).scale;
                    best = ds;
                }
                ds = (*ds).next;
            }

            if !(*best).next.is_null() {
                (*(*best).next).prev = (*best).prev;
            }
            if !(*best).prev.is_null() {
                (*(*best).prev).next = (*best).next;
            }

            (*best).next = &mut VSPRSORTEDHEAD as *mut Vissprite;
            (*best).prev = VSPRSORTEDHEAD.prev;
            if !VSPRSORTEDHEAD.prev.is_null() {
                (*VSPRSORTEDHEAD.prev).next = best;
            }
            VSPRSORTEDHEAD.prev = best;
        }
    }
}

fn r_draw_sprite(spr: *mut Vissprite) {
    use crate::rendering::defs::{SIL_BOTTOM, SIL_TOP};

    unsafe {
        let viewheight = state::with_state(|s| s.viewheight);
        let drawsegs = state::with_state_mut(|s| s.drawsegs.as_mut_ptr());
        let ds_p = state::with_state(|s| s.ds_p);

        let mut clipbot: [i16; 320] = [-2; 320];
        let mut cliptop: [i16; 320] = [-2; 320];

        for x in (*spr).x1..=(*spr).x2 {
            if (x as usize) < 320 {
                clipbot[x as usize] = -2;
                cliptop[x as usize] = -2;
            }
        }

        if !ds_p.is_null() && ds_p > drawsegs {
            let mut ds = ds_p.sub(1);
            while ds >= drawsegs {
                if (*ds).x1 > (*spr).x2
                    || (*ds).x2 < (*spr).x1
                    || ((*ds).silhouette == 0 && (*ds).maskedtexturecol.is_null())
                {
                    ds = if ds == drawsegs {
                        break;
                    } else {
                        ds.sub(1)
                    };
                    continue;
                }

                let r1 = (*ds).x1.max((*spr).x1);
                let r2 = (*ds).x2.min((*spr).x2);

                let (lowscale, scale) = if (*ds).scale1 > (*ds).scale2 {
                    ((*ds).scale2, (*ds).scale1)
                } else {
                    ((*ds).scale1, (*ds).scale2)
                };

                if scale < (*spr).scale
                    || (lowscale < (*spr).scale
                        && r_point_on_seg_side((*spr).gx, (*spr).gy, (*ds).curline) == 0)
                {
                    if !(*ds).maskedtexturecol.is_null() {
                        r_render_masked_seg_range(ds, r1, r2);
                    }
                    ds = if ds == drawsegs {
                        break;
                    } else {
                        ds.sub(1)
                    };
                    continue;
                }

                let mut silhouette = (*ds).silhouette;
                if (*spr).gz >= (*ds).bsilheight {
                    silhouette &= !SIL_BOTTOM;
                }
                if (*spr).gzt <= (*ds).tsilheight {
                    silhouette &= !SIL_TOP;
                }

                for x in r1..=r2 {
                    if (x as usize) < 320 {
                        if silhouette & SIL_BOTTOM != 0 && clipbot[x as usize] == -2 {
                            if !(*ds).sprbottomclip.is_null() {
                                clipbot[x as usize] =
                                    *(*ds).sprbottomclip.add((x - (*ds).x1) as usize);
                            }
                        }
                        if silhouette & SIL_TOP != 0 && cliptop[x as usize] == -2 {
                            if !(*ds).sprtopclip.is_null() {
                                cliptop[x as usize] =
                                    *(*ds).sprtopclip.add((x - (*ds).x1) as usize);
                            }
                        }
                    }
                }

                ds = if ds == drawsegs {
                    break;
                } else {
                    ds.sub(1)
                };
            }
        }

        for x in (*spr).x1..=(*spr).x2 {
            if (x as usize) < 320 {
                if clipbot[x as usize] == -2 {
                    clipbot[x as usize] = viewheight as i16;
                }
                if cliptop[x as usize] == -2 {
                    cliptop[x as usize] = -1;
                }
            }
        }

        MFLOORCLIP = clipbot.as_ptr();
        MCEILINGCLIP = cliptop.as_ptr();
        r_draw_vis_sprite(spr, (*spr).x1, (*spr).x2);
    }
}

fn r_draw_vis_sprite(vis: *mut Vissprite, x1: i32, x2: i32) {
    unsafe {
        let firstspritelump = state::with_state(|s| s.firstspritelump);
        let patch_ptr =
            w_cache_lump_num((*vis).patch + firstspritelump, PU_CACHE).as_ptr() as *const patch_t;
        let centeryfrac = CENTERYFRAC;
        let detailshift = crate::rendering::r_main::DETAILSHIFT;

        crate::rendering::r_draw::DC_COLORMAP = (*vis).colormap;
        crate::rendering::r_draw::DC_ISCALE = ((*vis).xiscale.abs() >> detailshift) as u32;
        crate::rendering::r_draw::DC_TEXTUREMID = (*vis).texturemid;
        let mut frac = (*vis).startfrac;
        let spryscale = (*vis).scale;
        let sprtopscreen = centeryfrac - fixed_mul(
            crate::rendering::r_draw::DC_TEXTUREMID,
            spryscale,
        );

        let mfloorclip = MFLOORCLIP;
        let mceilingclip = MCEILINGCLIP;

        for dc_x in x1..=x2 {
            let texturecolumn = (frac >> FRACBITS) as i32;
            let patch = &*patch_ptr;
            if texturecolumn < 0 || texturecolumn >= patch.width as i32 {
                frac += (*vis).xiscale;
                continue;
            }

            let colofs = if texturecolumn < 8 {
                patch.columnofs[texturecolumn as usize]
            } else {
                let ofs_ptr = (patch_ptr as *const u8).add(8).add(texturecolumn as usize * 4);
                i32::from_le_bytes([
                    *ofs_ptr,
                    *ofs_ptr.add(1),
                    *ofs_ptr.add(2),
                    *ofs_ptr.add(3),
                ])
            };
            let column = (patch_ptr as *const u8).add(colofs as usize) as *const post_t;

            r_draw_masked_column(
                column,
                sprtopscreen,
                spryscale,
                (*vis).texturemid,
                mfloorclip,
                mceilingclip,
            );
            frac += (*vis).xiscale;
        }
    }
}

fn r_draw_masked_column(
    column: *const post_t,
    sprtopscreen: Fixed,
    spryscale: Fixed,
    basetexturemid: Fixed,
    mfloorclip: *const i16,
    mceilingclip: *const i16,
) {
    use crate::rendering::r_draw::{DC_SOURCE, DC_TEXTUREMID, DC_X, DC_YH, DC_YL};

    unsafe {
        let mut column = column;

        while !column.is_null() && (*column).topdelta != 0xff {
            let topscreen = sprtopscreen + fixed_mul((*column).topdelta as i32, spryscale);
            let bottomscreen = topscreen + fixed_mul((*column).length as i32, spryscale);

            let mut dc_yl = (topscreen + FRACUNIT - 1) >> FRACBITS;
            let mut dc_yh = (bottomscreen - 1) >> FRACBITS;

            let dc_x = crate::rendering::r_draw::DC_X;
            if dc_x >= 0 && (dc_x as usize) < 320 && !mfloorclip.is_null() && !mceilingclip.is_null() {
                if dc_yh >= *mfloorclip.add(dc_x as usize) as i32 {
                    dc_yh = *mfloorclip.add(dc_x as usize) as i32 - 1;
                }
                if dc_yl <= *mceilingclip.add(dc_x as usize) as i32 {
                    dc_yl = *mceilingclip.add(dc_x as usize) as i32 + 1;
                }
            }

            if dc_yl <= dc_yh {
                DC_YL = dc_yl;
                DC_YH = dc_yh;
                DC_SOURCE = (column as *const u8).add(3);
                DC_TEXTUREMID = basetexturemid - ((*column).topdelta as i32) * FRACUNIT;

                colfunc();
            }

            column = (column as *const u8)
                .add(4 + (*column).length as usize) as *const post_t;
        }
    }
}
