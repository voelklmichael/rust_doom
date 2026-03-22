//! WAD command line parsing (w_main.h, w_main.c)
//! Original: w_main.h, w_main.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;

pub struct W_MainState;

impl W_MainState {
    /// Original: boolean W_ParseCommandLine(void)
    /// Parse the command line, merging WAD files. Returns true if at least one file was added.
    pub fn w_parse_command_line(&self) -> Boolean {
        todo!("Basic stage-0 stub")
    }
}
