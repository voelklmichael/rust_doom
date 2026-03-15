//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Game completion, final screen animation.
//
// Original: f_finale.h + f_finale.c

use crate::deh::deh_string;
use std::sync::{Mutex, OnceLock};
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
use crate::rendering::{v_draw_patch, v_mark_rect, v_video};
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

// =============================================================================
// FFinaleState - thread-safe via OnceLock + Mutex
// =============================================================================

static F_FINALE_STATE: OnceLock<Mutex<FFinaleState>> = OnceLock::new();

pub struct FFinaleState {
    pub finalestage: FinaleStage,
    pub finalecount: u32,
    pub finaletext: &'static str,
    pub finaleflat: &'static str,
    pub castnum: usize,
    pub castdeath: bool,
}

fn get_f_finale_state() -> &'static Mutex<FFinaleState> {
    F_FINALE_STATE.get_or_init(|| {
        Mutex::new(FFinaleState {
            finalestage: FinaleStage::Text,
            finalecount: 0,
            finaletext: "",
            finaleflat: "FLOOR4_8",
            castnum: 0,
            castdeath: false,
        })
    })
}

/// Access FFinaleState.
pub fn with_f_finale_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut FFinaleState) -> R,
{
    let mut guard = get_f_finale_state().lock().unwrap();
    f(&mut guard)
}

// Cast call (simplified: name only; sprite drawing stubbed)
static CASTORDER: &[&'static str] = &[
    CC_ZOMBIE, CC_SHOTGUN, CC_HEAVY, CC_IMP, CC_DEMON, CC_LOST, CC_CACO, CC_HELL,
    CC_BARON, CC_ARACH, CC_PAIN, CC_REVEN, CC_MANCU, CC_ARCH, CC_SPIDER, CC_CYBER,
    CC_HERO,
];

/// Draw a single column of a patch. Used by F_BunnyScroll.
fn f_draw_patch_col(x: i32, patch: *const patch_t, col: i32) {
    v_video::with_v_video_state(|vv| {
        unsafe {
            if patch.is_null() || vv.viewimage.is_null() || col < 0 {
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
            let desttop = vv.viewimage.add(x as usize);

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
    });
}

fn f_text_write() {
    let (finaleflat, finaletext, finalecount) = with_f_finale_state(|st| {
        (st.finaleflat, st.finaletext, st.finalecount as i32)
    });
    v_video::with_v_video_state(|vv| {
        let dest = vv.viewimage;
    unsafe {

        let src = w_cache_lump_name(deh_string(finaleflat), PU_CACHE);
        if src.is_null() {
            return;
        }
        let src = src.as_ptr();
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
    });
}

fn f_bunny_scroll() {
    let finalecount = with_f_finale_state(|st| st.finalecount as i32);
    v_video::with_v_video_state(|vv| {
        if vv.viewimage.is_null() {
            return;
        }
    unsafe {
        let p1 = w_cache_lump_name(deh_string("PFUB2"), PU_LEVEL).as_ptr() as *const patch_t;
        let p2 = w_cache_lump_name(deh_string("PFUB1"), PU_LEVEL).as_ptr() as *const patch_t;

        if p1.is_null() || p2.is_null() {
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
    });
}

fn f_art_screen_drawer() {
    let (episode, gamemode) = with_doomstat_state(|st| (st.gameepisode, st.gamemode));

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
    let castnum = with_f_finale_state(|st| st.castnum);
    unsafe {
        let lump = w_cache_lump_name(deh_string("BOSSBACK"), PU_CACHE).as_ptr() as *const patch_t;
        if !lump.is_null() {
            v_draw_patch(0, 0, lump);
        }
        if castnum < CASTORDER.len() {
            f_cast_print(deh_string(CASTORDER[castnum]));
        }
        // Sprite drawing stubbed: would need states, mobjinfo, sprites
    }
}

fn f_start_cast() {
    with_f_finale_state(|st| {
        st.castnum = 0;
        st.castdeath = false;
        st.finalestage = FinaleStage::Cast;
    });
    with_doomstat_state(|st| st.wipegamestate = Gamestate::Level);
    s_change_music(MusicEnum::Evil as usize, true);
}

/// Handle input during finale. Returns true if event consumed.
/// Original: F_Responder
pub fn f_responder(ev: &crate::game::d_event::Event) -> bool {
    use crate::game::d_event::EvType;
    if ev.ev_type != EvType::KeyDown {
        return false;
    }
    with_f_finale_state(|st| {
        if st.finalestage != FinaleStage::Cast {
            return false;
        }
        if st.castdeath {
            return true;
        }
        st.castdeath = true;
        true
    })
}

/// Start the finale sequence. Called when level 30 (Doom 2) or episode end.
/// Original: F_StartFinale
pub fn f_start_finale() {
    with_d_main_state(|st| st.gameaction = Gameaction::Nothing);
    with_doomstat_state(|st| {
        st.gamestate = Gamestate::Finale;
        st.viewactive = false;
        st.automapactive = false;
    });
    if logical_gamemission() == GameMission::Doom {
        s_change_music(MusicEnum::Victor as usize, true);
    } else {
        s_change_music(MusicEnum::ReadM as usize, true);
    }
    let mission = logical_gamemission();
    let (episode, map) = with_doomstat_state(|st| (st.gameepisode, st.gamemap));
    with_f_finale_state(|st| {
        for screen in TEXTSCREENS {
            let level = screen.level;
            if mission == screen.mission
                && (mission != GameMission::Doom || episode == screen.episode)
                && map == level
            {
                st.finaletext = deh_string(screen.text);
                st.finaleflat = deh_string(screen.background);
                break;
            }
        }
        st.finalestage = FinaleStage::Text;
        st.finalecount = 0;
    });
}

/// Advance finale animation one tic.
/// Original: F_Ticker
pub fn f_ticker() {
    let (gamemode, gameepisode) = with_doomstat_state(|st| (st.gamemode, st.gameepisode));
    with_f_finale_state(|st| {
        st.finalecount += 1;
        if st.finalestage == FinaleStage::Cast {
            if st.castdeath {
                st.finalecount += 1;
                if st.finalecount % 70 == 0 {
                    st.castnum = (st.castnum + 1) % CASTORDER.len();
                    st.castdeath = false;
                }
            }
            return;
        }
        if gamemode == GameMode::Commercial {
            return;
        }
        if st.finalestage == FinaleStage::Text {
            let len = st.finaletext.len() as i32;
            if (st.finalecount as i32) > len * TEXTSPEED + TEXTWAIT {
                st.finalecount = 0;
                st.finalestage = FinaleStage::ArtScreen;
                with_doomstat_state(|s| s.wipegamestate = Gamestate::Level);
                if gameepisode == 3 {
                    s_start_music(MusicEnum::Bunny as i32);
                }
            }
        }
    });
}

/// Draw the current finale frame.
/// Original: F_Drawer
pub fn f_drawer() {
    let stage = with_f_finale_state(|st| st.finalestage);
    match stage {
        FinaleStage::Cast => f_cast_drawer(),
        FinaleStage::Text => f_text_write(),
        FinaleStage::ArtScreen => f_art_screen_drawer(),
    }
}
