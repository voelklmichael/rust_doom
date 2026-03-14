use doom_rust::{game::statdump, m_argv, wad, z_zone};

fn main() {
    // Initialize command line
    m_argv::m_argv_init(std::env::args().collect());

    // Initialize zone allocator (required before WAD operations)
    z_zone::z_init();

    // Parse command line for -file, -merge, etc.
    let modified = wad::w_parse_command_line();
    if modified {
        println!("WAD files loaded. Lump count: {}", wad::numlumps());
    } else {
        println!("Hello, world! (No WAD files specified - use -file <wad> to load)");
    }

    // Dump statistics if -statdump was used (captured during level completions)
    statdump::stat_dump();
}
