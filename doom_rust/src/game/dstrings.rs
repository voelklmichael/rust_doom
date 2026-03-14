//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  DOOM strings, by language.
//
// Original: dstrings.h + dstrings.c + d_englsh.h (partial)

// =============================================================================
// dstrings.h
// =============================================================================

/// Savegame filename prefix.
pub const SAVEGAMENAME: &str = "doomsav";

/// Number of quit messages per game type.
pub const NUM_QUITMESSAGES: usize = 8;

/// Doom 1 quit messages.
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

/// Doom 2 quit messages.
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

// =============================================================================
// d_englsh.h - commonly used strings (partial)
// =============================================================================

/// "press a key."
pub const PRESSKEY: &str = "press a key.";

/// "press y or n."
pub const PRESSYN: &str = "press y or n.";

/// "are you sure you want to\nquit this great game?"
pub const QUITMSG: &str = "are you sure you want to\nquit this great game?";

/// "you can't do load while in a net game!\n\n" + PRESSKEY
pub const LOADNET: &str = "you can't do load while in a net game!\n\npress a key.";

/// "you can't quickload during a netgame!\n\n" + PRESSKEY
pub const QLOADNET: &str = "you can't quickload during a netgame!\n\npress a key.";

/// "you haven't picked a quicksave slot yet!\n\n" + PRESSKEY
pub const QSAVESPOT: &str = "you haven't picked a quicksave slot yet!\n\npress a key.";

/// "you can't save if you aren't playing!\n\n" + PRESSKEY
pub const SAVEDEAD: &str = "you can't save if you aren't playing!\n\npress a key.";

/// "quicksave over your game named\n\n'%s'?\n\n" + PRESSYN
pub const QSPROMPT: &str = "quicksave over your game named\n\n'%s'?\n\npress y or n.";

/// "do you want to quickload the game named\n\n'%s'?\n\n" + PRESSYN
pub const QLPROMPT: &str = "do you want to quickload the game named\n\n'%s'?\n\npress y or n.";

/// "game saved."
pub const GGSAVED: &str = "game saved.";

/// "empty slot"
pub const EMPTYSTRING: &str = "empty slot";

// Automap strings (AMSTR_*)
pub const AMSTR_FOLLOWON: &str = "Follow Mode ON";
pub const AMSTR_FOLLOWOFF: &str = "Follow Mode OFF";
pub const AMSTR_GRIDON: &str = "Grid ON";
pub const AMSTR_GRIDOFF: &str = "Grid OFF";
pub const AMSTR_MARKEDSPOT: &str = "Marked Spot";
pub const AMSTR_MARKSCLEARED: &str = "All Marks Cleared";
