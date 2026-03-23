//! Parser for `enum [name] { ... };` or `enum name ;`

use super::helpers::{is_ident_byte, skip_whitespace};
use super::super::{EnumVariant, Stage200Ast};

pub(super) fn parse(bytes: &[u8], i: &mut usize) -> Option<Stage200Ast> {
    *i = skip_whitespace(bytes, *i);
    let mut name = String::new();
    if *i < bytes.len() && is_ident_byte(bytes[*i]) {
        let tag_start = *i;
        while *i < bytes.len() && is_ident_byte(bytes[*i]) {
            *i += 1;
        }
        name = String::from_utf8_lossy(&bytes[tag_start..*i]).to_string();
    }
    *i = skip_whitespace(bytes, *i);
    if *i >= bytes.len() {
        return None;
    }
    if bytes[*i] == b';' {
        *i += 1;
        return Some(Stage200Ast::EnumDef {
            name,
            variants: vec![],
        });
    }
    if bytes[*i] != b'{' {
        return None;
    }
    *i += 1;

    let mut variants = Vec::new();
    loop {
        *i = skip_whitespace(bytes, *i);
        if *i >= bytes.len() {
            return None;
        }
        if bytes[*i] == b'}' {
            *i += 1;
            break;
        }

        let v_name_start = *i;
        while *i < bytes.len() && is_ident_byte(bytes[*i]) {
            *i += 1;
        }
        let v_name = String::from_utf8_lossy(&bytes[v_name_start..*i]).to_string();
        if v_name.is_empty() {
            return None;
        }

        *i = skip_whitespace(bytes, *i);
        let value = if *i < bytes.len() && bytes[*i] == b'=' {
            *i += 1;
            *i = skip_whitespace(bytes, *i);
            let val_start = *i;
            while *i < bytes.len() && bytes[*i] != b',' && bytes[*i] != b'}' {
                *i += 1;
            }
            Some(
                String::from_utf8_lossy(&bytes[val_start..*i])
                    .trim()
                    .to_string(),
            )
        } else {
            None
        };

        variants.push(EnumVariant {
            name: v_name,
            value,
        });
        *i = skip_whitespace(bytes, *i);
        if *i < bytes.len() && bytes[*i] == b',' {
            *i += 1;
        }
    }

    *i = skip_whitespace(bytes, *i);
    if *i < bytes.len() && bytes[*i] == b';' {
        *i += 1;
    }

    let name = if name.is_empty() {
        variants
            .first()
            .map(|v| format!("anonymous_{}", v.name))
            .unwrap_or_else(|| "__anonymous_enum".to_string())
    } else {
        name
    };

    Some(Stage200Ast::EnumDef { name, variants })
}
