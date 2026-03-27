//! Structured preprocessor directives after stage 300 (`#include`, `#define`).

use crate::stage_200_lexing::{LexedToken, Punctuator};
use crate::stage_300_parsing::{ExternalDecl, FunctionBody, TranslationUnit};

#[derive(Debug, Clone, PartialEq)]
pub enum IncludeDirective {
    /// `#include "path"`
    Quoted(String),
    /// `#include <path>` (path reconstructed from tokens between `<` and `>`).
    System(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefineDirective {
    pub name: String,
    /// `None` for object-like macros; parameter names for function-like.
    pub parameters: Option<Vec<String>>,
    pub replacement: Vec<LexedToken>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreprocessorDirective {
    Include(IncludeDirective),
    Define(DefineDirective),
    /// `#undef name`
    Undef(String),
    /// Other `#` directives or unrecognised shapes (keeps original tokens).
    /// Never used in c-codebase
    Other(Vec<LexedToken>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternalDecl320 {
    Preprocessor(PreprocessorDirective),
    Declaration(Vec<LexedToken>),
    FunctionDefinition { signature_tokens: Vec<LexedToken>, body: FunctionBody },
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TranslationUnit320(pub Vec<ExternalDecl320>);

pub(crate) fn parsing_stage_320(tu: TranslationUnit) -> TranslationUnit320 {
    TranslationUnit320(
        tu.0.into_iter()
            .map(|d| match d {
                ExternalDecl::Preprocessor(tokens) => ExternalDecl320::Preprocessor(parse_preprocessor_directive(tokens)),
                ExternalDecl::Declaration(t) => ExternalDecl320::Declaration(t),
                ExternalDecl::FunctionDefinition { signature_tokens, body } => ExternalDecl320::FunctionDefinition { signature_tokens, body },
                ExternalDecl::Comment(s) => ExternalDecl320::Comment(s),
            })
            .collect(),
    )
}

fn is_comment(t: &LexedToken) -> bool {
    matches!(t, LexedToken::LineComment(_) | LexedToken::BlockComment(_))
}

fn is_punct(t: &LexedToken, p: Punctuator) -> bool {
    matches!(t, LexedToken::Punctuator(x) if *x == p)
}

fn skip_comments(tokens: &[LexedToken], mut i: usize) -> usize {
    while i < tokens.len() && is_comment(&tokens[i]) {
        i += 1;
    }
    i
}

fn parse_preprocessor_directive(tokens: Vec<LexedToken>) -> PreprocessorDirective {
    let mut i = skip_comments(&tokens, 0);
    if i >= tokens.len() || !matches!(tokens[i], LexedToken::Hash) {
        return PreprocessorDirective::Other(tokens);
    }
    i += 1;
    i = skip_comments(&tokens, i);
    let Some(LexedToken::Identifier(directive)) = tokens.get(i) else {
        return PreprocessorDirective::Other(tokens);
    };
    match directive.as_str() {
        "include" => parse_include(&tokens, i + 1).unwrap_or(PreprocessorDirective::Other(tokens)),
        "define" => parse_define(&tokens, i + 1).unwrap_or(PreprocessorDirective::Other(tokens)),
        "undef" => parse_undef(&tokens, i + 1).unwrap_or(PreprocessorDirective::Other(tokens)),
        _ => PreprocessorDirective::Other(tokens),
    }
}

fn parse_include(tokens: &[LexedToken], mut i: usize) -> Option<PreprocessorDirective> {
    i = skip_comments(tokens, i);
    let t = tokens.get(i)?;
    match t {
        LexedToken::StringLiteral(path) => Some(PreprocessorDirective::Include(IncludeDirective::Quoted(path.clone()))),
        LexedToken::Punctuator(Punctuator::Less) => {
            let mut parts = Vec::new();
            i += 1;
            while i < tokens.len() {
                if let LexedToken::Punctuator(Punctuator::Greater) = &tokens[i] {
                    let path = parts.join("");
                    if path.is_empty() {
                        return None;
                    }
                    return Some(PreprocessorDirective::Include(IncludeDirective::System(path)));
                }
                parts.push(token_to_path_fragment(&tokens[i])?);
                i += 1;
            }
            None
        }
        _ => None,
    }
}

fn token_to_path_fragment(t: &LexedToken) -> Option<String> {
    match t {
        LexedToken::Identifier(s) => Some(s.clone()),
        LexedToken::Punctuator(p) if matches!(*p, Punctuator::Dot | Punctuator::Slash | Punctuator::Minus) => Some(p.as_str().to_string()),
        LexedToken::IntegerLiteral { value, .. } => Some(value.clone()),
        _ => None,
    }
}

fn parse_undef(tokens: &[LexedToken], mut i: usize) -> Option<PreprocessorDirective> {
    i = skip_comments(tokens, i);
    let LexedToken::Identifier(name) = tokens.get(i)? else {
        return None;
    };
    let name = name.clone();
    i += 1;
    i = skip_comments(tokens, i);
    if i < tokens.len() {
        return None;
    }
    Some(PreprocessorDirective::Undef(name))
}

fn parse_define(tokens: &[LexedToken], mut i: usize) -> Option<PreprocessorDirective> {
    i = skip_comments(tokens, i);
    let LexedToken::Identifier(name) = tokens.get(i)? else {
        return None;
    };
    let name = name.clone();
    i += 1;
    i = skip_comments(tokens, i);

    if i < tokens.len()
        && is_punct(&tokens[i], Punctuator::LParen)
        && let Some((params, after)) = try_parse_function_like_params(tokens, i)
    {
        let replacement = tokens[after..].to_vec();
        return Some(PreprocessorDirective::Define(DefineDirective {
            name,
            parameters: Some(params),
            replacement,
        }));
    }

    let replacement = tokens[i..].to_vec();
    Some(PreprocessorDirective::Define(DefineDirective {
        name,
        parameters: None,
        replacement,
    }))
}

/// If `tokens[i]` is `(`, treat as function-like only when the parenthesised segment is a
/// comma-separated list of identifiers (and optional `...`); otherwise `None` so the caller
/// keeps object-like parsing (e.g. `#define foo (x)`).
fn try_parse_function_like_params(tokens: &[LexedToken], open_idx: usize) -> Option<(Vec<String>, usize)> {
    if !is_punct(tokens.get(open_idx)?, Punctuator::LParen) {
        return None;
    }
    let close = matching_paren_close(tokens, open_idx)?;
    let inner = &tokens[open_idx + 1..close];
    let params = parse_parameter_list_tokens(inner)?;
    Some((params, close + 1))
}

fn matching_paren_close(tokens: &[LexedToken], open_idx: usize) -> Option<usize> {
    let mut depth = 1i32;
    let mut i = open_idx + 1;
    while i < tokens.len() {
        match &tokens[i] {
            LexedToken::Punctuator(Punctuator::LParen) => depth += 1,
            LexedToken::Punctuator(Punctuator::RParen) => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn parse_parameter_list_tokens(inner: &[LexedToken]) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut i = skip_comments(inner, 0);
    if i >= inner.len() {
        return Some(out);
    }
    loop {
        i = skip_comments(inner, i);
        if i >= inner.len() {
            break;
        }
        match &inner[i] {
            LexedToken::Identifier(s) => {
                out.push(s.clone());
                i += 1;
            }
            LexedToken::Punctuator(Punctuator::Ellipsis) => {
                out.push("...".to_string());
                i += 1;
            }
            _ => return None,
        }
        i = skip_comments(inner, i);
        if i >= inner.len() {
            break;
        }
        if is_punct(&inner[i], Punctuator::Comma) {
            i += 1;
            continue;
        }
        return None;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage_200_lexing::lexing;
    use crate::stage_300_parsing::parsing_stage_300;

    #[test]
    fn include_angle_and_quoted() {
        let tu = parsing_stage_300(lexing("#include <stdio.h>\n#include \"x.h\"".to_string()));
        let tu320 = parsing_stage_320(tu);
        assert_eq!(tu320.0.len(), 2);
        match &tu320.0[0] {
            ExternalDecl320::Preprocessor(PreprocessorDirective::Include(path)) => {
                assert_eq!(path, &IncludeDirective::System("stdio.h".to_string()));
            }
            _ => panic!("expected include"),
        }
        match &tu320.0[1] {
            ExternalDecl320::Preprocessor(PreprocessorDirective::Include(path)) => {
                assert_eq!(path, &IncludeDirective::Quoted("x.h".to_string()));
            }
            _ => panic!("expected include"),
        }
    }

    #[test]
    fn define_object_like() {
        let tu = parsing_stage_300(lexing("#define FOO 42".to_string()));
        let tu320 = parsing_stage_320(tu);
        match &tu320.0[0] {
            ExternalDecl320::Preprocessor(PreprocessorDirective::Define(d)) => {
                assert_eq!(d.name, "FOO");
                assert_eq!(d.parameters, None);
                assert!(!d.replacement.is_empty());
            }
            _ => panic!("expected define"),
        }
    }

    #[test]
    fn undef_macro_name() {
        let tu = parsing_stage_300(lexing("#undef OLD_MACRO".to_string()));
        let tu320 = parsing_stage_320(tu);
        match &tu320.0[0] {
            ExternalDecl320::Preprocessor(PreprocessorDirective::Undef(name)) => {
                assert_eq!(name, "OLD_MACRO");
            }
            _ => panic!("expected undef"),
        }
    }

    #[test]
    fn define_function_like() {
        let tu = parsing_stage_300(lexing("#define BAR(x,y) ((x)+(y))".to_string()));
        let tu320 = parsing_stage_320(tu);
        match &tu320.0[0] {
            ExternalDecl320::Preprocessor(PreprocessorDirective::Define(d)) => {
                assert_eq!(d.name, "BAR");
                assert_eq!(d.parameters, Some(vec!["x".to_string(), "y".to_string()]));
                assert!(!d.replacement.is_empty());
            }
            _ => panic!("expected define"),
        }
    }
}
