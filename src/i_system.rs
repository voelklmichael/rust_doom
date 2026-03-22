//! System interface (i_system.h, i_system.c)
//! Original: i_system.h, i_system.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::d_ticcmd::TiccmdT;
use crate::doomtype::Boolean;

pub type AtexitFuncT = fn();

pub struct I_SystemState;

impl I_SystemState {
    /// Original: void I_Init(void)
    pub fn i_init(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: byte *I_ZoneBase(int *size)
    pub fn i_zone_base(&self) -> (Vec<u8>, usize) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean I_ConsoleStdout(void)
    pub fn i_console_stdout(&self) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: ticcmd_t *I_BaseTiccmd(void)
    pub fn i_base_ticcmd(&self) -> TiccmdT {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_Quit(void)
    pub fn i_quit(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_Error(char *error, ...)
    pub fn i_error(&self, _error: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_Tactile(int on, int off, int total)
    pub fn i_tactile(&self, _on: i32, _off: i32, _total: i32) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean I_GetMemoryValue(unsigned int offset, void *value, int size)
    pub fn i_get_memory_value(&self, _offset: u32, _value: &mut [u8]) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_AtExit(atexit_func_t func, boolean run_if_error)
    pub fn i_at_exit(&self, _func: AtexitFuncT, _run_if_error: Boolean) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_BindVariables(void)
    pub fn i_bind_variables(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_PrintStartupBanner(char *gamedescription)
    pub fn i_print_startup_banner(&self, _gamedescription: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_PrintBanner(char *text)
    pub fn i_print_banner(&self, _text: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void I_PrintDivider(void)
    pub fn i_print_divider(&self) {
        todo!("Basic stage-0 stub")
    }
}
