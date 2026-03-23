use super::stage_100_if_directives::IncludeDirective;

pub fn if_directives_whitelist(ast: Vec<IncludeDirective>) -> String {
    let mut results = Vec::new();
    for node in ast {
        match node {
            IncludeDirective::NonDirective(code) => results.push(code),
            IncludeDirective::IfDef {
                symbol,
                then_branch,
                else_branch,
            } => {
                let next_tokens = if check_condition(&symbol) {
                    then_branch
                } else {
                    else_branch.unwrap_or_default()
                };
                results.push(if_directives_whitelist(next_tokens))
            }

            IncludeDirective::IfNDef {
                symbol,
                then_branch,
                else_branch: _,
            } => {
                const INCLUDE_GUARD_LIST: &[&str] = &[
                    "__AMMAP_H__",
                    "DOOM_GENERIC", // doomgeneric.h
                    "__D_ENGLSH__",
                    "__D_EVENT__",
                    "__D_ITEMS__",
                    "__D_IWAD__",
                    "__D_LOOP__",
                    "__D_MAIN__",
                    "__D_MODE__",
                    "__D_PLAYER__",
                    "__D_STATE__",
                    "__DSTRINGS__",
                    "__D_TICCMD__",
                    "__D_TEXTUR__",
                    "__D_THINK__",
                    "__DOOMDATA__",
                    "__DOOMDEF__",
                    "__DOOMKEYS__",
                    "__DOOMTYPE__",
                    "__F_FINALE__",
                    "__F_WIPE_H__",
                    "__G_GAME__",
                    "__GUSCONF_H__",
                    "__HULIB__",
                    "__HU_STUFF_H__",
                    "__I_ENDOOM__",
                    "__I_JOYSTICK__",
                    "__I_SCALE__",
                    "__I_SOUND__",
                    "__I_SYSTEM__",
                    "__I_TIMER__",
                    "__I_VIDEO__",
                    "__I_SWAP__",
                    "__ICDMUS__",
                    "__INFO__",
                    "__M_ARGV__",
                    "__M_BBOX__",
                    "__M_CHEAT__",
                    "__M_CONFIG__",
                    "__M_CONTROLS_H__",
                    "__M_FIXED__",
                    "__M_MENU__",
                    "__M_MISC__",
                    "__M_RANDOM__",
                    "__P_INTER__",
                    "__P_MOBJ__",
                    "__P_PSPR__",
                    "__P_SAVEG__", // p_saveg.h
                    "__P_SETUP__",
                    "__P_SPEC__",
                    "__P_TICK__",
                    "__P_LOCAL__",
                    "__R_BSP__",
                    "__R_DATA__",
                    "__R_DEFS__",
                    "__R_DRAW__",
                    "__R_LOCAL__",
                    "__R_MAIN__",
                    "__R_PLANE__",
                    "__R_SEGS__",
                    "__R_SKY__",
                    "__R_STATE__",
                    "__R_THINGS__",
                    "__S_SOUND__",
                    "__SHA1_H__",
                    "__SOUNDS__",
                    "__STLIB__",
                    "__STSTUFF_H__",
                    "__TABLES__",
                    "__V_VIDEO__",
                    "__W_FILE__",
                    "__W_WAD__",
                    "__WI_STUFF__",
                    "__Z_ZONE__",
                    "DEH_MAIN_H",
                    "DEH_MISC_H",
                    "DEH_STR_H",
                    "DOOM_FEATURES_H",
                    "DOOM_STATDUMP_H",
                    "MEMIO_H",
                    "MUS2MID_H",
                    "NET_CLIENT_H",
                    "NET_DEDICATED_H",
                    "NET_DEFS_H",
                    "NET_GUI_H",
                    "NET_IO_H",
                    "NET_LOOP_H",
                    "NET_PACKET_H",
                    "NET_QUERY_H",
                    "NET_SERVER_H",
                    "SRC_CHOCDOOM_DOOM_H_",
                    "V_PATCH_H",
                    "W_CHECKSUM_H",
                    "W_MAIN_H",
                    "W_MERGE_H",
                ];
                let next_tokens = if INCLUDE_GUARD_LIST.contains(&symbol.as_str()) {
                    then_branch
                } else {
                    todo!("{symbol}: This never happens in the Doom Codebase")
                    //else_branch.unwrap_or_default()
                };
                results.push(if_directives_whitelist(next_tokens))
            }
            IncludeDirective::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => {
                let mut branches = elif_branches;
                branches.insert(0, (condition, then_branch));

                let mut next_tokens = else_branch.unwrap_or_default();
                for (condition, branch) in branches {
                    if check_condition(&condition) {
                        next_tokens = branch;
                        break;
                    }
                }
                results.push(if_directives_whitelist(next_tokens))
            }
        }
    }
    results.join("\n")
}

fn check_condition(condition: &str) -> bool {
    let condition = condition
        .split_once("//")
        .map(|x| x.0)
        .unwrap_or(condition)
        .trim();
    /// Features from the migration plan that should be included (code kept).
    const FEATURE_WHITELIST: &[&str] = &[
        "SYS_LITTLE_ENDIAN",
        "RANGECHECK",
        "ORIGCODE",
        "DOOM_GENERIC",
        "( __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__ )",
        "1",
    ];

    /// Features that should be excluded (take #else branch or drop block).
    const FEATURE_BLACKLIST: &[&str] = &[
        "FEATURE_WAD_MERGE",
        "FEATURE_DEHACKED",
        "FEATURE_MULTIPLAYER",
        "FEATURE_SOUND",
        "HAVE_LIBPNG",
        "HAVE_MMAP",
        "HAVE_LIBZ",
        "RANGECHECKING",
        "CHECK_MUS_HEADER",
        "STANDALONE",
        "_WIN32",
        "__MACOSX__",
        "__DJGPP__",
        "__GNUC__",
        "__MSC_VER",
        "__cplusplus",
        "CMAP256",
        "SYS_BIG_ENDIAN",
        "defined(_WIN32) || defined(__DJGPP__)",
        "defined(_WIN32) && !defined(_WIN32_WCE)",
        "defined(_MSC_VER) && !defined(__cplusplus)",
        "defined(FEATURE_SOUND) && !defined(__DJGPP__)",
        "0",
        "( __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__ )",
        "!defined(_WIN32) && !defined(__MACOSX__) && !defined(__DJGPP__)",
    ];
    if FEATURE_WHITELIST.contains(&condition) {
        true
    } else if FEATURE_BLACKLIST.contains(&condition) {
        false
    } else {
        todo!("{condition}")
    }
}
