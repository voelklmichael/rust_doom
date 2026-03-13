//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// DESCRIPTION:
//	Handles WAD file header, directory, lump I/O.
//
// Original: w_wad.h (public) + w_wad.c (private)

use crate::game::d_iwad;
use crate::game::d_mode::GameMission;
use crate::i_system;
use crate::m_misc;
use crate::z_zone::{z_change_tag, z_free, z_malloc, PU_CACHE, PU_STATIC};
use crate::wad::w_file::WadFile;
use std::cell::RefCell;

// --- Public types (from w_wad.h) ---

/// Lump info - corresponds to lumpinfo_t
#[derive(Clone)]
pub struct LumpInfo {
    pub name: [u8; 8],
    pub wad_file_index: usize,
    pub position: i32,
    pub size: i32,
    pub cache: Option<*mut u8>,
    pub next: Option<usize>, // index for hash chain
}

// Global WAD state - thread_local for single-threaded access (avoids Sync requirement)
thread_local! {
    static WAD_STATE: RefCell<WadState> = RefCell::new(WadState {
        lumpinfo: Vec::new(),
        lumphash: None,
        open_wad_files: Vec::new(),
    });
}

struct WadState {
    lumpinfo: Vec<LumpInfo>,
    lumphash: Option<Vec<Option<usize>>>, // hash table: bucket -> first lump index
    open_wad_files: Vec<WadFile>,
}

fn wad_state_read<F, R>(f: F) -> R
where
    F: FnOnce(&WadState) -> R,
{
    WAD_STATE.with(|state| f(&state.borrow()))
}

fn wad_state_write<F, R>(f: F) -> R
where
    F: FnOnce(&mut WadState) -> R,
{
    WAD_STATE.with(|state| f(&mut state.borrow_mut()))
}

/// Hash function for lump names. Original: W_LumpNameHash
pub fn w_lump_name_hash(s: &str) -> u32 {
    let mut result: u32 = 5381;
    for c in s.bytes().take(8) {
        if c == 0 {
            break;
        }
        let up = if (b'a'..=b'z').contains(&c) { c - 32 } else { c };
        result = ((result << 5) ^ result) ^ up as u32;
    }
    result
}

fn lump_name_matches(lump_name: &[u8; 8], name: &str) -> bool {
    let name_bytes = name.as_bytes();
    for i in 0..8 {
        let a = lump_name.get(i).copied().unwrap_or(0);
        let b = name_bytes.get(i).copied().unwrap_or(0);
        let a_up = if (b'a'..=b'z').contains(&a) { a - 32 } else { a };
        let b_up = if (b'a'..=b'z').contains(&b) { b - 32 } else { b };
        if a_up != b_up {
            return false;
        }
        if a == 0 {
            return true;
        }
    }
    name_bytes.len() <= 8
}

/// Original: W_AddFile
pub fn w_add_file(filename: &str) -> Option<usize> {
    let wad_file = crate::wad::w_file::w_open_file(filename)?;
    wad_state_write(|state| {
        let wad_index = state.open_wad_files.len();
        state.open_wad_files.push(wad_file);

        let _newnumlumps = if filename.len() >= 3
        && filename[filename.len() - 3..].eq_ignore_ascii_case("wad")
    {
        // WAD file - read header and directory
        let mut header_buf = [0u8; 12];
        let wad = &state.open_wad_files[wad_index];
        let n = wad.read(0, &mut header_buf);
        if n < 12 {
            return None as Option<usize>;
        }
        let id = std::str::from_utf8(&header_buf[0..4]).unwrap_or("");
        if id != "IWAD" && id != "PWAD" {
            i_system::i_error(&format!(
                "Wad file {} doesn't have IWAD or PWAD id",
                filename
            ));
        }
        let numlumps = i32::from_le_bytes(header_buf[4..8].try_into().unwrap());
        let infotableofs = i32::from_le_bytes(header_buf[8..12].try_into().unwrap());

        let length = (numlumps as usize) * 16; // sizeof(filelump_t) = 16
        let fileinfo = unsafe {
            z_malloc(length, PU_STATIC, std::ptr::null_mut()) as *mut u8
        };
        let n = state.open_wad_files[wad_index].read(infotableofs as u32, unsafe {
            std::slice::from_raw_parts_mut(fileinfo, length)
        });
        if n < length {
            z_free(fileinfo);
            return None;
        }

        let startlump = state.lumpinfo.len();
        let new_count = startlump + numlumps as usize;
        extend_lump_info(state, new_count);

        for i in 0..numlumps as usize {
            let offset = i * 16;
            let filepos = i32::from_le_bytes(
                unsafe { *(fileinfo.add(offset) as *const [u8; 4]) }
            );
            let size = i32::from_le_bytes(
                unsafe { *(fileinfo.add(offset + 4) as *const [u8; 4]) }
            );
            let mut name = [0u8; 8];
            name.copy_from_slice(unsafe {
                &*(fileinfo.add(offset + 8) as *const [u8; 8])
            });

            state.lumpinfo[startlump + i] = LumpInfo {
                name,
                wad_file_index: wad_index,
                position: filepos,
                size,
                cache: None,
                next: None,
            };
        }
        z_free(fileinfo);
        new_count
    } else {
        // Single lump file
        let mut name = [0u8; 8];
        m_misc::m_extract_file_base(filename, &mut name);
        let wad = &state.open_wad_files[wad_index];
        let length = wad.length as i32;

        let startlump = state.lumpinfo.len();
        extend_lump_info(state, startlump + 1);

        state.lumpinfo[startlump] = LumpInfo {
            name,
            wad_file_index: wad_index,
            position: 0,
            size: length,
            cache: None,
            next: None,
        };
        startlump + 1
    };

        state.lumphash = None;
        Some(wad_index)
    })
}

fn extend_lump_info(state: &mut WadState, newnumlumps: usize) {
    state.lumpinfo.resize(newnumlumps, LumpInfo {
        name: [0; 8],
        wad_file_index: 0,
        position: 0,
        size: 0,
        cache: None,
        next: None,
    });
}

/// Original: W_NumLumps
pub fn w_num_lumps() -> i32 {
    wad_state_read(|state| state.lumpinfo.len() as i32)
}

/// Original: W_CheckNumForName - returns -1 if not found
pub fn w_check_num_for_name(name: &str) -> i32 {
    wad_state_read(|state| {
    let numlumps = state.lumpinfo.len();
    if numlumps == 0 {
        return -1;
    }

    let name_bytes = name.as_bytes();
    let cmp_len = name_bytes.len().min(8);

    if let Some(ref lumphash) = state.lumphash {
        let hash = (w_lump_name_hash(name) as usize) % numlumps;
        let mut idx = lumphash[hash];
        while let Some(i) = idx {
            if i < state.lumpinfo.len() {
                let lump = &state.lumpinfo[i];
                if lump_name_matches(&lump.name, name) {
                    return i as i32;
                }
                idx = lump.next;
            } else {
                break;
            }
        }
    } else {
        for i in (0..numlumps).rev() {
            let lump = &state.lumpinfo[i];
            if lump_name_matches(&lump.name, name) {
                return i as i32;
            }
        }
    }
    -1
    })
}

/// Original: W_GetNumForName
pub fn w_get_num_for_name(name: &str) -> i32 {
    let i = w_check_num_for_name(name);
    if i < 0 {
        i_system::i_error(&format!("W_GetNumForName: {} not found!", name));
    }
    i
}

/// Original: W_LumpLength
pub fn w_lump_length(lump: usize) -> i32 {
    wad_state_read(|state| {
    if lump >= state.lumpinfo.len() {
        i_system::i_error(&format!("W_LumpLength: {} >= numlumps", lump));
    }
    state.lumpinfo[lump].size
    })
}

/// Original: W_ReadLump
pub fn w_read_lump(lump: usize, dest: &mut [u8]) {
    let (wad_index, position, size) = wad_state_read(|state| {
        if lump >= state.lumpinfo.len() {
            i_system::i_error(&format!("W_ReadLump: {} >= numlumps", lump));
        }
        let lump_info = &state.lumpinfo[lump];
        (lump_info.wad_file_index, lump_info.position, lump_info.size as usize)
    });
    i_system::i_begin_read();
    let n = wad_state_read(|state| {
        let wad = &state.open_wad_files[wad_index];
        wad.read(position as u32, dest)
    });
    i_system::i_end_read();
    if n < size {
        i_system::i_error(&format!(
            "W_ReadLump: only read {} of {} on lump {}",
            n, size, lump
        ));
    }
}

/// Original: W_CacheLumpNum
pub fn w_cache_lump_num(lumpnum: i32, tag: i32) -> *mut u8 {
    let (wad_index, position, size, has_mapped) = wad_state_read(|state| {
        let numlumps = state.lumpinfo.len();
        if (lumpnum as usize) >= numlumps {
            i_system::i_error(&format!("W_CacheLumpNum: {} >= numlumps", lumpnum));
        }
        let lump = &state.lumpinfo[lumpnum as usize];
        let wad = &state.open_wad_files[lump.wad_file_index];
        (
            lump.wad_file_index,
            lump.position,
            lump.size as usize,
            wad.mapped.is_some(),
        )
    });

    let cache_opt = wad_state_read(|state| {
        let lump = &state.lumpinfo[lumpnum as usize];
        lump.cache
    });

    if has_mapped {
        wad_state_read(|state| {
            let wad = &state.open_wad_files[wad_index];
            unsafe { wad.mapped.unwrap().add(position as usize) }
        })
    } else if let Some(cache) = cache_opt {
        z_change_tag(cache, tag);
        cache
    } else {
        let cache = z_malloc(size, tag, std::ptr::null_mut());
        wad_state_write(|state| {
            state.lumpinfo[lumpnum as usize].cache = Some(cache);
        });
        i_system::i_begin_read();
        let n = wad_state_read(|state| {
            let wad = &state.open_wad_files[wad_index];
            wad.read(position as u32, unsafe {
                std::slice::from_raw_parts_mut(cache, size)
            })
        });
        i_system::i_end_read();
        if n < size {
            i_system::i_error(&format!(
                "W_ReadLump: only read {} of {} on lump {}",
                n, size, lumpnum
            ));
        }
        cache
    }
}

/// Original: W_CacheLumpName
pub fn w_cache_lump_name(name: &str, tag: i32) -> *mut u8 {
    w_cache_lump_num(w_get_num_for_name(name), tag)
}

/// Original: W_ReleaseLumpNum
pub fn w_release_lump_num(lumpnum: i32) {
    wad_state_write(|state| {
        if (lumpnum as usize) >= state.lumpinfo.len() {
            i_system::i_error(&format!("W_ReleaseLumpNum: {} >= numlumps", lumpnum));
        }
        let lump = &state.lumpinfo[lumpnum as usize];
        let wad = &state.open_wad_files[lump.wad_file_index];
        if wad.mapped.is_none() {
            if let Some(cache) = lump.cache {
                z_change_tag(cache, PU_CACHE);
            }
        }
    });
}

/// Original: W_ReleaseLumpName
pub fn w_release_lump_name(name: &str) {
    w_release_lump_num(w_get_num_for_name(name));
}

/// Original: W_GenerateHashTable
pub fn w_generate_hash_table() {
    wad_state_write(|state| {
    let numlumps = state.lumpinfo.len();
    state.lumphash = None;
    if numlumps > 0 {
        let mut lumphash = vec![None; numlumps];
        for i in 0..numlumps {
            let name = std::str::from_utf8(&state.lumpinfo[i].name)
                .unwrap_or("")
                .trim_end_matches('\0');
            let hash = (w_lump_name_hash(name) as usize) % numlumps;
            state.lumpinfo[i].next = lumphash[hash];
            lumphash[hash] = Some(i);
        }
        state.lumphash = Some(lumphash);
    }
    });
}

/// Original: W_CheckCorrectIWAD
pub fn w_check_correct_iwad(mission: GameMission) {
    let unique_lumps = [
        (GameMission::Doom, "POSSA1"),
        (GameMission::Heretic, "IMPXA1"),
        (GameMission::Hexen, "ETTNA1"),
        (GameMission::Strife, "AGRDA1"),
    ];
    for (lump_mission, lumpname) in unique_lumps {
        if mission != lump_mission {
            let lumpnum = w_check_num_for_name(lumpname);
            if lumpnum >= 0 {
                i_system::i_error(&format!(
                    "\nYou are trying to use a {} IWAD file with the doomgeneric binary.\nThis isn't going to work.\n",
                    d_iwad::d_suggest_game_name(lump_mission, crate::game::d_mode::GameMode::Indetermined)
                ));
            }
        }
    }
}

/// Get number of lumps. Original: numlumps
pub fn numlumps() -> usize {
    wad_state_read(|state| state.lumpinfo.len())
}

/// Run a function with access to lumpinfo (for checksum, etc.)
pub fn with_lumpinfo<F, R>(f: F) -> R
where
    F: FnOnce(&[LumpInfo]) -> R,
{
    wad_state_read(|state| f(&state.lumpinfo))
}
