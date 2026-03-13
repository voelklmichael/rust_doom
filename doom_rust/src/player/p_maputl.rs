//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Movement/collision utility functions, blockmap iterators.
//
// Original: p_maputl.c (partial - P_AproxDistance implemented)

use crate::m_fixed::Fixed;

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

// TODO: P_PointOnLineSide, P_PointOnDivlineSide, P_MakeDivline, P_InterceptVector,
// P_BoxOnLineSide, P_LineOpening, P_BlockLinesIterator, P_BlockThingsIterator,
// P_PathTraverse, P_UnsetThingPosition, P_SetThingPosition
// Require: line_t, mobj_t, sector_t, blockmap, etc.
