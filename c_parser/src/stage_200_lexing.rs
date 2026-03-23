#[derive(Debug, Clone, PartialEq)]
pub enum LexedToken {
    // Keywords
    Keyword(Keyword),

    // Identifiers
    Identifier(String),

    // Literals
    IntegerLiteral {
        value: String,
        suffix: Option<String>,
    },
    FloatLiteral {
        value: String,
        suffix: Option<String>,
    },
    CharLiteral(String),
    StringLiteral(String),

    // Operators and punctuators (single and multi-char)
    Punctuator(String),

    // Preprocessor
    Hash,

    // Comments (with string content)
    LineComment(String),
    BlockComment(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

const KEYWORDS: &[(&str, Keyword)] = &[
    ("auto", Keyword::Auto),
    ("break", Keyword::Break),
    ("case", Keyword::Case),
    ("char", Keyword::Char),
    ("const", Keyword::Const),
    ("continue", Keyword::Continue),
    ("default", Keyword::Default),
    ("do", Keyword::Do),
    ("double", Keyword::Double),
    ("else", Keyword::Else),
    ("enum", Keyword::Enum),
    ("extern", Keyword::Extern),
    ("float", Keyword::Float),
    ("for", Keyword::For),
    ("goto", Keyword::Goto),
    ("if", Keyword::If),
    ("inline", Keyword::Inline),
    ("int", Keyword::Int),
    ("long", Keyword::Long),
    ("register", Keyword::Register),
    ("return", Keyword::Return),
    ("short", Keyword::Short),
    ("signed", Keyword::Signed),
    ("sizeof", Keyword::Sizeof),
    ("static", Keyword::Static),
    ("struct", Keyword::Struct),
    ("switch", Keyword::Switch),
    ("typedef", Keyword::Typedef),
    ("union", Keyword::Union),
    ("unsigned", Keyword::Unsigned),
    ("void", Keyword::Void),
    ("volatile", Keyword::Volatile),
    ("while", Keyword::While),
];

const PUNCTUATORS: &[&str] = &[
    "...", ">>=", "<<=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "==", "!=", "<=", ">=",
    "->", "++", "--", "<<", ">>", "&&", "||", "<:", ":>", "<%", "%>", "%:", "%:%:",
];

pub(crate) fn lexing(whitelisted: String) -> Vec<LexedToken> {
    let whitelisted = whitelisted.replace("\\\n", "");
    let mut tokens = Vec::new();
    let bytes = whitelisted.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Skip whitespace
        if bytes[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }

        // Line comment
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
            let start = i + 2;
            i += 2;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            let content = String::from_utf8(bytes[start..i].to_vec()).unwrap();
            tokens.push(LexedToken::LineComment(content));
            continue;
        }

        // Block comment
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
            let start = i + 2;
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            let content = String::from_utf8(bytes[start..i].to_vec()).unwrap();
            tokens.push(LexedToken::BlockComment(content));
            if i + 1 < bytes.len() {
                i += 2;
            }
            continue;
        }

        // String literal
        if bytes[i] == b'"' {
            i += 1;
            let mut s = String::new();
            while i < bytes.len() {
                match bytes[i] {
                    b'\\' if i + 1 < bytes.len() => {
                        i += 1;
                        let c = match bytes[i] {
                            b'n' => '\n',
                            b't' => '\t',
                            b'r' => '\r',
                            b'\\' => '\\',
                            b'"' => '"',
                            b'\'' => '\'',
                            b'0' => '\0',
                            b'x' if i + 2 < bytes.len() => {
                                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("0");
                                i += 2;
                                char::from_u32(u32::from_str_radix(hex, 16).unwrap_or(0))
                                    .unwrap_or('\0')
                            }
                            _ => bytes[i] as char,
                        };
                        s.push(c);
                        i += 1;
                    }
                    b'"' => {
                        i += 1;
                        break;
                    }
                    _ => {
                        s.push(bytes[i] as char);
                        i += 1;
                    }
                }
            }
            tokens.push(LexedToken::StringLiteral(s));
            continue;
        }

        // Char literal
        if bytes[i] == b'\'' {
            i += 1;
            let mut c = '\0';
            if i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 1;
                    c = match bytes[i] {
                        b'n' => '\n',
                        b't' => '\t',
                        b'r' => '\r',
                        b'\\' => '\\',
                        b'\'' => '\'',
                        b'0' => '\0',
                        _ => bytes[i] as char,
                    };
                    i += 1;
                } else {
                    c = bytes[i] as char;
                    i += 1;
                }
            }
            if i < bytes.len() && bytes[i] == b'\'' {
                i += 1;
            }
            tokens.push(LexedToken::CharLiteral(c.to_string()));
            continue;
        }

        // Preprocessor
        if bytes[i] == b'#' {
            tokens.push(LexedToken::Hash);
            i += 1;
            continue;
        }

        // Multi-char punctuator (try longest first)
        let mut found = false;
        for p in PUNCTUATORS {
            if i + p.len() <= bytes.len() && &bytes[i..i + p.len()] == p.as_bytes() {
                tokens.push(LexedToken::Punctuator(p.to_string()));
                i += p.len();
                found = true;
                break;
            }
        }
        if found {
            continue;
        }

        // Single-char punctuator
        if is_punctuator(bytes[i]) {
            tokens.push(LexedToken::Punctuator((bytes[i] as char).to_string()));
            i += 1;
            continue;
        }

        // Identifier or keyword
        if is_ident_start(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }
            let s = String::from_utf8(bytes[start..i].to_vec()).unwrap();
            if let Some(&(_, kw)) = KEYWORDS.iter().find(|(k, _)| *k == s) {
                tokens.push(LexedToken::Keyword(kw));
            } else {
                tokens.push(LexedToken::Identifier(s));
            }
            continue;
        }

        // Numeric literal
        if bytes[i].is_ascii_digit()
            || (bytes[i] == b'.' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit())
        {
            let start = i;

            // Hex: 0x or 0X
            if i + 2 < bytes.len()
                && bytes[i] == b'0'
                && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X')
            {
                i += 2;
                while i < bytes.len() && bytes[i].is_ascii_hexdigit() {
                    i += 1;
                }
                let suffix_start = i;
                while i < bytes.len()
                    && (bytes[i] == b'u'
                        || bytes[i] == b'U'
                        || bytes[i] == b'l'
                        || bytes[i] == b'L')
                {
                    i += 1;
                }
                let value = String::from_utf8(bytes[start..suffix_start].to_vec()).unwrap();
                let suffix = if suffix_start < i {
                    Some(String::from_utf8(bytes[suffix_start..i].to_vec()).unwrap())
                } else {
                    None
                };
                tokens.push(LexedToken::IntegerLiteral { value, suffix });
                continue;
            }

            // Octal: 0[0-7]*
            if bytes[i] == b'0'
                && i + 1 < bytes.len()
                && (bytes[i + 1] as char).is_ascii_digit()
                && bytes[i + 1] < b'8'
            {
                i += 1;
                while i < bytes.len() && bytes[i] >= b'0' && bytes[i] <= b'7' {
                    i += 1;
                }
                let suffix_start = i;
                while i < bytes.len()
                    && (bytes[i] == b'u'
                        || bytes[i] == b'U'
                        || bytes[i] == b'l'
                        || bytes[i] == b'L')
                {
                    i += 1;
                }
                let value = String::from_utf8(bytes[start..suffix_start].to_vec()).unwrap();
                let suffix = if suffix_start < i {
                    Some(String::from_utf8(bytes[suffix_start..i].to_vec()).unwrap())
                } else {
                    None
                };
                tokens.push(LexedToken::IntegerLiteral { value, suffix });
                continue;
            }

            // Decimal or float
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            let mut is_float = false;

            if i < bytes.len() && bytes[i] == b'.' {
                is_float = true;
                i += 1;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    i += 1;
                }
            }

            if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
                is_float = true;
                i += 1;
                if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
                    i += 1;
                }
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    i += 1;
                }
            }

            if i < bytes.len()
                && (bytes[i] == b'f' || bytes[i] == b'F' || bytes[i] == b'l' || bytes[i] == b'L')
            {
                is_float = true;
                i += 1;
            }

            let value = String::from_utf8(bytes[start..i].to_vec()).unwrap();
            if is_float {
                tokens.push(LexedToken::FloatLiteral {
                    value: value.clone(),
                    suffix: None,
                });
            } else {
                let suffix_start = i;
                while i < bytes.len()
                    && (bytes[i] == b'u'
                        || bytes[i] == b'U'
                        || bytes[i] == b'l'
                        || bytes[i] == b'L')
                {
                    i += 1;
                }
                let suffix = if suffix_start < i {
                    Some(String::from_utf8(bytes[suffix_start..i].to_vec()).unwrap())
                } else {
                    None
                };
                tokens.push(LexedToken::IntegerLiteral { value, suffix });
            }
            continue;
        }

        panic!(
            "Unknown character: {}",
            str::from_utf8(&bytes[i..i + 100]).unwrap()
        );
    }

    tokens
}

fn is_ident_start(b: u8) -> bool {
    (b as char).is_alphabetic() || b == b'_'
}

fn is_ident_continue(b: u8) -> bool {
    (b as char).is_alphanumeric() || b == b'_'
}

fn is_punctuator(b: u8) -> bool {
    matches!(
        b,
        b'[' | b']'
            | b'('
            | b')'
            | b'{'
            | b'}'
            | b'.'
            | b'-'
            | b'+'
            | b'*'
            | b'&'
            | b'/'
            | b'%'
            | b'<'
            | b'>'
            | b'^'
            | b'|'
            | b'!'
            | b'?'
            | b':'
            | b';'
            | b','
            | b'='
            | b'~'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_simple_declaration() {
        let tokens = lexing("int x = 1;".to_string());
        assert_eq!(
            tokens,
            vec![
                LexedToken::Keyword(Keyword::Int),
                LexedToken::Identifier("x".to_string()),
                LexedToken::Punctuator("=".to_string()),
                LexedToken::IntegerLiteral {
                    value: "1".to_string(),
                    suffix: None
                },
                LexedToken::Punctuator(";".to_string()),
            ]
        );
    }

    #[test]
    fn test_lex_string_literal() {
        let tokens = lexing(r#""hello""#.to_string());
        assert_eq!(tokens, vec![LexedToken::StringLiteral("hello".to_string())]);
    }

    #[test]
    fn test_lex_emits_comment_tokens() {
        let tokens = lexing("int x; // comment\nreturn 0;".to_string());
        assert_eq!(
            tokens,
            vec![
                LexedToken::Keyword(Keyword::Int),
                LexedToken::Identifier("x".to_string()),
                LexedToken::Punctuator(";".to_string()),
                LexedToken::LineComment(" comment".to_string()),
                LexedToken::Keyword(Keyword::Return),
                LexedToken::IntegerLiteral {
                    value: "0".to_string(),
                    suffix: None
                },
                LexedToken::Punctuator(";".to_string()),
            ]
        );
    }

    #[test]
    fn test_lex_block_comment() {
        let tokens = lexing("x /* inner */ y".to_string());
        assert_eq!(
            tokens,
            vec![
                LexedToken::Identifier("x".to_string()),
                LexedToken::BlockComment(" inner ".to_string()),
                LexedToken::Identifier("y".to_string()),
            ]
        );
    }

    #[test]
    fn test_lex_preprocessor() {
        let tokens = lexing("#include <stdio.h>".to_string());
        assert!(tokens.first() == Some(&LexedToken::Hash));
        assert_eq!(tokens[1], LexedToken::Identifier("include".to_string()));
    }

    #[test]
    fn test_lex_struct() {
        let tokens = lexing("static const iwad_t iwads[] = {".to_string());
        assert_eq!(tokens[0], LexedToken::Keyword(Keyword::Static));
        assert_eq!(tokens[1], LexedToken::Keyword(Keyword::Const));
        assert_eq!(tokens[2], LexedToken::Identifier("iwad_t".to_string()));
    }
}
