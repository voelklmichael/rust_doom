//! Ceiling animation (p_ceilng.c)
//! Original: p_ceilng.c

pub struct P_CeilngState;

impl P_CeilngState {
    /// Original: void T_MoveCeiling(ceiling_t *ceiling)
    pub fn t_move_ceiling(&self, _ceiling: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: int EV_DoCeiling(line_t *line, ceiling_e type)
    pub fn ev_do_ceiling(&self, _line: &(), _ceiling_type: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int EV_CeilingCrushStop(line_t *line)
    pub fn ev_ceiling_crush_stop(&self, _line: &()) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_AddActiveCeiling(ceiling_t *c)
    pub fn p_add_active_ceiling(&self, _c: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_RemoveActiveCeiling(ceiling_t *c)
    pub fn p_remove_active_ceiling(&self, _c: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void P_ActivateInStasisCeiling(line_t *line)
    pub fn p_activate_in_stasis_ceiling(&self, _line: &()) {
        todo!("Basic stage-0 stub")
    }
}
