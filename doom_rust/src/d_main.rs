//! Rust translation of doomgeneric/d_main.h
//! System specific interface stuff.

use crate::config::PACKAGE_STRING;
use crate::config::PROGRAM_PREFIX;
use crate::d_englsh::D_CDROM;
use crate::d_englsh::D_DEVSTR;
use crate::d_iwad::{IWAD_MASK_DOOM, d_find_iwad, d_save_game_iwad_name};
use crate::d_mode::GameModeT;
use crate::d_mode::{GameMissionT, SkillT};
use crate::deh_main::deh_parse_command_line;
use crate::deh_str::{deh_add_string_replacement, deh_printf, deh_snprintf, deh_string};
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::Boolean;
use crate::g_game::*;
use crate::hu_stuff::hu_init;
use crate::i_joystick::i_init_joystick;
use crate::i_sound::{i_init_music, i_init_sound};
use crate::i_system::{
    i_at_exit, i_error, i_print_banner, i_print_divider, i_print_startup_banner,
};
use crate::i_timer::i_init_timer;
use crate::i_video::{i_check_is_screensaver, i_display_fps_dots};
use crate::m_argv::{m_check_parm, m_check_parm_with_args, m_parm_exists};
use crate::m_config::{
    m_get_save_game_dir, m_load_defaults, m_save_defaults, m_set_config_dir, m_set_config_filenames,
};
use crate::m_menu::m_init;
use crate::m_misc::{m_string_copy, m_string_ends_with};
use crate::p_saveg::p_save_game_file;
use crate::p_setup::p_init;
use crate::r_main::r_init;
use crate::s_sound::s_init;
use crate::st_stuff::st_init;
use crate::statdump::stat_dump;
use crate::v_video::v_init;
use crate::w_main::w_parse_command_line;
use crate::w_wad::{w_add_file, w_check_correct_iwad, w_check_num_for_name, w_generate_hash_table};
use crate::z_zone::z_init;

/// Wrapper for I_AtExit(D_Endoom, ...)
extern "C" fn d_endoom_atexit() {
    // D_Endoom loads ENDOOM lump and calls I_Endoom
    todo!("original: D_Endoom")
}

/// Wrapper for I_AtExit(M_SaveDefaults, ...)
extern "C" fn m_save_defaults_atexit() {
    m_save_defaults();
}

/// Wrapper for I_AtExit(G_CheckDemoStatus, ...)
extern "C" fn g_check_demo_status_atexit() {
    g_check_demo_status();
}

/// Wrapper for I_AtExit(StatDump, ...)
extern "C" fn stat_dump_atexit() {
    stat_dump();
}

#[cfg(feature = "multiplayer")]
use crate::net_client::net_init;
#[cfg(feature = "multiplayer")]
use crate::net_dedicated::net_dedicated_server;
#[cfg(feature = "multiplayer")]
use crate::net_query::{net_lan_query, net_master_query, net_query_address};

#[cfg(feature = "dehacked")]
use crate::deh_main::deh_load_lump;

#[cfg(feature = "origcode")]
use crate::deh_main::load_iwad_deh;

/// C function: D_ProcessEvents
pub fn d_process_events() {
    todo!("original: D_ProcessEvents")
}

/// C function: D_PageTicker
pub fn d_page_ticker() {
    todo!("original: D_PageTicker")
}

/// C function: D_PageDrawer
pub fn d_page_drawer() {
    todo!("original: D_PageDrawer")
}

/// C function: D_AdvanceDemo
pub fn d_advance_demo() {
    todo!("original: D_AdvanceDemo")
}

/// C function: D_DoAdvanceDemo
pub fn d_do_advance_demo() {
    todo!("original: D_DoAdvanceDemo")
}

/// C function: D_StartTitle
pub fn d_start_title() {
    todo!("original: D_StartTitle")
}

/// C function: D_AddFile (static in d_main.c)
fn d_add_file(filename: &str) -> Boolean {
    println!(" adding {}\n", filename);
    let _handle = w_add_file(filename);
    // In C: return handle != NULL; w_add_file returns Arc<Mutex<WadFileT>> which is never null
    Boolean::True
}

/// C function: D_ConnectNetGame
pub fn d_connect_net_game() {
    todo!("original: D_ConnectNetGame")
}

/// C function: D_CheckNetGame
pub fn d_check_net_game() {
    todo!("original: D_CheckNetGame")
}

/// C function: D_DoomLoop
pub fn d_doom_loop() {
    todo!("original: D_DoomLoop")
}

/// C function: D_IdentifyVersion
pub fn d_identify_version() {
    todo!("original: D_IdentifyVersion")
}

/// C function: D_SetGameDescription
pub fn d_set_game_description() {
    todo!("original: D_SetGameDescription")
}

/// C function: D_BindVariables
pub fn d_bind_variables() {
    todo!("original: D_BindVariables")
}

/// C function: InitGameVersion
pub fn init_game_version() {
    todo!("original: InitGameVersion")
}

/// C function: PrintDehackedBanners
pub fn print_dehacked_banners() {
    todo!("original: PrintDehackedBanners")
}

/// C function: PrintGameVersion
pub fn print_game_version() {
    todo!("original: PrintGameVersion")
}

pub static mut gameaction: GameactionT = GameactionT::Nothing;

/// C function: D_DoomMain
pub fn d_doom_main() {
    #[cfg(feature = "origcode")]
    let numiwadlumps: u32;

    i_at_exit(d_endoom_atexit, Boolean::False);
    i_print_banner(PACKAGE_STRING);
    deh_printf("Z_Init: Init zone memory allocation daemon. \n");
    z_init();

    #[cfg(feature = "multiplayer")]
    {
        if m_check_parm("-dedicated") > 0 {
            println!("Dedicated server mode.\n");
            net_dedicated_server();
            // Never returns
        }

        if m_check_parm("-search") > 0 {
            net_master_query();
            std::process::exit(0);
        }

        let p = m_check_parm_with_args("-query", 1);
        if p > 0 {
            // myargv[p+1] - would need to get from myargv
            net_query_address("");
            std::process::exit(0);
        }

        if m_check_parm("-localsearch") > 0 {
            net_lan_query();
            std::process::exit(0);
        }
    }

    unsafe {
        nomonsters = if m_check_parm("-nomonsters") > 0 {
            Boolean::True
        } else {
            Boolean::False
        };
        respawnparm = if m_check_parm("-respawn") > 0 {
            Boolean::True
        } else {
            Boolean::False
        };
        fastparm = if m_check_parm("-fast") > 0 {
            Boolean::True
        } else {
            Boolean::False
        };
        devparm = if m_check_parm("-devparm") > 0 {
            Boolean::True
        } else {
            Boolean::False
        };
    }
    i_display_fps_dots(unsafe { devparm });

    if m_check_parm("-deathmatch") > 0 {
        unsafe { deathmatch = 1 };
    }
    if m_check_parm("-altdeath") > 0 {
        unsafe { deathmatch = 2 };
    }
    if unsafe { devparm == Boolean::True } {
        deh_printf(D_DEVSTR);
    }

    #[cfg(target_os = "windows")]
    if m_parm_exists("-cdrom") == Boolean::True {
        println!("{}", D_CDROM);
        m_set_config_dir("c:\\doomdata\\");
    } else {
        m_set_config_dir_auto();
    }

    #[cfg(not(target_os = "windows"))]
    m_set_config_dir(None);

    let p_turbo = m_check_parm("-turbo");
    if p_turbo > 0 {
        let mut scale = 200;
        // extern forwardmove, sidemove - would need m_controls
        if p_turbo < unsafe { crate::m_argv::myargc - 1 } {
            // scale = atoi(myargv[p_turbo+1]);
            scale = 200;
        }
        if scale < 10 {
            scale = 10;
        }
        if scale > 400 {
            scale = 400;
        }
        deh_printf(&format!("turbo scale: {}%\n", scale));
        // forwardmove/sidemove modification - todo when m_controls exists
    }

    deh_printf("V_Init: allocate screens.\n");
    v_init();

    deh_printf("M_LoadDefaults: Load system defaults.\n");
    m_set_config_filenames("default.cfg", &format!("{}doom.cfg", PROGRAM_PREFIX));
    d_bind_variables();
    m_load_defaults();

    i_at_exit(m_save_defaults_atexit, Boolean::False);

    let mut mission = GameMissionT::None;
    let iwad = d_find_iwad(IWAD_MASK_DOOM, &mut mission);
    if iwad.is_empty() {
        i_error(
            "Game mode indeterminate.  No IWAD file was found.  Try\nspecifying one with the '-iwad' command line parameter.\n",
        );
    }

    let iwadfile;
    unsafe {
        modifiedgame = Boolean::False;
        iwadfile = iwad.clone();
        gamemission = mission;
    }

    deh_printf("W_Init: Init WADfiles.\n");
    d_add_file(&iwad);

    #[cfg(feature = "origcode")]
    let _numiwadlumps = unsafe { crate::w_wad::numlumps };

    w_check_correct_iwad(GameMissionT::Doom);
    d_identify_version();
    init_game_version();

    #[cfg(feature = "origcode")]
    if m_parm_exists("-nodeh") != Boolean::True {
        load_iwad_deh();
    }

    if w_check_num_for_name("dmenupic") >= 0 {
        println!("BFG Edition: Using workarounds as needed.\n");
        unsafe { bfgedition = Boolean::True };
        deh_add_string_replacement("level 31: idkfa", "level 31: idkfa");
        deh_add_string_replacement("level 32: keen", "level 32: keen");
        deh_add_string_replacement("level 33: betray", "level 33: betray");
        deh_add_string_replacement("M_GDHIGH", "M_MSGON");
        deh_add_string_replacement("M_GDLOW", "M_MSGOFF");
    }

    #[cfg(feature = "dehacked")]
    deh_parse_command_line();

    unsafe { modifiedgame = w_parse_command_line() };

    let p_playdemo = m_check_parm_with_args("-playdemo", 1);
    let p = if p_playdemo > 0 {
        p_playdemo
    } else {
        m_check_parm_with_args("-timedemo", 1)
    };

    if p > 0 {
        // Would need myargv access - simplified for stub
        let demo_arg = "";
        let mut file_buf = [0u8; 256];
        let mut demolumpname_buf = [0u8; 9];
        if m_string_ends_with(demo_arg, ".lmp") == Boolean::True {
            let _ = m_string_copy(&mut file_buf, demo_arg, 256);
        } else {
            deh_snprintf(&mut file_buf, 256, &format!("{}.lmp", demo_arg));
        }
        if d_add_file(&String::from_utf8_lossy(&file_buf)) == Boolean::True {
            // M_StringCopy(demolumpname, lumpinfo[numlumps - 1].name, ...)
        } else {
            let _ = m_string_copy(&mut demolumpname_buf, demo_arg, 9);
        }
        println!("Playing demo {}.\n", String::from_utf8_lossy(&file_buf));
    }

    i_at_exit(g_check_demo_status_atexit, Boolean::True);
    w_generate_hash_table();

    #[cfg(feature = "origcode")]
    if m_parm_exists("-dehlump") == Boolean::True {
        let loaded = 0;
        // for i in numiwadlumps..numlumps - DEH_LoadLump
        println!("  loaded {} DEHACKED lumps from PWAD files.\n", loaded);
    }

    d_set_game_description();

    #[cfg(target_os = "windows")]
    if m_parm_exists("-cdrom") == Boolean::True {
        // savegamedir = configdir
    }
    #[cfg(not(target_os = "windows"))]
    {
        // savegamedir = M_GetSaveGameDir(D_SaveGameIWADName(gamemission))
        let _ = m_get_save_game_dir(&d_save_game_iwad_name(mission).lock().unwrap().clone());
    }

    if unsafe { modifiedgame == Boolean::True } {
        if unsafe { gamemode } == GameModeT::Shareware {
            i_error(&deh_string(
                "\nYou cannot -file with the shareware version. Register!",
            ));
        }
        if unsafe { gamemode } == GameModeT::Registered {
            let name = [
                "e2m1", "e2m2", "e2m3", "e2m4", "e2m5", "e2m6", "e2m7", "e2m8", "e2m9", "e3m1",
                "e3m3", "e3m3", "e3m4", "e3m5", "e3m6", "e3m7", "e3m8", "e3m9", "dphoof", "bfgga0",
                "heada1", "cybra1", "spida1d1",
            ];
            for n in name {
                if w_check_num_for_name(n) < 0 {
                    i_error(&deh_string("\nThis is not the registered version."));
                }
            }
        }
    }

    if w_check_num_for_name("SS_START") >= 0 || w_check_num_for_name("FF_END") >= 0 {
        i_print_divider();
        println!(
            " WARNING: The loaded WAD file contains modified sprites or\n floor textures.  You may want to use the '-merge' command\n line option instead of '-file'.\n"
        );
    }

    i_print_startup_banner("");
    print_dehacked_banners();

    if w_check_num_for_name("FREEDOOM") >= 0 && w_check_num_for_name("FREEDM") < 0 {
        println!(
            " WARNING: You are playing using one of the Freedoom IWAD\n files, which might not work in this port. See this page\n for more information on how to play using Freedoom:\n   http://www.chocolate-doom.org/wiki/index.php/Freedoom\n"
        );
        i_print_divider();
    }

    deh_printf("I_Init: Setting up machine state.\n");
    i_check_is_screensaver();
    i_init_timer();
    i_init_joystick();
    i_init_sound(Boolean::True);
    i_init_music();

    #[cfg(feature = "multiplayer")]
    {
        println!("NET_Init: Init network subsystem.\n");
        net_init();
    }

    d_connect_net_game();

    unsafe {
        startskill = SkillT::Medium;
        startepisode = 1;
        startmap = 1;
        autostart = Boolean::False;
    }

    let p_skill = m_check_parm_with_args("-skill", 1);
    if p_skill > 0 {
        unsafe { autostart = Boolean::True };
    }

    let p_episode = m_check_parm_with_args("-episode", 1);
    if p_episode > 0 {
        unsafe {
            startmap = 1;
            autostart = Boolean::True;
        };
    }

    unsafe { timelimit = 0 };

    let p_timer = m_check_parm_with_args("-timer", 1);
    if p_timer > 0 {
        // timelimit = atoi(myargv[p_timer+1]);
    }

    if m_check_parm("-avg") > 0 {
        unsafe { timelimit = 20 };
    }

    let p_warp = m_check_parm_with_args("-warp", 1);
    if p_warp > 0 {
        if unsafe { gamemode } == GameModeT::Commercial {
            // startmap = atoi(myargv[p_warp+1]);
        } else {
            if p_warp + 2 >= unsafe { crate::m_argv::myargc } {
                unsafe { startmap = 1 };
            }
        }
        unsafe { autostart = Boolean::True };
    }

    if m_check_parm("-testcontrols") > 0 {
        unsafe {
            startepisode = 1;
            startmap = 1;
            autostart = Boolean::True;
            testcontrols = Boolean::True;
        };
    }

    let p_loadgame = m_check_parm_with_args("-loadgame", 1);
    if p_loadgame <= 0 {
        unsafe { startloadgame = -1 };
    }

    deh_printf("M_Init: Init miscellaneous info.\n");
    m_init();

    deh_printf("R_Init: Init DOOM refresh daemon - ");
    r_init();

    deh_printf("\nP_Init: Init Playloop state.\n");
    p_init();

    deh_printf("S_Init: Setting up sound.\n");
    s_init(unsafe { sfx_volume } * 8, unsafe { music_volume } * 8);

    deh_printf("D_CheckNetGame: Checking network game status.\n");
    d_check_net_game();

    print_game_version();

    deh_printf("HU_Init: Setting up heads up display.\n");
    hu_init();

    deh_printf("ST_Init: Init status bar.\n");
    st_init();

    let storedemo =
        if unsafe { gamemode } == GameModeT::Commercial && w_check_num_for_name("map01") < 0 {
            Boolean::True
        } else {
            Boolean::False
        };

    if m_check_parm_with_args("-statdump", 1) > 0 {
        i_at_exit(stat_dump_atexit, Boolean::True);
        deh_printf("External statistics registered.\n");
    }

    let p_record = m_check_parm_with_args("-record", 1);
    if p_record > 0 {
        unsafe { autostart = Boolean::True };
    }

    let p_playdemo_final = m_check_parm_with_args("-playdemo", 1);
    if p_playdemo_final > 0 {
        unsafe { singledemo = Boolean::True };
        g_defered_play_demo("");
        d_doom_loop();
        return;
    }

    let p_timedemo = m_check_parm_with_args("-timedemo", 1);
    if p_timedemo > 0 {
        g_time_demo("");
        d_doom_loop();
        return;
    }

    if unsafe { startloadgame } >= 0 {
        let load_file = p_save_game_file(unsafe { startloadgame });
        g_load_game(&load_file);
    }

    if unsafe { gameaction } != GameactionT::Loadgame {
        if unsafe { autostart == Boolean::True } || unsafe { netgame == Boolean::True } {
            g_init_new(unsafe { startskill }, unsafe { startepisode }, unsafe {
                startmap
            });
        } else {
            d_start_title();
        }
    }

    d_doom_loop();
}
