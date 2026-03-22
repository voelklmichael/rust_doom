//! Command line argument parsing (m_argv.h, m_argv.c)
//! Original: m_argv.h, m_argv.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;

pub struct M_ArgvState {
    pub myargc: Arc<Mutex<i32>>,
    pub myargv: Arc<Mutex<Vec<String>>>,
}

impl M_ArgvState {
    /// Original: int M_CheckParm(char *check)
    pub fn m_check_parm(&self, _check: &str) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: int M_CheckParmWithArgs(char *check, int num_args)
    pub fn m_check_parm_with_args(&self, _check: &str, _num_args: i32) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_FindResponseFile(void)
    pub fn m_find_response_file(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean M_ParmExists(char *check)
    pub fn m_parm_exists(&self, _check: &str) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *M_GetExecutableName(void)
    pub fn m_get_executable_name(&self) -> String {
        todo!("Basic stage-0 stub")
    }
}
