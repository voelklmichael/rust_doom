// dstrings.h / dstrings.c

pub use crate::d_englsh::*;

use std::cell::RefCell;

// Original: #define SAVEGAMENAME "doomsav"
pub const SAVEGAMENAME: &str = "doomsav";

// Original: #define NUM_QUITMESSAGES 8
pub const NUM_QUITMESSAGES: usize = 8;

// Original: char *doom1_endmsg[]
pub static DOOM1_ENDMSG: [&str; NUM_QUITMESSAGES] = [
    "are you sure you want to\nquit this great game?",
    "please don't leave, there's more\ndemons to toast!",
    "let's beat it -- this is turning\ninto a bloodbath!",
    "i wouldn't leave if i were you.\ndos is much worse.",
    "you're trying to say you like dos\nbetter than me, right?",
    "don't leave yet -- there's a\ndemon around that corner!",
    "ya know, next time you come in here\ni'm gonna toast ya.",
    "go ahead and leave. see if i care.",
];

// Original: char *doom2_endmsg[]
pub static DOOM2_ENDMSG: [&str; NUM_QUITMESSAGES] = [
    "are you sure you want to\nquit this great game?",
    "you want to quit?\nthen, thou hast lost an eighth!",
    "don't go now, there's a \ndimensional shambler waiting\nat the dos prompt!",
    "get outta here and go back\nto your boring programs.",
    "if i were your boss, i'd \n deathmatch ya in a minute!",
    "look, bud. you leave now\nand you forfeit your body count!",
    "just leave. when you come\nback, i'll be waiting with a bat.",
    "you're lucky i don't smack\nyou for thinking about leaving.",
];

#[allow(non_camel_case_types)]
pub struct DstringsState {
    _ph: RefCell<()>,
}

impl DstringsState {
    pub fn new() -> Self {
        Self {
            _ph: RefCell::new(()),
        }
    }
}
