//! MUS to MIDI conversion (mus2mid.h, mus2mid.c)
//! Original: mus2mid.h, mus2mid.c

use std::sync::Arc;
use std::sync::Mutex;

use crate::doomtype::Boolean;
use crate::memio::Memfile;

pub struct Mus2midState;

impl Mus2midState {
    /// Original: boolean mus2mid(MEMFILE *musinput, MEMFILE *midioutput)
    pub fn mus2mid(&self, _mus_input: &Memfile, _midi_output: &Memfile) -> Boolean {
        todo!("Basic stage-0 stub")
    }
}
