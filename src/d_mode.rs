//! Game mode/mission (d_mode.h)
//! Original: d_mode.h

use crate::doomtype::Boolean;

// typedef enum GameMission_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMissionT {
    Doom = 0,
    Doom2,
    PackTnt,
    PackPlut,
    PackChex,
    PackHacx,
    Heretic,
    Hexen,
    Strife,
    None,
}

// typedef enum GameMode_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameModeT {
    Shareware = 0,
    Registered,
    Commercial,
    Retail,
    Indetermined,
}

// typedef enum GameVersion_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameVersionT {
    ExeDoom12 = 0,
    ExeDoom1666,
    ExeDoom17,
    ExeDoom18,
    ExeDoom19,
    ExeHacx,
    ExeUltimate,
    ExeFinal,
    ExeFinal2,
    ExeChex,
    ExeHeretic13,
    ExeHexen11,
    ExeStrife12,
    ExeStrife131,
}

// typedef enum skill_t
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillT {
    SkNoitems = -1,
    SkBaby = 0,
    SkEasy,
    SkMedium,
    SkHard,
    SkNightmare,
}
