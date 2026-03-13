//! Extract all lumps from a Doom WAD file (e.g. doom1.wad) into a directory structure.
//!
//! Usage: cargo run --example example_wad -- <path/to/doom1.wad> [output_dir]
//!
//! Folder structure:
//!   maps/       - Map data (E1M1, MAP01, etc. with THINGS, LINEDEFS, etc.)
//!   sprites/    - Sprites (between S_START and S_END)
//!   flats/      - Floor/ceiling textures (between F_START and F_END)
//!   patches/    - Wall patches (between P_START and P_END)
//!   palette/    - PLAYPAL, COLORMAP
//!   textures/   - TEXTURE1, TEXTURE2
//!   sounds/     - Sound effects (DP*)
//!   music/      - Music (D_*)
//!   data/       - Other lumps (DEMO, etc.)

use doom_rust::{m_argv, wad, z_zone};
use std::fs;
use std::path::Path;

fn lump_name_to_str(name: &[u8; 8]) -> String {
    let end = name.iter().position(|&b| b == 0).unwrap_or(8);
    String::from_utf8_lossy(&name[..end]).trim().to_string()
}

fn main() {
    let wad_path = "../Doom1.WAD";
    let out_dir = "target/doom1_extracted";

    println!("Extracting {} to {}", wad_path, out_dir);

    let args = vec![wad_path.to_string(), out_dir.to_string()];

    m_argv::m_argv_init(args.clone());
    z_zone::z_init();

    if wad::w_add_file(wad_path).is_none() {
        eprintln!("Failed to open WAD file: {}", wad_path);
        eprintln!("Make sure the file exists and is a valid Doom WAD (IWAD or PWAD).");
        std::process::exit(1);
    }

    let out_path = Path::new(out_dir);
    fs::create_dir_all(out_path).expect("Failed to create output directory");

    let mut current_section = "data";
    let mut map_name: Option<String> = None;

    wad::with_lumpinfo(|lumpinfo| {
        for (i, lump) in lumpinfo.iter().enumerate() {
            let name = lump_name_to_str(&lump.name);
            if name.is_empty() {
                continue;
            }

            let (folder, filename) = match &*name {
                "S_START" => {
                    current_section = "sprites";
                    continue;
                }
                "S_END" => {
                    current_section = "data";
                    continue;
                }
                "F_START" | "F1_START" => {
                    current_section = "flats";
                    continue;
                }
                "F_END" | "F1_END" => {
                    current_section = "data";
                    continue;
                }
                "P_START" | "P1_START" => {
                    current_section = "patches";
                    continue;
                }
                "P_END" | "P1_END" => {
                    current_section = "data";
                    continue;
                }
                "PLAYPAL" | "COLORMAP" => ("palette".to_string(), name.clone()),
                "TEXTURE1" | "TEXTURE2" => ("textures".to_string(), name.clone()),
                "TEXTURE3" | "TEXTURE4" => ("textures".to_string(), name.clone()),
                "TEXTURE5" | "TEXTURE6" | "TEXTURE7" | "TEXTURE8" => {
                    ("textures".to_string(), name.clone())
                }
                "TEXTURE9" | "TEXTURE0" => ("textures".to_string(), name.clone()),
                "TEXTURE10" | "TEXTURE11" | "TEXTURE12" => ("textures".to_string(), name.clone()),
                _ if (name.starts_with("MAP") && name.len() <= 5)
                    || (name.starts_with("E")
                        && name.len() >= 4
                        && name.len() <= 5
                        && name.contains('M')) =>
                {
                    map_name = Some(name.clone());
                    current_section = "maps";
                    let map_dir = out_path.join("maps").join(&name);
                    fs::create_dir_all(&map_dir).expect("Failed to create map dir");
                    continue;
                }
                "THINGS" | "LINEDEFS" | "SIDEDEFS" | "VERTEXES" | "SEGS" | "SSECTORS" | "NODES"
                | "REJECT" | "BLOCKMAP" => {
                    if let Some(ref map) = map_name {
                        (format!("maps/{}", map), name.clone())
                    } else {
                        ("data".to_string(), name.clone())
                    }
                }
                _ if name.starts_with("DP") && name.len() <= 5 => {
                    ("sounds".to_string(), name.clone())
                }
                _ if name.starts_with("D_") && name.len() <= 5 => {
                    ("music".to_string(), name.clone())
                }
                _ if current_section == "sprites" => ("sprites".to_string(), name.clone()),
                _ if current_section == "flats" => ("flats".to_string(), name.clone()),
                _ if current_section == "patches" => ("patches".to_string(), name.clone()),
                _ if current_section == "maps" => {
                    if let Some(ref map) = map_name {
                        (format!("maps/{}", map), name.clone())
                    } else {
                        ("data".to_string(), name.clone())
                    }
                }
                _ => ("data".to_string(), name.clone()),
            };

            let dir = out_path.join(&folder);
            fs::create_dir_all(&dir).expect("Failed to create output dir");
            let file_path = dir.join(&filename);

            let size = lump.size as usize;
            if size > 0 {
                let mut buf = vec![0u8; size];
                wad::w_read_lump(i, &mut buf);
                fs::write(&file_path, &buf).expect("Failed to write lump");
                println!("  {} -> {}", filename, file_path.display());
            }
        }
    });

    println!("Extraction complete.");
}
