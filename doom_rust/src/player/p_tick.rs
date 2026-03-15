//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Thinker list, level ticker.
//
// Original: p_tick.h / p_tick.c

use crate::player::mobjs;
use crate::player::p_mobj::p_mobj_thinker_safe;

/// Initialize thinker list. Mobjs use index-based storage; no-op for new system.
/// Original: P_InitThinkers
pub fn p_init_thinkers() {}

/// Run all thinkers. Uses Arc<Mutex<Vec<Option<Mobj>>>> + thinker_indices.
/// Original: P_RunThinkers
pub fn p_run_thinkers() {
    mobjs::with_mobjs_state(|s| {
        for &idx in &s.thinker_indices {
            if s.to_remove.contains(&idx) {
                continue;
            }
            if let Some(mo) = s.mobjs.get_mut(idx).and_then(|o| o.as_mut()) {
                p_mobj_thinker_safe(mo);
            }
        }
        let to_remove: Vec<_> = s.to_remove.drain().collect();
        for idx in &to_remove {
            if let Some(slot) = s.mobjs.get_mut(*idx) {
                *slot = None;
            }
        }
        s.thinker_indices.retain(|idx| !to_remove.contains(idx));
    });
}

/// Advance all thinkers one tic. Original: P_Ticker (partial - no player/menu logic)
pub fn p_ticker() {
    p_run_thinkers();
}
