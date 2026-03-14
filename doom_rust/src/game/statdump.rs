//
// Copyright(C) 2005-2014 Simon Howard
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// DESCRIPTION:
//  Functions for presenting the information captured from the statistics
//  buffer to a file. Matches output from statdump.exe (ctrlapi.zip).
//
// Original: statdump.h + statdump.c

use crate::doomdef::MAXPLAYERS;
use crate::doomstat::WbStartStruct;
use crate::game::d_mode::GameMission;
use crate::m_argv::{m_check_parm_with_args, m_parm_exists, myargv};
use std::io::Write;
use std::sync::Mutex;

const TICRATE: i32 = 35;
const MAX_CAPTURES: usize = 32;

/// Par times for E1M1-E1M9 (Doom 1).
const DOOM1_PAR_TIMES: [i32; 9] =
    [30, 75, 120, 90, 165, 180, 180, 30, 165];

/// Par times for MAP01-MAP09 (Doom 2).
const DOOM2_PAR_TIMES: [i32; 9] =
    [30, 90, 120, 120, 90, 150, 120, 120, 270];

/// Player colors for display.
const PLAYER_COLORS: [&str; 4] = ["Green", "Indigo", "Brown", "Red"];

static CAPTURED_STATS: Mutex<Vec<WbStartStruct>> = Mutex::new(Vec::new());

/// Discovered game mission (for statdump.exe compatibility).
static DISCOVERED_GAMEMISSION: Mutex<GameMission> = Mutex::new(GameMission::None);

/// Try to determine Doom 1 vs Doom 2 from episode/map and par times.
fn discover_gamemode(stats: &[WbStartStruct], num_stats: usize) {
    let mut mission = DISCOVERED_GAMEMISSION.lock().unwrap();
    if *mission != GameMission::None {
        return;
    }
    for i in 0..num_stats {
        let level = stats[i].last;
        // Episode 2, 3 or 4 => Doom 1
        if stats[i].epsd > 0 {
            *mission = GameMission::Doom;
            return;
        }
        // Episode 1, level 10+ => Doom 2
        if level >= 9 {
            *mission = GameMission::Doom2;
            return;
        }
        let partime = stats[i].partime;
        let level_idx = level.min(8) as usize;
        if partime == DOOM1_PAR_TIMES[level_idx] * TICRATE
            && partime != DOOM2_PAR_TIMES[level_idx] * TICRATE
        {
            *mission = GameMission::Doom;
            return;
        }
        if partime != DOOM1_PAR_TIMES[level_idx] * TICRATE
            && partime == DOOM2_PAR_TIMES[level_idx] * TICRATE
        {
            *mission = GameMission::Doom2;
            return;
        }
    }
}

fn get_num_players(stats: &WbStartStruct) -> usize {
    let mut n = 0;
    for i in 0..MAXPLAYERS {
        if stats.plyr[i].in_game != 0 {
            n += 1;
        }
    }
    n
}

fn print_banner(stream: &mut dyn Write) -> std::io::Result<()> {
    writeln!(stream, "===========================================")
}

fn print_percentage(stream: &mut dyn Write, amount: i32, total: i32) -> std::io::Result<()> {
    if total == 0 {
        write!(stream, "0")?;
    } else {
        write!(stream, "{} / {}", amount, total)?;
        // Cast to i16 for statdump.exe compatibility (16-bit overflow)
        let pct = (amount as i16).wrapping_mul(100) / total as i16;
        write!(stream, " ({}%)", pct)?;
    }
    Ok(())
}

fn print_player_stats(
    stream: &mut dyn Write,
    stats: &WbStartStruct,
    player_num: usize,
) -> std::io::Result<()> {
    let player = &stats.plyr[player_num];
    let color = PLAYER_COLORS.get(player_num).unwrap_or(&"???");
    writeln!(stream, "Player {} ({}):", player_num + 1, color)?;
    write!(stream, "\tKills: ")?;
    print_percentage(stream, player.kills, stats.maxkills)?;
    writeln!(stream)?;
    write!(stream, "\tItems: ")?;
    print_percentage(stream, player.items, stats.maxitems)?;
    writeln!(stream)?;
    write!(stream, "\tSecrets: ")?;
    print_percentage(stream, player.secret, stats.maxsecret)?;
    writeln!(stream)?;
    Ok(())
}

fn print_frags_table(stream: &mut dyn Write, stats: &WbStartStruct) -> std::io::Result<()> {
    writeln!(stream, "Frags:")?;
    write!(stream, "\t\t")?;
    for x in 0..MAXPLAYERS {
        if stats.plyr[x].in_game == 0 {
            continue;
        }
        let color = PLAYER_COLORS.get(x).unwrap_or(&"???");
        write!(stream, "{}\t", color)?;
    }
    writeln!(stream)?;
    writeln!(stream, "\t\t-------------------------------- VICTIMS")?;
    for y in 0..MAXPLAYERS {
        if stats.plyr[y].in_game == 0 {
            continue;
        }
        let color = PLAYER_COLORS.get(y).unwrap_or(&"???");
        write!(stream, "\t{}\t|", color)?;
        for x in 0..MAXPLAYERS {
            if stats.plyr[x].in_game == 0 {
                continue;
            }
            write!(stream, "{}\t", stats.plyr[y].frags[x])?;
        }
        writeln!(stream)?;
    }
    writeln!(stream, "\t\t|")?;
    writeln!(stream, "\t     KILLERS")?;
    Ok(())
}

fn print_level_name(stream: &mut dyn Write, episode: i32, level: i32) -> std::io::Result<()> {
    print_banner(stream)?;
    let mission = *DISCOVERED_GAMEMISSION.lock().unwrap();
    match mission {
        GameMission::Doom => writeln!(stream, "E{}M{}", episode + 1, level + 1)?,
        GameMission::Doom2 => writeln!(stream, "MAP{:02}", level + 1)?,
        _ => writeln!(stream, "E{}M{} / MAP{:02}", episode + 1, level + 1, level + 1)?,
    }
    print_banner(stream)?;
    Ok(())
}

fn print_stats(stream: &mut dyn Write, stats: &WbStartStruct) -> std::io::Result<()> {
    print_level_name(stream, stats.epsd, stats.last)?;
    writeln!(stream)?;
    let leveltime = stats.plyr[0].time / TICRATE;
    let partime = stats.partime / TICRATE;
    writeln!(
        stream,
        "Time: {}:{:02} (par: {}:{:02})",
        leveltime / 60,
        leveltime % 60,
        partime / 60,
        partime % 60
    )?;
    writeln!(stream)?;
    for i in 0..MAXPLAYERS {
        if stats.plyr[i].in_game != 0 {
            print_player_stats(stream, stats, i)?;
        }
    }
    if get_num_players(stats) >= 2 {
        print_frags_table(stream, stats)?;
    }
    writeln!(stream)?;
    Ok(())
}

/// Copy stats to buffer when -statdump is present. Call from G_DoCompleted before WI_Start.
/// Original: StatCopy
pub fn stat_copy(stats: &WbStartStruct) {
    if m_parm_exists("-statdump") {
        if let Ok(mut captured) = CAPTURED_STATS.lock() {
            if captured.len() < MAX_CAPTURES {
                captured.push(stats.clone());
            }
        }
    }
}

/// Dump captured statistics to file. Call when game exits (e.g. from D_DoomMain or similar).
/// -statdump filename: write to file
/// -statdump -: write to stdout
/// Original: StatDump
pub fn stat_dump() {
    let i = m_check_parm_with_args("-statdump", 1);
    if i == 0 {
        return;
    }
    let captured = match CAPTURED_STATS.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => return,
    };
    let num = captured.len();
    println!("Statistics captured for {} level(s)", num);
    if num == 0 {
        return;
    }
    discover_gamemode(&captured, num);
    let argv = myargv();
    let filename = argv.get(i + 1).map(|s| s.as_str()).unwrap_or("");
    let write_result = if filename == "-" {
        let mut out = std::io::stdout();
        for stats in captured.iter() {
            let _ = print_stats(&mut out, stats);
        }
        Ok(())
    } else {
        let mut file = match std::fs::File::create(filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("statdump: failed to create {}: {}", filename, e);
                return;
            }
        };
        for stats in captured.iter() {
            if let Err(e) = print_stats(&mut file, stats) {
                eprintln!("statdump: write error: {}", e);
                return;
            }
        }
        file.flush()
    };
    if let Err(e) = write_result {
        eprintln!("statdump: {}", e);
    }
}
