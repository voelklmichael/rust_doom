// net_dedicated.h - dedicated server
// No dependencies (leaf module)

use std::cell::RefCell;

#[allow(non_camel_case_types)]
pub struct Net_DedicatedState {
    _placeholder: RefCell<()>,
}

impl Net_DedicatedState {
    pub fn new() -> Self {
        Self {
            _placeholder: RefCell::new(()),
        }
    }

    // Original: NET_DedicatedServer
    pub fn net_dedicated_server(&self) {
        todo!("NET_DedicatedServer")
    }
}
