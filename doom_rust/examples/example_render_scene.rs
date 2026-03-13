//! Render a Doom level scene to a PNG image.
//!
//! Loads a map from the WAD, sets up the player view, renders one frame,
//! and saves the result as a PNG.
//!
//! Usage: cargo run --example example_render_scene [wad_path] [map_name]
//!
//! Default: ../Doom1.WAD E1M1
//!
//! Output: target/rendering/scene.png

use doom_rust::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use doom_rust::m_argv;
use doom_rust::m_fixed::FRACUNIT;
use doom_rust::p_setup;
use doom_rust::rendering::{
    r_init, r_precache_level, r_render_player_view, ViewPlayerStub, VIEWIMAGE,
};
use doom_rust::wad;
use doom_rust::z_zone;
use image::{ImageBuffer, RgbaImage};
use std::env;
use std::fs;
use std::path::Path;

fn read_i16(data: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes([data[offset], data[offset + 1]])
}

/// Get player 1 start position from THINGS lump.
fn get_player_start(_wad_path: &str, map_name: &str) -> (i32, i32, u32, i32) {
    let map_lump = wad::w_get_num_for_name(map_name) as usize;
    let things_lump = map_lump + 1;
    let things_size = wad::w_lump_length(things_lump) as usize;
    let num_things = things_size / 10;

    let mut things_buf = vec![0u8; things_size];
    wad::w_read_lump(things_lump, &mut things_buf);

    for i in 0..num_things {
        let x = read_i16(&things_buf, i * 10) as i32;
        let y = read_i16(&things_buf, i * 10 + 2) as i32;
        let angle = read_i16(&things_buf, i * 10 + 4) as u16;
        let type_ = read_i16(&things_buf, i * 10 + 6);

        if type_ == 1 {
            // Player 1 start: convert to Fixed, angle to BAM
            let angle_bam = (angle as u32) << 16;
            let viewz = 41 * FRACUNIT; // Default player height
            return (x * FRACUNIT, y * FRACUNIT, angle_bam, viewz);
        }
    }

    // Fallback: E1M1 default spawn
    (1056 * FRACUNIT, -3616 * FRACUNIT, 0x4000_0000, 41 * FRACUNIT) // ANG90 = south
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let wad_path = args.get(1).map(|s| s.as_str()).unwrap_or("../Doom1.WAD");
    let map_name = args.get(2).map(|s| s.as_str()).unwrap_or("E1M1");

    println!("Loading {} and rendering {}", wad_path, map_name);

    m_argv::m_argv_init(vec![
        "example_render_scene".to_string(),
        wad_path.to_string(),
    ]);
    z_zone::z_init();

    if wad::w_add_file(wad_path).is_none() {
        eprintln!("Failed to open WAD file: {}", wad_path);
        eprintln!("Make sure the file exists (e.g. ../Doom1.WAD)");
        std::process::exit(1);
    }

    r_init();

    p_setup::p_load_level(map_name).unwrap_or_else(|e| {
        eprintln!("Failed to load map {}: {}", map_name, e);
        std::process::exit(1);
    });

    // r_precache_level(); // Skip - can trigger Z_ChangeTag errors with current zone setup

    let (mo_x, mo_y, mo_angle, viewz) = get_player_start(wad_path, map_name);

    let player = ViewPlayerStub {
        mo_x,
        mo_y,
        mo_angle,
        viewz,
        extralight: 0,
        fixedcolormap: 0,
    };

    r_render_player_view(&player);

    // Convert screen buffer (8-bit palette indices) to RGBA and save
    let palette_data = wad::w_cache_lump_name("PLAYPAL", doom_rust::z_zone::PU_STATIC);
    let palette = unsafe { std::slice::from_raw_parts(palette_data, 768) }; // First 256 colors

    let mut rgba = Vec::with_capacity((SCREENWIDTH * SCREENHEIGHT * 4) as usize);
    unsafe {
        let screen = std::slice::from_raw_parts(VIEWIMAGE, (SCREENWIDTH * SCREENHEIGHT) as usize);
        for &idx in screen {
            let i = (idx as usize) * 3;
            if i + 2 < palette.len() {
                rgba.extend_from_slice(&[palette[i], palette[i + 1], palette[i + 2], 255]);
            } else {
                rgba.extend_from_slice(&[0, 0, 0, 255]);
            }
        }
    }

    let img: RgbaImage =
        ImageBuffer::from_raw(SCREENWIDTH as u32, SCREENHEIGHT as u32, rgba).expect("valid dimensions");

    let out_dir = Path::new("target/rendering");
    let out_path = out_dir.join("scene.png");
    fs::create_dir_all(out_dir).expect("create output dir");
    img.save(&out_path).expect("save PNG");

    println!("Rendered scene to {}", out_path.display());
}
