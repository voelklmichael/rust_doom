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
use crate::rendering::defs::{DrawSeg, SpriteFrame, Spritedef, Vissprite};
use crate::rendering::r_data::r_get_column;
use crate::rendering::r_draw::colfunc;
use crate::rendering::r_main::{
    r_point_on_seg_side, r_point_to_angle, with_r_main_state, LIGHTLEVELS, LIGHTSCALESHIFT,
    MAXLIGHTSCALE,
};
use crate::rendering::r_segs::r_render_masked_seg_range;
use crate::rendering::state;
use crate::rendering::v_patch::{patch_t, post_t};
use crate::wad::{w_cache_lump_num, with_lumpinfo};
use crate::z_zone::{z_malloc, PU_CACHE, PU_STATIC};
use std::collections::BTreeSet;
use std::ptr::{self, addr_of, addr_of_mut};
use std::sync::{Mutex, OnceLock};

const MINZ: Fixed = FRACUNIT * 4;
const MAXVISSPRITES: usize = 128;
const LIGHTSEGSHIFT: i32 = 4;

fn sprite_name_upper4(src: &[u8]) -> [u8; 4] {
    let mut out = [0u8; 4];
    for i in 0..4 {
        let b = src.get(i).copied().unwrap_or(0);
        out[i] = if b.is_ascii_lowercase() { b - 32 } else { b };
    }
    out
}

// =============================================================================
// State - thread-safe via OnceLock + Mutex
// =============================================================================

struct ThingsState {
    vissprites: [Vissprite; MAXVISSPRITES],
    visprite_p: usize,
    vsprsortedhead: Vissprite,
    sprite_names: Option<Vec<[u8; 4]>>,
}

impl Default for ThingsState {
    fn default() -> Self {
        Self {
            vissprites: unsafe { std::mem::zeroed() },
            visprite_p: 0,
            vsprsortedhead: unsafe { std::mem::zeroed() },
            sprite_names: None,
        }
    }
}

static THINGS_STATE: OnceLock<Mutex<ThingsState>> = OnceLock::new();

fn with_things_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut ThingsState) -> R,
{
    let mut guard = THINGS_STATE
        .get_or_init(|| Mutex::new(ThingsState::default()))
        .lock()
        .unwrap();
    f(&mut guard)
}

// =============================================================================
// Public API
// =============================================================================

/// Clear sprites at start of frame.
pub fn r_clear_sprites() {
    with_things_state(|state| {
        state.visprite_p = 0;
    });
}

/// Load sprite definitions from WAD. Call from R_InitData after R_InitSpriteLumps.
pub fn r_init_sprites() {
    let (first, last, numlumps) =
        state::with_state(|s| (s.firstspritelump, s.lastspritelump, s.numspritelumps));
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
        with_things_state(|state| state.sprite_names = Some(sprite_vec.clone()));
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
    with_things_state(|state| {
        state
            .sprite_names
            .as_ref()
            .and_then(|names| names.iter().position(|n| n[..] == key[..]))
            .map_or(-1, |i| i as i32)
    })
}

/// Add sprites for sector things.
pub fn r_add_sprites(sector_idx: usize) {
    let (validcount, lightlevel, thinglist) = state::with_state_mut(|s| {
        let sec = match s.sectors.get_mut(sector_idx) {
            Some(sec) => sec,
            None => return (0, 0, None),
        };
        let validcount = crate::rendering::r_main::with_r_main_state(|rm| rm.validcount);
        if sec.validcount == validcount {
            return (validcount, 0, None);
        }
        sec.validcount = validcount;
        (validcount, sec.lightlevel as i32, sec.thinglist)
    });

    if thinglist.is_none() {
        return;
    }

    with_things_state(|state| {
        let sprites = state::with_state(|s| s.sprites);
        let numsprites = state::with_state(|s| s.numsprites);
        if sprites.is_null() || numsprites <= 0 {
            return;
        }

        let extralight = crate::rendering::r_main::with_r_main_state(|rm| rm.extralight);
        let mut lightnum = (lightlevel >> LIGHTSEGSHIFT) + extralight;
        if lightnum < 0 {
            lightnum = 0;
        } else if lightnum >= LIGHTLEVELS as i32 {
            lightnum = LIGHTLEVELS as i32 - 1;
        }
        let spritelights = crate::rendering::r_main::with_r_main_state(|rm| {
            rm.scalelight[lightnum as usize].as_ptr() as *mut *mut u8
        });

        let mut thing = thinglist;
        while let Some(idx) = thing {
            let ptr = crate::player::mobjs::mobj_ptr_from_index(idx);
            if !ptr.is_null() {
                unsafe {
                    r_project_sprite(ptr, spritelights, state);
                    thing = crate::player::mobjs::mobj_index_from_ptr((*ptr).snext);
                }
            } else {
                break;
            }
        }
    });
}

/// Draw masked things: sprites and masked walls. Call after R_DrawPlanes.
pub fn r_draw_masked() {
    with_things_state(|state| {
        r_sort_vis_sprites_impl(state);

        if state.visprite_p > 0 {
            let visptr = state.vissprites.as_mut_ptr();
            let head_ptr = &state.vsprsortedhead as *const Vissprite as *mut Vissprite;
            let mut spr = state.vsprsortedhead.next;
            while spr != head_ptr {
                r_draw_sprite_impl(spr, state);
                spr = unsafe { (*spr).next };
            }
        }
    });

    unsafe {
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

fn r_new_vis_sprite(state: &mut ThingsState) -> *mut Vissprite {
    if state.visprite_p >= MAXVISSPRITES {
        return ptr::null_mut();
    }
    let vis = unsafe { state.vissprites.as_mut_ptr().add(state.visprite_p) };
    state.visprite_p += 1;
    vis
}

fn r_project_sprite(thing: *mut Mobj, spritelights: *const *mut u8, state: &mut ThingsState) {
    unsafe {
        let viewx = state::with_state(|s| s.viewx);
        let viewy = state::with_state(|s| s.viewy);
        let viewz = state::with_state(|s| s.viewz);
        let (viewcos, viewsin, centerxfrac) = crate::rendering::r_main::with_r_main_state(|rm| {
            (rm.viewcos, rm.viewsin, rm.centerxfrac)
        });
        let viewwidth = state::with_state(|s| s.viewwidth);
        let colormaps = state::with_state(|s| s.colormaps);
        let firstspritelump = state::with_state(|s| s.firstspritelump);
        let sprites = state::with_state(|s| s.sprites);
        let spritewidth = state::with_state(|s| s.spritewidth);
        let spriteoffset = state::with_state(|s| s.spriteoffset);
        let spritetopoffset = state::with_state(|s| s.spritetopoffset);

        if sprites.is_null()
            || spritewidth.is_null()
            || spriteoffset.is_null()
            || spritetopoffset.is_null()
        {
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

        let projection = crate::rendering::r_main::with_r_main_state(|rm| rm.projection);
        let xscale = fixed_div(projection, tz);

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
        let x1 = (centerxfrac + fixed_mul(tx, xscale)) >> FRACBITS;

        if x1 > viewwidth {
            return;
        }

        tx += sw;
        let x2 = ((centerxfrac + fixed_mul(tx, xscale)) >> FRACBITS) - 1;

        if x2 < 0 {
            return;
        }

        let vis = r_new_vis_sprite(state);
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

        let fixedcolormap = crate::rendering::r_main::with_r_main_state(|rm| rm.fixedcolormap);
        if (*thing).flags & MF_SHADOW != 0 {
            (*vis).colormap = ptr::null_mut();
        } else if !fixedcolormap.is_null() {
            (*vis).colormap = fixedcolormap;
        } else if (*thing).frame & FF_FULLBRIGHT != 0 {
            (*vis).colormap = colormaps;
        } else {
            let detailshift = crate::rendering::r_main::with_r_main_state(|rm| rm.detailshift);
            let mut index = (xscale >> (LIGHTSCALESHIFT - detailshift)) as usize;
            index = index.min(MAXLIGHTSCALE - 1);
            (*vis).colormap = spritelights.add(index).read();
        }
    }
}

fn r_sort_vis_sprites_impl(state: &mut ThingsState) {
    let count = state.visprite_p;
    if count == 0 {
        return;
    }

    unsafe {
        let mut unsorted: Vissprite = std::mem::zeroed();
        let unsorted_ptr = addr_of_mut!(unsorted);
        unsorted.next = unsorted_ptr as *mut Vissprite;
        unsorted.prev = unsorted_ptr as *mut Vissprite;

        let visptr = state.vissprites.as_mut_ptr();
        for i in 0..count {
            let ds = visptr.add(i);
            (*ds).next = if i + 1 < count {
                visptr.add(i + 1)
            } else {
                unsorted_ptr as *mut Vissprite
            };
            (*ds).prev = if i > 0 {
                visptr.add(i - 1)
            } else {
                unsorted_ptr as *mut Vissprite
            };
        }

        (*visptr).prev = unsorted_ptr as *mut Vissprite;
        unsorted.next = visptr;
        (*visptr.add(count - 1)).next = unsorted_ptr as *mut Vissprite;
        unsorted.prev = visptr.add(count - 1);

        let head_ptr = &mut state.vsprsortedhead as *mut Vissprite;
        state.vsprsortedhead.next = head_ptr;
        state.vsprsortedhead.prev = head_ptr;

        for _ in 0..count {
            let mut bestscale = i32::MIN;
            let mut best = unsorted.next;

            let mut ds = unsorted.next;
            while ds != unsorted_ptr as *mut Vissprite {
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

            (*best).next = head_ptr;
            (*best).prev = state.vsprsortedhead.prev;
            if !state.vsprsortedhead.prev.is_null() {
                (*state.vsprsortedhead.prev).next = best;
            }
            state.vsprsortedhead.prev = best;
        }
    }
}

fn r_draw_sprite_impl(spr: *mut Vissprite, state: &mut ThingsState) {
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
                        && !(*ds).curline.is_null()
                        && r_point_on_seg_side((*spr).gx, (*spr).gy, unsafe { &*(*ds).curline })
                            == 0)
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

        r_draw_vis_sprite(
            spr,
            (*spr).x1,
            (*spr).x2,
            clipbot.as_ptr(),
            cliptop.as_ptr(),
        );
    }
}

fn r_draw_vis_sprite(
    vis: *mut Vissprite,
    x1: i32,
    x2: i32,
    mfloorclip: *const i16,
    mceilingclip: *const i16,
) {
    let (centeryfrac, detailshift) =
        crate::rendering::r_main::with_r_main_state(|rm| (rm.centeryfrac, rm.detailshift));
    unsafe {
        let firstspritelump = state::with_state(|s| s.firstspritelump);
        let patch_ptr =
            w_cache_lump_num((*vis).patch + firstspritelump, PU_CACHE).as_ptr() as *const patch_t;

        crate::rendering::r_draw::with_r_draw_state_mut(|rd| {
            rd.dc_colormap = (*vis).colormap;
            rd.dc_iscale = ((*vis).xiscale.abs() >> detailshift) as u32;
            rd.dc_texturemid = (*vis).texturemid;
        });
        let mut frac = (*vis).startfrac;
        let spryscale = (*vis).scale;
        let dc_texturemid = crate::rendering::r_draw::with_r_draw_state(|rd| rd.dc_texturemid);
        let sprtopscreen = centeryfrac - fixed_mul(dc_texturemid, spryscale);

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
                let ofs_ptr = (patch_ptr as *const u8)
                    .add(8)
                    .add(texturecolumn as usize * 4);
                i32::from_le_bytes([*ofs_ptr, *ofs_ptr.add(1), *ofs_ptr.add(2), *ofs_ptr.add(3)])
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
    use crate::rendering::r_draw::{colfunc, with_r_draw_state, with_r_draw_state_mut};

    unsafe {
        let mut column = column;

        while !column.is_null() && (*column).topdelta != 0xff {
            let topscreen = sprtopscreen + fixed_mul((*column).topdelta as i32, spryscale);
            let bottomscreen = topscreen + fixed_mul((*column).length as i32, spryscale);

            let mut dc_yl = (topscreen + FRACUNIT - 1) >> FRACBITS;
            let mut dc_yh = (bottomscreen - 1) >> FRACBITS;

            let dc_x = with_r_draw_state(|rd| rd.dc_x);
            if dc_x >= 0
                && (dc_x as usize) < 320
                && !mfloorclip.is_null()
                && !mceilingclip.is_null()
            {
                if dc_yh >= *mfloorclip.add(dc_x as usize) as i32 {
                    dc_yh = *mfloorclip.add(dc_x as usize) as i32 - 1;
                }
                if dc_yl <= *mceilingclip.add(dc_x as usize) as i32 {
                    dc_yl = *mceilingclip.add(dc_x as usize) as i32 + 1;
                }
            }

            if dc_yl <= dc_yh {
                with_r_draw_state_mut(|rd| {
                    rd.dc_yl = dc_yl;
                    rd.dc_yh = dc_yh;
                    rd.dc_source = (column as *const u8).add(3);
                    rd.dc_texturemid = basetexturemid - ((*column).topdelta as i32) * FRACUNIT;
                });
                colfunc();
            }

            column = (column as *const u8).add(4 + (*column).length as usize) as *const post_t;
        }
    }
}
