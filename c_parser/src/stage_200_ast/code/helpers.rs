//! Shared parsing utilities.

pub(super) fn skip_whitespace(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\r') {
        i += 1;
    }
    i
}

pub(super) fn match_prefix(bytes: &[u8], i: &mut usize, prefix: &str) -> bool {
    let p = prefix.as_bytes();
    if *i + p.len() <= bytes.len() {
        if bytes[*i..*i + p.len()] == *p {
            let end = *i + p.len();
            if end >= bytes.len() || !is_ident_byte(bytes[end]) {
                *i = end;
                return true;
            }
        }
    }
    false
}

pub(super) fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

pub(super) fn find_next_byte(bytes: &[u8], start: usize, needle: u8) -> Option<usize> {
    for j in start..bytes.len() {
        if bytes[j] == needle {
            return Some(j);
        }
    }
    None
}

pub(super) fn advance_past_balanced_braces(bytes: &[u8], i: &mut usize) {
    if *i >= bytes.len() || bytes[*i] != b'{' {
        return;
    }
    *i += 1;
    let mut depth: u32 = 1;
    while *i < bytes.len() && depth > 0 {
        match bytes[*i] {
            b'{' => {
                depth = depth.saturating_add(1);
                *i += 1;
            }
            b'}' => {
                depth = depth.saturating_sub(1);
                *i += 1;
                if depth == 0 {
                    return;
                }
            }
            b'"' => {
                *i += 1;
                while *i < bytes.len() && bytes[*i] != b'"' {
                    if bytes[*i] == b'\\' {
                        *i += 1;
                    }
                    *i += 1;
                }
                if *i < bytes.len() {
                    *i += 1;
                }
            }
            b'\'' => {
                *i += 1;
                while *i < bytes.len() && bytes[*i] != b'\'' {
                    if bytes[*i] == b'\\' {
                        *i += 1;
                    }
                    *i += 1;
                }
                if *i < bytes.len() {
                    *i += 1;
                }
            }
            _ => *i += 1,
        }
    }
}

pub(super) fn find_next(bytes: &[u8], start: usize, needle: u8) -> Option<usize> {
    let mut i = start;
    let mut in_string = false;
    let mut in_char = false;
    let mut depth = 0i32;
    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if b == b'"' && (i == 0 || bytes[i - 1] != b'\\') {
                in_string = false;
            }
            i += 1;
        } else if in_char {
            if b == b'\'' && (i == 0 || bytes[i - 1] != b'\\') {
                in_char = false;
            }
            i += 1;
        } else if b == b'"' {
            in_string = true;
            i += 1;
        } else if b == b'\'' {
            in_char = true;
            i += 1;
        } else if b == b'(' || b == b'[' {
            depth += 1;
            if depth == 1 && b == needle {
                return Some(i);
            }
            i += 1;
        } else if b == b')' || b == b']' {
            depth -= 1;
            i += 1;
        } else if depth == 0 && b == needle {
            return Some(i);
        } else {
            i += 1;
        }
    }
    None
}

pub(super) fn split_return_type_and_name(s: &str) -> Option<(String, String)> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let bytes = s.as_bytes();
    let mut last_ident_start = None;
    let mut last_ident_end = 0;
    let mut i = 0;
    while i < bytes.len() {
        if is_ident_byte(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_ident_byte(bytes[i]) {
                i += 1;
            }
            last_ident_start = Some(start);
            last_ident_end = i;
        } else if bytes[i] == b'*' || bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n' {
            i += 1;
        } else {
            i += 1;
        }
    }
    let name_start = last_ident_start?;
    let name = String::from_utf8_lossy(&bytes[name_start..last_ident_end]).to_string();
    let return_type = String::from_utf8_lossy(&bytes[..name_start])
        .trim()
        .to_string();
    if name.is_empty() {
        None
    } else {
        Some((return_type, name))
    }
}

pub(super) fn read_balanced_parens(bytes: &[u8], i: &mut usize) -> Option<Vec<u8>> {
    if *i >= bytes.len() || bytes[*i] != b'(' {
        return None;
    }
    let start = *i;
    *i += 1;
    let mut depth: i32 = 1;
    while *i < bytes.len() && depth > 0 {
        match bytes[*i] {
            b'(' => {
                depth = depth.saturating_add(1);
                if depth > 1_000_000 {
                    return None;
                }
            }
            b')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let content = bytes[start + 1..*i].to_vec();
                    *i += 1;
                    return Some(content);
                }
                *i += 1;
            }
            b'"' => {
                *i += 1;
                while *i < bytes.len() && bytes[*i] != b'"' {
                    if bytes[*i] == b'\\' {
                        *i += 1;
                    }
                    *i += 1;
                }
                if *i < bytes.len() {
                    *i += 1;
                }
            }
            b'\'' => {
                *i += 1;
                while *i < bytes.len() && bytes[*i] != b'\'' {
                    if bytes[*i] == b'\\' {
                        *i += 1;
                    }
                    *i += 1;
                }
                if *i < bytes.len() {
                    *i += 1;
                }
            }
            _ => *i += 1,
        }
    }
    None
}

pub(super) fn read_until_semicolon(bytes: &[u8], i: &mut usize) -> String {
    let start = *i;
    let mut in_string = false;
    let mut in_char = false;
    while *i < bytes.len() {
        let b = bytes[*i];
        if in_string {
            if b == b'"' && (*i == start || bytes[*i - 1] != b'\\') {
                in_string = false;
            }
            *i += 1;
        } else if in_char {
            if b == b'\'' && (*i == start || bytes[*i - 1] != b'\\') {
                in_char = false;
            }
            *i += 1;
        } else if b == b';' {
            let s = String::from_utf8_lossy(&bytes[start..*i]).to_string();
            *i += 1;
            return s;
        } else if b == b'"' {
            in_string = true;
            *i += 1;
        } else if b == b'\'' {
            in_char = true;
            *i += 1;
        } else if b == b'(' || b == b'{' {
            *i += 1;
        } else if b == b')' || b == b'}' {
            *i += 1;
        } else {
            *i += 1;
        }
    }
    String::from_utf8_lossy(&bytes[start..]).to_string()
}

pub(super) fn is_valid_simple_type_name(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}
