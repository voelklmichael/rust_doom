// doom.h - top-level Doom entry
// No dependencies (leaf module)
// D_DoomMain is implemented in d_main

#[allow(non_camel_case_types)]
pub struct DoomState;

impl DoomState {
    pub fn new() -> Self {
        Self
    }

    // Original: D_DoomMain - declared here, implemented in d_main
    pub fn d_doom_main(&self) {
        todo!("D_DoomMain - wire to d_main when migrated")
    }
}
