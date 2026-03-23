//! Parser for function declarations and definitions.

use super::helpers::{
    advance_past_balanced_braces, find_next, read_balanced_parens, skip_whitespace,
    split_return_type_and_name,
};
use super::super::{Stage200Ast, TokenStream};

pub(super) fn try_parse(
    code: &str,
    bytes: &[u8],
    i: &mut usize,
    prefix: &str,
    stream: &mut TokenStream,
) -> Option<Stage200Ast> {
    let start = *i;
    let lp_pos = find_next(bytes, *i, b'(')?;
    let before_paren = String::from_utf8_lossy(&bytes[*i..lp_pos]).to_string();
    let (return_type, name) = split_return_type_and_name(&before_paren)?;
    *i = lp_pos;
    let params = read_balanced_parens(bytes, i)?;
    *i = skip_whitespace(bytes, *i);

    if *i >= bytes.len() {
        *i = start;
        return None;
    }

    if bytes[*i] == b';' {
        *i += 1;
        return Some(Stage200Ast::FunctionDecl {
            return_type: format!("{}{}", prefix, return_type),
            name,
            params: String::from_utf8_lossy(&params).to_string(),
        });
    }

    if bytes[*i] == b'{' {
        let (body_chunks, consumed) = stream.read_balanced_block(code, *i)?;
        if consumed == 1 {
            advance_past_balanced_braces(bytes, i);
        } else {
            *i = bytes.len();
        }
        *i = skip_whitespace(bytes, *i);
        if *i < bytes.len() && bytes[*i] == b';' {
            *i += 1;
        }
        return Some(Stage200Ast::FunctionDef {
            return_type: format!("{}{}", prefix, return_type),
            name,
            params: String::from_utf8_lossy(&params).to_string(),
            body: body_chunks,
        });
    }

    *i = start;
    None
}
