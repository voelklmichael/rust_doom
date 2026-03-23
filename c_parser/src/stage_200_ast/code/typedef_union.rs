//! Parser for `typedef union [tag] { ... } name;`

use super::helpers::{advance_past_balanced_braces, is_ident_byte, skip_whitespace};
use super::super::{Stage200Ast, TokenStream};

pub(super) fn parse(
    code: &str,
    bytes: &[u8],
    i: &mut usize,
    stream: &mut TokenStream,
) -> Option<Stage200Ast> {
    *i = skip_whitespace(bytes, *i);
    if *i >= bytes.len() {
        return None;
    }

    let mut union_tag = None;
    if is_ident_byte(bytes[*i]) {
        let tag_start = *i;
        while *i < bytes.len() && is_ident_byte(bytes[*i]) {
            *i += 1;
        }
        union_tag = Some(String::from_utf8_lossy(&bytes[tag_start..*i]).to_string());
        *i = skip_whitespace(bytes, *i);
    }

    if *i >= bytes.len() {
        return None;
    }

    if bytes[*i] == b';' {
        *i += 1;
        let name = union_tag.unwrap_or_default();
        if name.is_empty() {
            return None;
        }
        return Some(Stage200Ast::TypedefUnion { name, body: vec![] });
    }

    if bytes[*i] == b'{' {
        let (body_chunks, consumed) = stream.read_balanced_block(code, *i)?;
        if consumed == 1 {
            advance_past_balanced_braces(bytes, i);
        } else {
            *i = bytes.len();
        }
        *i = skip_whitespace(bytes, *i);

        let name_start = *i;
        while *i < bytes.len() && is_ident_byte(bytes[*i]) {
            *i += 1;
        }
        let name = String::from_utf8_lossy(&bytes[name_start..*i]).to_string();
        if name.is_empty() && union_tag.is_some() {
            return Some(Stage200Ast::TypedefUnion {
                name: union_tag.unwrap(),
                body: body_chunks,
            });
        }
        if name.is_empty() {
            return None;
        }
        *i = skip_whitespace(bytes, *i);
        if *i < bytes.len() && bytes[*i] == b';' {
            *i += 1;
        }

        return Some(Stage200Ast::TypedefUnion {
            name,
            body: body_chunks,
        });
    }

    if union_tag.is_some() && is_ident_byte(bytes[*i]) {
        let name_start = *i;
        while *i < bytes.len() && is_ident_byte(bytes[*i]) {
            *i += 1;
        }
        let name = String::from_utf8_lossy(&bytes[name_start..*i]).to_string();
        *i = skip_whitespace(bytes, *i);
        if *i < bytes.len() && bytes[*i] == b';' {
            *i += 1;
        }
        return Some(Stage200Ast::TypedefUnion { name, body: vec![] });
    }

    None
}
