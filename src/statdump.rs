//! Statistics dump (statdump.h, statdump.c)
//! Original: statdump.h, statdump.c

use std::sync::Arc;
use std::sync::Mutex;

pub struct WbstartstructT;

pub struct StatdumpState;

impl StatdumpState {
    /// Original: void StatCopy(wbstartstruct_t *stats)
    pub fn stat_copy(&self, _stats: &WbstartstructT) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void StatDump(void)
    pub fn stat_dump(&self) {
        todo!("Basic stage-0 stub")
    }
}
