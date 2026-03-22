//! Dummy/stub implementations (dummy.c)
//! Original: dummy.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;

pub struct DummyState {
    // boolean net_client_connected
    pub net_client_connected: Arc<Mutex<Boolean>>,
    // boolean drone
    pub drone: Arc<Mutex<Boolean>>,
}

impl DummyState {
    /// Original: void I_InitTimidityConfig(void) - empty when FEATURE_SOUND disabled
    pub fn i_init_timidity_config(&self) {
        // C body: (empty)
    }
}
