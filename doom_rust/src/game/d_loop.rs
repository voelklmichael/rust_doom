//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Main loop stuff.
//
// Original: d_loop.h + d_loop.c (stub - single-player only)

use super::d_ticcmd::Ticcmd;
use std::ptr;

/// Callback: run one game tic with given ticcmds.
pub type RunTicFn = fn(*const Ticcmd, *const bool);

/// Callback: build ticcmd for a tic.
pub type BuildTiccmdFn = fn(*mut Ticcmd, i32);

/// Callback: process input events.
pub type ProcessEventsFn = fn();

/// Callback: run menu.
pub type RunMenuFn = fn();

/// Loop interface - callbacks for the main loop.
#[repr(C)]
pub struct LoopInterface {
    pub process_events: ProcessEventsFn,
    pub build_ticcmd: BuildTiccmdFn,
    pub run_tic: RunTicFn,
    pub run_menu: RunMenuFn,
}

/// When true, run one tic per TryRunTics call. Original: singletics
pub static mut SINGLETICS: bool = true;

/// Game tic counter. Original: gametic
pub static mut GAMETIC: i32 = 0;

/// Tic duplication for net sync. Original: ticdup
pub static mut TICDUP: i32 = 1;

static mut LOOP_INTERFACE: *const LoopInterface = ptr::null();

/// Register callback functions for the main loop.
/// Original: D_RegisterLoopCallbacks
pub fn d_register_loop_callbacks(i: *const LoopInterface) {
    unsafe {
        LOOP_INTERFACE = i;
    }
}

/// Create any new ticcmds and broadcast to other players.
/// Original: NetUpdate (stub - no net)
pub fn net_update() {
    // Stub
}

/// Original: D_QuitNetGame
pub fn d_quit_net_game() {
    // Stub
}

/// Run pending tics. Single-player: runs one tic if singletics.
/// Original: TryRunTics
pub fn try_run_tics() {
    unsafe {
        let iface = LOOP_INTERFACE;
        if iface.is_null() {
            return;
        }
        let iface = &*iface;
        (iface.process_events)();
        if SINGLETICS {
            let mut cmd = Ticcmd::default();
            (iface.build_ticcmd)(&mut cmd, GAMETIC);
            let ingame = true;
            (iface.run_tic)(&cmd, &ingame);
            GAMETIC += 1;
        }
        // TODO: multi-tic logic when !singletics
    }
}

/// Called at start of game loop to initialize timers.
/// Original: D_StartGameLoop
pub fn d_start_game_loop() {
    crate::i_timer::i_init_timer();
}
