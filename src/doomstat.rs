// doomstat.h - stub
use std::cell::RefCell;
#[allow(non_camel_case_types)]
pub struct DoomstatState { _ph: RefCell<()> }
impl DoomstatState { pub fn new() -> Self { Self { _ph: RefCell::new(()) } } }
