//! Parses `ExternalDecl320::Declaration` token slices into structured declarations.

use crate::stage_200_lexing::{Keyword, LexedToken, Punctuator};
use crate::stage_300_parsing::FunctionBody;
use crate::stage_320_parsing::{ExternalDecl320, PreprocessorDirective, TranslationUnit320};

#[derive(Debug, Clone, PartialEq)]
pub enum SpecifierPiece {
    Storage(Keyword),
    Qualifier(Keyword),
    Type(Keyword),
    Struct {
        tag: Option<String>,
        /// Parsed member declarations when possible; see [`StructMember`].
        fields: Option<Vec<StructMember>>,
    },
    Union {
        tag: Option<String>,
        fields: Option<Vec<StructMember>>,
    },
    Enum {
        tag: Option<String>,
        /// Tokens between `{` and `}` when a definition is present.
        enumerators: Option<Vec<LexedToken>>,
    },
    /// Type name from a `typedef` (or other alias) in specifier position.
    TypedefName(String),
}

/// One declaration inside a `struct` / `union` body, with any `//` / `/* */` tokens before it.
#[derive(Debug, Clone, PartialEq)]
pub struct StructMemberDeclaration {
    /// Bodies of line/block comment tokens before this member (newlines are not stored).
    pub leading_comments: Vec<String>,
    pub declaration: Declaration,
}

/// One declaration inside a `struct` / `union` body.
#[derive(Debug, Clone, PartialEq)]
pub enum StructMember {
    Declaration(Box<StructMemberDeclaration>),
    /// Bit-fields, nested edge cases, etc.
    Unparsed(Vec<LexedToken>),
}

/// Parsed declarator: pointers, then direct declarator, then postfix `[]` / `()` chains.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclaratorAst {
    /// For each `*`, optional `const` / `volatile` immediately after that `*`.
    pub pointer_levels: Vec<Vec<Keyword>>,
    pub direct: DirectDeclarator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DirectDeclarator {
    Identifier(String),
    Parenthesized(Box<DeclaratorAst>),
    Array {
        base: Box<DirectDeclarator>,
        size: Option<Vec<LexedToken>>,
    },
    Function {
        base: Box<DirectDeclarator>,
        parameters: Vec<LexedToken>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclaratorWithInit {
    pub declarator: Vec<LexedToken>,
    /// Filled when the declarator slice parses cleanly as a full [`DeclaratorAst`].
    pub ast: Option<DeclaratorAst>,
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
        tu.0.into_iter()
            .map(|d| match d {
                ExternalDecl320::Preprocessor(p) => ExternalDecl340::Preprocessor(p),
                ExternalDecl320::Declaration(tokens) => match parse_declaration(&tokens) {
                    Some(decl) => ExternalDecl340::Declaration(decl),
                    None => ExternalDecl340::UnparsedDeclaration(tokens),
                },
                ExternalDecl320::FunctionDefinition { signature_tokens, body } => ExternalDecl340::FunctionDefinition { signature_tokens, body },
                ExternalDecl320::Comment(s) => ExternalDecl340::Comment(s),
            })
            .collect(),
    )
}

fn is_trivia(t: &LexedToken) -> bool {
    matches!(t, LexedToken::LineComment(_) | LexedToken::BlockComment(_) | LexedToken::Newline)
}

fn skip_trivia(tokens: &[LexedToken], mut i: usize) -> usize {
    while i < tokens.len() && is_trivia(&tokens[i]) {
        i += 1;
    }
    i
}

/// Consumes leading newlines and comment tokens; returns comment bodies in order.
fn collect_leading_member_comments(tokens: &[LexedToken]) -> (Vec<String>, usize) {
    let mut comments = Vec::new();
    let mut i = 0usize;
    while i < tokens.len() {
        match &tokens[i] {
            LexedToken::Newline => i += 1,
            LexedToken::LineComment(s) => {
                comments.push(s.clone());
                i += 1;
            }
            LexedToken::BlockComment(s) => {
                comments.push(s.clone());
                i += 1;
            }
            _ => break,
        }
    }
    (comments, i)
}

fn is_punct(t: &LexedToken, p: Punctuator) -> bool {
    matches!(t, LexedToken::Punctuator(x) if *x == p)
}

fn parse_declaration(tokens: &[LexedToken]) -> Option<Declaration> {
    let tokens = trim_trailing_trivia(tokens);
    if tokens.is_empty() {
        return None;
    }
    let last = tokens.last()?;
    if !is_punct(last, Punctuator::Semicolon) {
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
    Some(Declaration { specifiers, declarators })
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
        LexedToken::Punctuator(p) if *p == Punctuator::Star || *p == Punctuator::LParen => true,
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

fn parse_struct_or_union(tokens: &[LexedToken], start: usize, is_union: bool) -> Option<(SpecifierPiece, usize)> {
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
    if i < tokens.len() && is_punct(&tokens[i], Punctuator::LBrace) {
        let close = matching_brace_close(tokens, i)?;
        fields = Some(parse_struct_body_members(&tokens[i + 1..close]));
        i = close + 1;
    }

    let piece = if is_union {
        SpecifierPiece::Union { tag, fields }
    } else {
        SpecifierPiece::Struct { tag, fields }
    };
    Some((piece, i))
}
fn parse_struct_body_members(body: &[LexedToken]) -> Vec<StructMember> {
    let mut members = Vec::new();
    let mut start = 0usize;
    let mut brace_depth = 0i32;
    let mut i = 0usize;
    while i < body.len() {
        match &body[i] {
            LexedToken::Punctuator(Punctuator::LBrace) => brace_depth += 1,
            LexedToken::Punctuator(Punctuator::RBrace) => brace_depth -= 1,
            LexedToken::Punctuator(Punctuator::Semicolon) if brace_depth == 0 => {
                let seg = trim_trailing_trivia(&body[start..=i]);
                let (leading_comments, rest) = collect_leading_member_comments(seg);
                let decl_slice = trim_trailing_trivia(&seg[rest..]);
                let m = if let Some(d) = parse_declaration(decl_slice) {
                    StructMember::Declaration(Box::new(StructMemberDeclaration {
                        leading_comments,
                        declaration: d,
                    }))
                } else {
                    StructMember::Unparsed(seg.to_vec())
                };
                members.push(m);
                i += 1;
                start = i;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    let tail = trim_trailing_trivia(&body[start..]);
    if !tail.is_empty() {
        members.push(StructMember::Unparsed(tail.to_vec()));
    }
    members
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
    if i < tokens.len() && is_punct(&tokens[i], Punctuator::LBrace) {
        let close = matching_brace_close(tokens, i)?;
        enumerators = Some(tokens[i + 1..close].to_vec());
        i = close + 1;
    }

    Some((SpecifierPiece::Enum { tag, enumerators }, i))
}

fn matching_brace_close(tokens: &[LexedToken], open: usize) -> Option<usize> {
    let mut depth = 1i32;
    let mut j = open + 1;
    while j < tokens.len() {
        match &tokens[j] {
            LexedToken::Punctuator(Punctuator::LBrace) => depth += 1,
            LexedToken::Punctuator(Punctuator::RBrace) => {
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
            LexedToken::Punctuator(p) => match p {
                Punctuator::LParen => paren += 1,
                Punctuator::RParen => paren -= 1,
                Punctuator::LBracket => bracket += 1,
                Punctuator::RBracket => bracket -= 1,
                Punctuator::LBrace => brace += 1,
                Punctuator::RBrace => brace -= 1,
                Punctuator::Comma if paren == 0 && bracket == 0 && brace == 0 => {
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
            LexedToken::Punctuator(p) => match p {
                Punctuator::LParen => paren += 1,
                Punctuator::RParen => paren -= 1,
                Punctuator::LBracket => bracket += 1,
                Punctuator::RBracket => bracket -= 1,
                Punctuator::LBrace => brace += 1,
                Punctuator::RBrace => brace -= 1,
                Punctuator::Equal if paren == 0 && bracket == 0 && brace == 0 => {
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
        let ast = try_parse_declarator_ast(decl);
        let init_start = skip_trivia(seg, eq + 1);
        let init = seg[init_start..].to_vec();
        Some(DeclaratorWithInit {
            declarator: decl.to_vec(),
            ast,
            initializer: Some(init),
        })
    } else {
        let decl = trim_trailing_trivia(seg);
        let ast = try_parse_declarator_ast(decl);
        Some(DeclaratorWithInit {
            declarator: decl.to_vec(),
            ast,
            initializer: None,
        })
    }
}

fn parse_pointer_levels(tokens: &[LexedToken], mut i: usize) -> (Vec<Vec<Keyword>>, usize) {
    let mut levels = Vec::new();
    loop {
        i = skip_trivia(tokens, i);
        if i >= tokens.len() || !is_punct(&tokens[i], Punctuator::Star) {
            break;
        }
        i += 1;
        let mut quals = Vec::new();
        loop {
            i = skip_trivia(tokens, i);
            match tokens.get(i) {
                Some(LexedToken::Keyword(Keyword::Const)) => {
                    quals.push(Keyword::Const);
                    i += 1;
                }
                Some(LexedToken::Keyword(Keyword::Volatile)) => {
                    quals.push(Keyword::Volatile);
                    i += 1;
                }
                _ => break,
            }
        }
        levels.push(quals);
    }
    (levels, i)
}

fn matching_paren_close_decl(tokens: &[LexedToken], open: usize) -> Option<usize> {
    let mut depth = 1i32;
    let mut j = open + 1;
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

fn matching_bracket_close_decl(tokens: &[LexedToken], open: usize) -> Option<usize> {
    let mut depth = 1i32;
    let mut j = open + 1;
    while j < tokens.len() {
        match &tokens[j] {
            LexedToken::Punctuator(Punctuator::LBracket) => depth += 1,
            LexedToken::Punctuator(Punctuator::RBracket) => {
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

fn parse_declarator_ast(tokens: &[LexedToken], start: usize) -> Option<(DeclaratorAst, usize)> {
    let (pointer_levels, mut i) = parse_pointer_levels(tokens, start);
    i = skip_trivia(tokens, i);
    if i >= tokens.len() {
        return None;
    }
    let (mut direct, mut i) = match &tokens[i] {
        LexedToken::Identifier(s) => (DirectDeclarator::Identifier(s.clone()), i + 1),
        LexedToken::Punctuator(Punctuator::LParen) => {
            let close = matching_paren_close_decl(tokens, i)?;
            let inner = &tokens[i + 1..close];
            let inner_ast = try_parse_declarator_ast(inner)?;
            (DirectDeclarator::Parenthesized(Box::new(inner_ast)), close + 1)
        }
        _ => return None,
    };
    loop {
        i = skip_trivia(tokens, i);
        if i < tokens.len() && is_punct(&tokens[i], Punctuator::LBracket) {
            let close = matching_bracket_close_decl(tokens, i)?;
            let size = if close > i + 1 {
                let s = trim_trailing_trivia(&tokens[i + 1..close]);
                if s.is_empty() { None } else { Some(s.to_vec()) }
            } else {
                None
            };
            direct = DirectDeclarator::Array {
                base: Box::new(direct),
                size,
            };
            i = close + 1;
            continue;
        }
        if i < tokens.len() && is_punct(&tokens[i], Punctuator::LParen) {
            let close = matching_paren_close_decl(tokens, i)?;
            let params = tokens[i + 1..close].to_vec();
            direct = DirectDeclarator::Function {
                base: Box::new(direct),
                parameters: params,
            };
            i = close + 1;
            continue;
        }
        break;
    }
    Some((DeclaratorAst { pointer_levels, direct }, i))
}

fn try_parse_declarator_ast(decl: &[LexedToken]) -> Option<DeclaratorAst> {
    let decl = trim_trailing_trivia(decl);
    if decl.is_empty() {
        return None;
    }
    let (ast, end) = parse_declarator_ast(decl, 0)?;
    let end = skip_trivia(decl, end);
    if end != decl.len() {
        return None;
    }
    Some(ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage_200_lexing::lexing;
    use crate::stage_300_parsing::parsing_stage_300;
    use crate::stage_320_parsing::parsing_stage_320;

    fn name_from_declarator_ast(ast: &DeclaratorAst) -> Option<String> {
        name_from_direct(&ast.direct)
    }

    fn name_from_direct(d: &DirectDeclarator) -> Option<String> {
        match d {
            DirectDeclarator::Identifier(s) => Some(s.clone()),
            DirectDeclarator::Parenthesized(inner) => name_from_declarator_ast(inner),
            DirectDeclarator::Array { base, .. } => name_from_direct(base),
            DirectDeclarator::Function { base, .. } => name_from_direct(base),
        }
    }

    /// Best-effort name introduced by a declarator (`*p` → `p`, `(*fp)` → `fp`).
    pub fn declarator_introduced_name(decl: &[LexedToken]) -> Option<String> {
        if let Some(ast) = try_parse_declarator_ast(decl) {
            return name_from_declarator_ast(&ast);
        }
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
                LexedToken::Punctuator(p) => match p {
                    Punctuator::LParen => paren += 1,
                    Punctuator::RParen => paren -= 1,
                    Punctuator::LBracket => bracket += 1,
                    Punctuator::RBracket => bracket -= 1,
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
        assert_eq!(declarator_introduced_name(&d.declarators[0].declarator), Some("x".to_string()));
    }

    #[test]
    fn static_two_declarators() {
        let d = parse_decl_src("static int a, b;").expect("parse");
        assert_eq!(d.declarators.len(), 2);
        assert_eq!(declarator_introduced_name(&d.declarators[0].declarator), Some("a".to_string()));
        assert_eq!(declarator_introduced_name(&d.declarators[1].declarator), Some("b".to_string()));
    }

    #[test]
    fn struct_with_declarator() {
        let d = parse_decl_src("struct S { int x; } v;").expect("parse");
        assert!(matches!(
            d.specifiers[0],
            SpecifierPiece::Struct {
                tag: Some(ref t),
                fields: Some(ref members)
            } if t == "S"
                && matches!(members.as_slice(), [StructMember::Declaration(m)]
                    if m.leading_comments.is_empty()
                        && m.declaration.specifiers.len() == 1
                        && matches!(m.declaration.specifiers[0], SpecifierPiece::Type(Keyword::Int))
                        && m.declaration.declarators.len() == 1
                        && declarator_introduced_name(&m.declaration.declarators[0].declarator)
                            == Some("x".to_string()))
        ));
        assert_eq!(declarator_introduced_name(&d.declarators[0].declarator), Some("v".to_string()));
    }

    #[test]
    fn initializer_split() {
        let d = parse_decl_src("int z = 42;").expect("parse");
        assert!(d.declarators[0].initializer.is_some());
    }

    #[test]
    fn declarator_ast_pointer_and_function() {
        let d = parse_decl_src("int (*fp)(void);").expect("parse");
        let ast = d.declarators[0].ast.as_ref().expect("ast");
        assert_eq!(ast.pointer_levels.len(), 0);
        let DirectDeclarator::Function { base, .. } = &ast.direct else {
            panic!("expected function declarator");
        };
        let DirectDeclarator::Parenthesized(inner) = base.as_ref() else {
            panic!("expected parenthesized");
        };
        assert_eq!(inner.pointer_levels.len(), 1);
        assert!(matches!(
            &inner.direct,
            DirectDeclarator::Identifier(s) if s == "fp"
        ));
    }

    #[test]
    fn declarator_ast_array() {
        let d = parse_decl_src("int arr[10];").expect("parse");
        let ast = d.declarators[0].ast.as_ref().expect("ast");
        assert!(matches!(
            ast.direct,
            DirectDeclarator::Array {
                ref base,
                size: Some(_)
            } if matches!(base.as_ref(), DirectDeclarator::Identifier(s) if s == "arr")
        ));
    }

    /// Line comment above the `struct`, then line comments above each of two fields.
    #[test]
    fn struct_definition_with_comments_above_struct_and_fields() {
        let src = "// header before struct\n\
                    struct Point {\n\
                        // x coordinate\n\
                        int x;\n\
                        // y coordinate\n\
                        int y;\n\
                    };\n";
        let tu = parsing_stage_340(parsing_stage_320(parsing_stage_300(lexing(src.to_string()))));
        assert!(
            matches!(tu.0.first(), Some(ExternalDecl340::Comment(s)) if s == " header before struct"),
            "expected top-level Comment before struct, got {:?}",
            tu.0.first()
        );
        let Some(ExternalDecl340::Declaration(decl)) = tu.0.get(1) else {
            panic!("expected Declaration after comment, got {:?}", tu.0.get(1));
        };
        assert!(decl.declarators.is_empty(), "anonymous struct should have no trailing declarators");
        let fields = match &decl.specifiers[0] {
            SpecifierPiece::Struct {
                tag: Some(t),
                fields: Some(m),
            } if t == "Point" => m,
            other => panic!("expected struct Point with fields: {:?}", other),
        };
        assert_eq!(fields.len(), 2);

        let StructMember::Declaration(f0) = &fields[0] else {
            panic!("field 0: {:?}", fields[0]);
        };
        assert_eq!(f0.leading_comments, vec![" x coordinate".to_string()]);
        assert!(matches!(f0.declaration.specifiers[0], SpecifierPiece::Type(Keyword::Int)));
        assert_eq!(
            declarator_introduced_name(&f0.declaration.declarators[0].declarator),
            Some("x".to_string())
        );

        let StructMember::Declaration(f1) = &fields[1] else {
            panic!("field 1: {:?}", fields[1]);
        };
        assert_eq!(f1.leading_comments, vec![" y coordinate".to_string()]);
        assert!(matches!(f1.declaration.specifiers[0], SpecifierPiece::Type(Keyword::Int)));
        assert_eq!(
            declarator_introduced_name(&f1.declaration.declarators[0].declarator),
            Some("y".to_string())
        );
    }
}
