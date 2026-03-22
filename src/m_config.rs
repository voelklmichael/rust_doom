//! Configuration file (m_config.h, m_config.c)
//! Original: m_config.h, m_config.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;

pub struct M_ConfigState {
    pub configdir: Arc<Mutex<Option<String>>>,
}

impl M_ConfigState {
    /// Original: void M_LoadDefaults(void)
    pub fn m_load_defaults(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_SaveDefaults(void)
    pub fn m_save_defaults(&self) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_SaveDefaultsAlternate(char *main, char *extra)
    pub fn m_save_defaults_alternate(&self, _main: &str, _extra: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_SetConfigDir(char *dir)
    pub fn m_set_config_dir(&self, _dir: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_BindVariable(char *name, void *variable)
    pub fn m_bind_variable(&self, _name: &str, _variable: &()) {
        todo!("Basic stage-0 stub")
    }

    /// Original: boolean M_SetVariable(char *name, char *value)
    pub fn m_set_variable(&self, _name: &str, _value: &str) -> Boolean {
        todo!("Basic stage-0 stub")
    }

    /// Original: int M_GetIntVariable(char *name)
    pub fn m_get_int_variable(&self, _name: &str) -> i32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: const char *M_GetStrVariable(char *name)
    pub fn m_get_str_variable(&self, _name: &str) -> Option<String> {
        todo!("Basic stage-0 stub")
    }

    /// Original: float M_GetFloatVariable(char *name)
    pub fn m_get_float_variable(&self, _name: &str) -> f32 {
        todo!("Basic stage-0 stub")
    }

    /// Original: void M_SetConfigFilenames(char *main_config, char *extra_config)
    pub fn m_set_config_filenames(&self, _main_config: &str, _extra_config: &str) {
        todo!("Basic stage-0 stub")
    }

    /// Original: char *M_GetSaveGameDir(char *iwadname)
    pub fn m_get_save_game_dir(&self, _iwadname: &str) -> String {
        todo!("Basic stage-0 stub")
    }
}
