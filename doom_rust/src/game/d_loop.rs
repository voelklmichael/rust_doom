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
use std::sync::{Mutex, OnceLock};

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

// =============================================================================
// DLoopState - thread-safe via OnceLock + Mutex
// =============================================================================

static D_LOOP_STATE: OnceLock<Mutex<DLoopState>> = OnceLock::new();

/// Safety: Raw pointer in DLoopState is only used while holding the Mutex lock.
unsafe impl Send for DLoopState {}

pub struct DLoopState {
    pub loop_interface: *const LoopInterface,
    /// When true, run one tic per TryRunTics call. Original: singletics
    pub singletics: bool,
    /// Game tic counter. Original: gametic
    pub gametic: i32,
    /// Tic duplication for net sync. Original: ticdup
    pub ticdup: i32,
}

fn get_d_loop_state() -> &'static Mutex<DLoopState> {
    D_LOOP_STATE.get_or_init(|| Mutex::new(DLoopState {
        loop_interface: ptr::null(),
        singletics: true,
        gametic: 0,
        ticdup: 1,
    }))
}

/// Access DLoopState.
pub fn with_d_loop_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut DLoopState) -> R,
{
    let mut guard = get_d_loop_state().lock().unwrap();
    f(&mut guard)
}

/// Register callback functions for the main loop.
/// Original: D_RegisterLoopCallbacks
pub fn d_register_loop_callbacks(i: *const LoopInterface) {
    with_d_loop_state(|st| st.loop_interface = i);
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
    let iface = with_d_loop_state(|st| st.loop_interface);
    if iface.is_null() {
        return;
    }
    let iface = unsafe { &*iface };
    (iface.process_events)();
    with_d_loop_state(|st| {
        if st.singletics {
            let mut cmd = Ticcmd::default();
            (iface.build_ticcmd)(&mut cmd, st.gametic);
            let ingame = true;
            (iface.run_tic)(&cmd, &ingame);
            st.gametic += 1;
        }
        // TODO: multi-tic logic when !singletics
    });
}

/// Called at start of game loop to initialize timers.
/// Original: D_StartGameLoop
pub fn d_start_game_loop() {
    crate::i_timer::i_init_timer();
}
