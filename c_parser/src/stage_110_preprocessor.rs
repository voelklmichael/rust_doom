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

fn is_include_guard_pattern(sym: &str) -> bool {
    let s = sym.trim();
    s.starts_with("__") && s.ends_with("__")
}

fn is_include_guard(sym: &str) -> bool {
    let s = sym.trim();
    INCLUDE_GUARD_LIST.contains(&s) || is_include_guard_pattern(s)
}

fn is_symbol_defined(sym: &str) -> bool {
    let s = sym.trim();
    FEATURE_WHITELIST.contains(&s) || FEATURE_BLACKLIST.contains(&s) || is_include_guard(s)
}

fn is_whitelisted(sym: &str) -> bool {
    let s = sym.trim();
    if FEATURE_BLACKLIST.contains(&s) {
        return false;
    }
    FEATURE_WHITELIST.contains(&s) || is_include_guard(s)
}

fn ensure_symbol_defined(sym: &str) {
    if !is_symbol_defined(sym) {
        panic!(
            "preprocessor symbol `{}` is not defined (add to FEATURE_WHITELIST, FEATURE_BLACKLIST, or INCLUDE_GUARD_LIST)",
            sym.trim()
        );
    }
}

/// Evaluate #if / #elif expression for known symbols. Returns true/false.
/// Handles: bare identifier, defined(X), literal 0/1. Unknown -> false.
fn eval_preprocessor_expr(rest: &str) -> bool {
    let rest = rest.trim();
    if rest.is_empty() {
        return false;
    }
    if rest == "0" {
        return false;
    }
    if rest == "1" {
        return true;
    }
    if let Some(inner) = rest.strip_prefix("defined(") {
        let inner = inner.trim_end_matches(')').trim();
        let sym = inner.split_ascii_whitespace().next().unwrap_or("");
        return is_symbol_defined(sym);
    }
    if let Some(inner) = rest.strip_prefix("!defined(") {
        let inner = inner.trim_end_matches(')').trim();
        let sym = inner.split_ascii_whitespace().next().unwrap_or("");
        return !is_symbol_defined(sym);
    }
    if let Some(inner) = rest.strip_prefix("! defined(") {
        let inner = inner.trim_end_matches(')').trim();
        let sym = inner.split_ascii_whitespace().next().unwrap_or("");
        return !is_symbol_defined(sym);
    }
    if rest.starts_with('!') {
        let inner = rest[1..].trim();
        return !eval_preprocessor_expr(inner);
    }
    let sym = rest.split_ascii_whitespace().next().unwrap_or("");
    if sym.is_empty() {
        return false;
    }
    is_whitelisted(sym)
}

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
                        ensure_symbol_defined(&sym);
                        let (then_content, else_content, consumed) =
                            read_conditional_block(content, i);
                        i = consumed;
                        let chosen = if is_whitelisted(&sym) {
                            then_content
                        } else {
                            // Blacklisted: X not defined → take else branch
                            else_content
                        };
                        result.extend(Self::parse_content(&chosen));
                    }
                    "ifndef" => {
                        let sym = rest
                            .split_ascii_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_string();
                        ensure_symbol_defined(&sym);
                        let (then_content, else_content, consumed) =
                            read_conditional_block(content, i);
                        i = consumed;
                        let chosen = if is_whitelisted(&sym) {
                            // Whitelisted: X defined → #ifndef false → take else branch
                            else_content
                        } else {
                            // Blacklisted: X not defined → #ifndef true → take then branch
                            then_content
                        };
                        result.extend(Self::parse_content(&chosen));
                    }
                    "if" | "elif" => {
                        // Evaluate #if/#elif exprs (known symbols, defined(), 0/1)
                        let (branch_content, consumed) =
                            read_conditional_block_evaluated(content, i);
                        i = consumed;
                        result.extend(Self::parse_content(&branch_content));
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
            "ifdef" | "ifndef" | "if" => {
                depth += 1;
                if depth > 1 {
                    let mut j = line_start;
                    while j < bytes.len() && bytes[j] != b'\n' {
                        j += 1;
                    }
                    let line = std::str::from_utf8(&bytes[line_start..j]).unwrap_or("");
                    let line_with_nl = format!("{line}\n");
                    if in_else {
                        else_content.push_str(&line_with_nl);
                    } else {
                        then_content.push_str(&line_with_nl);
                    }
                    i = j;
                    if i < bytes.len() {
                        i += 1;
                    }
                } else {
                    while i < bytes.len() && bytes[i] != b'\n' {
                        i += 1;
                    }
                    if i < bytes.len() {
                        i += 1;
                    }
                }
            }
            "endif" => {
                depth -= 1;
                if depth >= 1 {
                    let mut j = line_start;
                    while j < bytes.len() && bytes[j] != b'\n' {
                        j += 1;
                    }
                    let line = std::str::from_utf8(&bytes[line_start..j]).unwrap_or("");
                    let line_with_nl = format!("{line}\n");
                    if in_else {
                        else_content.push_str(&line_with_nl);
                    } else {
                        then_content.push_str(&line_with_nl);
                    }
                    i = j;
                }
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
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
            "else" | "elif" if depth > 1 => {
                let mut j = line_start;
                while j < bytes.len() && bytes[j] != b'\n' {
                    j += 1;
                }
                let line = std::str::from_utf8(&bytes[line_start..j]).unwrap_or("");
                let line_with_nl = format!("{line}\n");
                if in_else {
                    else_content.push_str(&line_with_nl);
                } else {
                    then_content.push_str(&line_with_nl);
                }
                i = j;
                if i < bytes.len() {
                    i += 1;
                }
            }
            _ => {
                if depth > 1 {
                    let mut j = i;
                    while j < bytes.len() && bytes[j] != b'\n' {
                        j += 1;
                    }
                    let line = std::str::from_utf8(&bytes[line_start..j]).unwrap_or("");
                    let line_with_nl = format!("{line}\n");
                    if in_else {
                        else_content.push_str(&line_with_nl);
                    } else {
                        then_content.push_str(&line_with_nl);
                    }
                    i = j;
                }
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

/// For #if/#elif: evaluate each condition and return the first matching branch.
/// If none match, returns the #else branch (or empty if no #else).
fn read_conditional_block_evaluated(content: &str, start: usize) -> (String, usize) {
    let bytes = content.as_bytes();
    let mut i = start;
    let mut depth = 1u32;
    let mut result_content = String::new();
    let mut collecting = false;

    while i < bytes.len() && depth > 0 {
        let line_start = i;
        while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        if bytes[i] != b'#' {
            if collecting && depth >= 1 {
                let mut j = i;
                while j < bytes.len() && bytes[j] != b'\n' {
                    j += 1;
                }
                let line = std::str::from_utf8(&bytes[line_start..j]).unwrap_or("");
                result_content.push_str(&format!("{line}\n"));
                i = j;
                if i < bytes.len() {
                    i += 1;
                }
            } else {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
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
        while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
            i += 1;
        }
        let rest_start = i;
        while i < bytes.len() && bytes[i] != b'\n' {
            i += 1;
        }
        let rest = std::str::from_utf8(&bytes[rest_start..i]).unwrap_or("");
        if i < bytes.len() {
            i += 1;
        }

        match directive {
            "if" if depth == 1 => {
                if eval_preprocessor_expr(rest) {
                    collecting = true;
                }
            }
            "elif" if depth == 1 => {
                if !collecting && eval_preprocessor_expr(rest) {
                    collecting = true;
                }
            }
            "else" if depth == 1 => {
                collecting = !collecting;
            }
            "endif" if depth == 1 => break,
            "ifdef" | "ifndef" | "if" => {
                depth += 1;
                if collecting && depth > 1 {
                    let mut j = line_start;
                    while j < bytes.len() && bytes[j] != b'\n' {
                        j += 1;
                    }
                    result_content.push_str(&format!(
                        "{}\n",
                        std::str::from_utf8(&bytes[line_start..j]).unwrap_or("")
                    ));
                    i = line_start;
                    while i < bytes.len() && bytes[i] != b'\n' {
                        i += 1;
                    }
                    if i < bytes.len() {
                        i += 1;
                    }
                }
            }
            "endif" => {
                depth -= 1;
                if collecting && depth >= 1 {
                    let mut j = line_start;
                    while j < bytes.len() && bytes[j] != b'\n' {
                        j += 1;
                    }
                    result_content.push_str(&format!(
                        "{}\n",
                        std::str::from_utf8(&bytes[line_start..j]).unwrap_or("")
                    ));
                    i = j;
                    if i < bytes.len() {
                        i += 1;
                    }
                }
            }
            "else" | "elif" if depth > 1 => {
                if collecting {
                    let mut j = line_start;
                    while j < bytes.len() && bytes[j] != b'\n' {
                        j += 1;
                    }
                    result_content.push_str(&format!(
                        "{}\n",
                        std::str::from_utf8(&bytes[line_start..j]).unwrap_or("")
                    ));
                    i = j;
                    if i < bytes.len() {
                        i += 1;
                    }
                }
            }
            _ => {
                if collecting && depth > 1 {
                    let mut j = line_start;
                    while j < bytes.len() && bytes[j] != b'\n' {
                        j += 1;
                    }
                    result_content.push_str(&format!(
                        "{}\n",
                        std::str::from_utf8(&bytes[line_start..j]).unwrap_or("")
                    ));
                    i = j;
                    if i < bytes.len() {
                        i += 1;
                    }
                }
            }
        }
    }

    (result_content, i)
}

#[cfg(test)]
mod tests {
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

    fn parse_conditionals(content: &str) -> Vec<Stage110Preprocessor> {
        let stage100 = crate::stage_100_comments::Stage100Comments::parse(content);
        Stage110Preprocessor::parse(stage100)
    }

    fn emitted_code(result: &[Stage110Preprocessor]) -> String {
        result
            .iter()
            .filter_map(|r| match r {
                Stage110Preprocessor::Code(c) => Some(c.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("")
    }

    #[test]
    fn test_ifdef_blacklisted_takes_else() {
        let content = r#"#ifdef FEATURE_SOUND
sound_then
#else
sound_else
#endif"#;
        let result = parse_conditionals(content);
        let code = emitted_code(&result);
        assert!(!code.contains("sound_then"), "blacklisted should take else");
        assert!(code.contains("sound_else"), "blacklisted should take else");
    }

    #[test]
    fn test_ifdef_whitelisted_takes_then() {
        let content = r#"#ifdef DOOM_GENERIC
doom_then
#else
doom_else
#endif"#;
        let result = parse_conditionals(content);
        let code = emitted_code(&result);
        assert!(code.contains("doom_then"), "whitelisted should take then");
        assert!(!code.contains("doom_else"), "whitelisted should take then");
    }

    #[test]
    fn test_ifndef_blacklisted_takes_then() {
        let content = r#"#ifndef FEATURE_SOUND
no_sound_then
#else
no_sound_else
#endif"#;
        let result = parse_conditionals(content);
        let code = emitted_code(&result);
        assert!(
            code.contains("no_sound_then"),
            "blacklisted = not defined, #ifndef true -> then"
        );
        assert!(!code.contains("no_sound_else"), "blacklisted = not defined");
    }

    #[test]
    fn test_ifndef_whitelisted_takes_else() {
        let content = r#"#ifndef DOOM_GENERIC
doom_not_then
#else
doom_not_else
#endif"#;
        let result = parse_conditionals(content);
        let code = emitted_code(&result);
        assert!(
            !code.contains("doom_not_then"),
            "whitelisted = defined, #ifndef false"
        );
        assert!(
            code.contains("doom_not_else"),
            "whitelisted should take else"
        );
    }

    #[test]
    fn test_if_unevaluated_takes_else() {
        let content = r#"#if FEATURE_SOUND
if_then
#else
if_else
#endif"#;
        let result = parse_conditionals(content);
        let code = emitted_code(&result);
        assert!(
            !code.contains("if_then"),
            "#if unevaluated -> false -> else"
        );
        assert!(
            code.contains("if_else"),
            "#if unevaluated takes else branch"
        );
    }

    #[test]
    fn test_elif_evaluates_known_symbols() {
        // #if FEATURE_SOUND: blacklisted (false) -> skip
        // #elif DOOM_GENERIC: whitelisted (true) -> take this branch
        let content = r#"#if FEATURE_SOUND
aaa_if_then
#elif DOOM_GENERIC
bbb_elif_then
#else
ccc_else_only
#endif"#;
        let result = parse_conditionals(content);
        let code = emitted_code(&result);
        assert!(!code.contains("aaa_if_then"), "FEATURE_SOUND blacklisted");
        assert!(
            code.contains("bbb_elif_then"),
            "DOOM_GENERIC is defined -> take #elif branch"
        );
        assert!(
            !code.contains("ccc_else_only"),
            "skip #else when #elif matches"
        );
    }

    #[test]
    fn test_if_elif_elif_else_only_final() {
        // Multiple #elif -> take ONLY the final #else
        let content = r#"#if A
branch_a
#elif B
branch_b
#elif C
branch_c
#else
branch_final
#endif"#;
        let result = parse_conditionals(content);
        let code = emitted_code(&result);
        assert!(!code.contains("branch_a"));
        assert!(!code.contains("branch_b"));
        assert!(!code.contains("branch_c"));
        assert!(code.contains("branch_final"), "only final #else");
    }

    #[test]
    fn test_nested_ifdef_outer_blacklisted() {
        let content = r#"#ifdef FEATURE_SOUND
outer_then
#else
#ifdef DOOM_GENERIC
inner_then
#else
inner_else
#endif
#endif"#;
        let result = parse_conditionals(content);
        let code = emitted_code(&result);
        assert!(!code.contains("outer_then"), "outer blacklisted -> else");
        assert!(
            code.contains("inner_then"),
            "nested: inner whitelisted -> then"
        );
        assert!(!code.contains("inner_else"), "nested: inner whitelisted");
    }

    #[test]
    fn test_nested_ifdef_both_blacklisted() {
        let content = r#"#ifdef FEATURE_SOUND
outer_then
#else
#ifdef FEATURE_DEHACKED
inner_then
#else
inner_else
#endif
#endif"#;
        let result = parse_conditionals(content);
        let code = emitted_code(&result);
        assert!(!code.contains("outer_then"));
        assert!(!code.contains("inner_then"));
        assert!(
            code.contains("inner_else"),
            "both blacklisted -> else of outer, else of inner"
        );
    }

    #[test]
    fn test_nested_ifdef_outer_whitelisted_inner_blacklisted() {
        let content = r#"#ifdef DOOM_GENERIC
#ifdef FEATURE_SOUND
inner_then
#else
inner_else
#endif
#else
outer_else
#endif"#;
        let result = parse_conditionals(content);
        let code = emitted_code(&result);
        assert!(!code.contains("outer_else"), "outer whitelisted");
        assert!(!code.contains("inner_then"), "inner blacklisted");
        assert!(code.contains("inner_else"), "inner blacklisted -> else");
    }
}
