//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Movement/collision utility functions, blockmap iterators.
//
// Original: p_maputl.c

use crate::m_fixed::{fixed_div, fixed_mul, Fixed, FRACBITS};
use crate::rendering::defs::{Line, SlopeType};
use crate::rendering::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP};

use super::Divline;

/// Gives an estimation of distance (not exact).
/// Original: P_AproxDistance
pub fn p_aprox_distance(dx: Fixed, dy: Fixed) -> Fixed {
    let dx = dx.abs();
    let dy = dy.abs();
    if dx < dy {
        dx + dy - (dx >> 1)
    } else {
        dx + dy - (dy >> 1)
    }
}

/// Returns 0 (front) or 1 (back).
/// Original: P_PointOnLineSide
pub fn p_point_on_line_side(x: Fixed, y: Fixed, line: *const Line) -> i32 {
    if line.is_null() {
        return 0;
    }
    let ld = unsafe { &*line };
    let v1_x = unsafe { (*ld.v1).x };
    let v1_y = unsafe { (*ld.v1).y };

    if ld.dx == 0 {
        return if x <= v1_x {
            if ld.dy > 0 {
                1
            } else {
                0
            }
        } else if ld.dy < 0 {
            1
        } else {
            0
        };
    }
    if ld.dy == 0 {
        return if y <= v1_y {
            if ld.dx < 0 {
                1
            } else {
                0
            }
        } else if ld.dx > 0 {
            1
        } else {
            0
        };
    }

    let dx = x - v1_x;
    let dy = y - v1_y;
    let left = fixed_mul(ld.dy >> FRACBITS, dx);
    let right = fixed_mul(dy, ld.dx >> FRACBITS);

    if right < left {
        0
    } else {
        1
    }
}

/// Returns 0 (front), 1 (back), or 2 (on line).
/// Original: P_DivlineSide (from p_sight.c, used for intercepts)
pub fn p_divline_side(x: Fixed, y: Fixed, line: &Divline) -> i32 {
    if line.dx == 0 {
        if x == line.x {
            return 2;
        }
        return if x <= line.x {
            if line.dy > 0 {
                1
            } else {
                0
            }
        } else if line.dy < 0 {
            1
        } else {
            0
        };
    }
    if line.dy == 0 {
        if y == line.y {
            return 2; // on line
        }
        return if y <= line.y {
            if line.dx < 0 {
                1
            } else {
                0
            }
        } else if line.dx > 0 {
            1
        } else {
            0
        };
    }

    let dx = x - line.x;
    let dy = y - line.y;
    let left = (line.dy >> FRACBITS) * (dx >> FRACBITS);
    let right = (dy >> FRACBITS) * (line.dx >> FRACBITS);

    if right < left {
        0
    } else if left == right {
        2
    } else {
        1
    }
}

/// Returns 0 (front) or 1 (back). Original: P_PointOnDivlineSide
pub fn p_point_on_divline_side(x: Fixed, y: Fixed, line: &Divline) -> i32 {
    if line.dx == 0 {
        return if x <= line.x {
            if line.dy > 0 {
                1
            } else {
                0
            }
        } else if line.dy < 0 {
            1
        } else {
            0
        };
    }
    if line.dy == 0 {
        return if y <= line.y {
            if line.dx < 0 {
                1
            } else {
                0
            }
        } else if line.dx > 0 {
            1
        } else {
            0
        };
    }

    let dx = x - line.x;
    let dy = y - line.y;

    const SIGN_BIT: i32 = i32::MIN; // 0x8000_0000
    if (line.dy ^ line.dx ^ dx ^ dy) & SIGN_BIT != 0 {
        if (line.dy ^ dx) & SIGN_BIT != 0 {
            return 1;
        }
        return 0;
    }

    let left = fixed_mul(line.dy >> 8, dx >> 8);
    let right = fixed_mul(dy >> 8, line.dx >> 8);

    if right < left {
        0
    } else {
        1
    }
}

/// Fill divline from linedef. Original: P_MakeDivline
pub fn p_make_divline(li: *const Line, dl: &mut Divline) {
    if li.is_null() {
        return;
    }
    let line = unsafe { &*li };
    dl.x = unsafe { (*line.v1).x };
    dl.y = unsafe { (*line.v1).y };
    dl.dx = line.dx;
    dl.dy = line.dy;
}

/// Returns fractional intercept along first divline. Original: P_InterceptVector
pub fn p_intercept_vector(v2: &Divline, v1: &Divline) -> Fixed {
    let den = fixed_mul(v1.dy >> 8, v2.dx) - fixed_mul(v1.dx >> 8, v2.dy);
    if den == 0 {
        return 0;
    }
    let num = fixed_mul((v1.x - v2.x) >> 8, v1.dy) + fixed_mul((v2.y - v1.y) >> 8, v1.dx);
    fixed_div(num, den)
}

/// Returns 0, 1, or -1 if box crosses line. Original: P_BoxOnLineSide
pub fn p_box_on_line_side(tmbox: &[Fixed; 4], ld: *const Line) -> i32 {
    if ld.is_null() {
        return 0;
    }
    let line = unsafe { &*ld };
    let v1_x = unsafe { (*line.v1).x };
    let v1_y = unsafe { (*line.v1).y };

    let (p1, p2) = match line.slopetype {
        SlopeType::Horizontal => {
            let mut p1 = if tmbox[BOXTOP] > v1_y { 1 } else { 0 };
            let mut p2 = if tmbox[BOXBOTTOM] > v1_y { 1 } else { 0 };
            if line.dx < 0 {
                p1 ^= 1;
                p2 ^= 1;
            }
            (p1, p2)
        }
        SlopeType::Vertical => {
            let mut p1 = if tmbox[BOXRIGHT] < v1_x { 1 } else { 0 };
            let mut p2 = if tmbox[BOXLEFT] < v1_x { 1 } else { 0 };
            if line.dy < 0 {
                p1 ^= 1;
                p2 ^= 1;
            }
            (p1, p2)
        }
        SlopeType::Positive => (
            p_point_on_line_side(tmbox[BOXLEFT], tmbox[BOXTOP], ld),
            p_point_on_line_side(tmbox[BOXRIGHT], tmbox[BOXBOTTOM], ld),
        ),
        SlopeType::Negative => (
            p_point_on_line_side(tmbox[BOXRIGHT], tmbox[BOXTOP], ld),
            p_point_on_line_side(tmbox[BOXLEFT], tmbox[BOXBOTTOM], ld),
        ),
    };

    if p1 == p2 {
        p1
    } else {
        -1
    }
}

/// P_LineOpening globals - set by p_line_opening.
pub static mut OPENTOP: Fixed = 0;
pub static mut OPENBOTTOM: Fixed = 0;
pub static mut OPENRANGE: Fixed = 0;
pub static mut LOWFLOOR: Fixed = 0;

/// Sets OPENTOP, OPENBOTTOM, OPENRANGE, LOWFLOOR for two-sided line.
/// Original: P_LineOpening
pub fn p_line_opening(linedef: *const Line) {
    if linedef.is_null() {
        unsafe {
            OPENRANGE = 0;
        }
        return;
    }
    let ld = unsafe { &*linedef };
    if ld.sidenum[1] == -1 {
        unsafe {
            OPENRANGE = 0;
        }
        return;
    }

    let front = ld.frontsector;
    let back = ld.backsector;
    if front.is_null() || back.is_null() {
        unsafe {
            OPENRANGE = 0;
        }
        return;
    }

    let front = unsafe { &*front };
    let back = unsafe { &*back };

    let opentop = if front.ceilingheight < back.ceilingheight {
        front.ceilingheight
    } else {
        back.ceilingheight
    };

    let (openbottom, lowfloor) = if front.floorheight > back.floorheight {
        (front.floorheight, back.floorheight)
    } else {
        (back.floorheight, front.floorheight)
    };

    unsafe {
        OPENTOP = opentop;
        OPENBOTTOM = openbottom;
        OPENRANGE = opentop - openbottom;
        LOWFLOOR = lowfloor;
    }
}

// TODO: P_BlockLinesIterator, P_BlockThingsIterator, P_PathTraverse,
// P_UnsetThingPosition, P_SetThingPosition - require blockmap
