//! Render a Doom sprite (patch format) to a PNG image.
//!
//! Reads sprite and palette from extracted WAD data, renders the patch
//! into an RGBA image, and saves it.
//!
//! Usage: cargo run --example example_render_sprite
//!
//! Prerequisites: Run `cargo run --example example_wad` first to extract
//! sprites and palette to target/doom1_extracted/
//!
//! Output: target/rendering/sprite/<sprite_name>.png for each sprite in sprites/

use image::{ImageBuffer, RgbaImage};
use std::fs;
use std::path::Path;

const TRANSPARENT: u8 = 247; // Doom uses palette index 247 for transparency (black in colormap)

fn read_i16_le(data: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes([data[offset], data[offset + 1]])
}

fn read_i32_le(data: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

/// Parse Doom patch format and render to 8-bit palette indices (width x height).
/// Returns (width, height, leftoffset, topoffset, pixel_buffer).
fn parse_patch(data: &[u8]) -> Option<(u32, u32, i16, i16, Vec<u8>)> {
    if data.len() < 8 {
        return None;
    }
    let width = read_i16_le(data, 0) as u32;
    let height = read_i16_le(data, 2) as u32;
    let leftoffset = read_i16_le(data, 4);
    let topoffset = read_i16_le(data, 6);

    let header_size = 8 + (width as usize) * 4;
    if data.len() < header_size {
        return None;
    }

    let mut pixels = vec![TRANSPARENT; (width * height) as usize];

    for col in 0..width as usize {
        let col_ofs = read_i32_le(data, 8 + col * 4) as usize;
        if col_ofs >= data.len() {
            continue;
        }

        let mut pos = col_ofs;
        loop {
            if pos + 2 >= data.len() {
                break;
            }
            let topdelta = data[pos];
            if topdelta == 0xff {
                break;
            }
            let length = data[pos + 1] as usize;
            pos += 3; // header: topdelta, length, 1 padding
            if pos + length > data.len() {
                break;
            }

            let start_y = topdelta as usize;
            for (i, &pal_idx) in data[pos..pos + length].iter().enumerate() {
                let y = start_y + i;
                if y < height as usize && pal_idx != TRANSPARENT {
                    pixels[y * (width as usize) + col] = pal_idx;
                }
            }
            pos += length;
        }
    }

    Some((width, height, leftoffset, topoffset, pixels))
}

/// Convert palette index buffer to RGBA using Doom PLAYPAL.
/// PLAYPAL: 256 colors * 3 bytes (RGB) per palette; we use palette 0.
fn palette_to_rgba(pixels: &[u8], palette: &[u8]) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(pixels.len() * 4);
    for &idx in pixels {
        if idx == TRANSPARENT {
            rgba.extend_from_slice(&[0, 0, 0, 0]);
        } else {
            let i = (idx as usize) * 3;
            if i + 2 < palette.len() {
                rgba.extend_from_slice(&[palette[i], palette[i + 1], palette[i + 2], 255]);
            } else {
                rgba.extend_from_slice(&[0, 0, 0, 255]);
            }
        }
    }
    rgba
}

fn main() {
    let base = Path::new("target/doom1_extracted");
    let sprites_dir = base.join("sprites");
    let palette_path = base.join("palette/PLAYPAL");
    let out_dir = Path::new("target/rendering/sprite");

    let palette_data = fs::read(&palette_path).unwrap_or_else(|e| {
        eprintln!("Failed to read palette {}: {}", palette_path.display(), e);
        std::process::exit(1);
    });

    // PLAYPAL: first 768 bytes = 256 colors * 3 (RGB) for palette 0
    let palette = if palette_data.len() >= 768 {
        &palette_data[..768]
    } else {
        eprintln!("PLAYPAL too small");
        std::process::exit(1);
    };

    let entries = fs::read_dir(&sprites_dir).unwrap_or_else(|e| {
        eprintln!(
            "Failed to read sprites dir {}: {}. Run 'cargo run --example example_wad' first.",
            sprites_dir.display(),
            e
        );
        std::process::exit(1);
    });

    fs::create_dir_all(out_dir).expect("create output dir");

    let mut count = 0;
    for entry in entries {
        let entry = entry.expect("read dir entry");
        let path = entry.path();
        if path.is_file() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
            let sprite_data = match fs::read(&path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Failed to read sprite {}: {}", path.display(), e);
                    continue;
                }
            };

            let (width, height, _left, _top, pixels) = match parse_patch(&sprite_data) {
                Some(parsed) => parsed,
                None => {
                    eprintln!("Failed to parse patch {}", name);
                    continue;
                }
            };

            let rgba = palette_to_rgba(&pixels, palette);
            let img: RgbaImage = ImageBuffer::from_raw(width, height, rgba).expect("valid dimensions");
            let out_path = out_dir.join(format!("{}.png", name));
            img.save(&out_path).expect("save PNG");
            count += 1;
        }
    }

    println!("Rendered {} sprites to {}", count, out_dir.display());
}
