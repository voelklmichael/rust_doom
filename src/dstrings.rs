//! DOOM strings (dstrings.h, dstrings.c)
//! Original: dstrings.h, dstrings.c

use std::sync::Arc;
use std::sync::Mutex;

// Plan §2.1: #include "d_englsh.h" -> pub use d_englsh::* (when d_englsh migrated)

// #define SAVEGAMENAME "doomsav"
pub static SAVEGAMENAME: &str = "doomsav";
// #define NUM_QUITMESSAGES 8
pub static NUM_QUITMESSAGES: i32 = 8;

pub struct DstringsState {
    // extern char *doom1_endmsg[]
    pub doom1_endmsg: Arc<Mutex<Vec<&'static str>>>,
    // extern char *doom2_endmsg[]
    pub doom2_endmsg: Arc<Mutex<Vec<&'static str>>>,
}

impl DstringsState {
    pub fn new() -> Self {
        Self {
            doom1_endmsg: Arc::new(Mutex::new(vec![
                "are you sure you want to\nquit this great game?",
                "please don't leave, there's more\ndemons to toast!",
                "let's beat it -- this is turning\ninto a bloodbath!",
                "i wouldn't leave if i were you.\ndos is much worse.",
                "you're trying to say you like dos\nbetter than me, right?",
                "don't leave yet -- there's a\ndemon around that corner!",
                "ya know, next time you come in here\ni'm gonna toast ya.",
                "go ahead and leave. see if i care.",
            ])),
            doom2_endmsg: Arc::new(Mutex::new(vec![
                "are you sure you want to\nquit this great game?",
                "you want to quit?\nthen, thou hast lost an eighth!",
                "don't go now, there's a \ndimensional shambler waiting\nat the dos prompt!",
                "get outta here and go back\nto your boring programs.",
                "if i were your boss, i'd \n deathmatch ya in a minute!",
                "look, bud. you leave now\nand you forfeit your body count!",
                "just leave. when you come\nback, i'll be waiting with a bat.",
                "you're lucky i don't smack\nyou for thinking about leaving.",
            ])),
        }
    }
}
