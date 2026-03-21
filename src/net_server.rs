// net_server.h - network server
// No dependencies (leaf module)
// Note: NET_SV_AddModule uses net_module_t from net_defs - will need wiring when net_defs is migrated

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct Net_ServerState {
    _placeholder: RefCell<()>,
}

impl Net_ServerState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: NET_SV_Init
    pub fn net_sv_init(&self) {
        todo!("NET_SV_Init")
    }

    // Original: NET_SV_Run
    pub fn net_sv_run(&self) {
        todo!("NET_SV_Run")
    }

    // Original: NET_SV_Shutdown
    pub fn net_sv_shutdown(&self) {
        todo!("NET_SV_Shutdown")
    }

    // Original: NET_SV_AddModule(net_module_t *module)
    pub fn net_sv_add_module(&self, _module: *mut std::ffi::c_void) {
        todo!("NET_SV_AddModule - wire net_module_t when net_defs migrated")
    }

    // Original: NET_SV_RegisterWithMaster
    pub fn net_sv_register_with_master(&self) {
        todo!("NET_SV_RegisterWithMaster")
    }
}
