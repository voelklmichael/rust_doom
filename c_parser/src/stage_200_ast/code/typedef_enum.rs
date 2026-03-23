//! Parser for `typedef enum { ... } name;`

use super::helpers::{is_ident_byte, skip_whitespace};
use super::super::{EnumVariant, Stage200Ast, TokenStream};

pub(super) fn parse(
    bytes: &[u8],
    i: &mut usize,
    stream: &mut TokenStream,
) -> Option<Stage200Ast> {
    *i = skip_whitespace(bytes, *i);

    // If `{` is not in current token, try next token (handles comments between typedef enum and {)
    let (bytes, i) = if *i >= bytes.len() || bytes[*i] != b'{' {
        let next = stream.advance_to_next_code()?;
        let next_bytes = next.as_bytes();
        let mut pos = skip_whitespace(next_bytes, 0);
        if pos >= next_bytes.len() || next_bytes[pos] != b'{' {
            return None;
        }
        (next_bytes, &mut pos)
    } else {
        (bytes, i)
    };

    let mut idx = *i;
    idx += 1; // skip '{'

    let mut variants = Vec::new();
    loop {
        idx = skip_whitespace(bytes, idx);
        if idx >= bytes.len() {
            return None;
        }
        if bytes[idx] == b'}' {
            idx += 1;
            break;
        }

        let name_start = idx;
        while idx < bytes.len() && is_ident_byte(bytes[idx]) {
            idx += 1;
        }
        let name = String::from_utf8_lossy(&bytes[name_start..idx]).to_string();
        if name.is_empty() {
            return None;
        }

        idx = skip_whitespace(bytes, idx);
        let value = if idx < bytes.len() && bytes[idx] == b'=' {
            idx += 1;
            idx = skip_whitespace(bytes, idx);
            let val_start = idx;
            while idx < bytes.len() && bytes[idx] != b',' && bytes[idx] != b'}' {
                idx += 1;
            }
            Some(
                String::from_utf8_lossy(&bytes[val_start..idx])
                    .trim()
                    .to_string(),
            )
        } else {
            None
        };

        variants.push(EnumVariant { name, value });
        idx = skip_whitespace(bytes, idx);
        if idx < bytes.len() && bytes[idx] == b',' {
            idx += 1;
        }
    }

    idx = skip_whitespace(bytes, idx);
    let name_start = idx;
    while idx < bytes.len() && is_ident_byte(bytes[idx]) {
        idx += 1;
    }
    let name = String::from_utf8_lossy(&bytes[name_start..idx]).to_string();
    if name.is_empty() {
        return None;
    }
    idx = skip_whitespace(bytes, idx);
    if idx < bytes.len() && bytes[idx] == b';' {
        idx += 1;
    }

    *i = idx;

    // If we used the next token, advance stream past it
    if !std::ptr::eq(bytes.as_ptr(), (*stream as *const TokenStream).cast_mut() as *const u8) {
        stream.advance();
    }
    // Actually we can't compare pointers like that. Simpler: when we took the next token path, we need to advance. We can pass a flag or check: if the bytes we're using came from advance_to_next_code, we need to advance. The simplest: always try to advance when we successfully parsed from "next token" - but we don't have a clean way to know. Alternative: pass a flag. Or: when we call advance_to_next_code, we've consumed the current token. So the stream is now on the "next code" token. We parsed it. We need to advance past it. So we always advance when we took the next-token path. We can detect that: we only take that path when *i was >= bytes.len() originally. So we'd need to track that. Let me use a different approach: return a (Option<Stage200Ast>, tokens_to_advance) or have the parser call stream.advance() when it used the next token. The typedef_enum receives stream. When we use advance_to_next_code and succeed, we must advance. So we'll add stream.advance() at the end when we used the next token. The challenge: we don't know if we used it. We could use a flag: let mut used_next = false; at the start, if we take the next-token path, used_next = true. At the end, if used_next { stream.advance(); }.

Actually the issue is that when we use the next-token path, we get `next` from advance_to_next_code. We then use `next_bytes` - we don't have a mutable reference to update the original `i`. The function signature has `i: &mut usize`. When we take the next-token path, we create a local `pos` and use that. We'd need to return the result. And we need to call stream.advance(). So at the end, we need to know if we used the next token. The simplest: use a boolean.
</think>

<｜tool▁calls▁begin｜><｜tool▁call▁begin｜>
Read