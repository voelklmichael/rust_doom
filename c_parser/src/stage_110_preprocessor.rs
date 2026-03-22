use super::stage_100_comments::Stage100Comments;

/// Features from the migration plan that should be included (code kept).
const FEATURE_WHITELIST: &[&str] = &[
    "SYS_LITTLE_ENDIAN",
    "RANGECHECK",
    "ORIGCODE",
    "DOOM_GENERIC",
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
];

/// Include guards to always take the "then" branch (header content).
/// One entry per header file in doomgeneric/ (config.h has no include guard).
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

#[derive(PartialEq, Debug)]
pub(crate) enum Stage110Preprocessor {
    Comment(String),
    Code(String),
    Include {
        path: String,
        is_system: bool,
    },
    Define {
        name: String,
        params: Option<Vec<String>>,
        value: String,
    },
    Undef {
        name: String,
    },
}

impl Stage110Preprocessor {
    pub fn parse(previous: Vec<Stage100Comments>) -> Vec<Self> {
        let mut output = Vec::new();
        for item in previous {
            output.extend(Self::parse_single(item));
        }
        output
    }

    fn parse_single(previous: Stage100Comments) -> Vec<Self> {
        match previous {
            Stage100Comments::Comment(x) => vec![Self::Comment(x)],
            Stage100Comments::NonComment(x) => Self::parse_content(&x),
        }
    }

    fn parse_content(content: &str) -> Vec<Self> {
        let mut result = Vec::new();
        let mut i = 0;
        let bytes = content.as_bytes();

        while i < bytes.len() {
            // Skip whitespace, but track line start for # detection
            let line_start = i;
            while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
                i += 1;
            }
            if i >= bytes.len() {
                break;
            }

            if bytes[i] == b'#' {
                i += 1;
                while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
                    i += 1;
                }
                let directive_start = i;
                while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
                    i += 1;
                }
                let directive = std::str::from_utf8(&bytes[directive_start..i]).unwrap_or("");
                while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
                    i += 1;
                }
                let rest_start = i;
                let rest = read_line_with_continuation(content, rest_start, &mut i);

                match directive {
                    "include" => {
                        let (path, is_system) = if rest.starts_with('<') {
                            (
                                rest.trim_start_matches('<')
                                    .trim_end_matches('>')
                                    .trim()
                                    .to_string(),
                                true,
                            )
                        } else if rest.starts_with('"') {
                            (rest.trim_matches('"').trim().to_string(), false)
                        } else {
                            (rest.to_string(), false)
                        };
                        result.push(Self::Include { path, is_system });
                    }
                    "define" => {
                        let (name, params, value) = parse_define_rest(&rest);
                        if params.is_none()
                            && value.is_empty()
                            && !INCLUDE_GUARD_LIST.contains(&name.trim())
                        {
                            continue;
                        }
                        result.push(Self::Define {
                            name,
                            params,
                            value,
                        });
                    }
                    "undef" => {
                        let name = rest
                            .split_ascii_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_string();
                        result.push(Self::Undef { name });
                    }
                    "ifdef" => {
                        let sym = rest
                            .split_ascii_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_string();
                        if FEATURE_BLACKLIST.contains(&sym.trim()) {
                            panic!("Skip to #endif, correct for nested ifs");
                            continue;
                        } else if !FEATURE_WHITELIST.contains(&sym.trim()) {
                            panic!("Continue from the next line?")
                        } else {
                            panic!("Symbol {} is not defined in any list", sym.trim());
                        }

                        let (then_content, else_content, consumed) =
                            read_conditional_block(content, i);
                        i = consumed;
                        let chosen = if include { then_content } else { else_content };
                        result.extend(Self::parse_content(&chosen));
                    }
                    "ifndef" => {
                        let sym = rest
                            .split_ascii_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_string();
                        if FEATURE_WHITELIST.contains(&sym.trim()) {
                            panic!("Skip to #endif, correct for nested ifs");
                            continue;
                        } else if !FEATURE_BLACKLIST.contains(&sym.trim()) {
                            panic!("Continue from the next line?")
                        } else {
                            panic!("Symbol {} is not defined in any list", sym.trim());
                        }
                    }
                    "if" | "elif" => {
                        // For #if expr, we don't evaluate - treat as excluded (empty)
                        let (_then, _else, consumed) = read_conditional_block(content, i);
                        i = consumed;
                    }
                    "else" | "endif" => {
                        // Orphaned #else/#endif - skip
                    }
                    _ => {
                        result.push(Self::Code(format!(
                            "#{}\n",
                            std::str::from_utf8(&bytes[line_start..i.min(content.len())])
                                .unwrap_or("")
                        )));
                    }
                }
            } else {
                // Non-directive: collect until next # at line start
                let chunk_start = i;
                while i < bytes.len() {
                    if bytes[i] == b'\n' {
                        i += 1;
                        let next_line_start = i;
                        while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
                            i += 1;
                        }
                        if i < bytes.len() && bytes[i] == b'#' {
                            i = next_line_start;
                            break;
                        }
                        i = next_line_start;
                    } else {
                        i += 1;
                    }
                }
                let chunk = std::str::from_utf8(&bytes[chunk_start..i])
                    .unwrap_or("")
                    .to_string();
                if !chunk.trim().is_empty() {
                    result.push(Self::Code(chunk));
                }
            }
        }

        result
    }
}

fn parse_define_rest(rest: &str) -> (String, Option<Vec<String>>, String) {
    let rest = rest.trim();
    if rest.is_empty() {
        return (String::new(), None, String::new());
    }
    let name_end = rest
        .find(|c: char| !c.is_alphabetic() && c != '_' && !c.is_numeric())
        .unwrap_or(rest.len());
    let name = rest[..name_end].to_string();
    let after_name = rest[name_end..].trim_start();
    if name.is_empty() {
        return (String::new(), None, String::new());
    }
    if after_name.starts_with('(') {
        let paren_end = find_matching_paren(after_name, 0);
        let params_str = &after_name[1..paren_end];
        let params: Vec<String> = params_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let value = after_name[paren_end + 1..].trim().to_string();
        (name, Some(params), value)
    } else {
        let value = after_name.to_string();
        (name, None, value)
    }
}

/// Read a preprocessor line, joining continued lines (ending with \).
fn read_line_with_continuation(content: &str, start: usize, i: &mut usize) -> String {
    let bytes = content.as_bytes();
    let mut result = String::new();
    *i = start;

    loop {
        let line_start = *i;
        while *i < bytes.len() && bytes[*i] != b'\n' {
            *i += 1;
        }
        let line = std::str::from_utf8(&bytes[line_start..*i])
            .unwrap_or("")
            .to_string();
        if *i < bytes.len() {
            *i += 1; // consume newline
        }
        // Trim trailing backslash and whitespace for continuation
        let trimmed = line.trim_end();
        if trimmed.ends_with('\\') {
            let without_bs = trimmed[..trimmed.len() - 1].trim_end();
            result.push_str(without_bs);
            // Continue to next line
            if *i >= bytes.len() {
                break;
            }
        } else {
            result.push_str(&line);
            break;
        }
    }

    result.trim().to_string()
}

fn find_matching_paren(s: &str, start: usize) -> usize {
    let mut depth = 1u32; // we're already inside the opening (
    for (i, c) in s.chars().enumerate().skip(start + 1) {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return i;
                }
            }
            _ => {}
        }
    }
    s.len()
}

fn read_conditional_block(content: &str, start: usize) -> (String, String, usize) {
    let bytes = content.as_bytes();
    let mut i = start;
    let mut depth = 1u32;
    let mut then_content = String::new();
    let mut else_content = String::new();
    let mut in_else = false;

    while i < bytes.len() && depth > 0 {
        let line_start = i;
        while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        if bytes[i] != b'#' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            let end = i;
            if i < bytes.len() {
                i += 1; // consume newline
            }
            let line = std::str::from_utf8(&bytes[line_start..end]).unwrap_or("");
            let line_with_nl = format!("{line}\n");
            if in_else {
                else_content.push_str(&line_with_nl);
            } else {
                then_content.push_str(&line_with_nl);
            }
            continue;
        }

        i += 1;
        while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
            i += 1;
        }
        let dir_start = i;
        while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
            i += 1;
        }
        let directive = std::str::from_utf8(&bytes[dir_start..i]).unwrap_or("");

        match directive {
            "ifdef" | "ifndef" | "if" => depth += 1,
            "endif" => {
                depth -= 1;
                if depth == 0 {
                    while i < bytes.len() && bytes[i] != b'\n' {
                        i += 1;
                    }
                    if i < bytes.len() {
                        i += 1;
                    }
                }
            }
            "else" | "elif" if depth == 1 => {
                in_else = true;
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
            }
            _ => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
            }
        }
    }

    (then_content, else_content, i)
}

#[test]
fn test_parse_multiline_define() {
    let content = r#"
        #define NEWGAME	\
        "you can't start a new game\n"\
        "while in a network game.\n\n"PRESSKEY   "#;
    let stage100 = crate::stage_100_comments::Stage100Comments::parse(content);
    let result = Stage110Preprocessor::parse(stage100);
    dbg!(&result);
    assert_eq!(result.len(), 1);
    let Stage110Preprocessor::Define {
        name,
        params,
        value,
        ..
    } = &result[0]
    else {
        panic!("expected Define, got {:?}", result[0]);
    };
    assert_eq!(name, "NEWGAME");
    assert_eq!(params, &None);
    assert!(value.contains("you can't start a new game"));
    assert!(value.contains("while in a network game"));
    assert!(value.contains("PRESSKEY"));
}
