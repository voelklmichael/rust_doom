//! Translation-unit parser: preprocessor lines, declarations ending in `;`, and function
//! definitions. Function bodies are not parsed—only stored as raw tokens.

use crate::stage_200_lexing::LexedToken;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody(pub Vec<LexedToken>);

#[derive(Debug, Clone, PartialEq)]
pub enum ExternalDecl {
    /// Tokens from `#` up to (but not including) the next `#`, or end of input.
    Preprocessor(Vec<LexedToken>),
    /// Tokens from the start of a declaration through the terminating `;` (inclusive).
    Declaration(Vec<LexedToken>),
    FunctionDefinition {
        /// Tokens from declaration start through the closing `)` of the parameter list (inclusive).
        signature_tokens: Vec<LexedToken>,
        body: FunctionBody,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TranslationUnit(pub Vec<ExternalDecl>);

pub(crate) fn parsing_stage_300(lexed: Vec<LexedToken>) -> TranslationUnit {
    let tokens = lexed;
    let mut decls = Vec::new();
    let mut i = 0usize;

    while i < tokens.len() {
        while i < tokens.len() && is_comment(&tokens[i]) {
            i += 1;
        }
        if i >= tokens.len() {
            break;
        }

        if matches!(tokens[i], LexedToken::Hash) {
            let start = i;
            let end = preprocessor_end_exclusive(&tokens, start);
            decls.push(ExternalDecl::Preprocessor(tokens[start..end].to_vec()));
            i = end;
            continue;
        }

        let (decl, next_i) = parse_declaration_or_function(&tokens, i);
        decls.push(decl);
        i = next_i;
    }

    TranslationUnit(decls)
}

/// Where the preprocessor directive that starts at `start` (`#`) ends (exclusive index).
fn preprocessor_end_exclusive(tokens: &[LexedToken], start: usize) -> usize {
    debug_assert!(matches!(tokens[start], LexedToken::Hash));
    let mut i = start + 1;
    if i >= tokens.len() {
        return tokens.len();
    }

    if let LexedToken::Identifier(name) = &tokens[i] {
        if name == "include" {
            i += 1;
            if i >= tokens.len() {
                return tokens.len();
            }
            match &tokens[i] {
                LexedToken::StringLiteral(_) => return i + 1,
                LexedToken::Punctuator(p) if p == "<" => {
                    i += 1;
                    while i < tokens.len() {
                        if let LexedToken::Punctuator(p) = &tokens[i] {
                            if p == ">" {
                                return i + 1;
                            }
                        }
                        i += 1;
                    }
                    return tokens.len();
                }
                _ => {}
            }
        }
    }

    while i < tokens.len() && !matches!(tokens[i], LexedToken::Hash) {
        i += 1;
    }
    i
}

fn is_comment(t: &LexedToken) -> bool {
    matches!(t, LexedToken::LineComment(_) | LexedToken::BlockComment(_))
}

fn is_punct(t: &LexedToken, s: &str) -> bool {
    matches!(t, LexedToken::Punctuator(p) if p == s)
}

/// Index of the last non-comment token at or before `j` (inclusive).
fn prev_significant(tokens: &[LexedToken], j: usize) -> Option<usize> {
    let mut k = j;
    loop {
        if !is_comment(&tokens[k]) {
            return Some(k);
        }
        if k == 0 {
            return None;
        }
        k -= 1;
    }
}

/// Tokens inside the outer `{` … `}`; `open_brace` is the index of `{`. Returns index past closing `}`.
fn extract_function_body(tokens: &[LexedToken], open_brace: usize) -> (FunctionBody, usize) {
    debug_assert!(is_punct(&tokens[open_brace], "{"));
    let mut depth = 1usize;
    let mut i = open_brace + 1;
    let mut inner = Vec::new();

    while i < tokens.len() && depth > 0 {
        match &tokens[i] {
            LexedToken::Punctuator(s) if s == "{" => {
                depth += 1;
                inner.push(tokens[i].clone());
            }
            LexedToken::Punctuator(s) if s == "}" => {
                depth -= 1;
                if depth > 0 {
                    inner.push(tokens[i].clone());
                }
            }
            _ => inner.push(tokens[i].clone()),
        }
        i += 1;
    }

    (FunctionBody(inner), i)
}

fn parse_declaration_or_function(tokens: &[LexedToken], start: usize) -> (ExternalDecl, usize) {
    let mut paren: i32 = 0;
    let mut bracket: i32 = 0;
    let mut brace: i32 = 0;
    let mut i = start;

    while i < tokens.len() {
        let t = &tokens[i];

        // Function definition: `) {` at depth 0 (after `)`), where `{` opens the body.
        if is_punct(t, "{") && paren == 0 && brace == 0 && bracket == 0 {
            if let Some(pidx) = prev_significant(tokens, i.saturating_sub(1)) {
                if is_punct(&tokens[pidx], ")") {
                    let signature_tokens = tokens[start..i].to_vec();
                    let (body, after_body) = extract_function_body(tokens, i);
                    return (
                        ExternalDecl::FunctionDefinition {
                            signature_tokens,
                            body,
                        },
                        after_body,
                    );
                }
            }
        }

        match t {
            LexedToken::Punctuator(s) => match s.as_str() {
                "(" => paren += 1,
                ")" => paren -= 1,
                "[" => bracket += 1,
                "]" => bracket -= 1,
                "{" => brace += 1,
                "}" => brace -= 1,
                ";" if paren == 0 && brace == 0 && bracket == 0 => {
                    let decl = tokens[start..=i].to_vec();
                    return (ExternalDecl::Declaration(decl), i + 1);
                }
                _ => {}
            },
            _ => {}
        }

        i += 1;
    }

    // No terminating `;` (truncated input): treat remainder as one declaration-like blob.
    let decl = tokens[start..].to_vec();
    (ExternalDecl::Declaration(decl), tokens.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage_200_lexing::{Keyword, lexing};

    #[test]
    fn parse_decl_and_function() {
        let src = "int x;\nvoid foo(void) { return 1; }\n";
        let tokens = lexing(src.to_string());
        let tu = parsing_stage_300(tokens);
        assert_eq!(tu.0.len(), 2);
        match &tu.0[0] {
            ExternalDecl::Declaration(d) => {
                assert!(
                    d.iter()
                        .any(|t| matches!(t, LexedToken::Keyword(Keyword::Int)))
                );
            }
            _ => panic!("expected declaration"),
        }
        match &tu.0[1] {
            ExternalDecl::FunctionDefinition {
                signature_tokens,
                body,
            } => {
                assert!(
                    signature_tokens
                        .iter()
                        .any(|t| matches!(t, LexedToken::Identifier(s) if s == "foo"))
                );
                assert!(!body.0.is_empty());
            }
            _ => panic!("expected function"),
        }
    }

    #[test]
    fn parse_preprocessor_split_on_hash() {
        let src = "#include <a.h>\n#include <b.h>\nint x;";
        let tokens = lexing(src.to_string());
        let tu = parsing_stage_300(tokens);
        assert!(tu.0.len() >= 3);
        assert!(matches!(tu.0[0], ExternalDecl::Preprocessor(_)));
        assert!(matches!(tu.0[1], ExternalDecl::Preprocessor(_)));
    }

    #[test]
    fn struct_then_var_is_one_declaration() {
        let src = "struct S { int x; } v;";
        let tokens = lexing(src.to_string());
        let tu = parsing_stage_300(tokens);
        assert_eq!(tu.0.len(), 1);
        assert!(matches!(tu.0[0], ExternalDecl::Declaration(_)));
    }
}
