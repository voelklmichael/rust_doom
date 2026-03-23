//! Parsing of Code tokens - C declarations like struct, enum, function, etc.

mod enum_def;
mod function;
mod helpers;
mod struct_def;
mod typedef_enum;
mod typedef_simple;
mod typedef_struct;
mod typedef_union;
mod union_def;

use super::{Stage200Ast, TokenStream};
use helpers::{match_prefix, read_until_semicolon, skip_whitespace};

pub(super) fn handle(code: &str, stream: &mut TokenStream) -> Vec<Stage200Ast> {
    parse_code_with_stream(code, stream)
}

fn parse_code_with_stream(code: &str, stream: &mut TokenStream) -> Vec<Stage200Ast> {
    let mut results = Vec::new();
    let code = code.trim();
    let start_token = stream.token_idx;
    if code.is_empty() {
        stream.advance();
        return results;
    }

    let mut i = 0;
    let bytes = code.as_bytes();

    while i < bytes.len() {
        i = skip_whitespace(bytes, i);
        if i >= bytes.len() {
            break;
        }

        let decl_start = i;
        match parse_one_declaration_with_stream(code.trim(), bytes, &mut i, stream) {
            Some(ast) => results.push(ast),
            None => {
                let rest = String::from_utf8_lossy(&bytes[decl_start..]).to_string();
                if !rest.trim().is_empty() {
                    results.push(Stage200Ast::Unparsed(rest));
                }
                break;
            }
        }
    }

    let consumed = stream.token_idx.saturating_sub(start_token) + 1;
    let target = start_token + consumed;
    if stream.token_idx < target {
        stream.advance_by(target - stream.token_idx);
    }
    results
}

fn parse_one_declaration_with_stream(
    code: &str,
    bytes: &[u8],
    i: &mut usize,
    stream: &mut TokenStream,
) -> Option<Stage200Ast> {
    let start = *i;
    *i = skip_whitespace(bytes, *i);
    if *i >= bytes.len() {
        return None;
    }

    if match_prefix(bytes, i, "typedef") {
        *i = skip_whitespace(bytes, *i);
        if match_prefix(bytes, i, "enum") {
            return typedef_enum::parse(bytes, i);
        }
        if match_prefix(bytes, i, "struct") {
            return typedef_struct::parse(code, bytes, i, stream);
        }
        if match_prefix(bytes, i, "union") {
            return typedef_union::parse(code, bytes, i, stream);
        }
        if let Some(ast) = typedef_simple::parse(bytes, i) {
            return Some(ast);
        }
        *i = start;
    }

    if match_prefix(bytes, i, "struct") {
        return struct_def::parse(code, bytes, i, stream);
    }

    if match_prefix(bytes, i, "union") {
        return union_def::parse(code, bytes, i, stream);
    }

    if match_prefix(bytes, i, "enum") {
        return enum_def::parse(bytes, i);
    }

    if match_prefix(bytes, i, "extern") {
        let decl = read_until_semicolon(bytes, i);
        return Some(Stage200Ast::OtherDecl(decl));
    }

    if match_prefix(bytes, i, "static") {
        *i = skip_whitespace(bytes, *i);
        if let Some(ast) = function::try_parse(code, bytes, i, "static ", stream) {
            return Some(ast);
        }
        *i = start;
    }

    if let Some(ast) = function::try_parse(code, bytes, i, "", stream) {
        return Some(ast);
    }

    let decl = read_until_semicolon(bytes, i);
    if !decl.trim().is_empty() {
        return Some(Stage200Ast::OtherDecl(decl));
    }

    None
}
