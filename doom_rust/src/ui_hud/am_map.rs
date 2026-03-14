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
use crate::rendering::state::{
    BMAPORGX, BMAPORGY, LINES, NUMLINES, NUMSECTORS, NUMVERTEXES, SECTORS, VERTEXES,
};
use crate::rendering::{patch_t, v_draw_patch, v_mark_rect, VIEWIMAGE};
use crate::ui_hud::cheat::{cht_check_cheat, CheatSeq};
use crate::ui_hud::controls::{
    KEY_MAP_CLEARMARK, KEY_MAP_EAST, KEY_MAP_FOLLOW, KEY_MAP_GRID, KEY_MAP_MARK,
    KEY_MAP_MAXZOOM, KEY_MAP_NORTH, KEY_MAP_SOUTH, KEY_MAP_TOGGLE, KEY_MAP_WEST,
    KEY_MAP_ZOOMIN, KEY_MAP_ZOOMOUT,
};
use crate::wad::{w_cache_lump_name, w_release_lump_name};
use crate::z_zone::PU_STATIC;
use std::ptr;

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

static mut CHEATING: i32 = 0;
static mut GRID: i32 = 0;
static mut LEVELJUSTSTARTED: i32 = 1;
static mut FINIT_WIDTH: i32 = SCREENWIDTH;
static mut FINIT_HEIGHT: i32 = SCREENHEIGHT - 32;

static mut F_X: i32 = 0;
static mut F_Y: i32 = 0;
static mut F_W: i32 = SCREENWIDTH;
static mut F_H: i32 = SCREENHEIGHT - 32;

static mut LIGHTLEV: i32 = 0;
static mut AMCLOCK: i32 = 0;

static mut M_PANINC: Mpoint = Mpoint { x: 0, y: 0 };
static mut MTOF_ZOOMMUL: Fixed = FRACUNIT;
static mut FTOM_ZOOMMUL: Fixed = FRACUNIT;

static mut M_X: Fixed = 0;
static mut M_Y: Fixed = 0;
static mut M_X2: Fixed = 0;
static mut M_Y2: Fixed = 0;
static mut M_W: Fixed = 0;
static mut M_H: Fixed = 0;

static mut MIN_X: Fixed = 0;
static mut MIN_Y: Fixed = 0;
static mut MAX_X: Fixed = 0;
static mut MAX_Y: Fixed = 0;
static mut MAX_W: Fixed = 0;
static mut MAX_H: Fixed = 0;
static mut MIN_W: Fixed = 0;
static mut MIN_H: Fixed = 0;

static mut MIN_SCALE_MTOF: Fixed = 0;
static mut MAX_SCALE_MTOF: Fixed = 0;

static mut OLD_M_W: Fixed = 0;
static mut OLD_M_H: Fixed = 0;
static mut OLD_M_X: Fixed = 0;
static mut OLD_M_Y: Fixed = 0;
static mut F_OLDLOC: Mpoint = Mpoint { x: 0, y: 0 };

static mut SCALE_MTOF: Fixed = INITSCALEMTOF;
static mut SCALE_FTOM: Fixed = 0;

static mut MARKNUMS: [*const patch_t; 10] = [ptr::null(); 10];
static mut MARKPOINTS: [Mpoint; AM_NUMMARKPOINTS] = [Mpoint { x: -1, y: -1 }; AM_NUMMARKPOINTS];
static mut MARKPOINTNUM: usize = 0;

static mut FOLLOWPLAYER: i32 = 1;
static mut STOPPED: bool = true;
static mut LASTLEVEL: i32 = -1;
static mut LASTEPISODE: i32 = -1;

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

fn f_to_m(x: i32) -> Fixed {
    fixed_mul((x as Fixed) << 16, unsafe { SCALE_FTOM })
}

fn m_to_f(x: Fixed) -> i32 {
    (fixed_mul(x, unsafe { SCALE_MTOF }) >> 16) as i32
}

fn cxm_to_f(x: Fixed) -> i32 {
    unsafe { F_X + m_to_f(x - M_X) }
}

fn cym_to_f(y: Fixed) -> i32 {
    unsafe { F_Y + (F_H - m_to_f(y - M_Y)) }
}

// =============================================================================
// Implementation
// =============================================================================

fn am_activate_new_scale() {
    unsafe {
        M_X += M_W / 2;
        M_Y += M_H / 2;
        M_W = f_to_m(F_W);
        M_H = f_to_m(F_H);
        M_X -= M_W / 2;
        M_Y -= M_H / 2;
        M_X2 = M_X + M_W;
        M_Y2 = M_Y + M_H;
    }
}

fn am_save_scale_and_loc() {
    unsafe {
        OLD_M_X = M_X;
        OLD_M_Y = M_Y;
        OLD_M_W = M_W;
        OLD_M_H = M_H;
    }
}

fn am_restore_scale_and_loc() {
    unsafe {
        M_W = OLD_M_W;
        M_H = OLD_M_H;
        if FOLLOWPLAYER == 0 {
            M_X = OLD_M_X;
            M_Y = OLD_M_Y;
        } else {
            let plr = &*crate::doomstat::PLAYERS.as_ptr().add(plr_index());
            let mo = (*plr).mo as *const Mobj;
            if !mo.is_null() {
                M_X = (*mo).x - M_W / 2;
                M_Y = (*mo).y - M_H / 2;
            }
        }
        M_X2 = M_X + M_W;
        M_Y2 = M_Y + M_H;
        SCALE_MTOF = fixed_div((F_W as Fixed) << FRACBITS, M_W);
        SCALE_FTOM = fixed_div(FRACUNIT, SCALE_MTOF);
    }
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

fn am_add_mark() {
    unsafe {
        MARKPOINTS[MARKPOINTNUM].x = M_X + M_W / 2;
        MARKPOINTS[MARKPOINTNUM].y = M_Y + M_H / 2;
        MARKPOINTNUM = (MARKPOINTNUM + 1) % AM_NUMMARKPOINTS;
    }
}

fn am_find_min_max_boundaries() {
    unsafe {
        if VERTEXES.is_null() || NUMVERTEXES <= 0 {
            return;
        }
        MIN_X = i32::MAX;
        MIN_Y = i32::MAX;
        MAX_X = i32::MIN;
        MAX_Y = i32::MIN;

        for i in 0..NUMVERTEXES as usize {
            let v = &*VERTEXES.add(i);
            if v.x < MIN_X {
                MIN_X = v.x;
            }
            if v.x > MAX_X {
                MAX_X = v.x;
            }
            if v.y < MIN_Y {
                MIN_Y = v.y;
            }
            if v.y > MAX_Y {
                MAX_Y = v.y;
            }
        }

        MAX_W = MAX_X - MIN_X;
        MAX_H = MAX_Y - MIN_Y;
        MIN_W = 2 * PLAYERRADIUS;
        MIN_H = 2 * PLAYERRADIUS;

        let a = fixed_div((F_W as Fixed) << FRACBITS, MAX_W);
        let b = fixed_div((F_H as Fixed) << FRACBITS, MAX_H);
        MIN_SCALE_MTOF = if a < b { a } else { b };
        MAX_SCALE_MTOF = fixed_div((F_H as Fixed) << FRACBITS, 2 * PLAYERRADIUS);
    }
}

fn am_change_window_loc() {
    unsafe {
        if M_PANINC.x != 0 || M_PANINC.y != 0 {
            FOLLOWPLAYER = 0;
            F_OLDLOC.x = i32::MAX;
        }

        M_X += M_PANINC.x;
        M_Y += M_PANINC.y;

        if M_X + M_W / 2 > MAX_X {
            M_X = MAX_X - M_W / 2;
        } else if M_X + M_W / 2 < MIN_X {
            M_X = MIN_X - M_W / 2;
        }
        if M_Y + M_H / 2 > MAX_Y {
            M_Y = MAX_Y - M_H / 2;
        } else if M_Y + M_H / 2 < MIN_Y {
            M_Y = MIN_Y - M_H / 2;
        }

        M_X2 = M_X + M_W;
        M_Y2 = M_Y + M_H;
    }
}

fn am_init_variables() {
    use crate::game::dstrings::{AMSTR_FOLLOWOFF, AMSTR_FOLLOWON};
    use crate::ui_hud::st_stuff::st_responder;

    unsafe {
        AUTOMAPACTIVE = true;
        F_OLDLOC.x = i32::MAX;
        AMCLOCK = 0;
        LIGHTLEV = 0;

        M_PANINC.x = 0;
        M_PANINC.y = 0;
        FTOM_ZOOMMUL = FRACUNIT;
        MTOF_ZOOMMUL = FRACUNIT;

        M_W = f_to_m(F_W);
        M_H = f_to_m(F_H);

        let plr_idx = plr_index();
        let plr = &mut *PLAYERS.as_mut_ptr().add(plr_idx);

        M_X = (*plr).mo.cast::<Mobj>().read().x - M_W / 2;
        M_Y = (*plr).mo.cast::<Mobj>().read().y - M_H / 2;
        am_change_window_loc();

        OLD_M_X = M_X;
        OLD_M_Y = M_Y;
        OLD_M_W = M_W;
        OLD_M_H = M_H;

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

fn am_load_pics() {
    for i in 0..10 {
        let name = format!("AMMNUM{}", i);
        unsafe {
            MARKNUMS[i] = w_cache_lump_name(deh_string(&name), PU_STATIC).as_ptr() as *const patch_t;
        }
    }
}

fn am_unload_pics() {
    for i in 0..10 {
        let name = format!("AMMNUM{}", i);
        unsafe {
            w_release_lump_name(deh_string(&name));
        }
    }
}

fn am_clear_marks() {
    for i in 0..AM_NUMMARKPOINTS {
        unsafe {
            MARKPOINTS[i].x = -1;
        }
    }
    unsafe {
        MARKPOINTNUM = 0;
    }
}

fn am_level_init() {
    unsafe {
        LEVELJUSTSTARTED = 0;
        F_X = 0;
        F_Y = 0;
        F_W = FINIT_WIDTH;
        F_H = FINIT_HEIGHT;
        am_clear_marks();
        am_find_min_max_boundaries();
        SCALE_MTOF = fixed_div(MIN_SCALE_MTOF, (0.7 * FRACUNIT as f32) as i32);
        if SCALE_MTOF > MAX_SCALE_MTOF {
            SCALE_MTOF = MIN_SCALE_MTOF;
        }
        SCALE_FTOM = fixed_div(FRACUNIT, SCALE_MTOF);
    }
}

/// Force automap to quit (e.g. level completed while up).
pub fn am_stop() {
    use crate::ui_hud::st_stuff::st_responder;

    unsafe {
        am_unload_pics();
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
        STOPPED = true;
    }
}

/// Start automap.
pub fn am_start() {
    unsafe {
        if !STOPPED {
            am_stop();
        }
        STOPPED = false;
        if LASTLEVEL != GAMEMAP || LASTEPISODE != GAMEEPISODE {
            am_level_init();
            LASTLEVEL = GAMEMAP;
            LASTEPISODE = GAMEEPISODE;
        }
        am_init_variables();
        am_load_pics();
    }
}

fn am_min_out_window_scale() {
    unsafe {
        SCALE_MTOF = MIN_SCALE_MTOF;
        SCALE_FTOM = fixed_div(FRACUNIT, SCALE_MTOF);
        am_activate_new_scale();
    }
}

fn am_max_out_window_scale() {
    unsafe {
        SCALE_MTOF = MAX_SCALE_MTOF;
        SCALE_FTOM = fixed_div(FRACUNIT, SCALE_MTOF);
        am_activate_new_scale();
    }
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
    let mut rc = false;

    unsafe {
        if !AUTOMAPACTIVE {
            if ev.ev_type == EvType::KeyDown && key == KEY_MAP_TOGGLE as i32 {
                am_start();
                VIEWACTIVE = false;
                return true;
            }
            return false;
        }

        if ev.ev_type == EvType::KeyDown {
            rc = true;
            let plr = &mut *PLAYERS.as_mut_ptr().add(plr_index());

            if key == KEY_MAP_EAST as i32 {
                if FOLLOWPLAYER == 0 {
                    M_PANINC.x = f_to_m(F_PANINC);
                } else {
                    rc = false;
                }
            } else if key == KEY_MAP_WEST as i32 {
                if FOLLOWPLAYER == 0 {
                    M_PANINC.x = -f_to_m(F_PANINC);
                } else {
                    rc = false;
                }
            } else if key == KEY_MAP_NORTH as i32 {
                if FOLLOWPLAYER == 0 {
                    M_PANINC.y = f_to_m(F_PANINC);
                } else {
                    rc = false;
                }
            } else if key == KEY_MAP_SOUTH as i32 {
                if FOLLOWPLAYER == 0 {
                    M_PANINC.y = -f_to_m(F_PANINC);
                } else {
                    rc = false;
                }
            } else if key == KEY_MAP_ZOOMOUT as i32 {
                MTOF_ZOOMMUL = M_ZOOMOUT;
                FTOM_ZOOMMUL = M_ZOOMIN;
            } else if key == KEY_MAP_ZOOMIN as i32 {
                MTOF_ZOOMMUL = M_ZOOMIN;
                FTOM_ZOOMMUL = M_ZOOMOUT;
            } else if key == KEY_MAP_TOGGLE as i32 {
                VIEWACTIVE = true;
                am_stop();
            } else if key == KEY_MAP_MAXZOOM as i32 {
                static mut BIGSTATE: i32 = 0;
                BIGSTATE = 1 - BIGSTATE;
                if BIGSTATE != 0 {
                    am_save_scale_and_loc();
                    am_min_out_window_scale();
                } else {
                    am_restore_scale_and_loc();
                }
            } else if key == KEY_MAP_FOLLOW as i32 {
                FOLLOWPLAYER = 1 - FOLLOWPLAYER;
                F_OLDLOC.x = i32::MAX;
                (*plr).message = Some(
                    if FOLLOWPLAYER != 0 {
                        deh_string(AMSTR_FOLLOWON)
                    } else {
                        deh_string(AMSTR_FOLLOWOFF)
                    }
                    .to_string(),
                );
            } else if key == KEY_MAP_GRID as i32 {
                GRID = 1 - GRID;
                (*plr).message = Some(
                    if GRID != 0 {
                        deh_string(AMSTR_GRIDON)
                    } else {
                        deh_string(AMSTR_GRIDOFF)
                    }
                    .to_string(),
                );
            } else if key == KEY_MAP_MARK as i32 {
                let mut buf = [0u8; 20];
                let s = format!(
                    "{} {}",
                    deh_string(AMSTR_MARKEDSPOT),
                    MARKPOINTNUM
                );
                let bytes = s.as_bytes();
                let len = bytes.len().min(19);
                buf[..len].copy_from_slice(&bytes[..len]);
                (*plr).message = Some(String::from_utf8_lossy(&buf[..len]).to_string());
                am_add_mark();
            } else if key == KEY_MAP_CLEARMARK as i32 {
                am_clear_marks();
                (*plr).message = Some(deh_string(AMSTR_MARKSCLEARED).to_string());
            } else {
                rc = false;
            }

            // iddt cheat
            if !crate::doomstat::NETGAME {
                let mut seq = cheat_amap();
                if cht_check_cheat(&mut seq, ev.data2 as u8) {
                    rc = false;
                    CHEATING = (CHEATING + 1) % 3;
                }
            }
        } else {
            // KeyUp
            rc = false;
            if key == KEY_MAP_EAST as i32 || key == KEY_MAP_WEST as i32 {
                if FOLLOWPLAYER == 0 {
                    M_PANINC.x = 0;
                }
            } else if key == KEY_MAP_NORTH as i32 || key == KEY_MAP_SOUTH as i32 {
                if FOLLOWPLAYER == 0 {
                    M_PANINC.y = 0;
                }
            } else if key == KEY_MAP_ZOOMOUT as i32 || key == KEY_MAP_ZOOMIN as i32 {
                MTOF_ZOOMMUL = FRACUNIT;
                FTOM_ZOOMMUL = FRACUNIT;
            }
        }
    }

    rc
}

fn am_change_window_scale() {
    unsafe {
        SCALE_MTOF = fixed_mul(SCALE_MTOF, MTOF_ZOOMMUL);
        SCALE_FTOM = fixed_div(FRACUNIT, SCALE_MTOF);

        if SCALE_MTOF < MIN_SCALE_MTOF {
            am_min_out_window_scale();
        } else if SCALE_MTOF > MAX_SCALE_MTOF {
            am_max_out_window_scale();
        } else {
            am_activate_new_scale();
        }
    }
}

fn am_do_follow_player() {
    unsafe {
        let plr_idx = plr_index();
        let plr = &*PLAYERS.as_ptr().add(plr_idx);
        let mo = (*plr).mo as *const Mobj;
        if mo.is_null() {
            return;
        }
        let mx = (*mo).x;
        let my = (*mo).y;
        if F_OLDLOC.x != mx || F_OLDLOC.y != my {
            M_X = f_to_m(m_to_f(mx)) - M_W / 2;
            M_Y = f_to_m(m_to_f(my)) - M_H / 2;
            M_X2 = M_X + M_W;
            M_Y2 = M_Y + M_H;
            F_OLDLOC.x = mx;
            F_OLDLOC.y = my;
        }
    }
}

/// Automap tick.
pub fn am_ticker() {
    unsafe {
        if !AUTOMAPACTIVE {
            return;
        }
        AMCLOCK += 1;

        if FOLLOWPLAYER != 0 {
            am_do_follow_player();
        }
        if FTOM_ZOOMMUL != FRACUNIT {
            am_change_window_scale();
        }
        if M_PANINC.x != 0 || M_PANINC.y != 0 {
            am_change_window_loc();
        }
    }
}

fn am_clear_fb(color: i32) {
    unsafe {
        if VIEWIMAGE.is_null() {
            return;
        }
        let base = VIEWIMAGE.add(F_Y as usize * SCREENWIDTH as usize + F_X as usize);
        for i in 0..(F_W * F_H) as usize {
            *base.add(i) = color as u8;
        }
    }
}

fn am_clip_mline(ml: &Mline, fl: &mut Fline) -> bool {
    const LEFT: i32 = 1;
    const RIGHT: i32 = 2;
    const BOTTOM: i32 = 4;
    const TOP: i32 = 8;

    unsafe {
        let mut outcode1 = 0i32;
        let mut outcode2 = 0i32;

        if ml.a.y > M_Y2 {
            outcode1 = TOP;
        } else if ml.a.y < M_Y {
            outcode1 = BOTTOM;
        }
        if ml.b.y > M_Y2 {
            outcode2 = TOP;
        } else if ml.b.y < M_Y {
            outcode2 = BOTTOM;
        }
        if (outcode1 & outcode2) != 0 {
            return false;
        }
        if ml.a.x < M_X {
            outcode1 |= LEFT;
        } else if ml.a.x > M_X2 {
            outcode1 |= RIGHT;
        }
        if ml.b.x < M_X {
            outcode2 |= LEFT;
        } else if ml.b.x > M_X2 {
            outcode2 |= RIGHT;
        }
        if (outcode1 & outcode2) != 0 {
            return false;
        }

        fl.a.x = cxm_to_f(ml.a.x);
        fl.a.y = cym_to_f(ml.a.y);
        fl.b.x = cxm_to_f(ml.b.x);
        fl.b.y = cym_to_f(ml.b.y);

        let mut do_outcode = |oc: &mut i32, mx: i32, my: i32| {
            *oc = 0;
            if my < 0 {
                *oc |= TOP;
            } else if my >= F_H {
                *oc |= BOTTOM;
            }
            if mx < 0 {
                *oc |= LEFT;
            } else if mx >= F_W {
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
                tmp.x = fl.a.x + (dx * (fl.a.y - F_H)) / dy.max(1);
                tmp.y = F_H - 1;
            } else if (outside & RIGHT) != 0 {
                let dy = fl.b.y - fl.a.y;
                let dx = fl.b.x - fl.a.x;
                tmp.y = fl.a.y + (dy * (F_W - 1 - fl.a.x)) / dx.max(1);
                tmp.x = F_W - 1;
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
    }
    true
}

fn am_draw_fline(fl: &Fline, color: i32) {
    unsafe {
        if VIEWIMAGE.is_null() {
            return;
        }
        let base = VIEWIMAGE.add(F_Y as usize * SCREENWIDTH as usize + F_X as usize);
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
                if x >= 0 && x < F_W && y >= 0 && y < F_H {
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
                if x >= 0 && x < F_W && y >= 0 && y < F_H {
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
}

fn am_draw_mline(ml: &Mline, color: i32) {
    let mut fl = Fline {
        a: Fpoint { x: 0, y: 0 },
        b: Fpoint { x: 0, y: 0 },
    };
    if am_clip_mline(ml, &mut fl) {
        am_draw_fline(&fl, color);
    }
}

fn am_draw_grid(color: i32) {
    unsafe {
        let bmaporgx = BMAPORGX;
        let bmaporgy = BMAPORGY;
        let block_units = (MAPBLOCKUNITS as Fixed) << FRACBITS;

        let mut start = M_X;
        let rem = (start - bmaporgx) % block_units;
        if rem != 0 {
            start += block_units - rem;
        }
        let end_x = M_X + M_W;

        let mut ml = Mline {
            a: Mpoint { x: 0, y: M_Y },
            b: Mpoint { x: 0, y: M_Y + M_H },
        };
        let mut x = start;
        while x < end_x {
            ml.a.x = x;
            ml.b.x = x;
            am_draw_mline(&ml, color);
            x += block_units;
        }

        start = M_Y;
        let rem = (start - bmaporgy) % block_units;
        if rem != 0 {
            start += block_units - rem;
        }
        let end_y = M_Y + M_H;

        ml = Mline {
            a: Mpoint { x: M_X, y: 0 },
            b: Mpoint { x: M_X + M_W, y: 0 },
        };
        let mut y = start;
        while y < end_y {
            ml.a.y = y;
            ml.b.y = y;
            am_draw_mline(&ml, color);
            y += block_units;
        }
    }
}

fn am_rotate(x: &mut Fixed, y: &mut Fixed, a: u32) {
    let idx = (a >> ANGLETOFINESHIFT) as usize;
    let tmpx = fixed_mul(*x, finecosine(idx)) - fixed_mul(*y, finesine(idx));
    *y = fixed_mul(*x, finesine(idx)) + fixed_mul(*y, finecosine(idx));
    *x = tmpx;
}

fn am_draw_line_character(
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
        am_draw_mline(&ml, color);
    }
}

fn am_draw_walls() {
    use crate::doomdef::NUMPOWERS;

    unsafe {
        if LINES.is_null() || NUMLINES <= 0 {
            return;
        }
        let plr_idx = plr_index();
        let plr = &*PLAYERS.as_ptr().add(plr_idx);
        let pw_allmap = 4usize; // Powertype::Allmap
        let has_allmap = plr.powers[pw_allmap] > 0;

        for i in 0..NUMLINES as usize {
            let ld = &*LINES.add(i);
            if ld.v1.is_null() || ld.v2.is_null() {
                continue;
            }
            let v1 = &*ld.v1;
            let v2 = &*ld.v2;
            let mut ml = Mline {
                a: Mpoint { x: v1.x, y: v1.y },
                b: Mpoint { x: v2.x, y: v2.y },
            };

            let draw = if CHEATING != 0 || (ld.flags & ML_MAPPED) != 0 {
                if (ld.flags & LINE_NEVERSEE) != 0_i16 && CHEATING == 0 {
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
                WALLCOLORS + LIGHTLEV
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
                    if CHEATING != 0 {
                        SECRETWALLCOLORS + LIGHTLEV
                    } else {
                        WALLCOLORS + LIGHTLEV
                    }
                } else if back.floorheight != front.floorheight {
                    FDWALLCOLORS + LIGHTLEV
                } else if back.ceilingheight != front.ceilingheight {
                    CDWALLCOLORS + LIGHTLEV
                } else if CHEATING != 0 {
                    TSWALLCOLORS + LIGHTLEV
                } else {
                    continue;
                }
            };

            am_draw_mline(&ml, color);
        }
    }
}

fn am_draw_players() {
    use crate::doomstat::NETGAME;

    unsafe {
        let plr_idx = plr_index();
        let plr = &*PLAYERS.as_ptr().add(plr_idx);
        let mo = (*plr).mo as *const Mobj;
        if mo.is_null() {
            return;
        }

        if !NETGAME {
            let arrow: &[Mline] = if CHEATING != 0 {
                &cheat_player_arrow()[..]
            } else {
                &player_arrow()[..]
            };
            am_draw_line_character(
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

fn am_draw_things(colors: i32, _colorrange: i32) {
    unsafe {
        if SECTORS.is_null() || NUMSECTORS <= 0 {
            return;
        }
        let tri = thintriangle_guy();
        for i in 0..NUMSECTORS as usize {
            let sec = &*SECTORS.add(i);
            let mut t = sec.thinglist as *const Mobj;
            while !t.is_null() {
                let mobj = &*t;
                am_draw_line_character(
                    &tri,
                    16 << FRACBITS,
                    mobj.angle,
                    colors + LIGHTLEV,
                    mobj.x,
                    mobj.y,
                );
                t = mobj.snext as *const Mobj;
            }
        }
    }
}

fn am_draw_marks() {
    unsafe {
        for i in 0..AM_NUMMARKPOINTS {
            if MARKPOINTS[i].x == -1 {
                continue;
            }
            let fx = cxm_to_f(MARKPOINTS[i].x);
            let fy = cym_to_f(MARKPOINTS[i].y);
            let w = 5;
            let h = 6;
            if fx >= F_X && fx <= F_W - w && fy >= F_Y && fy <= F_H - h {
                let patch = MARKNUMS[i];
                if !patch.is_null() {
                    v_draw_patch(fx, fy, patch);
                }
            }
        }
    }
}

fn am_draw_crosshair(color: i32) {
    unsafe {
        if VIEWIMAGE.is_null() {
            return;
        }
        let base = VIEWIMAGE.add(F_Y as usize * SCREENWIDTH as usize + F_X as usize);
        let idx = (F_W * (F_H + 1)) / 2;
        *base.add(idx as usize) = color as u8;
    }
}

/// Draw automap. Called instead of view when automap active.
pub fn am_drawer() {
    unsafe {
        if !AUTOMAPACTIVE {
            return;
        }
        am_clear_fb(BACKGROUND);
        if GRID != 0 {
            am_draw_grid(GRIDCOLORS);
        }
        am_draw_walls();
        am_draw_players();
        if CHEATING == 2 {
            am_draw_things(THINGCOLORS, THINGRANGE);
        }
        am_draw_crosshair(XHAIRCOLORS);
        am_draw_marks();
        v_mark_rect(F_X, F_Y, F_W, F_H);
    }
}
