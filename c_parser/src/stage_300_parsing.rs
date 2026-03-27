//! Translation-unit parser: preprocessor lines, declarations ending in `;`, and function
//! definitions. Function bodies are not parsed—only stored as raw tokens.

use crate::stage_200_lexing::{LexedToken, Punctuator};

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
    /// Top-level comment (`//` or `/* */` body text, same as lexer payloads).
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TranslationUnit(pub Vec<ExternalDecl>);

pub(crate) fn parsing_stage_300(lexed: Vec<LexedToken>) -> TranslationUnit {
    let tokens = lexed;
    let mut decls = Vec::new();
    let mut i = 0usize;

    while i < tokens.len() {
        while i < tokens.len() && matches!(tokens[i], LexedToken::Newline) {
            i += 1;
        }
        if i >= tokens.len() {
            break;
        }

        if let Some(text) = comment_payload(&tokens[i]) {
            decls.push(ExternalDecl::Comment(text));
            i += 1;
            continue;
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

fn skip_pp_comments(tokens: &[LexedToken], mut i: usize) -> usize {
    while i < tokens.len() && is_comment(&tokens[i]) {
        i += 1;
    }
    i
}

/// Where the preprocessor directive that starts at `start` (`#`) ends (exclusive index).
fn preprocessor_end_exclusive(tokens: &[LexedToken], start: usize) -> usize {
    debug_assert!(matches!(tokens[start], LexedToken::Hash));
    let mut i = start + 1;
    if i >= tokens.len() {
        return tokens.len();
    }
    i = skip_pp_comments(tokens, i);

    if let LexedToken::Identifier(name) = &tokens[i] {
        if name == "undef" {
            i += 1;
            i = skip_pp_comments(tokens, i);
            if i < tokens.len() && matches!(tokens[i], LexedToken::Identifier(_)) {
                i += 1;
            }
            i = skip_pp_comments(tokens, i);
            return i;
        }
        if name == "define" {
            return define_directive_end_exclusive(tokens, i + 1);
        }
        if name == "include" {
            i += 1;
            if i >= tokens.len() {
                return tokens.len();
            }
            match &tokens[i] {
                LexedToken::StringLiteral(_) => return i + 1,
                LexedToken::Punctuator(Punctuator::Less) => {
                    i += 1;
                    while i < tokens.len() {
                        if let LexedToken::Punctuator(Punctuator::Greater) = &tokens[i] {
                            return i + 1;
                        }
                        i += 1;
                    }
                    return tokens.len();
                }
                _ => {}
            }
        }
    }

    scan_pp_directive_line_tail(tokens, i)
}

/// Rest of a `#directive …` line: stops at `Newline` or next `#` at nesting depth 0.
fn scan_pp_directive_line_tail(tokens: &[LexedToken], mut i: usize) -> usize {
    let mut paren: i32 = 0;
    let mut bracket: i32 = 0;
    let mut brace: i32 = 0;
    while i < tokens.len() {
        match &tokens[i] {
            LexedToken::Hash if paren == 0 && bracket == 0 && brace == 0 => return i,
            LexedToken::Newline if paren == 0 && bracket == 0 && brace == 0 => return i,
            LexedToken::Punctuator(p) => match p {
                Punctuator::LParen => paren += 1,
                Punctuator::RParen => paren -= 1,
                Punctuator::LBracket => bracket += 1,
                Punctuator::RBracket => bracket -= 1,
                Punctuator::LBrace => brace += 1,
                Punctuator::RBrace => brace -= 1,
                _ => {}
            },
            _ => {}
        }
        i += 1;
    }
    tokens.len()
}

/// After the `define` identifier: macro name, optional function-like `(...)`, replacement until newline/`#`.
fn define_directive_end_exclusive(tokens: &[LexedToken], mut i: usize) -> usize {
    i = skip_pp_comments(tokens, i);
    if i >= tokens.len() || !matches!(tokens[i], LexedToken::Identifier(_)) {
        return scan_pp_directive_line_tail(tokens, i);
    }
    i += 1;
    i = skip_pp_comments(tokens, i);
    if i < tokens.len()
        && is_punct(&tokens[i], Punctuator::LParen)
        && let Some(close) = matching_paren_close_pp(tokens, i)
    {
        let inner = &tokens[i + 1..close];
        if define_parameter_list_ok(inner) {
            i = close + 1;
            i = skip_pp_comments(tokens, i);
        }
    }
    scan_pp_directive_line_tail(tokens, i)
}

fn matching_paren_close_pp(tokens: &[LexedToken], open_idx: usize) -> Option<usize> {
    let mut depth = 1i32;
    let mut j = open_idx + 1;
    while j < tokens.len() {
        match &tokens[j] {
            LexedToken::Punctuator(Punctuator::LParen) => depth += 1,
            LexedToken::Punctuator(Punctuator::RParen) => {
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

fn skip_define_inner_comments(tokens: &[LexedToken], mut i: usize) -> usize {
    while i < tokens.len() && is_comment(&tokens[i]) {
        i += 1;
    }
    i
}

fn define_parameter_list_ok(inner: &[LexedToken]) -> bool {
    let mut i = skip_define_inner_comments(inner, 0);
    if i >= inner.len() {
        return true;
    }
    loop {
        i = skip_define_inner_comments(inner, i);
        if i >= inner.len() {
            break;
        }
        match &inner[i] {
            LexedToken::Identifier(_) => i += 1,
            LexedToken::Punctuator(Punctuator::Ellipsis) => i += 1,
            _ => return false,
        }
        i = skip_define_inner_comments(inner, i);
        if i >= inner.len() {
            break;
        }
        if is_punct(&inner[i], Punctuator::Comma) {
            i += 1;
            continue;
        }
        return false;
    }
    true
}

fn is_comment(t: &LexedToken) -> bool {
    matches!(t, LexedToken::LineComment(_) | LexedToken::BlockComment(_))
}

fn comment_payload(t: &LexedToken) -> Option<String> {
    match t {
        LexedToken::LineComment(s) | LexedToken::BlockComment(s) => Some(s.clone()),
        _ => None,
    }
}

fn is_punct(t: &LexedToken, p: Punctuator) -> bool {
    matches!(t, LexedToken::Punctuator(x) if *x == p)
}

/// Index of the last non-comment token at or before `j` (inclusive).
fn prev_significant(tokens: &[LexedToken], j: usize) -> Option<usize> {
    let mut k = j;
    loop {
        if !is_comment(&tokens[k]) && !matches!(tokens[k], LexedToken::Newline) {
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
    debug_assert!(is_punct(&tokens[open_brace], Punctuator::LBrace));
    let mut depth = 1usize;
    let mut i = open_brace + 1;
    let mut inner = Vec::new();

    while i < tokens.len() && depth > 0 {
        match &tokens[i] {
            LexedToken::Punctuator(Punctuator::LBrace) => {
                depth += 1;
                inner.push(tokens[i].clone());
            }
            LexedToken::Punctuator(Punctuator::RBrace) => {
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
        if is_punct(t, Punctuator::LBrace)
            && paren == 0
            && brace == 0
            && bracket == 0
            && let Some(pidx) = prev_significant(tokens, i.saturating_sub(1))
            && is_punct(&tokens[pidx], Punctuator::RParen)
        {
            let signature_tokens = tokens[start..i].to_vec();
            let (body, after_body) = extract_function_body(tokens, i);
            return (ExternalDecl::FunctionDefinition { signature_tokens, body }, after_body);
        }

        if let LexedToken::Punctuator(p) = t {
            match p {
                Punctuator::LParen => paren += 1,
                Punctuator::RParen => paren -= 1,
                Punctuator::LBracket => bracket += 1,
                Punctuator::RBracket => bracket -= 1,
                Punctuator::LBrace => brace += 1,
                Punctuator::RBrace => brace -= 1,
                Punctuator::Semicolon if paren == 0 && brace == 0 && bracket == 0 => {
                    let decl = tokens[start..=i].to_vec();
                    return (ExternalDecl::Declaration(decl), i + 1);
                }
                _ => {}
            }
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
                assert!(d.iter().any(|t| matches!(t, LexedToken::Keyword(Keyword::Int))));
            }
            _ => panic!("expected declaration"),
        }
        match &tu.0[1] {
            ExternalDecl::FunctionDefinition { signature_tokens, body } => {
                assert!(signature_tokens.iter().any(|t| matches!(t, LexedToken::Identifier(s) if s == "foo")));
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
    fn undef_does_not_swallow_following_code() {
        let src = "#undef R\nstatic int x = 0;";
        let tokens = lexing(src.to_string());
        let tu = parsing_stage_300(tokens);
        assert_eq!(tu.0.len(), 2);
        assert!(matches!(tu.0[0], ExternalDecl::Preprocessor(_)));
        assert!(matches!(tu.0[1], ExternalDecl::Declaration(_)));
    }

    #[test]
    fn define_ends_at_newline_before_next_declaration() {
        let src = "#define R (FRACUNIT)\nmline_t triangle_guy[];\n";
        let tokens = lexing(src.to_string());
        let tu = parsing_stage_300(tokens);
        assert_eq!(tu.0.len(), 2);
        assert!(matches!(tu.0[0], ExternalDecl::Preprocessor(_)));
        assert!(matches!(tu.0[1], ExternalDecl::Declaration(_)));
    }

    #[test]
    fn struct_then_var_is_one_declaration() {
        let src = "struct S { int x; } v;";
        let tokens = lexing(src.to_string());
        let tu = parsing_stage_300(tokens);
        assert_eq!(tu.0.len(), 1);
        assert!(matches!(tu.0[0], ExternalDecl::Declaration(_)));
    }

    #[test]
    fn top_level_comment_before_decl() {
        let src = "// header\nint x;";
        let tokens = lexing(src.to_string());
        let tu = parsing_stage_300(tokens);
        assert_eq!(tu.0.len(), 2);
        assert_eq!(tu.0[0], ExternalDecl::Comment(" header".to_string()));
        assert!(matches!(tu.0[1], ExternalDecl::Declaration(_)));
    }
}
