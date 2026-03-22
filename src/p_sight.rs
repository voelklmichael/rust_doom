//! Line-of-sight / visibility checks (p_sight.c)
//! Original: p_sight.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::m_fixed::FixedT;
use crate::p_local::DivlineT;

pub struct P_SightState {
    pub sightzstart: Arc<Mutex<FixedT>>,
    pub topslope: Arc<Mutex<FixedT>>,
    pub bottomslope: Arc<Mutex<FixedT>>,
    pub t2x: Arc<Mutex<FixedT>>,
    pub t2y: Arc<Mutex<FixedT>>,
}

impl P_SightState {
    /// Original: int P_DivlineSide(fixed_t x, fixed_t y, divline_t *node)
    pub fn p_divline_side(&self, _x: FixedT, _y: FixedT, _node: &DivlineT) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_CrossSubsector(int num)
    pub fn p_cross_subsector(&self, _num: i32) -> bool {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean P_CheckSight(mobj_t *t1, mobj_t *t2)
    pub fn p_check_sight(&self, _t1: &(), _t2: &()) -> bool {
        todo!("Basic stage-0 stub")
    }
}
