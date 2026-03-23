//! Parser for `typedef type name;`

use super::helpers::{find_next_byte, is_valid_simple_type_name, skip_whitespace};
use super::super::Stage200Ast;

pub(super) fn parse(bytes: &[u8], i: &mut usize) -> Option<Stage200Ast> {
    *i = skip_whitespace(bytes, *i);
    if *i >= bytes.len() {
        return None;
    }
    let decl_start = *i;
    let semi = find_next_byte(bytes, *i, b';')?;
    let between = String::from_utf8_lossy(&bytes[decl_start..semi]).trim().to_string();
    if between.is_empty() {
        return None;
    }
    let parts: Vec<&str> = between.split_ascii_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    let name = parts.last().unwrap().to_string();
    let base_type = parts[..parts.len() - 1].join(" ");
    if !is_valid_simple_type_name(&name) {
        return None;
    }
    *i = semi + 1;
    Some(Stage200Ast::TypedefSimple {
        base_type,
        name,
    })
}
