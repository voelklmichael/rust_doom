//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  System-specific timer interface.
//
// Original: i_timer.h + i_timer.c

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use crate::doomdef::TICRATE;

/// Ticks per second (1/35th second tics).
pub const TICRATE_CONST: i32 = 35;

static BASETIME: AtomicU64 = AtomicU64::new(0);
static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

fn get_start() -> Instant {
    *START.get_or_init(Instant::now)
}

/// Returns current time in tics (1/35th second).
/// Original: I_GetTime
pub fn i_get_time() -> i32 {
    let start = get_start();
    let elapsed_ms = start.elapsed().as_millis() as u64;
    let base = BASETIME.load(Ordering::Relaxed);
    let base = if base == 0 {
        BASETIME.store(elapsed_ms, Ordering::Relaxed);
        elapsed_ms
    } else {
        base
    };
    let ticks = elapsed_ms.saturating_sub(base);
    ((ticks * TICRATE as u64) / 1000) as i32
}

/// Returns current time in milliseconds.
/// Original: I_GetTimeMS
pub fn i_get_time_ms() -> i32 {
    let start = get_start();
    let elapsed_ms = start.elapsed().as_millis() as u64;
    let base = BASETIME.load(Ordering::Relaxed);
    let base = if base == 0 {
        BASETIME.store(elapsed_ms, Ordering::Relaxed);
        elapsed_ms
    } else {
        base
    };
    (elapsed_ms.saturating_sub(base)) as i32
}

/// Pause for a specified number of ms.
/// Original: I_Sleep
pub fn i_sleep(ms: u32) {
    std::thread::sleep(Duration::from_millis(ms as u64));
}

/// Wait for vertical retrace or pause a bit.
/// Original: I_WaitVBL
pub fn i_wait_vbl(_count: i32) {
    // Stub: no-op
}

/// Initialize timer.
/// Original: I_InitTimer
pub fn i_init_timer() {
    let _ = get_start();
}
