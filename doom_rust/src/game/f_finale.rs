//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Game completion, final screen animation.
//
// Original: f_finale.h + f_finale.c

use crate::deh::deh_string;
use crate::doomdef::{Gameaction, Gamestate, SCREENHEIGHT, SCREENWIDTH};
use crate::doomstat::{logical_gamemission, GAMEEPISODE, GAMEMAP, GAMEMODE, GAMESTATE, WIPEGAMESTATE};
use crate::game::d_main::GAMEACTION;
use crate::game::d_mode::{GameMission, GameMode};
use crate::game::dstrings::{
    C1TEXT, C2TEXT, C3TEXT, C4TEXT, C5TEXT, C6TEXT, E1TEXT, E2TEXT, E3TEXT, E4TEXT,
    P1TEXT, P2TEXT, P3TEXT, P4TEXT, P5TEXT, P6TEXT, T1TEXT, T2TEXT, T3TEXT, T4TEXT,
    T5TEXT, T6TEXT,
    CC_ARCH, CC_ARACH, CC_BARON, CC_CACO, CC_CYBER, CC_DEMON, CC_HEAVY, CC_HELL,
    CC_HERO, CC_IMP, CC_LOST, CC_MANCU, CC_PAIN, CC_REVEN, CC_SHOTGUN, CC_SPIDER,
    CC_ZOMBIE,
};
use crate::rendering::patch_t;
use crate::rendering::{v_draw_patch, v_mark_rect, VIEWIMAGE};
use crate::sound::{s_change_music, s_start_music, MusicEnum};
use crate::ui_hud::hu_stuff::{hu_draw_char, hu_string_width};
use crate::wad::w_cache_lump_name;
use crate::z_zone::{PU_CACHE, PU_LEVEL};

/// Finale stage (finalestage_t in C).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FinaleStage {
    Text,
    ArtScreen,
    Cast,
}

const TEXTSPEED: i32 = 3;
const TEXTWAIT: i32 = 250;

struct TextScreen {
    mission: GameMission,
    episode: i32,
    level: i32,
    background: &'static str,
    text: &'static str,
}

static TEXTSCREENS: &[TextScreen] = &[
    TextScreen { mission: GameMission::Doom, episode: 1, level: 8, background: "FLOOR4_8", text: E1TEXT },
    TextScreen { mission: GameMission::Doom, episode: 2, level: 8, background: "SFLR6_1", text: E2TEXT },
    TextScreen { mission: GameMission::Doom, episode: 3, level: 8, background: "MFLR8_4", text: E3TEXT },
    TextScreen { mission: GameMission::Doom, episode: 4, level: 8, background: "MFLR8_3", text: E4TEXT },
    TextScreen { mission: GameMission::Doom2, episode: 1, level: 6, background: "SLIME16", text: C1TEXT },
    TextScreen { mission: GameMission::Doom2, episode: 1, level: 11, background: "RROCK14", text: C2TEXT },
    TextScreen { mission: GameMission::Doom2, episode: 1, level: 20, background: "RROCK07", text: C3TEXT },
    TextScreen { mission: GameMission::Doom2, episode: 1, level: 30, background: "RROCK17", text: C4TEXT },
    TextScreen { mission: GameMission::Doom2, episode: 1, level: 15, background: "RROCK13", text: C5TEXT },
    TextScreen { mission: GameMission::Doom2, episode: 1, level: 31, background: "RROCK19", text: C6TEXT },
    TextScreen { mission: GameMission::PackTnt, episode: 1, level: 6, background: "SLIME16", text: T1TEXT },
    TextScreen { mission: GameMission::PackTnt, episode: 1, level: 11, background: "RROCK14", text: T2TEXT },
    TextScreen { mission: GameMission::PackTnt, episode: 1, level: 20, background: "RROCK07", text: T3TEXT },
    TextScreen { mission: GameMission::PackTnt, episode: 1, level: 30, background: "RROCK17", text: T4TEXT },
    TextScreen { mission: GameMission::PackTnt, episode: 1, level: 15, background: "RROCK13", text: T5TEXT },
    TextScreen { mission: GameMission::PackTnt, episode: 1, level: 31, background: "RROCK19", text: T6TEXT },
    TextScreen { mission: GameMission::PackPlut, episode: 1, level: 6, background: "SLIME16", text: P1TEXT },
    TextScreen { mission: GameMission::PackPlut, episode: 1, level: 11, background: "RROCK14", text: P2TEXT },
    TextScreen { mission: GameMission::PackPlut, episode: 1, level: 20, background: "RROCK07", text: P3TEXT },
    TextScreen { mission: GameMission::PackPlut, episode: 1, level: 30, background: "RROCK17", text: P4TEXT },
    TextScreen { mission: GameMission::PackPlut, episode: 1, level: 15, background: "RROCK13", text: P5TEXT },
    TextScreen { mission: GameMission::PackPlut, episode: 1, level: 31, background: "RROCK19", text: P6TEXT },
];

static mut FINALESTAGE: FinaleStage = FinaleStage::Text;
static mut FINALECOUNT: u32 = 0;
static mut FINALETEXT: &'static str = "";
static mut FINALEFLAT: &'static str = "FLOOR4_8";

// Cast call (simplified: name only; sprite drawing stubbed)
static CASTORDER: &[&'static str] = &[
    CC_ZOMBIE, CC_SHOTGUN, CC_HEAVY, CC_IMP, CC_DEMON, CC_LOST, CC_CACO, CC_HELL,
    CC_BARON, CC_ARACH, CC_PAIN, CC_REVEN, CC_MANCU, CC_ARCH, CC_SPIDER, CC_CYBER,
    CC_HERO,
];

static mut CASTNUM: usize = 0;
static mut CASTDEATH: bool = false;

/// Draw a single column of a patch. Used by F_BunnyScroll.
fn f_draw_patch_col(x: i32, patch: *const patch_t, col: i32) {
    unsafe {
        if patch.is_null() || VIEWIMAGE.is_null() || col < 0 {
            return;
        }
        let patch_bytes = patch as *const u8;
        let width = (*patch).width as i32;
        if col >= width {
            return;
        }
        let ofs = i32::from_le_bytes(std::ptr::read_unaligned(
            patch_bytes.add(8 + col as usize * 4) as *const [u8; 4],
        ));
        let column = patch_bytes.add(ofs as usize);
        let mut col_ptr = column;
        let desttop = VIEWIMAGE.add(x as usize);

        loop {
            let topdelta = *col_ptr;
            if topdelta == 0xff {
                break;
            }
            let length = *col_ptr.add(1) as usize;
            let source = col_ptr.add(3);
            let dest = desttop.add(topdelta as usize * SCREENWIDTH as usize);
            for row in 0..length {
                *dest.add(row * SCREENWIDTH as usize) = *source.add(row);
            }
            col_ptr = col_ptr.add(4 + length);
        }
    }
}

fn f_text_write() {
    unsafe {
        let finaleflat = FINALEFLAT;
        let finaletext = FINALETEXT;
        let finalecount = FINALECOUNT as i32;

        let src = w_cache_lump_name(deh_string(finaleflat), PU_CACHE);
        if src.is_null() {
            return;
        }
        let src = src.as_ptr();
        let dest = VIEWIMAGE;
        if dest.is_null() {
            return;
        }

        for y in 0..SCREENHEIGHT {
            let mut dest_ptr = dest.add((y as usize) * SCREENWIDTH as usize);
            let row_ofs = ((y & 63) << 6) as usize;
            for _ in 0..(SCREENWIDTH / 64) {
                std::ptr::copy_nonoverlapping(src.add(row_ofs), dest_ptr, 64);
                dest_ptr = dest_ptr.add(64);
            }
            let remainder = (SCREENWIDTH & 63) as usize;
            if remainder > 0 {
                std::ptr::copy_nonoverlapping(src.add(row_ofs), dest_ptr, remainder);
            }
        }

        v_mark_rect(0, 0, SCREENWIDTH, SCREENHEIGHT);

        let mut cx = 10;
        let mut cy = 10;
        let mut ch_iter = finaletext.bytes();
        let mut count = (finalecount - 10) / TEXTSPEED;
        if count < 0 {
            count = 0;
        }

        while count > 0 {
            count -= 1;
            let c = match ch_iter.next() {
                Some(b) => b,
                None => break,
            };
            if c == 0 {
                break;
            }
            if c == b'\n' {
                cx = 10;
                cy += 11;
                continue;
            }
            let w = hu_draw_char(cx, cy, c);
            if cx + w > SCREENWIDTH {
                break;
            }
            cx += w;
        }
    }
}

fn f_bunny_scroll() {
    unsafe {
        let finalecount = FINALECOUNT as i32;
        let p1 = w_cache_lump_name(deh_string("PFUB2"), PU_LEVEL).as_ptr() as *const patch_t;
        let p2 = w_cache_lump_name(deh_string("PFUB1"), PU_LEVEL).as_ptr() as *const patch_t;

        if p1.is_null() || p2.is_null() || VIEWIMAGE.is_null() {
            return;
        }

        v_mark_rect(0, 0, SCREENWIDTH, SCREENHEIGHT);

        let mut scrolled = 320 - ((finalecount - 230) / 2);
        scrolled = scrolled.clamp(0, 320);

        for x in 0..SCREENWIDTH {
            if x + scrolled < 320 {
                f_draw_patch_col(x, p1, x + scrolled);
            } else {
                f_draw_patch_col(x, p2, x + scrolled - 320);
            }
        }

        if finalecount < 1130 {
            return;
        }
        if finalecount < 1180 {
            let lump = w_cache_lump_name(deh_string("END0"), PU_CACHE).as_ptr() as *const patch_t;
            if !lump.is_null() {
                v_draw_patch((SCREENWIDTH - 13 * 8) / 2, (SCREENHEIGHT - 8 * 8) / 2, lump);
            }
            return;
        }

        let stage = ((finalecount - 1180) / 5).min(6);
        let name = format!("END{}", stage);
        let lump = w_cache_lump_name(deh_string(&name), PU_CACHE).as_ptr() as *const patch_t;
        if !lump.is_null() {
            v_draw_patch((SCREENWIDTH - 13 * 8) / 2, (SCREENHEIGHT - 8 * 8) / 2, lump);
        }
    }
}

fn f_art_screen_drawer() {
    unsafe {
        let episode = GAMEEPISODE;
        let gamemode = GAMEMODE;

        if episode == 3 {
            f_bunny_scroll();
        } else {
            let lumpname = match episode {
                1 => {
                    if gamemode == GameMode::Retail {
                        "CREDIT"
                    } else {
                        "HELP2"
                    }
                }
                2 => "VICTORY2",
                4 => "ENDPIC",
                _ => return,
            };

            let lump = w_cache_lump_name(deh_string(lumpname), PU_CACHE).as_ptr() as *const patch_t;
            if !lump.is_null() {
                v_draw_patch(0, 0, lump);
            }
        }
    }
}

fn f_cast_print(text: &str) {
    let width = hu_string_width(text);
    let mut cx = 160 - width / 2;
    let cy = 180;
    for b in text.bytes() {
        let c = if b >= b'a' && b <= b'z' { b - 32 } else { b };
        let w = hu_draw_char(cx, cy, c);
        cx += w;
    }
}

fn f_cast_drawer() {
    unsafe {
        let lump = w_cache_lump_name(deh_string("BOSSBACK"), PU_CACHE).as_ptr() as *const patch_t;
        if !lump.is_null() {
            v_draw_patch(0, 0, lump);
        }
        if CASTNUM < CASTORDER.len() {
            f_cast_print(deh_string(CASTORDER[CASTNUM]));
        }
        // Sprite drawing stubbed: would need states, mobjinfo, sprites
    }
}

fn f_start_cast() {
    unsafe {
        WIPEGAMESTATE = Gamestate::Level;
        CASTNUM = 0;
        CASTDEATH = false;
        FINALESTAGE = FinaleStage::Cast;
        s_change_music(MusicEnum::Evil as usize, true);
    }
}

fn f_cast_ticker() {
    // Stub: full implementation needs states, mobjinfo, sprites.
    // For now, advance cast every ~3 seconds when in death state.
    unsafe {
        if CASTDEATH {
            FINALECOUNT += 1;
            if FINALECOUNT % 70 == 0 {
                CASTNUM = (CASTNUM + 1) % CASTORDER.len();
                CASTDEATH = false;
            }
        }
    }
}

/// Handle input during finale. Returns true if event consumed.
/// Original: F_Responder
pub fn f_responder(ev: &crate::game::d_event::Event) -> bool {
    use crate::game::d_event::EvType;
    unsafe {
        if FINALESTAGE != FinaleStage::Cast {
            return false;
        }
        if ev.ev_type != EvType::KeyDown {
            return false;
        }
        if CASTDEATH {
            return true;
        }
        CASTDEATH = true;
        // Death state/sound stubbed
        true
    }
}

/// Start the finale sequence. Called when level 30 (Doom 2) or episode end.
/// Original: F_StartFinale
pub fn f_start_finale() {
    unsafe {
        GAMEACTION = Gameaction::Nothing;
        GAMESTATE = Gamestate::Finale;
        crate::doomstat::VIEWACTIVE = false;
        crate::doomstat::AUTOMAPACTIVE = false;

        if logical_gamemission() == GameMission::Doom {
            s_change_music(MusicEnum::Victor as usize, true);
        } else {
            s_change_music(MusicEnum::ReadM as usize, true);
        }

        let mission = logical_gamemission();
        let episode = GAMEEPISODE;
        let map = GAMEMAP;

        for screen in TEXTSCREENS {
            let level = screen.level;
            if mission == screen.mission
                && (mission != GameMission::Doom || episode == screen.episode)
                && map == level
            {
                FINALETEXT = deh_string(screen.text);
                FINALEFLAT = deh_string(screen.background);
                break;
            }
        }

        FINALESTAGE = FinaleStage::Text;
        FINALECOUNT = 0;
    }
}

/// Advance finale animation one tic.
/// Original: F_Ticker
pub fn f_ticker() {
    unsafe {
        // Skip check (commercial): C uses players[i].cmd.buttons; we omit for now

        FINALECOUNT += 1;

        if FINALESTAGE == FinaleStage::Cast {
            f_cast_ticker();
            return;
        }

        if GAMEMODE == GameMode::Commercial {
            return;
        }

        if FINALESTAGE == FinaleStage::Text {
            let len = FINALETEXT.len() as i32;
            if (FINALECOUNT as i32) > len * TEXTSPEED + TEXTWAIT {
                FINALECOUNT = 0;
                FINALESTAGE = FinaleStage::ArtScreen;
                WIPEGAMESTATE = Gamestate::Level;
                if GAMEEPISODE == 3 {
                    s_start_music(MusicEnum::Bunny as i32);
                }
            }
        }
    }
}

/// Draw the current finale frame.
/// Original: F_Drawer
pub fn f_drawer() {
    unsafe {
        match FINALESTAGE {
            FinaleStage::Cast => f_cast_drawer(),
            FinaleStage::Text => f_text_write(),
            FinaleStage::ArtScreen => f_art_screen_drawer(),
        }
    }
}
