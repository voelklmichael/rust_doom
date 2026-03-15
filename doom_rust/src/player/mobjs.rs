//! Mobj storage - Arc<Mutex<Vec<Option<Mobj>>>> per UNSAFFE_ELIMINATION_PLAN.md
//!
//! Replaces z_malloc + raw pointers with index-based storage.

use crate::player::p_mobj::Mobj;
use std::sync::{Arc, Mutex, OnceLock};

/// Index into the mobjs vec. Replaces *mut Mobj.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MobjIndex(pub usize);

impl MobjIndex {
    pub const NULL: MobjIndex = MobjIndex(usize::MAX);

    pub fn is_null(self) -> bool {
        self.0 == usize::MAX
    }
}

/// Shared mobj storage. Thread-safe via Mutex.
static MOBJS_STATE: OnceLock<Arc<Mutex<MobjsState>>> = OnceLock::new();


pub struct MobjsState {
    pub mobjs: Vec<Option<Mobj>>,
    pub thinker_indices: Vec<usize>,
    pub to_remove: std::collections::HashSet<usize>,
}

fn get_mobjs_state() -> &'static Arc<Mutex<MobjsState>> {
    MOBJS_STATE.get_or_init(|| {
        Arc::new(Mutex::new(MobjsState {
            mobjs: Vec::new(),
            thinker_indices: Vec::new(),
            to_remove: std::collections::HashSet::new(),
        }))
    })
}

/// Access mobj state.
pub fn with_mobjs_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut MobjsState) -> R,
{
    let mut guard = get_mobjs_state().lock().unwrap();
    f(&mut guard)
}

/// Get ref to mobj by index, if present.
pub fn with_mobj_ref<F, R>(idx: MobjIndex, f: F) -> Option<R>
where
    F: FnOnce(&crate::player::p_mobj::Mobj) -> R,
{
    with_mobjs_state(|s| s.mobjs.get(idx.0).and_then(|o| o.as_ref()).map(f))
}

/// Get MobjIndex from raw pointer (address of Mobj in our Vec). Returns None if not found.
pub fn mobj_index_from_ptr(ptr: *mut crate::player::p_mobj::Mobj) -> Option<MobjIndex> {
    if ptr.is_null() {
        return None;
    }
    with_mobjs_state(|s| {
        s.mobjs
            .iter()
            .position(|o| o.as_ref().map(|m| m as *const _ == ptr).unwrap_or(false))
            .map(MobjIndex)
    })
}

/// Get raw pointer to Mobj from index. Used for legacy snext/sprev linking.
pub fn mobj_ptr_from_index(idx: MobjIndex) -> *mut crate::player::p_mobj::Mobj {
    if idx.is_null() {
        return std::ptr::null_mut();
    }
    with_mobjs_state(|s| {
        s.mobjs
            .get(idx.0)
            .and_then(|o| o.as_ref())
            .map(|m| m as *const _ as *mut _)
            .unwrap_or(std::ptr::null_mut())
    })
}

/// Allocate a new mobj slot. Returns the index.
pub fn mobj_alloc(mobj: Mobj) -> MobjIndex {
    with_mobjs_state(|s| {
        let idx = s.mobjs.len();
        s.mobjs.push(Some(mobj));
        s.thinker_indices.push(idx);
        MobjIndex(idx)
    })
}

/// Mark mobj for removal. Actual removal happens in p_run_thinkers.
pub fn mobj_mark_removed(idx: MobjIndex) {
    if idx.is_null() {
        return;
    }
    with_mobjs_state(|s| {
        s.to_remove.insert(idx.0);
    });
}
