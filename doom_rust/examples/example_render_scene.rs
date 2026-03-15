//! Render a Doom level scene to a PNG image.
//!
//! Loads a map from the WAD, spawns the player at the player 1 start,
//! renders one frame, and saves the result as a PNG.
//!
//! Usage: cargo run --example example_render_scene [wad_path] [map_name]
//!
//! Default: ../Doom1.WAD E1M1
//!
//! Output: target/rendering/scene.png

use doom_rust::doomdef::{SCREENHEIGHT, SCREENWIDTH};
use doom_rust::m_argv;
use doom_rust::player::p_setup;
use doom_rust::rendering::{r_init, r_render_player_view, view_player_from_console, with_v_video_state};
use doom_rust::wad;
use doom_rust::z_zone;
use image::{ImageBuffer, RgbaImage};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let wad_path = args.get(1).map(|s| s.as_str()).unwrap_or("../Doom1.WAD");
    dbg!(&wad_path);
    let map_name = args.get(2).map(|s| s.as_str()).unwrap_or("E1M1");

    println!("Loading {} and rendering {}", wad_path, map_name);

    m_argv::m_argv_init(vec![wad_path.to_string()]);
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

    let player = view_player_from_console().unwrap_or_else(|| {
        eprintln!("No player spawned - ensure player 1 start exists in map");
        std::process::exit(1);
    });

    r_render_player_view(&player);

    // Convert screen buffer (8-bit palette indices) to RGBA and save
    let palette_data = wad::w_cache_lump_name("PLAYPAL", doom_rust::z_zone::PU_STATIC);
    let palette = unsafe { std::slice::from_raw_parts(palette_data.as_ptr(), 768) }; // First 256 colors

    let mut rgba = Vec::with_capacity((SCREENWIDTH * SCREENHEIGHT * 4) as usize);
    let viewimage = with_v_video_state(|vv| vv.viewimage);
    unsafe {
        let screen = std::slice::from_raw_parts(viewimage, (SCREENWIDTH * SCREENHEIGHT) as usize);
        for &idx in screen {
            let i = (idx as usize) * 3;
            if i + 2 < palette.len() {
                rgba.extend_from_slice(&[palette[i], palette[i + 1], palette[i + 2], 255]);
            } else {
                rgba.extend_from_slice(&[0, 0, 0, 255]);
            }
        }
    }

    let img: RgbaImage = ImageBuffer::from_raw(SCREENWIDTH as u32, SCREENHEIGHT as u32, rgba)
        .expect("valid dimensions");

    let out_dir = Path::new("target/rendering");
    let out_path = out_dir.join("scene.png");
    fs::create_dir_all(out_dir).expect("create output dir");
    img.save(&out_path).expect("save PNG");

    println!("Rendered scene to {}", out_path.display());
}
