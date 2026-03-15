//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//  Savegame I/O, archiving, persistence.
//
// Original: p_saveg.h / p_saveg.c

use std::io::{Read, Write};

use crate::doomdef::MAXPLAYERS;
use crate::doomstat::with_doomstat_state;
use crate::game::d_mode::Skill;

/// Maximum size of a savegame description string.
pub const SAVESTRINGSIZE: usize = 24;

const SAVEGAME_EOF: u8 = 0x1d;
const VERSIONSIZE: usize = 16;

/// Vanilla version code for savegame compatibility.
fn g_vanilla_version_code() -> i32 {
    111 // Doom 1.9
}

/// Temporary filename while saving. Original: P_TempSaveGameFile
pub fn p_temp_save_game_file() -> std::path::PathBuf {
    let dir = with_doomstat_state(|st| st.savegamedir.as_deref().unwrap_or(".").to_string());
    std::path::Path::new(&dir).join("temp.dsg")
}

/// Filename for save slot. Original: P_SaveGameFile
pub fn p_save_game_file(slot: i32) -> std::path::PathBuf {
    let dir = with_doomstat_state(|st| st.savegamedir.as_deref().unwrap_or(".").to_string());
    std::path::Path::new(&dir).join(format!("{}{}.dsg", crate::game::dstrings::SAVEGAMENAME, slot))
}

// --- Low-level I/O ---

fn saveg_write8<W: Write>(w: &mut W, value: u8) -> std::io::Result<()> {
    w.write_all(&[value])
}

fn saveg_read8<R: Read>(r: &mut R) -> std::io::Result<u8> {
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn saveg_write16<W: Write>(w: &mut W, value: i16) -> std::io::Result<()> {
    let v = value as u16;
    w.write_all(&[v as u8, (v >> 8) as u8])
}

fn saveg_read16<R: Read>(r: &mut R) -> std::io::Result<i16> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(i16::from_le_bytes(buf))
}

fn saveg_write32<W: Write>(w: &mut W, value: i32) -> std::io::Result<()> {
    w.write_all(&value.to_le_bytes())
}

fn saveg_read32<R: Read>(r: &mut R) -> std::io::Result<i32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

/// Read savegame header. Original: P_ReadSaveGameHeader
pub fn p_read_save_game_header<R: Read>(r: &mut R) -> std::io::Result<bool> {
    let mut _desc = [0u8; SAVESTRINGSIZE];
    r.read_exact(&mut _desc)?;

    let mut read_vcheck = [0u8; VERSIONSIZE];
    r.read_exact(&mut read_vcheck)?;

    let vcheck = format!("version {}", g_vanilla_version_code());
    let vcheck_bytes = vcheck.as_bytes();
    if read_vcheck[..vcheck_bytes.len().min(VERSIONSIZE)] != vcheck_bytes[..vcheck_bytes.len().min(VERSIONSIZE)] {
        return Ok(false);
    }

    let gameskill = saveg_read8(r)? as i32;
    let gameepisode = saveg_read8(r)? as i32;
    let gamemap = saveg_read8(r)? as i32;

    with_doomstat_state(|st| {
        st.gameskill = match gameskill {
            0 => Skill::Baby,
            1 => Skill::Easy,
            2 => Skill::Medium,
            3 => Skill::Hard,
            4 => Skill::Nightmare,
            _ => Skill::Medium,
        };
        st.gameepisode = gameepisode;
        st.gamemap = gamemap;
    });

    for i in 0..MAXPLAYERS {
        let v = saveg_read8(r)? != 0;
        with_doomstat_state(|st| st.playeringame[i] = v);
    }

    let a = saveg_read8(r)? as i32;
    let b = saveg_read8(r)? as i32;
    let c = saveg_read8(r)? as i32;
    with_doomstat_state(|st| st.leveltime = (a << 16) | (b << 8) | c);

    Ok(true)
}

/// Write savegame header. Original: P_WriteSaveGameHeader
pub fn p_write_save_game_header<W: Write>(w: &mut W, description: &[u8]) -> std::io::Result<()> {
    let len = description.len().min(SAVESTRINGSIZE);
    w.write_all(&description[..len])?;
    for _ in len..SAVESTRINGSIZE {
        saveg_write8(w, 0)?;
    }

    let version = format!("version {}", g_vanilla_version_code());
    let vbytes = version.as_bytes();
    for i in 0..VERSIONSIZE {
        saveg_write8(w, vbytes.get(i).copied().unwrap_or(0))?;
    }

    let (gameskill, gameepisode, gamemap, playeringame, leveltime) = with_doomstat_state(|st| {
        (
            st.gameskill as i32,
            st.gameepisode,
            st.gamemap,
            st.playeringame,
            st.leveltime,
        )
    });
    saveg_write8(w, gameskill as u8)?;
    saveg_write8(w, gameepisode as u8)?;
    saveg_write8(w, gamemap as u8)?;

    for i in 0..MAXPLAYERS {
        saveg_write8(w, if playeringame[i] { 1 } else { 0 })?;
    }
    saveg_write8(w, ((leveltime >> 16) & 0xff) as u8)?;
    saveg_write8(w, ((leveltime >> 8) & 0xff) as u8)?;
    saveg_write8(w, (leveltime & 0xff) as u8)?;

    Ok(())
}

/// Read savegame EOF marker. Original: P_ReadSaveGameEOF
pub fn p_read_save_game_eof<R: Read>(r: &mut R) -> std::io::Result<bool> {
    let value = saveg_read8(r)?;
    Ok(value == SAVEGAME_EOF)
}

/// Write savegame EOF marker. Original: P_WriteSaveGameEOF
pub fn p_write_save_game_eof<W: Write>(w: &mut W) -> std::io::Result<()> {
    saveg_write8(w, SAVEGAME_EOF)
}

/// Archive players to save stream. Original: P_ArchivePlayers
pub fn p_archive_players<W: Write>(w: &mut W) -> std::io::Result<()> {
    use crate::doomdef::{NUMAMMO, NUMCARDS, NUMWEAPONS};
    use crate::doomstat::{Player, PlayerState, with_doomstat_state};
    use crate::game::d_ticcmd::Ticcmd;
    use crate::info::states;

    with_doomstat_state(|st| {
    for i in 0..MAXPLAYERS {
        if !st.playeringame[i] {
            continue;
        }
        let p = &st.players[i];

        // mo - stored as pointer (we use 0, will be set when unarc thinker)
        saveg_write32(w, 0)?;

        // playerstate
        saveg_write32(w, p.playerstate as i32)?;

        // ticcmd
        saveg_write8(w, p.cmd.forwardmove as u8)?;
        saveg_write8(w, p.cmd.sidemove as u8)?;
        saveg_write16(w, p.cmd.angleturn)?;
        saveg_write16(w, p.cmd.consistancy as i16)?;
        saveg_write8(w, p.cmd.chatchar)?;
        saveg_write8(w, p.cmd.buttons)?;

        saveg_write32(w, p.viewz)?;
        saveg_write32(w, p.viewheight)?;
        saveg_write32(w, p.deltaviewheight)?;
        saveg_write32(w, p.bob)?;
        saveg_write32(w, p.health)?;
        saveg_write32(w, p.armorpoints)?;
        saveg_write32(w, p.armortype)?;

        for j in 0..crate::doomdef::NUMPOWERS {
            saveg_write32(w, p.powers[j])?;
        }
        for j in 0..NUMCARDS {
            saveg_write32(w, if p.cards[j] { 1 } else { 0 })?;
        }
        saveg_write32(w, if p.backpack { 1 } else { 0 })?;
        for j in 0..MAXPLAYERS {
            saveg_write32(w, p.frags[j])?;
        }
        saveg_write32(w, p.readyweapon as i32)?;
        saveg_write32(w, p.pendingweapon as i32)?;
        for j in 0..NUMWEAPONS {
            saveg_write32(w, if p.weaponowned[j] { 1 } else { 0 })?;
        }
        for j in 0..NUMAMMO {
            saveg_write32(w, p.ammo[j])?;
        }
        for j in 0..NUMAMMO {
            saveg_write32(w, p.maxammo[j])?;
        }
        saveg_write32(w, p.attackdown)?;
        saveg_write32(w, p.usedown)?;
        saveg_write32(w, p.cheats)?;
        saveg_write32(w, p.refire)?;
        saveg_write32(w, p.killcount)?;
        saveg_write32(w, p.itemcount)?;
        saveg_write32(w, p.secretcount)?;
        saveg_write32(w, 0)?; // message
        saveg_write32(w, p.damagecount)?;
        saveg_write32(w, p.bonuscount)?;
        saveg_write32(w, 0)?; // attacker
        saveg_write32(w, p.extralight)?;
        saveg_write32(w, p.fixedcolormap)?;
        saveg_write32(w, p.colormap)?;

        for j in 0..crate::doomstat::NUMPSPRITES {
            let ps = &p.psprites[j];
            let state_idx = if ps.state.is_null() {
                0
            } else {
                let st = crate::info::states();
                let base = st.as_ptr() as usize;
                let ptr = ps.state as usize;
                ((ptr - base) / std::mem::size_of::<crate::info::types::State>()) as i32
            };
            saveg_write32(w, state_idx)?;
            saveg_write32(w, ps.tics)?;
            saveg_write32(w, ps.sx)?;
            saveg_write32(w, ps.sy)?;
        }
        saveg_write32(w, if p.didsecret { 1 } else { 0 })?;
    }
    Ok(())
    })
}

/// Unarchive players from save stream. Original: P_UnArchivePlayers
pub fn p_unarchive_players<R: Read>(r: &mut R) -> std::io::Result<()> {
    use crate::doomdef::{NUMAMMO, NUMCARDS, NUMWEAPONS};
    use crate::doomstat::{Player, PlayerState, with_doomstat_state};
    use crate::game::d_ticcmd::Ticcmd;
    use crate::info::states;

    with_doomstat_state(|st| {
    for i in 0..MAXPLAYERS {
        if !st.playeringame[i] {
            continue;
        }
        let p = &mut st.players[i];

        let _mo = saveg_read32(r)?;
        p.playerstate = match saveg_read32(r)? {
            0 => PlayerState::Live,
            1 => PlayerState::Dead,
            2 => PlayerState::Reborn,
            _ => PlayerState::Reborn,
        };

        p.cmd.forwardmove = saveg_read8(r)? as i8;
        p.cmd.sidemove = saveg_read8(r)? as i8;
        p.cmd.angleturn = saveg_read16(r)?;
        p.cmd.consistancy = saveg_read8(r)?;
        p.cmd.chatchar = saveg_read8(r)?;
        p.cmd.buttons = saveg_read8(r)?;

        p.viewz = saveg_read32(r)?;
        p.viewheight = saveg_read32(r)?;
        p.deltaviewheight = saveg_read32(r)?;
        p.bob = saveg_read32(r)?;
        p.health = saveg_read32(r)?;
        p.armorpoints = saveg_read32(r)?;
        p.armortype = saveg_read32(r)?;

        for j in 0..crate::doomdef::NUMPOWERS {
            p.powers[j] = saveg_read32(r)?;
        }
        for j in 0..NUMCARDS {
            p.cards[j] = saveg_read32(r)? != 0;
        }
        p.backpack = saveg_read32(r)? != 0;
        for j in 0..MAXPLAYERS {
            p.frags[j] = saveg_read32(r)?;
        }
        let rw = saveg_read32(r)?;
        let pw = saveg_read32(r)?;
        p.readyweapon = unsafe { std::mem::transmute(rw.clamp(0, 8)) };
        p.pendingweapon = unsafe { std::mem::transmute(pw.clamp(0, 9)) };
        for j in 0..NUMWEAPONS {
            p.weaponowned[j] = saveg_read32(r)? != 0;
        }
        for j in 0..NUMAMMO {
            p.ammo[j] = saveg_read32(r)?;
        }
        for j in 0..NUMAMMO {
            p.maxammo[j] = saveg_read32(r)?;
        }
        p.attackdown = saveg_read32(r)?;
        p.usedown = saveg_read32(r)?;
        p.cheats = saveg_read32(r)?;
        p.refire = saveg_read32(r)?;
        p.killcount = saveg_read32(r)?;
        p.itemcount = saveg_read32(r)?;
        p.secretcount = saveg_read32(r)?;
        let _msg = saveg_read32(r)?;
        p.damagecount = saveg_read32(r)?;
        p.bonuscount = saveg_read32(r)?;
        let _attacker = saveg_read32(r)?;
        p.extralight = saveg_read32(r)?;
        p.fixedcolormap = saveg_read32(r)?;
        p.colormap = saveg_read32(r)?;

        for j in 0..crate::doomstat::NUMPSPRITES {
            let state_idx = saveg_read32(r)?;
            p.psprites[j].state = if state_idx > 0 && (state_idx as usize) < crate::info::NUMSTATES {
                unsafe {
                    crate::info::states().as_ptr().add(state_idx as usize) as *mut std::ffi::c_void
                }
            } else {
                std::ptr::null_mut()
            };
            p.psprites[j].tics = saveg_read32(r)?;
            p.psprites[j].sx = saveg_read32(r)?;
            p.psprites[j].sy = saveg_read32(r)?;
        }
        p.didsecret = saveg_read32(r)? != 0;

        p.mo = std::ptr::null_mut();
        p.message = None;
        p.attacker = std::ptr::null_mut();
    }
    Ok(())
}

/// Archive world (sectors, lines, sides). Original: P_ArchiveWorld
pub fn p_archive_world<W: Write>(w: &mut W) -> std::io::Result<()> {
    use crate::m_fixed::FRACBITS;
    let (sectors, numsectors, lines, numlines, sides) = crate::rendering::state::with_state(|s| {
        (s.sectors, s.numsectors as usize, s.lines, s.numlines as usize, s.sides)
    });

    if sectors.is_null() || lines.is_null() {
        return Ok(());
    }

    for i in 0..numsectors {
        let sec = unsafe { &*sectors.add(i) };
        saveg_write16(w, (sec.floorheight >> FRACBITS) as i16)?;
        saveg_write16(w, (sec.ceilingheight >> FRACBITS) as i16)?;
        saveg_write16(w, sec.floorpic)?;
        saveg_write16(w, sec.ceilingpic)?;
        saveg_write16(w, sec.lightlevel)?;
        saveg_write16(w, sec.special)?;
        saveg_write16(w, sec.tag)?;
    }

    for i in 0..numlines {
        let li = unsafe { &*lines.add(i) };
        saveg_write16(w, li.flags)?;
        saveg_write16(w, li.special)?;
        saveg_write16(w, li.tag)?;
        for j in 0..2 {
            if li.sidenum[j] == -1 {
                continue;
            }
            let si = unsafe { &*sides.add(li.sidenum[j] as usize) };
            saveg_write16(w, (si.textureoffset >> FRACBITS) as i16)?;
            saveg_write16(w, (si.rowoffset >> FRACBITS) as i16)?;
            saveg_write16(w, si.toptexture)?;
            saveg_write16(w, si.bottomtexture)?;
            saveg_write16(w, si.midtexture)?;
        }
    }
    Ok(())
}

/// Unarchive world. Original: P_UnArchiveWorld
pub fn p_unarchive_world<R: Read>(r: &mut R) -> std::io::Result<()> {
    use crate::m_fixed::FRACBITS;
    let (sectors, numsectors, lines, numlines, sides) = crate::rendering::state::with_state(|s| {
        (s.sectors, s.numsectors as usize, s.lines, s.numlines as usize, s.sides)
    });

    if sectors.is_null() || lines.is_null() {
        return Ok(());
    }

    for i in 0..numsectors {
        let sec = unsafe { &mut *sectors.add(i) };
        sec.floorheight = (saveg_read16(r)? as i32) << FRACBITS;
        sec.ceilingheight = (saveg_read16(r)? as i32) << FRACBITS;
        sec.floorpic = saveg_read16(r)?;
        sec.ceilingpic = saveg_read16(r)?;
        sec.lightlevel = saveg_read16(r)?;
        sec.special = saveg_read16(r)?;
        sec.tag = saveg_read16(r)?;
        sec.specialdata = std::ptr::null_mut();
        sec.soundtarget = std::ptr::null_mut::<crate::player::p_mobj::Mobj>();
    }

    for i in 0..numlines {
        let li = unsafe { &mut *lines.add(i) };
        li.flags = saveg_read16(r)?;
        li.special = saveg_read16(r)?;
        li.tag = saveg_read16(r)?;
        for j in 0..2 {
            if li.sidenum[j] == -1 {
                continue;
            }
            let si = unsafe { &mut *sides.add(li.sidenum[j] as usize) };
            si.textureoffset = (saveg_read16(r)? as i32) << FRACBITS;
            si.rowoffset = (saveg_read16(r)? as i32) << FRACBITS;
            si.toptexture = saveg_read16(r)?;
            si.bottomtexture = saveg_read16(r)?;
            si.midtexture = saveg_read16(r)?;
        }
    }
    Ok(())
}

const TC_END: u8 = 0;
const TC_MOBJ: u8 = 1;

/// Archive thinkers (mobjs). Original: P_ArchiveThinkers
pub fn p_archive_thinkers<W: Write>(w: &mut W) -> std::io::Result<()> {
    use crate::doomdata::MapThing;
    use crate::game::d_think::Thinker;
    use crate::info::{states, MOBJINFO};
    use crate::player::p_mobj::p_mobj_thinker;
    use crate::player::p_tick::with_ptick_state;

    with_ptick_state(|s| {
        let cap = &s.thinkercap as *const Thinker as *mut Thinker;
        let mut th = s.thinkercap.next;

        while !th.is_null() && th != cap {
            let next = unsafe { (*th).next };
            if unsafe { (*th).function.acp1 } == p_mobj_thinker {
                let mo = th as *mut crate::player::p_mobj::Mobj;
                saveg_write8(w, TC_MOBJ)?;

                // thinker (prev, next - we skip, just write 0)
                saveg_write32(w, 0)?;
                saveg_write32(w, 0)?;

                let m = unsafe { &*mo };
                saveg_write32(w, m.x)?;
                saveg_write32(w, m.y)?;
                saveg_write32(w, m.z)?;
                saveg_write32(w, 0)?; // snext
                saveg_write32(w, 0)?; // sprev
                saveg_write32(w, m.angle as i32)?;
                saveg_write32(w, m.sprite)?;
                saveg_write32(w, m.frame)?;
                saveg_write32(w, 0)?; // bnext
                saveg_write32(w, 0)?; // bprev
                saveg_write32(w, 0)?; // subsector
                saveg_write32(w, m.floorz)?;
                saveg_write32(w, m.ceilingz)?;
                saveg_write32(w, m.radius)?;
                saveg_write32(w, m.height)?;
                saveg_write32(w, m.momx)?;
                saveg_write32(w, m.momy)?;
                saveg_write32(w, m.momz)?;
                saveg_write32(w, m.validcount)?;
                saveg_write32(w, m.type_ as i32)?;
                saveg_write32(w, 0)?; // info
                saveg_write32(w, m.tics)?;
                let state_idx = if m.state.is_null() {
                    0
                } else {
                    let st = states();
                    let base = st.as_ptr() as usize;
                    let ptr = m.state as usize;
                    ((ptr - base) / std::mem::size_of::<crate::info::types::State>()) as i32
                };
                saveg_write32(w, state_idx)?;
                saveg_write32(w, m.flags)?;
                saveg_write32(w, m.health)?;
                saveg_write32(w, m.movedir)?;
                saveg_write32(w, m.movecount)?;
                saveg_write32(w, 0)?; // target
                saveg_write32(w, m.reactiontime)?;
                saveg_write32(w, m.threshold)?;
                // player index (1-based)
                let pl_idx = if m.player.is_null() {
                    0
                } else {
                    unsafe {
                        let players = crate::doomstat::PLAYERS.as_ptr();
                        let p = m.player as *const crate::doomstat::Player;
                        (p as usize - players as usize) / std::mem::size_of::<crate::doomstat::Player>()
                            + 1
                    }
                };
                saveg_write32(w, pl_idx as i32)?;
                saveg_write32(w, m.lastlook)?;
                saveg_write16(w, m.spawnpoint.x)?;
                saveg_write16(w, m.spawnpoint.y)?;
                saveg_write16(w, m.spawnpoint.angle)?;
                saveg_write16(w, m.spawnpoint.type_)?;
                saveg_write16(w, m.spawnpoint.options)?;
                saveg_write32(w, 0)?; // tracer
            }
            th = next;
        }
        Ok(())
    })?;
    saveg_write8(w, TC_END)?;
    Ok(())
}

/// Unarchive thinkers. Original: P_UnArchiveThinkers
pub fn p_unarchive_thinkers<R: Read>(r: &mut R) -> std::io::Result<()> {
    use crate::doomstat::with_doomstat_state;
    use crate::game::d_think::Thinker;
    use crate::info::{states, MOBJINFO, NUMMOBJTYPES};
    use crate::player::p_mobj::{p_mobj_thinker, Mobj};
    use crate::player::p_maputl::p_set_thing_position;
    use crate::player::p_tick::{p_add_thinker, p_init_thinkers, with_ptick_state};
    use crate::z_zone::{z_free, z_malloc, PU_LEVEL};

    // Remove all current thinkers (unlink and free immediately)
    with_ptick_state(|s| {
        let cap = &mut s.thinkercap as *mut Thinker;
        let mut current = s.thinkercap.next;

        while !current.is_null() && current != cap {
            let next = unsafe { (*current).next };
            unsafe {
                (*(*current).prev).next = (*current).next;
                (*(*current).next).prev = (*current).prev;
            }
            if unsafe { (*current).function.acp1 } == p_mobj_thinker {
                super::p_maputl::p_unset_thing_position(current as *mut Mobj);
            }
            z_free(current as *mut u8);
            current = next;
        }
    });
    p_init_thinkers();

    // Read saved thinkers
    loop {
        let tclass = saveg_read8(r)?;
        if tclass == TC_END {
            return Ok(());
        }
        if tclass == TC_MOBJ {
            let _prev = saveg_read32(r)?;
            let _next = saveg_read32(r)?;

            let ptr = z_malloc(std::mem::size_of::<Mobj>(), PU_LEVEL, std::ptr::null_mut())
                as *mut Mobj;
            if ptr.is_null() {
                crate::i_system::i_error("P_UnArchiveThinkers: out of memory");
            }

            unsafe {
                (*ptr).x = saveg_read32(r)?;
                (*ptr).y = saveg_read32(r)?;
                (*ptr).z = saveg_read32(r)?;
                (*ptr).snext = std::ptr::null_mut();
                (*ptr).sprev = std::ptr::null_mut();
                (*ptr).angle = saveg_read32(r)? as u32;
                (*ptr).sprite = saveg_read32(r)?;
                (*ptr).frame = saveg_read32(r)?;
                (*ptr).bnext = std::ptr::null_mut();
                (*ptr).bprev = std::ptr::null_mut();
                (*ptr).subsector = std::ptr::null_mut();
                (*ptr).floorz = saveg_read32(r)?;
                (*ptr).ceilingz = saveg_read32(r)?;
                (*ptr).radius = saveg_read32(r)?;
                (*ptr).height = saveg_read32(r)?;
                (*ptr).momx = saveg_read32(r)?;
                (*ptr).momy = saveg_read32(r)?;
                (*ptr).momz = saveg_read32(r)?;
                (*ptr).validcount = saveg_read32(r)?;
                let type_ = saveg_read32(r)?;
                (*ptr).type_ = std::mem::transmute(type_.clamp(0, NUMMOBJTYPES as i32 - 1));
                (*ptr).info = &MOBJINFO[(*ptr).type_ as usize] as *const _;
                (*ptr).tics = saveg_read32(r)?;
                let state_idx = saveg_read32(r)?;
                (*ptr).state = if state_idx >= 0 && (state_idx as usize) < crate::info::NUMSTATES {
                    states().as_ptr().add(state_idx as usize)
                } else {
                    crate::info::states().as_ptr()
                };
                (*ptr).flags = saveg_read32(r)?;
                (*ptr).health = saveg_read32(r)?;
                (*ptr).movedir = saveg_read32(r)?;
                (*ptr).movecount = saveg_read32(r)?;
                (*ptr).target = std::ptr::null_mut();
                (*ptr).reactiontime = saveg_read32(r)?;
                (*ptr).threshold = saveg_read32(r)?;
                let pl = saveg_read32(r)?;
                if pl > 0 && (pl as usize) <= MAXPLAYERS {
                    with_doomstat_state(|st| {
                        let p = &mut st.players[pl as usize - 1];
                        (*ptr).player = p as *mut crate::doomstat::Player as *mut std::ffi::c_void;
                        p.mo = ptr as *mut std::ffi::c_void;
                    });
                } else {
                    (*ptr).player = std::ptr::null_mut();
                }
                (*ptr).lastlook = saveg_read32(r)?;
                (*ptr).spawnpoint.x = saveg_read16(r)?;
                (*ptr).spawnpoint.y = saveg_read16(r)?;
                (*ptr).spawnpoint.angle = saveg_read16(r)?;
                (*ptr).spawnpoint.type_ = saveg_read16(r)?;
                (*ptr).spawnpoint.options = saveg_read16(r)?;
                (*ptr).tracer = std::ptr::null_mut();

                (*ptr).thinker.prev = std::ptr::null_mut();
                (*ptr).thinker.next = std::ptr::null_mut();
                (*ptr).thinker.function.acp1 = p_mobj_thinker;
            }

            p_set_thing_position(ptr);
            unsafe {
                (*ptr).floorz = {
                    let ss = (*ptr).subsector.cast::<crate::rendering::defs::Subsector>();
                    if ss.is_null() {
                        (*ptr).floorz
                    } else {
                        (*(*ss).sector).floorheight
                    }
                };
                (*ptr).ceilingz = {
                    let ss = (*ptr).subsector.cast::<crate::rendering::defs::Subsector>();
                    if ss.is_null() {
                        (*ptr).ceilingz
                    } else {
                        (*(*ss).sector).ceilingheight
                    }
                };
            }
            p_add_thinker(unsafe { &mut (*ptr).thinker as *mut Thinker });
        } else {
            crate::i_system::i_error(&format!("Unknown tclass {} in savegame", tclass));
        }
    }
}

const TC_CEILING: u8 = 0;
const TC_DOOR: u8 = 1;
const TC_FLOOR: u8 = 2;
const TC_PLAT: u8 = 3;
const TC_FLASH: u8 = 4;
const TC_STROBE: u8 = 5;
const TC_GLOW: u8 = 6;
const TC_ENDSPECIALS: u8 = 7;

fn sector_index(sector: *mut crate::rendering::defs::Sector) -> i32 {
    let (sectors, numsectors) = crate::rendering::state::with_state(|s| (s.sectors, s.numsectors as usize));
    if sectors.is_null() || numsectors == 0 {
        return 0;
    }
    let base = sectors as usize;
    let ptr = sector as usize;
    let idx = (ptr - base) / std::mem::size_of::<crate::rendering::defs::Sector>();
    if idx < numsectors {
        idx as i32
    } else {
        0
    }
}

fn sector_from_index(idx: i32) -> *mut crate::rendering::defs::Sector {
    let (sectors, numsectors) = crate::rendering::state::with_state(|s| (s.sectors, s.numsectors));
    if sectors.is_null() || idx < 0 || idx >= numsectors {
        std::ptr::null_mut()
    } else {
        unsafe { sectors.add(idx as usize) }
    }
}

/// Archive specials (ceiling, floor, door, etc.). Original: P_ArchiveSpecials
pub fn p_archive_specials<W: Write>(w: &mut W) -> std::io::Result<()> {
    use crate::player::p_ceilng::{t_move_ceiling, CeilingMover};
    use crate::player::p_doors::{t_vertical_door, Vldoor};
    use crate::player::p_floor::{t_move_floor, FloorMover};
    use crate::player::p_lights::{t_light_flash, t_strobe_flash, t_glow, Glow, LightFlash, Strobe};
    use crate::player::p_plats::{t_plat_raise, Plat};
    use crate::player::p_tick::with_ptick_state;
    use crate::game::d_think::Thinker;

    with_ptick_state(|s| {
        let cap = &s.thinkercap as *const Thinker as *mut Thinker;
        let mut th = s.thinkercap.next;

        while !th.is_null() && th != cap {
            let next = unsafe { (*th).next };
            let acp1 = unsafe { (*th).function.acp1 };

            if acp1 == t_move_ceiling {
                let c = th as *mut CeilingMover;
                let m = unsafe { &*c };
                saveg_write8(w, TC_CEILING)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, m.ceilingtype)?;
                saveg_write32(w, sector_index(m.sector))?;
                saveg_write32(w, m.bottomheight)?;
                saveg_write32(w, m.topheight)?;
                saveg_write32(w, m.speed)?;
                saveg_write32(w, if m.crush { 1 } else { 0 })?;
                saveg_write32(w, m.direction)?;
                saveg_write32(w, m.tag)?;
                saveg_write32(w, m.olddirection)?;
            } else if acp1 == t_vertical_door {
                let d = th as *mut Vldoor;
                let m = unsafe { &*d };
                saveg_write8(w, TC_DOOR)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, m.doortype)?;
                saveg_write32(w, sector_index(m.sector))?;
                saveg_write32(w, m.topheight)?;
                saveg_write32(w, m.speed)?;
                saveg_write32(w, m.direction)?;
                saveg_write32(w, m.topwait)?;
                saveg_write32(w, m.topcountdown)?;
            } else if acp1 == t_move_floor {
                let f = th as *mut FloorMover;
                let m = unsafe { &*f };
                saveg_write8(w, TC_FLOOR)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, m.floortype)?;
                saveg_write32(w, if m.crush { 1 } else { 0 })?;
                saveg_write32(w, sector_index(m.sector))?;
                saveg_write32(w, m.direction)?;
                saveg_write32(w, m.newspecial)?;
                saveg_write16(w, m.texture)?;
                saveg_write32(w, m.floordestheight)?;
                saveg_write32(w, m.speed)?;
            } else if acp1 == t_plat_raise {
                let p = th as *mut Plat;
                let m = unsafe { &*p };
                saveg_write8(w, TC_PLAT)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, sector_index(m.sector))?;
                saveg_write32(w, m.speed)?;
                saveg_write32(w, m.low)?;
                saveg_write32(w, m.high)?;
                saveg_write32(w, m.wait)?;
                saveg_write32(w, m.count)?;
                saveg_write32(w, m.status)?;
                saveg_write32(w, m.oldstatus)?;
                saveg_write32(w, if m.crush { 1 } else { 0 })?;
                saveg_write32(w, m.tag)?;
                saveg_write32(w, m.plattype)?;
            } else if acp1 == t_light_flash {
                let f = th as *mut LightFlash;
                let m = unsafe { &*f };
                saveg_write8(w, TC_FLASH)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, sector_index(m.sector))?;
                saveg_write32(w, m.count)?;
                saveg_write32(w, m.maxlight)?;
                saveg_write32(w, m.minlight)?;
                saveg_write32(w, m.maxtime)?;
                saveg_write32(w, m.mintime)?;
            } else if acp1 == t_strobe_flash {
                let strobe = th as *mut Strobe;
                let m = unsafe { &*strobe };
                saveg_write8(w, TC_STROBE)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, sector_index(m.sector))?;
                saveg_write32(w, m.count)?;
                saveg_write32(w, m.minlight)?;
                saveg_write32(w, m.maxlight)?;
                saveg_write32(w, m.darktime)?;
                saveg_write32(w, m.brighttime)?;
            } else if acp1 == t_glow {
                let g = th as *mut Glow;
                let m = unsafe { &*g };
                saveg_write8(w, TC_GLOW)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, 0)?;
                saveg_write32(w, sector_index(m.sector))?;
                saveg_write32(w, m.minlight)?;
                saveg_write32(w, m.maxlight)?;
                saveg_write32(w, m.direction)?;
            }
            th = next;
        }
    });
    saveg_write8(w, TC_ENDSPECIALS)?;
    Ok(())
}

/// Unarchive specials. Original: P_UnArchiveSpecials
pub fn p_unarchive_specials<R: Read>(r: &mut R) -> std::io::Result<()> {
    use crate::game::d_think::Thinker;
    use crate::player::p_ceilng::{p_add_active_ceiling, t_move_ceiling, CeilingMover};
    use crate::player::p_doors::{t_vertical_door, Vldoor};
    use crate::player::p_floor::{t_move_floor, FloorMover};
    use crate::player::p_lights::{t_light_flash, t_strobe_flash, t_glow, Glow, LightFlash, Strobe};
    use crate::player::p_plats::{p_add_active_plat, t_plat_raise, Plat};
    use crate::player::p_tick::p_add_thinker;
    use crate::z_zone::{z_malloc, PU_LEVSPEC};

    loop {
        let tclass = saveg_read8(r)?;
        if tclass == TC_ENDSPECIALS {
            return Ok(());
        }
        let _prev = saveg_read32(r)?;
        let _next = saveg_read32(r)?;

        match tclass {
            TC_CEILING => {
                let ptr = z_malloc(std::mem::size_of::<CeilingMover>(), PU_LEVSPEC, std::ptr::null_mut())
                    as *mut CeilingMover;
                if ptr.is_null() {
                    crate::i_system::i_error("P_UnArchiveSpecials: out of memory");
                }
                unsafe {
                    (*ptr).ceilingtype = saveg_read32(r)?;
                    (*ptr).sector = sector_from_index(saveg_read32(r)?);
                    (*ptr).bottomheight = saveg_read32(r)?;
                    (*ptr).topheight = saveg_read32(r)?;
                    (*ptr).speed = saveg_read32(r)?;
                    (*ptr).crush = saveg_read32(r)? != 0;
                    (*ptr).direction = saveg_read32(r)?;
                    (*ptr).tag = saveg_read32(r)?;
                    (*ptr).olddirection = saveg_read32(r)?;
                    (*ptr).thinker.prev = std::ptr::null_mut();
                    (*ptr).thinker.next = std::ptr::null_mut();
                    (*ptr).thinker.function.acp1 = t_move_ceiling;
                }
                if !unsafe { (*ptr).sector }.is_null() {
                    unsafe { (*(*ptr).sector).specialdata = ptr as *mut std::ffi::c_void };
                }
                p_add_thinker(unsafe { &mut (*ptr).thinker as *mut Thinker });
                p_add_active_ceiling(ptr);
            }
            TC_DOOR => {
                let ptr = z_malloc(std::mem::size_of::<Vldoor>(), PU_LEVSPEC, std::ptr::null_mut())
                    as *mut Vldoor;
                if ptr.is_null() {
                    crate::i_system::i_error("P_UnArchiveSpecials: out of memory");
                }
                unsafe {
                    (*ptr).doortype = saveg_read32(r)?;
                    (*ptr).sector = sector_from_index(saveg_read32(r)?);
                    (*ptr).topheight = saveg_read32(r)?;
                    (*ptr).speed = saveg_read32(r)?;
                    (*ptr).direction = saveg_read32(r)?;
                    (*ptr).topwait = saveg_read32(r)?;
                    (*ptr).topcountdown = saveg_read32(r)?;
                    (*ptr).thinker.prev = std::ptr::null_mut();
                    (*ptr).thinker.next = std::ptr::null_mut();
                    (*ptr).thinker.function.acp1 = t_vertical_door;
                }
                if !unsafe { (*ptr).sector }.is_null() {
                    unsafe { (*(*ptr).sector).specialdata = ptr as *mut std::ffi::c_void };
                }
                p_add_thinker(unsafe { &mut (*ptr).thinker as *mut Thinker });
            }
            TC_FLOOR => {
                let ptr = z_malloc(std::mem::size_of::<FloorMover>(), PU_LEVSPEC, std::ptr::null_mut())
                    as *mut FloorMover;
                if ptr.is_null() {
                    crate::i_system::i_error("P_UnArchiveSpecials: out of memory");
                }
                unsafe {
                    (*ptr).floortype = saveg_read32(r)?;
                    (*ptr).crush = saveg_read32(r)? != 0;
                    (*ptr).sector = sector_from_index(saveg_read32(r)?);
                    (*ptr).direction = saveg_read32(r)?;
                    (*ptr).newspecial = saveg_read32(r)?;
                    (*ptr).texture = saveg_read16(r)?;
                    (*ptr).floordestheight = saveg_read32(r)?;
                    (*ptr).speed = saveg_read32(r)?;
                    (*ptr).thinker.prev = std::ptr::null_mut();
                    (*ptr).thinker.next = std::ptr::null_mut();
                    (*ptr).thinker.function.acp1 = t_move_floor;
                }
                if !unsafe { (*ptr).sector }.is_null() {
                    unsafe { (*(*ptr).sector).specialdata = ptr as *mut std::ffi::c_void };
                }
                p_add_thinker(unsafe { &mut (*ptr).thinker as *mut Thinker });
            }
            TC_PLAT => {
                let ptr = z_malloc(std::mem::size_of::<Plat>(), PU_LEVSPEC, std::ptr::null_mut())
                    as *mut Plat;
                if ptr.is_null() {
                    crate::i_system::i_error("P_UnArchiveSpecials: out of memory");
                }
                unsafe {
                    (*ptr).sector = sector_from_index(saveg_read32(r)?);
                    (*ptr).speed = saveg_read32(r)?;
                    (*ptr).low = saveg_read32(r)?;
                    (*ptr).high = saveg_read32(r)?;
                    (*ptr).wait = saveg_read32(r)?;
                    (*ptr).count = saveg_read32(r)?;
                    (*ptr).status = saveg_read32(r)?;
                    (*ptr).oldstatus = saveg_read32(r)?;
                    (*ptr).crush = saveg_read32(r)? != 0;
                    (*ptr).tag = saveg_read32(r)?;
                    (*ptr).plattype = saveg_read32(r)?;
                    (*ptr).thinker.prev = std::ptr::null_mut();
                    (*ptr).thinker.next = std::ptr::null_mut();
                    (*ptr).thinker.function.acp1 = t_plat_raise;
                }
                if !unsafe { (*ptr).sector }.is_null() {
                    unsafe { (*(*ptr).sector).specialdata = ptr as *mut std::ffi::c_void };
                }
                p_add_thinker(unsafe { &mut (*ptr).thinker as *mut Thinker });
                p_add_active_plat(ptr);
            }
            TC_FLASH => {
                let ptr = z_malloc(std::mem::size_of::<LightFlash>(), PU_LEVSPEC, std::ptr::null_mut())
                    as *mut LightFlash;
                if ptr.is_null() {
                    crate::i_system::i_error("P_UnArchiveSpecials: out of memory");
                }
                unsafe {
                    (*ptr).sector = sector_from_index(saveg_read32(r)?);
                    (*ptr).count = saveg_read32(r)?;
                    (*ptr).maxlight = saveg_read32(r)?;
                    (*ptr).minlight = saveg_read32(r)?;
                    (*ptr).maxtime = saveg_read32(r)?;
                    (*ptr).mintime = saveg_read32(r)?;
                    (*ptr).thinker.prev = std::ptr::null_mut();
                    (*ptr).thinker.next = std::ptr::null_mut();
                    (*ptr).thinker.function.acp1 = t_light_flash;
                }
                p_add_thinker(unsafe { &mut (*ptr).thinker as *mut Thinker });
            }
            TC_STROBE => {
                let ptr = z_malloc(std::mem::size_of::<Strobe>(), PU_LEVSPEC, std::ptr::null_mut())
                    as *mut Strobe;
                if ptr.is_null() {
                    crate::i_system::i_error("P_UnArchiveSpecials: out of memory");
                }
                unsafe {
                    (*ptr).sector = sector_from_index(saveg_read32(r)?);
                    (*ptr).count = saveg_read32(r)?;
                    (*ptr).minlight = saveg_read32(r)?;
                    (*ptr).maxlight = saveg_read32(r)?;
                    (*ptr).darktime = saveg_read32(r)?;
                    (*ptr).brighttime = saveg_read32(r)?;
                    (*ptr).thinker.prev = std::ptr::null_mut();
                    (*ptr).thinker.next = std::ptr::null_mut();
                    (*ptr).thinker.function.acp1 = t_strobe_flash;
                }
                p_add_thinker(unsafe { &mut (*ptr).thinker as *mut Thinker });
            }
            TC_GLOW => {
                let ptr = z_malloc(std::mem::size_of::<Glow>(), PU_LEVSPEC, std::ptr::null_mut())
                    as *mut Glow;
                if ptr.is_null() {
                    crate::i_system::i_error("P_UnArchiveSpecials: out of memory");
                }
                unsafe {
                    (*ptr).sector = sector_from_index(saveg_read32(r)?);
                    (*ptr).minlight = saveg_read32(r)?;
                    (*ptr).maxlight = saveg_read32(r)?;
                    (*ptr).direction = saveg_read32(r)?;
                    (*ptr).thinker.prev = std::ptr::null_mut();
                    (*ptr).thinker.next = std::ptr::null_mut();
                    (*ptr).thinker.function.acp1 = t_glow;
                }
                p_add_thinker(unsafe { &mut (*ptr).thinker as *mut Thinker });
            }
            _ => {
                crate::i_system::i_error(&format!(
                    "P_UnArchiveSpecials: Unknown tclass {} in savegame",
                    tclass
                ));
            }
        }
    }
}
