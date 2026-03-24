//! Parses `ExternalDecl320::Declaration` token slices into structured declarations.

use crate::stage_200_lexing::{Keyword, LexedToken};
use crate::stage_300_parsing::FunctionBody;
use crate::stage_320_parsing::{ExternalDecl320, PreprocessorDirective, TranslationUnit320};

#[derive(Debug, Clone, PartialEq)]
pub enum SpecifierPiece {
    Storage(Keyword),
    Qualifier(Keyword),
    Type(Keyword),
    Struct {
        tag: Option<String>,
        /// Tokens between `{` and `}` when a definition is present.
        fields: Option<Vec<LexedToken>>,
    },
    Union {
        tag: Option<String>,
        fields: Option<Vec<LexedToken>>,
    },
    Enum {
        tag: Option<String>,
        /// Tokens between `{` and `}` when a definition is present.
        enumerators: Option<Vec<LexedToken>>,
    },
    /// Type name from a `typedef` (or other alias) in specifier position.
    TypedefName(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclaratorWithInit {
    pub declarator: Vec<LexedToken>,
    pub initializer: Option<Vec<LexedToken>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub specifiers: Vec<SpecifierPiece>,
    pub declarators: Vec<DeclaratorWithInit>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternalDecl340 {
    Preprocessor(PreprocessorDirective),
    /// Successfully parsed declaration ending with `;`.
    Declaration(Declaration),
    /// Tokens could not be parsed as a declaration (kept for debugging / extension).
    UnparsedDeclaration(Vec<LexedToken>),
    FunctionDefinition {
        signature_tokens: Vec<LexedToken>,
        body: FunctionBody,
    },
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TranslationUnit340(pub Vec<ExternalDecl340>);

pub(crate) fn parsing_stage_340(tu: TranslationUnit320) -> TranslationUnit340 {
    TranslationUnit340(
        tu.0
            .into_iter()
            .map(|d| match d {
                ExternalDecl320::Preprocessor(p) => ExternalDecl340::Preprocessor(p),
                ExternalDecl320::Declaration(tokens) => match parse_declaration(&tokens) {
                    Some(decl) => ExternalDecl340::Declaration(decl),
                    None => ExternalDecl340::UnparsedDeclaration(tokens),
                },
                ExternalDecl320::FunctionDefinition {
                    signature_tokens,
                    body,
                } => ExternalDecl340::FunctionDefinition {
                    signature_tokens,
                    body,
                },
                ExternalDecl320::Comment(s) => ExternalDecl340::Comment(s),
            })
            .collect(),
    )
}

fn is_trivia(t: &LexedToken) -> bool {
    matches!(
        t,
        LexedToken::LineComment(_) | LexedToken::BlockComment(_) | LexedToken::Newline
    )
}

fn skip_trivia(tokens: &[LexedToken], mut i: usize) -> usize {
    while i < tokens.len() && is_trivia(&tokens[i]) {
        i += 1;
    }
    i
}

fn is_punct(t: &LexedToken, s: &str) -> bool {
    matches!(t, LexedToken::Punctuator(p) if p == s)
}

fn parse_declaration(tokens: &[LexedToken]) -> Option<Declaration> {
    let tokens = trim_trailing_trivia(tokens);
    if tokens.is_empty() {
        return None;
    }
    let last = tokens.last()?;
    if !is_punct(last, ";") {
        return None;
    }
    let without_semi = trim_trailing_trivia(&tokens[..tokens.len() - 1]);
    if without_semi.is_empty() {
        return Some(Declaration {
            specifiers: vec![],
            declarators: vec![],
        });
    }

    let (specifiers, mut i) = parse_declaration_specifiers(without_semi, 0)?;
    i = skip_trivia(without_semi, i);
    if i >= without_semi.len() {
        return Some(Declaration {
            specifiers,
            declarators: vec![],
        });
    }

    let declarators = parse_init_declarator_list(without_semi, i)?;
    Some(Declaration {
        specifiers,
        declarators,
    })
}

fn trim_trailing_trivia(tokens: &[LexedToken]) -> &[LexedToken] {
    let mut end = tokens.len();
    while end > 0 && is_trivia(&tokens[end - 1]) {
        end -= 1;
    }
    &tokens[..end]
}

fn is_storage(kw: Keyword) -> bool {
    matches!(
        kw,
        Keyword::Typedef | Keyword::Extern | Keyword::Static | Keyword::Auto | Keyword::Register
    )
}

fn is_qualifier(kw: Keyword) -> bool {
    matches!(kw, Keyword::Const | Keyword::Volatile)
}

fn is_type_keyword(kw: Keyword) -> bool {
    matches!(
        kw,
        Keyword::Void
            | Keyword::Char
            | Keyword::Short
            | Keyword::Int
            | Keyword::Long
            | Keyword::Float
            | Keyword::Double
            | Keyword::Signed
            | Keyword::Unsigned
    )
}

fn starts_declarator(tokens: &[LexedToken], i: usize) -> bool {
    let i = skip_trivia(tokens, i);
    if i >= tokens.len() {
        return false;
    }
    match &tokens[i] {
        LexedToken::Punctuator(s) if s == "*" || s == "(" => true,
        LexedToken::Identifier(_) => true,
        _ => false,
    }
}

fn parse_declaration_specifiers(tokens: &[LexedToken], mut i: usize) -> Option<(Vec<SpecifierPiece>, usize)> {
    let mut specifiers = Vec::new();
    loop {
        i = skip_trivia(tokens, i);
        if i >= tokens.len() {
            return Some((specifiers, i));
        }
        if starts_declarator(tokens, i) {
            return Some((specifiers, i));
        }

        match &tokens[i] {
            LexedToken::Keyword(kw) if is_storage(*kw) => {
                specifiers.push(SpecifierPiece::Storage(*kw));
                i += 1;
            }
            LexedToken::Keyword(kw) if is_qualifier(*kw) => {
                specifiers.push(SpecifierPiece::Qualifier(*kw));
                i += 1;
            }
            LexedToken::Keyword(kw) if is_type_keyword(*kw) => {
                specifiers.push(SpecifierPiece::Type(*kw));
                i += 1;
            }
            LexedToken::Keyword(Keyword::Struct) => {
                let (piece, ni) = parse_struct_or_union(tokens, i, false)?;
                specifiers.push(piece);
                i = ni;
            }
            LexedToken::Keyword(Keyword::Union) => {
                let (piece, ni) = parse_struct_or_union(tokens, i, true)?;
                specifiers.push(piece);
                i = ni;
            }
            LexedToken::Keyword(Keyword::Enum) => {
                let (piece, ni) = parse_enum_specifier(tokens, i)?;
                specifiers.push(piece);
                i = ni;
            }
            LexedToken::Identifier(name) => {
                specifiers.push(SpecifierPiece::TypedefName(name.clone()));
                i += 1;
            }
            _ => return None,
        }
    }
}

fn parse_struct_or_union(
    tokens: &[LexedToken],
    start: usize,
    is_union: bool,
) -> Option<(SpecifierPiece, usize)> {
    let mut i = start + 1;
    i = skip_trivia(tokens, i);
    let tag = if i < tokens.len() {
        if let LexedToken::Identifier(s) = &tokens[i] {
            let t = s.clone();
            i += 1;
            i = skip_trivia(tokens, i);
            Some(t)
        } else {
            None
        }
    } else {
        None
    };

    let mut fields = None;
    if i < tokens.len() && is_punct(&tokens[i], "{") {
        let close = matching_brace_close(tokens, i)?;
        fields = Some(tokens[i + 1..close].to_vec());
        i = close + 1;
    }

    let piece = if is_union {
        SpecifierPiece::Union { tag, fields }
    } else {
        SpecifierPiece::Struct { tag, fields }
    };
    Some((piece, i))
}

fn parse_enum_specifier(tokens: &[LexedToken], start: usize) -> Option<(SpecifierPiece, usize)> {
    let mut i = start + 1;
    i = skip_trivia(tokens, i);
    let tag = if i < tokens.len() {
        if let LexedToken::Identifier(s) = &tokens[i] {
            let t = s.clone();
            i += 1;
            i = skip_trivia(tokens, i);
            Some(t)
        } else {
            None
        }
    } else {
        None
    };

    let mut enumerators = None;
    if i < tokens.len() && is_punct(&tokens[i], "{") {
        let close = matching_brace_close(tokens, i)?;
        enumerators = Some(tokens[i + 1..close].to_vec());
        i = close + 1;
    }

    Some((
        SpecifierPiece::Enum {
            tag,
            enumerators,
        },
        i,
    ))
}

fn matching_brace_close(tokens: &[LexedToken], open: usize) -> Option<usize> {
    let mut depth = 1i32;
    let mut j = open + 1;
    while j < tokens.len() {
        match &tokens[j] {
            LexedToken::Punctuator(s) if s == "{" => depth += 1,
            LexedToken::Punctuator(s) if s == "}" => {
                depth -= 1;
                if depth == 0 {
                    return Some(j);
                }
            }
            _ => {}
        }
        j += 1;
    }
    None
}

fn parse_init_declarator_list(tokens: &[LexedToken], start: usize) -> Option<Vec<DeclaratorWithInit>> {
    let mut out = Vec::new();
    let mut seg_start = start;
    let mut paren = 0i32;
    let mut bracket = 0i32;
    let mut brace = 0i32;

    let mut i = start;
    while i < tokens.len() {
        match &tokens[i] {
            LexedToken::Punctuator(s) => match s.as_str() {
                "(" => paren += 1,
                ")" => paren -= 1,
                "[" => bracket += 1,
                "]" => bracket -= 1,
                "{" => brace += 1,
                "}" => brace -= 1,
                "," if paren == 0 && bracket == 0 && brace == 0 => {
                    let seg = trim_trailing_trivia(&tokens[seg_start..i]);
                    out.push(split_declarator_and_initializer(seg)?);
                    i += 1;
                    seg_start = i;
                    continue;
                }
                _ => {}
            },
            LexedToken::LineComment(_) | LexedToken::BlockComment(_) | LexedToken::Newline => {}
            _ => {}
        }
        i += 1;
    }

    let seg = trim_trailing_trivia(&tokens[seg_start..]);
    if !seg.is_empty() {
        out.push(split_declarator_and_initializer(seg)?);
    }
    Some(out)
}

fn split_declarator_and_initializer(seg: &[LexedToken]) -> Option<DeclaratorWithInit> {
    let mut paren = 0i32;
    let mut bracket = 0i32;
    let mut brace = 0i32;
    let mut eq_at = None;

    let mut i = 0;
    while i < seg.len() {
        i = skip_trivia(seg, i);
        if i >= seg.len() {
            break;
        }
        match &seg[i] {
            LexedToken::Punctuator(s) => match s.as_str() {
                "(" => paren += 1,
                ")" => paren -= 1,
                "[" => bracket += 1,
                "]" => bracket -= 1,
                "{" => brace += 1,
                "}" => brace -= 1,
                "=" if paren == 0 && bracket == 0 && brace == 0 => {
                    eq_at = Some(i);
                    break;
                }
                _ => {}
            },
            _ => {}
        }
        i += 1;
    }

    if let Some(eq) = eq_at {
        let decl = trim_trailing_trivia(&seg[..eq]);
        let init_start = skip_trivia(seg, eq + 1);
        let init = seg[init_start..].to_vec();
        Some(DeclaratorWithInit {
            declarator: decl.to_vec(),
            initializer: Some(init),
        })
    } else {
        Some(DeclaratorWithInit {
            declarator: seg.to_vec(),
            initializer: None,
        })
    }
}

/// Best-effort name introduced by a declarator (`*p` → `p`, `a[]` → `a`).
pub fn declarator_introduced_name(decl: &[LexedToken]) -> Option<String> {
    let mut last = None;
    let mut paren = 0i32;
    let mut bracket = 0i32;
    let mut i = 0;
    while i < decl.len() {
        i = skip_trivia(decl, i);
        if i >= decl.len() {
            break;
        }
        match &decl[i] {
            LexedToken::Punctuator(s) => match s.as_str() {
                "(" => paren += 1,
                ")" => paren -= 1,
                "[" => bracket += 1,
                "]" => bracket -= 1,
                _ => {}
            },
            LexedToken::Identifier(s) if paren == 0 && bracket == 0 => {
                last = Some(s.clone());
            }
            _ => {}
        }
        i += 1;
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage_200_lexing::lexing;
    use crate::stage_300_parsing::parsing_stage_300;
    use crate::stage_320_parsing::parsing_stage_320;

    fn parse_decl_src(src: &str) -> Option<Declaration> {
        let tu = parsing_stage_320(parsing_stage_300(lexing(src.to_string())));
        match tu.0.into_iter().next()? {
            ExternalDecl320::Declaration(t) => parse_declaration(&t),
            _ => None,
        }
    }

    #[test]
    fn simple_int_x() {
        let d = parse_decl_src("int x;").expect("parse");
        assert_eq!(d.specifiers.len(), 1);
        assert!(matches!(d.specifiers[0], SpecifierPiece::Type(Keyword::Int)));
        assert_eq!(d.declarators.len(), 1);
        assert_eq!(
            declarator_introduced_name(&d.declarators[0].declarator),
            Some("x".to_string())
        );
    }

    #[test]
    fn static_two_declarators() {
        let d = parse_decl_src("static int a, b;").expect("parse");
        assert_eq!(d.declarators.len(), 2);
        assert_eq!(
            declarator_introduced_name(&d.declarators[0].declarator),
            Some("a".to_string())
        );
        assert_eq!(
            declarator_introduced_name(&d.declarators[1].declarator),
            Some("b".to_string())
        );
    }

    #[test]
    fn struct_with_declarator() {
        let d = parse_decl_src("struct S { int x; } v;").expect("parse");
        assert!(matches!(
            d.specifiers[0],
            SpecifierPiece::Struct {
                tag: Some(ref t),
                fields: Some(_)
            } if t == "S"
        ));
        assert_eq!(
            declarator_introduced_name(&d.declarators[0].declarator),
            Some("v".to_string())
        );
    }

    #[test]
    fn initializer_split() {
        let d = parse_decl_src("int z = 42;").expect("parse");
        assert!(d.declarators[0].initializer.is_some());
    }
}
