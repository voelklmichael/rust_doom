//
// WAD module - Handles WAD file header, directory, lump I/O.
//

mod w_checksum;
mod w_file;
mod w_file_stdc;
mod w_main;
mod w_merge;
mod w_wad;

pub use w_checksum::w_checksum;
pub use w_file::{w_open_file, WadFile};
pub use w_main::w_parse_command_line;
pub use w_merge::{
    w_merge_file, w_nwt_dash_merge, w_nwt_merge_file, w_print_directory,
    W_NWT_MERGE_FLATS, W_NWT_MERGE_SPRITES,
};
pub use w_wad::{
    numlumps, w_add_file, w_cache_lump_name, w_cache_lump_num, w_check_correct_iwad,
    w_check_num_for_name, w_generate_hash_table, w_get_num_for_name, w_lump_length,
    w_lump_name_hash, w_num_lumps, w_read_lump, w_release_lump_name, w_release_lump_num,
    with_lumpinfo, LumpInfo,
};
