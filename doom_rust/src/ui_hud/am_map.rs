//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION: AutoMap module.
// Original: am_map.h + am_map.c

use crate::deh::deh_string;
use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use crate::doomstat::{
    AUTOMAPACTIVE, CONSOLEPLAYER, GAMEEPISODE, GAMEMAP, NETGAME, PLAYERS, PLAYERINGAME, VIEWACTIVE,
};
use crate::game::d_event::{Event, EvType};
use crate::geometry::{finecosine, finesine, ANGLETOFINESHIFT};
use crate::m_fixed::{fixed_div, fixed_mul, Fixed, FRACBITS, FRACUNIT};
use crate::player::p_mobj::Mobj;
use crate::player::{MAPBLOCKUNITS, PLAYERRADIUS};
use crate::rendering::defs::{Line, Sector, Vertex, ML_DONTDRAW, ML_MAPPED, ML_SECRET};
use crate::rendering::state;
use crate::rendering::{patch_t, v_draw_patch, v_mark_rect, v_video};
use crate::ui_hud::cheat::{cht_check_cheat, CheatSeq};
use crate::ui_hud::controls::{
    key_map_clearmark, key_map_east, key_map_follow, key_map_grid, key_map_mark,
    key_map_maxzoom, key_map_north, key_map_south, key_map_toggle, key_map_west,
    key_map_zoomin, key_map_zoomout,
};
use crate::wad::{w_cache_lump_name, w_release_lump_name};
use crate::z_zone::PU_STATIC;
use std::ptr;
use std::sync::{Mutex, OnceLock};

// =============================================================================
// Constants (from am_map.c)
// =============================================================================

const AM_NUMMARKPOINTS: usize = 10;
const INITSCALEMTOF: Fixed = ((0.2_f32 * FRACUNIT as f32) as i32);
const F_PANINC: i32 = 4;
const M_ZOOMIN: Fixed = ((1.02_f32 * FRACUNIT as f32) as i32);
const M_ZOOMOUT: Fixed = ((FRACUNIT as f32 / 1.02) as i32);

// Color indices (palette)
const REDS: i32 = 256 - 5 * 16;
const REDRANGE: i32 = 16;
const BLUES: i32 = 256 - 4 * 16 + 8;
const BLUERANGE: i32 = 8;
const GREENS: i32 = 7 * 16;
const GREENRANGE: i32 = 16;
const GRAYS: i32 = 6 * 16;
const GRAYSRANGE: i32 = 16;
const BROWNS: i32 = 4 * 16;
const BROWNRANGE: i32 = 16;
const YELLOWS: i32 = 256 - 32 + 7;
const YELLOWRANGE: i32 = 1;
const BLACK: i32 = 0;
const WHITE: i32 = 256 - 47;

const BACKGROUND: i32 = BLACK;
const YOURCOLORS: i32 = WHITE;
const WALLCOLORS: i32 = REDS;
const TSWALLCOLORS: i32 = GRAYS;
const FDWALLCOLORS: i32 = BROWNS;
const CDWALLCOLORS: i32 = YELLOWS;
const THINGCOLORS: i32 = GREENS;
const SECRETWALLCOLORS: i32 = WALLCOLORS;
const WALLRANGE: i32 = REDRANGE;
const THINGRANGE: i32 = GREENRANGE;
const GRIDCOLORS: i32 = GRAYS + GRAYSRANGE / 2;
const XHAIRCOLORS: i32 = GRAYS;

const LINE_NEVERSEE: i16 = ML_DONTDRAW;

// =============================================================================
// Types
// =============================================================================

#[derive(Clone, Copy)]
struct Fpoint {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Fline {
    a: Fpoint,
    b: Fpoint,
}

#[derive(Clone, Copy)]
struct Mpoint {
    x: Fixed,
    y: Fixed,
}

#[derive(Clone, Copy)]
struct Mline {
    a: Mpoint,
    b: Mpoint,
}

// =============================================================================
// State
// =============================================================================

static AM_MAP_STATE: OnceLock<Mutex<AmMapState>> = OnceLock::new();

/// Safety: Raw pointers (MARKNUMS) point to zone-allocated patches.
unsafe impl Send for AmMapState {}

pub struct AmMapState {
    pub cheating: i32,
    pub grid: i32,
    pub level_just_started: i32,
    pub finit_width: i32,
    pub finit_height: i32,
    pub f_x: i32,
    pub f_y: i32,
    pub f_w: i32,
    pub f_h: i32,
    pub lightlev: i32,
    pub amclock: i32,
    pub m_paninc: Mpoint,
    pub mtof_zoommul: Fixed,
    pub ftom_zoommul: Fixed,
    pub m_x: Fixed,
    pub m_y: Fixed,
    pub m_x2: Fixed,
    pub m_y2: Fixed,
    pub m_w: Fixed,
    pub m_h: Fixed,
    pub min_x: Fixed,
    pub min_y: Fixed,
    pub max_x: Fixed,
    pub max_y: Fixed,
    pub max_w: Fixed,
    pub max_h: Fixed,
    pub min_w: Fixed,
    pub min_h: Fixed,
    pub min_scale_mtof: Fixed,
    pub max_scale_mtof: Fixed,
    pub old_m_w: Fixed,
    pub old_m_h: Fixed,
    pub old_m_x: Fixed,
    pub old_m_y: Fixed,
    pub f_oldloc: Mpoint,
    pub scale_mtof: Fixed,
    pub scale_ftom: Fixed,
    pub marknums: [*const patch_t; 10],
    pub markpoints: [Mpoint; AM_NUMMARKPOINTS],
    pub markpointnum: usize,
    pub followplayer: i32,
    pub stopped: bool,
    pub lastlevel: i32,
    pub lastepisode: i32,
    pub bigstate: i32,
}

impl Default for AmMapState {
    fn default() -> Self {
        Self {
            cheating: 0,
            grid: 0,
            level_just_started: 1,
            finit_width: SCREENWIDTH,
            finit_height: SCREENHEIGHT - 32,
            f_x: 0,
            f_y: 0,
            f_w: SCREENWIDTH,
            f_h: SCREENHEIGHT - 32,
            lightlev: 0,
            amclock: 0,
            m_paninc: Mpoint { x: 0, y: 0 },
            mtof_zoommul: FRACUNIT,
            ftom_zoommul: FRACUNIT,
            m_x: 0,
            m_y: 0,
            m_x2: 0,
            m_y2: 0,
            m_w: 0,
            m_h: 0,
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
            max_w: 0,
            max_h: 0,
            min_w: 0,
            min_h: 0,
            min_scale_mtof: 0,
            max_scale_mtof: 0,
            old_m_w: 0,
            old_m_h: 0,
            old_m_x: 0,
            old_m_y: 0,
            f_oldloc: Mpoint { x: 0, y: 0 },
            scale_mtof: INITSCALEMTOF,
            scale_ftom: 0,
            marknums: [ptr::null(); 10],
            markpoints: [Mpoint { x: -1, y: -1 }; AM_NUMMARKPOINTS],
            markpointnum: 0,
            followplayer: 1,
            stopped: true,
            lastlevel: -1,
            lastepisode: -1,
            bigstate: 0,
        }
    }
}

fn get_am_map_state() -> &'static Mutex<AmMapState> {
    AM_MAP_STATE.get_or_init(|| Mutex::new(AmMapState::default()))
}

/// Access AmMapState.
pub fn with_am_map_state<F, R>(f: F) -> R
where
    F: FnOnce(&AmMapState) -> R,
{
    let guard = get_am_map_state().lock().unwrap();
    f(&guard)
}

/// Mutably access AmMapState.
pub fn with_am_map_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut AmMapState) -> R,
{
    let mut guard = get_am_map_state().lock().unwrap();
    f(&mut guard)
}

// Player arrow (vector graphics)
fn player_arrow() -> [Mline; 7] {
    let r = (8 * PLAYERRADIUS) / 7;
    [
        Mline {
            a: Mpoint { x: -r + r / 8, y: 0 },
            b: Mpoint { x: r, y: 0 },
        },
        Mline {
            a: Mpoint { x: r, y: 0 },
            b: Mpoint { x: r - r / 2, y: r / 4 },
        },
        Mline {
            a: Mpoint { x: r, y: 0 },
            b: Mpoint { x: r - r / 2, y: -r / 4 },
        },
        Mline {
            a: Mpoint { x: -r + r / 8, y: 0 },
            b: Mpoint { x: -r - r / 8, y: r / 4 },
        },
        Mline {
            a: Mpoint { x: -r + r / 8, y: 0 },
            b: Mpoint { x: -r - r / 8, y: -r / 4 },
        },
        Mline {
            a: Mpoint { x: -r + 3 * r / 8, y: 0 },
            b: Mpoint { x: -r + r / 8, y: r / 4 },
        },
        Mline {
            a: Mpoint { x: -r + 3 * r / 8, y: 0 },
            b: Mpoint { x: -r + r / 8, y: -r / 4 },
        },
    ]
}

fn cheat_player_arrow() -> [Mline; 16] {
    let r: Fixed = (8 * PLAYERRADIUS) / 7;
    [
        Mline {
            a: Mpoint { x: -r + r / 8, y: 0 },
            b: Mpoint { x: r, y: 0 },
        },
        Mline {
            a: Mpoint { x: r, y: 0 },
            b: Mpoint { x: r - r / 2, y: r / 6 },
        },
        Mline {
            a: Mpoint { x: r, y: 0 },
            b: Mpoint { x: r - r / 2, y: -r / 6 },
        },
        Mline {
            a: Mpoint { x: -r + r / 8, y: 0 },
            b: Mpoint { x: -r - r / 8, y: r / 6 },
        },
        Mline {
            a: Mpoint { x: -r + r / 8, y: 0 },
            b: Mpoint { x: -r - r / 8, y: -r / 6 },
        },
        Mline {
            a: Mpoint { x: -r + 3 * r / 8, y: 0 },
            b: Mpoint { x: -r + r / 8, y: r / 6 },
        },
        Mline {
            a: Mpoint { x: -r + 3 * r / 8, y: 0 },
            b: Mpoint { x: -r + r / 8, y: -r / 6 },
        },
        Mline {
            a: Mpoint { x: -r / 2, y: 0 },
            b: Mpoint { x: -r / 2, y: -r / 6 },
        },
        Mline {
            a: Mpoint { x: -r / 2, y: -r / 6 },
            b: Mpoint { x: -r / 2 + r / 6, y: -r / 6 },
        },
        Mline {
            a: Mpoint { x: -r / 2 + r / 6, y: -r / 6 },
            b: Mpoint { x: -r / 2 + r / 6, y: r / 4 },
        },
        Mline {
            a: Mpoint { x: -r / 6, y: 0 },
            b: Mpoint { x: -r / 6, y: -r / 6 },
        },
        Mline {
            a: Mpoint { x: -r / 6, y: -r / 6 },
            b: Mpoint { x: 0, y: -r / 6 },
        },
        Mline {
            a: Mpoint { x: 0, y: -r / 6 },
            b: Mpoint { x: 0, y: r / 4 },
        },
        Mline {
            a: Mpoint { x: r / 6, y: r / 4 },
            b: Mpoint { x: r / 6, y: -r / 7 },
        },
        Mline {
            a: Mpoint { x: r / 6, y: -r / 7 },
            b: Mpoint { x: r / 6 + r / 32, y: -r / 7 - r / 32 },
        },
        Mline {
            a: Mpoint { x: r / 6 + r / 32, y: -r / 7 - r / 32 },
            b: Mpoint { x: r / 6 + r / 10, y: -r / 7 },
        },
    ]
}

fn thintriangle_guy() -> [Mline; 3] {
    let r = FRACUNIT;
    [
        Mline {
            a: Mpoint {
                x: ((-0.5 * r as f64) as i32),
                y: ((-0.7 * r as f64) as i32),
            },
            b: Mpoint {
                x: r,
                y: 0,
            },
        },
        Mline {
            a: Mpoint { x: r, y: 0 },
            b: Mpoint {
                x: ((-0.5 * r as f64) as i32),
                y: ((0.7 * r as f64) as i32),
            },
        },
        Mline {
            a: Mpoint {
                x: ((-0.5 * r as f64) as i32),
                y: ((0.7 * r as f64) as i32),
            },
            b: Mpoint {
                x: ((-0.5 * r as f64) as i32),
                y: ((-0.7 * r as f64) as i32),
            },
        },
    ]
}

// Cheat sequence for iddt
pub fn cheat_amap() -> CheatSeq {
    CheatSeq::new("iddt", 0)
}

// =============================================================================
// Helpers
// =============================================================================

fn f_to_m(s: &AmMapState, x: i32) -> Fixed {
    fixed_mul((x as Fixed) << 16, s.scale_ftom)
}

fn m_to_f(s: &AmMapState, x: Fixed) -> i32 {
    (fixed_mul(x, s.scale_mtof) >> 16) as i32
}

fn cxm_to_f(s: &AmMapState, x: Fixed) -> i32 {
    s.f_x + m_to_f(s, x - s.m_x)
}

fn cym_to_f(s: &AmMapState, y: Fixed) -> i32 {
    s.f_y + (s.f_h - m_to_f(s, y - s.m_y))
}

// =============================================================================
// Implementation
// =============================================================================

fn am_activate_new_scale(s: &mut AmMapState) {
    s.m_x += s.m_w / 2;
    s.m_y += s.m_h / 2;
    s.m_w = f_to_m(s, s.f_w);
    s.m_h = f_to_m(s, s.f_h);
    s.m_x -= s.m_w / 2;
    s.m_y -= s.m_h / 2;
    s.m_x2 = s.m_x + s.m_w;
    s.m_y2 = s.m_y + s.m_h;
}

fn am_save_scale_and_loc(s: &mut AmMapState) {
    s.old_m_x = s.m_x;
    s.old_m_y = s.m_y;
    s.old_m_w = s.m_w;
    s.old_m_h = s.m_h;
}

fn am_restore_scale_and_loc(s: &mut AmMapState) {
    s.m_w = s.old_m_w;
    s.m_h = s.old_m_h;
    if s.followplayer == 0 {
        s.m_x = s.old_m_x;
        s.m_y = s.old_m_y;
    } else {
        let plr = unsafe { &*crate::doomstat::PLAYERS.as_ptr().add(plr_index()) };
        let mo = (*plr).mo as *const Mobj;
        if !mo.is_null() {
            s.m_x = unsafe { (*mo).x } - s.m_w / 2;
            s.m_y = unsafe { (*mo).y } - s.m_h / 2;
        }
    }
    s.m_x2 = s.m_x + s.m_w;
    s.m_y2 = s.m_y + s.m_h;
    s.scale_mtof = fixed_div((s.f_w as Fixed) << FRACBITS, s.m_w);
    s.scale_ftom = fixed_div(FRACUNIT, s.scale_mtof);
}

fn plr_index() -> usize {
    unsafe {
        let cp = CONSOLEPLAYER as usize;
        if cp < crate::doomdef::MAXPLAYERS && PLAYERINGAME[cp] {
            return cp;
        }
        for i in 0..crate::doomdef::MAXPLAYERS {
            if PLAYERINGAME[i] {
                return i;
            }
        }
        0
    }
}

fn am_add_mark(s: &mut AmMapState) {
    s.markpoints[s.markpointnum].x = s.m_x + s.m_w / 2;
    s.markpoints[s.markpointnum].y = s.m_y + s.m_h / 2;
    s.markpointnum = (s.markpointnum + 1) % AM_NUMMARKPOINTS;
}

fn am_find_min_max_boundaries(s: &mut AmMapState) {
    let (vertexes, numvertexes) = state::with_state(|st| (st.vertexes, st.numvertexes));
    if vertexes.is_null() || numvertexes <= 0 {
        return;
    }
    unsafe {
        s.min_x = i32::MAX;
        s.min_y = i32::MAX;
        s.max_x = i32::MIN;
        s.max_y = i32::MIN;

        for i in 0..numvertexes as usize {
            let v = &*vertexes.add(i);
            if v.x < s.min_x {
                s.min_x = v.x;
            }
            if v.x > s.max_x {
                s.max_x = v.x;
            }
            if v.y < s.min_y {
                s.min_y = v.y;
            }
            if v.y > s.max_y {
                s.max_y = v.y;
            }
        }

        s.max_w = s.max_x - s.min_x;
        s.max_h = s.max_y - s.min_y;
        s.min_w = 2 * PLAYERRADIUS;
        s.min_h = 2 * PLAYERRADIUS;

        let a = fixed_div((s.f_w as Fixed) << FRACBITS, s.max_w);
        let b = fixed_div((s.f_h as Fixed) << FRACBITS, s.max_h);
        s.min_scale_mtof = if a < b { a } else { b };
        s.max_scale_mtof = fixed_div((s.f_h as Fixed) << FRACBITS, 2 * PLAYERRADIUS);
    }
}

fn am_change_window_loc(s: &mut AmMapState) {
    if s.m_paninc.x != 0 || s.m_paninc.y != 0 {
        s.followplayer = 0;
        s.f_oldloc.x = i32::MAX;
    }

    s.m_x += s.m_paninc.x;
    s.m_y += s.m_paninc.y;

    if s.m_x + s.m_w / 2 > s.max_x {
        s.m_x = s.max_x - s.m_w / 2;
    } else if s.m_x + s.m_w / 2 < s.min_x {
        s.m_x = s.min_x - s.m_w / 2;
    }
    if s.m_y + s.m_h / 2 > s.max_y {
        s.m_y = s.max_y - s.m_h / 2;
    } else if s.m_y + s.m_h / 2 < s.min_y {
        s.m_y = s.min_y - s.m_h / 2;
    }

    s.m_x2 = s.m_x + s.m_w;
    s.m_y2 = s.m_y + s.m_h;
}

fn am_init_variables(s: &mut AmMapState) {
    use crate::game::dstrings::{AMSTR_FOLLOWOFF, AMSTR_FOLLOWON};
    use crate::ui_hud::st_stuff::st_responder;

    unsafe {
        AUTOMAPACTIVE = true;
        s.f_oldloc.x = i32::MAX;
        s.amclock = 0;
        s.lightlev = 0;

        s.m_paninc.x = 0;
        s.m_paninc.y = 0;
        s.ftom_zoommul = FRACUNIT;
        s.mtof_zoommul = FRACUNIT;

        s.m_w = f_to_m(s, s.f_w);
        s.m_h = f_to_m(s, s.f_h);

        let plr_idx = plr_index();
        let plr = &mut *PLAYERS.as_mut_ptr().add(plr_idx);

        s.m_x = (*plr).mo.cast::<Mobj>().read().x - s.m_w / 2;
        s.m_y = (*plr).mo.cast::<Mobj>().read().y - s.m_h / 2;
        am_change_window_loc(s);

        s.old_m_x = s.m_x;
        s.old_m_y = s.m_y;
        s.old_m_w = s.m_w;
        s.old_m_h = s.m_h;

        // Notify status bar (AM_MSGENTERED) - C: ev_keyup, AM_MSGENTERED in data1
        const AM_MSGHEADER: i32 = (('a' as i32) << 24) | (('m' as i32) << 16);
        const AM_MSGENTERED: i32 = AM_MSGHEADER | (('e' as i32) << 8);
        let ev = Event {
            ev_type: EvType::KeyUp,
            data1: AM_MSGENTERED,
            data2: 0,
            data3: 0,
            data4: 0,
        };
        st_responder(&ev);
    }
}

fn am_load_pics(s: &mut AmMapState) {
    for i in 0..10 {
        let name = format!("AMMNUM{}", i);
        s.marknums[i] = w_cache_lump_name(deh_string(&name), PU_STATIC).as_ptr() as *const patch_t;
    }
}

fn am_unload_pics() {
    for i in 0..10 {
        let name = format!("AMMNUM{}", i);
        w_release_lump_name(deh_string(&name));
    }
}

fn am_clear_marks(s: &mut AmMapState) {
    for i in 0..AM_NUMMARKPOINTS {
        s.markpoints[i].x = -1;
    }
    s.markpointnum = 0;
}

fn am_level_init(s: &mut AmMapState) {
    s.level_just_started = 0;
    s.f_x = 0;
    s.f_y = 0;
    s.f_w = s.finit_width;
    s.f_h = s.finit_height;
    am_clear_marks(s);
    am_find_min_max_boundaries(s);
    s.scale_mtof = fixed_div(s.min_scale_mtof, (0.7 * FRACUNIT as f32) as i32);
    if s.scale_mtof > s.max_scale_mtof {
        s.scale_mtof = s.min_scale_mtof;
    }
    s.scale_ftom = fixed_div(FRACUNIT, s.scale_mtof);
}

fn am_stop_inner(s: &mut AmMapState) {
    am_unload_pics();
    s.stopped = true;
}

/// Force automap to quit (e.g. level completed while up).
pub fn am_stop() {
    use crate::ui_hud::st_stuff::st_responder;

    with_am_map_state_mut(am_stop_inner);
    unsafe {
        AUTOMAPACTIVE = false;
        const AM_MSGHEADER: i32 = (('a' as i32) << 24) | (('m' as i32) << 16);
        const AM_MSGEXITED: i32 = AM_MSGHEADER | (('x' as i32) << 8);
        let ev = Event {
            ev_type: EvType::KeyUp,
            data1: AM_MSGEXITED,
            data2: 0,
            data3: 0,
            data4: 0,
        };
        st_responder(&ev);
    }
}

fn am_start_inner(s: &mut AmMapState) {
    use crate::ui_hud::st_stuff::st_responder;

    if !s.stopped {
        am_stop_inner(s);
        unsafe {
            AUTOMAPACTIVE = false;
            const AM_MSGHEADER: i32 = (('a' as i32) << 24) | (('m' as i32) << 16);
            const AM_MSGEXITED: i32 = AM_MSGHEADER | (('x' as i32) << 8);
            let ev = Event {
                ev_type: EvType::KeyUp,
                data1: AM_MSGEXITED,
                data2: 0,
                data3: 0,
                data4: 0,
            };
            st_responder(&ev);
        }
    }
    s.stopped = false;
    if s.lastlevel != GAMEMAP || s.lastepisode != GAMEEPISODE {
        am_level_init(s);
        s.lastlevel = GAMEMAP;
        s.lastepisode = GAMEEPISODE;
    }
    am_init_variables(s);
    am_load_pics(s);
}

/// Start automap.
pub fn am_start() {
    with_am_map_state_mut(am_start_inner);
}

fn am_min_out_window_scale(s: &mut AmMapState) {
    s.scale_mtof = s.min_scale_mtof;
    s.scale_ftom = fixed_div(FRACUNIT, s.scale_mtof);
    am_activate_new_scale(s);
}

fn am_max_out_window_scale(s: &mut AmMapState) {
    s.scale_mtof = s.max_scale_mtof;
    s.scale_ftom = fixed_div(FRACUNIT, s.scale_mtof);
    am_activate_new_scale(s);
}

/// Handle automap input. Returns true if event consumed.
pub fn am_responder(ev: &Event) -> bool {
    use crate::game::dstrings::{
        AMSTR_FOLLOWOFF, AMSTR_FOLLOWON, AMSTR_GRIDOFF, AMSTR_GRIDON, AMSTR_MARKEDSPOT,
        AMSTR_MARKSCLEARED,
    };

    if ev.ev_type != EvType::KeyDown && ev.ev_type != EvType::KeyUp {
        return false;
    }

    let key = ev.data1;

    with_am_map_state_mut(|s| {
        let mut rc = false;

        unsafe {
            if !AUTOMAPACTIVE {
                if ev.ev_type == EvType::KeyDown && key == key_map_toggle() {
                    am_start_inner(s);
                    VIEWACTIVE = false;
                    return true;
                }
                return false;
            }

        if ev.ev_type == EvType::KeyDown {
            rc = true;
            let plr = &mut *PLAYERS.as_mut_ptr().add(plr_index());

            if key == key_map_east() {
                if s.followplayer == 0 {
                    s.m_paninc.x = f_to_m(s, F_PANINC);
                } else {
                    rc = false;
                }
            } else if key == key_map_west() {
                if s.followplayer == 0 {
                    s.m_paninc.x = -f_to_m(s, F_PANINC);
                } else {
                    rc = false;
                }
            } else if key == key_map_north() {
                if s.followplayer == 0 {
                    s.m_paninc.y = f_to_m(s, F_PANINC);
                } else {
                    rc = false;
                }
            } else if key == key_map_south() {
                if s.followplayer == 0 {
                    s.m_paninc.y = -f_to_m(s, F_PANINC);
                } else {
                    rc = false;
                }
            } else if key == key_map_zoomout() {
                s.mtof_zoommul = M_ZOOMOUT;
                s.ftom_zoommul = M_ZOOMIN;
            } else if key == key_map_zoomin() {
                s.mtof_zoommul = M_ZOOMIN;
                s.ftom_zoommul = M_ZOOMOUT;
            } else if key == key_map_toggle() {
                VIEWACTIVE = true;
                am_stop_inner(s);
                AUTOMAPACTIVE = false;
                const AM_MSGHEADER: i32 = (('a' as i32) << 24) | (('m' as i32) << 16);
                const AM_MSGEXITED: i32 = AM_MSGHEADER | (('x' as i32) << 8);
                let ev = Event {
                    ev_type: EvType::KeyUp,
                    data1: AM_MSGEXITED,
                    data2: 0,
                    data3: 0,
                    data4: 0,
                };
                crate::ui_hud::st_stuff::st_responder(&ev);
            } else if key == key_map_maxzoom() {
                s.bigstate = 1 - s.bigstate;
                if s.bigstate != 0 {
                    am_save_scale_and_loc(s);
                    am_min_out_window_scale(s);
                } else {
                    am_restore_scale_and_loc(s);
                }
            } else if key == key_map_follow() {
                s.followplayer = 1 - s.followplayer;
                s.f_oldloc.x = i32::MAX;
                (*plr).message = Some(
                    if s.followplayer != 0 {
                        deh_string(AMSTR_FOLLOWON)
                    } else {
                        deh_string(AMSTR_FOLLOWOFF)
                    }
                    .to_string(),
                );
            } else if key == key_map_grid() {
                s.grid = 1 - s.grid;
                (*plr).message = Some(
                    if s.grid != 0 {
                        deh_string(AMSTR_GRIDON)
                    } else {
                        deh_string(AMSTR_GRIDOFF)
                    }
                    .to_string(),
                );
            } else if key == key_map_mark() {
                let mut buf = [0u8; 20];
                let txt = format!(
                    "{} {}",
                    deh_string(AMSTR_MARKEDSPOT),
                    s.markpointnum
                );
                let bytes = txt.as_bytes();
                let len = bytes.len().min(19);
                buf[..len].copy_from_slice(&bytes[..len]);
                (*plr).message = Some(String::from_utf8_lossy(&buf[..len]).to_string());
                am_add_mark(s);
            } else if key == key_map_clearmark() {
                am_clear_marks(s);
                (*plr).message = Some(deh_string(AMSTR_MARKSCLEARED).to_string());
            } else {
                rc = false;
            }

            // iddt cheat
            if !crate::doomstat::NETGAME {
                let mut seq = cheat_amap();
                if cht_check_cheat(&mut seq, ev.data2 as u8) {
                    rc = false;
                    s.cheating = (s.cheating + 1) % 3;
                }
            }
        } else {
            // KeyUp
            rc = false;
            if key == key_map_east() || key == key_map_west() {
                if s.followplayer == 0 {
                    s.m_paninc.x = 0;
                }
            } else if key == key_map_north() || key == key_map_south() {
                if s.followplayer == 0 {
                    s.m_paninc.y = 0;
                }
            } else if key == key_map_zoomout() || key == key_map_zoomin() {
                s.mtof_zoommul = FRACUNIT;
                s.ftom_zoommul = FRACUNIT;
            }
        }
        }

        rc
    })
}

fn am_change_window_scale(s: &mut AmMapState) {
    s.scale_mtof = fixed_mul(s.scale_mtof, s.mtof_zoommul);
    s.scale_ftom = fixed_div(FRACUNIT, s.scale_mtof);

    if s.scale_mtof < s.min_scale_mtof {
        am_min_out_window_scale(s);
    } else if s.scale_mtof > s.max_scale_mtof {
        am_max_out_window_scale(s);
    } else {
        am_activate_new_scale(s);
    }
}

fn am_do_follow_player(s: &mut AmMapState) {
    let plr_idx = plr_index();
    let plr = unsafe { &*PLAYERS.as_ptr().add(plr_idx) };
    let mo = (*plr).mo as *const Mobj;
    if mo.is_null() {
        return;
    }
    let mx = unsafe { (*mo).x };
    let my = unsafe { (*mo).y };
    if s.f_oldloc.x != mx || s.f_oldloc.y != my {
        s.m_x = f_to_m(s, m_to_f(s, mx)) - s.m_w / 2;
        s.m_y = f_to_m(s, m_to_f(s, my)) - s.m_h / 2;
        s.m_x2 = s.m_x + s.m_w;
        s.m_y2 = s.m_y + s.m_h;
        s.f_oldloc.x = mx;
        s.f_oldloc.y = my;
    }
}

/// Automap tick.
pub fn am_ticker() {
    with_am_map_state_mut(|s| {
        if !unsafe { AUTOMAPACTIVE } {
            return;
        }
        s.amclock += 1;

        if s.followplayer != 0 {
            am_do_follow_player(s);
        }
        if s.ftom_zoommul != FRACUNIT {
            am_change_window_scale(s);
        }
        if s.m_paninc.x != 0 || s.m_paninc.y != 0 {
            am_change_window_loc(s);
        }
    });
}

fn am_clear_fb(s: &AmMapState, color: i32) {
    v_video::with_v_video_state(|vv| {
        unsafe {
            if vv.viewimage.is_null() {
                return;
            }
            let base = vv.viewimage.add(s.f_y as usize * SCREENWIDTH as usize + s.f_x as usize);
            for i in 0..(s.f_w * s.f_h) as usize {
                *base.add(i) = color as u8;
            }
        }
    });
}

fn am_clip_mline(s: &AmMapState, ml: &Mline, fl: &mut Fline) -> bool {
    const LEFT: i32 = 1;
    const RIGHT: i32 = 2;
    const BOTTOM: i32 = 4;
    const TOP: i32 = 8;

    let mut outcode1 = 0i32;
    let mut outcode2 = 0i32;

    if ml.a.y > s.m_y2 {
        outcode1 = TOP;
    } else if ml.a.y < s.m_y {
        outcode1 = BOTTOM;
    }
    if ml.b.y > s.m_y2 {
        outcode2 = TOP;
    } else if ml.b.y < s.m_y {
        outcode2 = BOTTOM;
    }
    if (outcode1 & outcode2) != 0 {
        return false;
    }
    if ml.a.x < s.m_x {
        outcode1 |= LEFT;
    } else if ml.a.x > s.m_x2 {
        outcode1 |= RIGHT;
    }
    if ml.b.x < s.m_x {
        outcode2 |= LEFT;
    } else if ml.b.x > s.m_x2 {
        outcode2 |= RIGHT;
    }
    if (outcode1 & outcode2) != 0 {
        return false;
    }

    fl.a.x = cxm_to_f(s, ml.a.x);
    fl.a.y = cym_to_f(s, ml.a.y);
    fl.b.x = cxm_to_f(s, ml.b.x);
    fl.b.y = cym_to_f(s, ml.b.y);

    let mut do_outcode = |oc: &mut i32, mx: i32, my: i32| {
        *oc = 0;
        if my < 0 {
            *oc |= TOP;
        } else if my >= s.f_h {
            *oc |= BOTTOM;
        }
        if mx < 0 {
            *oc |= LEFT;
        } else if mx >= s.f_w {
            *oc |= RIGHT;
        }
    };

    do_outcode(&mut outcode1, fl.a.x, fl.a.y);
    do_outcode(&mut outcode2, fl.b.x, fl.b.y);
    if (outcode1 & outcode2) != 0 {
        return false;
    }

    while (outcode1 | outcode2) != 0 {
        let outside = if outcode1 != 0 { outcode1 } else { outcode2 };
        let mut tmp = Fpoint { x: 0, y: 0 };

        if (outside & TOP) != 0 {
            let dy = fl.a.y - fl.b.y;
            let dx = fl.b.x - fl.a.x;
            tmp.x = fl.a.x + (dx * fl.a.y) / dy.max(1);
            tmp.y = 0;
        } else if (outside & BOTTOM) != 0 {
            let dy = fl.a.y - fl.b.y;
            let dx = fl.b.x - fl.a.x;
            tmp.x = fl.a.x + (dx * (fl.a.y - s.f_h)) / dy.max(1);
            tmp.y = s.f_h - 1;
        } else if (outside & RIGHT) != 0 {
            let dy = fl.b.y - fl.a.y;
            let dx = fl.b.x - fl.a.x;
            tmp.y = fl.a.y + (dy * (s.f_w - 1 - fl.a.x)) / dx.max(1);
            tmp.x = s.f_w - 1;
        } else if (outside & LEFT) != 0 {
            let dy = fl.b.y - fl.a.y;
            let dx = fl.b.x - fl.a.x;
            tmp.y = fl.a.y + (dy * (-fl.a.x)) / dx.max(1);
            tmp.x = 0;
        }

        if outside == outcode1 {
            fl.a = tmp;
            do_outcode(&mut outcode1, fl.a.x, fl.a.y);
        } else {
            fl.b = tmp;
            do_outcode(&mut outcode2, fl.b.x, fl.b.y);
        }
        if (outcode1 & outcode2) != 0 {
            return false;
        }
    }
    true
}

fn am_draw_fline(s: &AmMapState, fl: &Fline, color: i32) {
    v_video::with_v_video_state(|vv| {
        unsafe {
            if vv.viewimage.is_null() {
                return;
            }
            let base = vv.viewimage.add(s.f_y as usize * SCREENWIDTH as usize + s.f_x as usize);
            let stride = SCREENWIDTH as usize;

            let mut x = fl.a.x;
        let mut y = fl.a.y;
        let dx = fl.b.x - fl.a.x;
        let dy = fl.b.y - fl.a.y;
        let ax = 2 * dx.abs();
        let ay = 2 * dy.abs();
        let sx = if dx < 0 { -1 } else { 1 };
        let sy = if dy < 0 { -1 } else { 1 };

        if ax > ay {
            let mut d = ay - ax / 2;
            loop {
                if x >= 0 && x < s.f_w && y >= 0 && y < s.f_h {
                    *base.add(y as usize * stride + x as usize) = color as u8;
                }
                if x == fl.b.x {
                    return;
                }
                if d >= 0 {
                    y += sy;
                    d -= ax;
                }
                x += sx;
                d += ay;
            }
        } else {
            let mut d = ax - ay / 2;
            loop {
                if x >= 0 && x < s.f_w && y >= 0 && y < s.f_h {
                    *base.add(y as usize * stride + x as usize) = color as u8;
                }
                if y == fl.b.y {
                    return;
                }
                if d >= 0 {
                    x += sx;
                    d -= ay;
                }
                y += sy;
                d += ax;
            }
        }
        }
    });
}

fn am_draw_mline(s: &AmMapState, ml: &Mline, color: i32) {
    let mut fl = Fline {
        a: Fpoint { x: 0, y: 0 },
        b: Fpoint { x: 0, y: 0 },
    };
    if am_clip_mline(s, ml, &mut fl) {
        am_draw_fline(s, &fl, color);
    }
}

fn am_draw_grid(s: &AmMapState, color: i32) {
    let (bmaporgx, bmaporgy) = state::with_state(|st| (st.bmaporgx, st.bmaporgy));
    let block_units = (MAPBLOCKUNITS as Fixed) << FRACBITS;

    let mut start = s.m_x;
    let rem = (start - bmaporgx) % block_units;
    let start = if rem != 0 {
        start + block_units - rem
    } else {
        start
    };
    let end_x = s.m_x + s.m_w;

    let mut ml = Mline {
        a: Mpoint { x: 0, y: s.m_y },
        b: Mpoint { x: 0, y: s.m_y + s.m_h },
    };
    let mut x = start;
    while x < end_x {
        ml.a.x = x;
        ml.b.x = x;
        am_draw_mline(s, &ml, color);
        x += block_units;
    }

    let mut start = s.m_y;
    let rem = (start - bmaporgy) % block_units;
    let start = if rem != 0 {
        start + block_units - rem
    } else {
        start
    };
    let end_y = s.m_y + s.m_h;

    ml = Mline {
        a: Mpoint { x: s.m_x, y: 0 },
        b: Mpoint { x: s.m_x + s.m_w, y: 0 },
    };
    let mut y = start;
    while y < end_y {
        ml.a.y = y;
        ml.b.y = y;
        am_draw_mline(s, &ml, color);
        y += block_units;
    }
}

fn am_rotate(x: &mut Fixed, y: &mut Fixed, a: u32) {
    let idx = (a >> ANGLETOFINESHIFT) as usize;
    let tmpx = fixed_mul(*x, finecosine(idx)) - fixed_mul(*y, finesine(idx));
    *y = fixed_mul(*x, finesine(idx)) + fixed_mul(*y, finecosine(idx));
    *x = tmpx;
}

fn am_draw_line_character(
    s: &AmMapState,
    lineguy: &[Mline],
    scale: Fixed,
    angle: u32,
    color: i32,
    x: Fixed,
    y: Fixed,
) {
    for line in lineguy {
        let mut la = line.a;
        let mut lb = line.b;
        if scale != 0 {
            la.x = fixed_mul(scale, la.x);
            la.y = fixed_mul(scale, la.y);
            lb.x = fixed_mul(scale, lb.x);
            lb.y = fixed_mul(scale, lb.y);
        }
        if angle != 0 {
            am_rotate(&mut la.x, &mut la.y, angle);
            am_rotate(&mut lb.x, &mut lb.y, angle);
        }
        la.x += x;
        la.y += y;
        lb.x += x;
        lb.y += y;
        let ml = Mline { a: la, b: lb };
        am_draw_mline(s, &ml, color);
    }
}

fn am_draw_walls(s: &AmMapState) {
    use crate::doomdef::NUMPOWERS;

    let (lines, numlines) = state::with_state(|st| (st.lines, st.numlines));
    if lines.is_null() || numlines <= 0 {
        return;
    }
    unsafe {
        let plr_idx = plr_index();
        let plr = &*PLAYERS.as_ptr().add(plr_idx);
        let pw_allmap = 4usize; // Powertype::Allmap
        let has_allmap = plr.powers[pw_allmap] > 0;

        for i in 0..numlines as usize {
            let ld = &*lines.add(i);
            if ld.v1.is_null() || ld.v2.is_null() {
                continue;
            }
            let v1 = &*ld.v1;
            let v2 = &*ld.v2;
            let mut ml = Mline {
                a: Mpoint { x: v1.x, y: v1.y },
                b: Mpoint { x: v2.x, y: v2.y },
            };

            let draw = if s.cheating != 0 || (ld.flags & ML_MAPPED) != 0 {
                if (ld.flags & LINE_NEVERSEE) != 0_i16 && s.cheating == 0 {
                    continue;
                }
                true
            } else if has_allmap {
                (ld.flags & LINE_NEVERSEE) == 0_i16
            } else {
                false
            };

            if !draw {
                continue;
            }

            let color = if ld.backsector.is_null() {
                WALLCOLORS + s.lightlev
            } else {
                let back = &*ld.backsector;
                let front = if ld.frontsector.is_null() {
                    continue;
                } else {
                    &*ld.frontsector
                };
                if ld.special == 39 {
                    WALLCOLORS + WALLRANGE / 2
                } else if (ld.flags & ML_SECRET) != 0 {
                    if s.cheating != 0 {
                        SECRETWALLCOLORS + s.lightlev
                    } else {
                        WALLCOLORS + s.lightlev
                    }
                } else if back.floorheight != front.floorheight {
                    FDWALLCOLORS + s.lightlev
                } else if back.ceilingheight != front.ceilingheight {
                    CDWALLCOLORS + s.lightlev
                } else if s.cheating != 0 {
                    TSWALLCOLORS + s.lightlev
                } else {
                    continue;
                }
            };

            am_draw_mline(s, &ml, color);
        }
    }
}

fn am_draw_players(s: &AmMapState) {
    use crate::doomstat::NETGAME;

    unsafe {
        let plr_idx = plr_index();
        let plr = &*PLAYERS.as_ptr().add(plr_idx);
        let mo = (*plr).mo as *const Mobj;
        if mo.is_null() {
            return;
        }

        if !NETGAME {
            let arrow: &[Mline] = if s.cheating != 0 {
                &cheat_player_arrow()[..]
            } else {
                &player_arrow()[..]
            };
            am_draw_line_character(
                s,
                arrow,
                0,
                (*mo).angle,
                WHITE,
                (*mo).x,
                (*mo).y,
            );
            return;
        }

        let their_colors = [GREENS, GRAYS, BROWNS, REDS];
        let pw_invisibility = 2usize;
        for (i, p) in PLAYERS.iter().enumerate() {
            if !PLAYERINGAME[i] {
                continue;
            }
            if i == plr_idx {
                continue;
            }
            let pmo = (*p).mo as *const Mobj;
            if pmo.is_null() {
                continue;
            }
            let color = if (*p).powers[pw_invisibility] > 0 {
                246
            } else {
                their_colors[i % 4]
            };
            am_draw_line_character(
                s,
                &player_arrow(),
                0,
                (*pmo).angle,
                color,
                (*pmo).x,
                (*pmo).y,
            );
        }
    }
}

fn am_draw_things(s: &AmMapState, colors: i32, _colorrange: i32) {
    let (sectors, numsectors) = state::with_state(|st| (st.sectors, st.numsectors));
    if sectors.is_null() || numsectors <= 0 {
        return;
    }
    unsafe {
        let tri = thintriangle_guy();
        for i in 0..numsectors as usize {
            let sec = &*sectors.add(i);
            let mut t = sec.thinglist as *const Mobj;
            while !t.is_null() {
                let mobj = &*t;
                am_draw_line_character(
                    s,
                    &tri,
                    16 << FRACBITS,
                    mobj.angle,
                    colors + s.lightlev,
                    mobj.x,
                    mobj.y,
                );
                t = mobj.snext as *const Mobj;
            }
        }
    }
}

fn am_draw_marks(s: &AmMapState) {
    for i in 0..AM_NUMMARKPOINTS {
        if s.markpoints[i].x == -1 {
            continue;
        }
        let fx = cxm_to_f(s, s.markpoints[i].x);
        let fy = cym_to_f(s, s.markpoints[i].y);
        let w = 5;
        let h = 6;
        if fx >= s.f_x && fx <= s.f_w - w && fy >= s.f_y && fy <= s.f_h - h {
            let patch = s.marknums[i];
            if !patch.is_null() {
                v_draw_patch(fx, fy, patch);
            }
        }
    }
}

fn am_draw_crosshair(s: &AmMapState, color: i32) {
    v_video::with_v_video_state(|vv| {
        unsafe {
            if vv.viewimage.is_null() {
                return;
            }
            let base = vv.viewimage.add(s.f_y as usize * SCREENWIDTH as usize + s.f_x as usize);
            let idx = (s.f_w * (s.f_h + 1)) / 2;
            *base.add(idx as usize) = color as u8;
        }
    });
}

/// Draw automap. Called instead of view when automap active.
pub fn am_drawer() {
    with_am_map_state(|s| {
        if !unsafe { AUTOMAPACTIVE } {
            return;
        }
        am_clear_fb(s, BACKGROUND);
        if s.grid != 0 {
            am_draw_grid(s, GRIDCOLORS);
        }
        am_draw_walls(s);
        am_draw_players(s);
        if s.cheating == 2 {
            am_draw_things(s, THINGCOLORS, THINGRANGE);
        }
        am_draw_crosshair(s, XHAIRCOLORS);
        am_draw_marks(s);
        v_mark_rect(s.f_x, s.f_y, s.f_w, s.f_h);
    });
}
