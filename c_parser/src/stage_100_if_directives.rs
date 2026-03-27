// AST node for preprocessor directives (#if, #ifdef, #ifndef, #elif, #else, #endif)

pub fn if_directives(content: &str) -> Vec<IncludeDirective> {
    let tokens = if_directives_lexing(content);
    let (ast, remaining) = parse_tokens(&tokens);
    assert!(remaining.is_empty());
    ast
}

#[derive(Debug, Clone, PartialEq)]
pub enum IncludeDirective {
    NonDirective(String),
    IfDef {
        symbol: String,
        then_branch: Vec<IncludeDirective>,
        else_branch: Option<Vec<IncludeDirective>>,
    },
    IfNDef {
        symbol: String,
        then_branch: Vec<IncludeDirective>,
        else_branch: Option<Vec<IncludeDirective>>,
    },
    If {
        condition: String,
        then_branch: Vec<IncludeDirective>,
        elif_branches: Vec<(String, Vec<IncludeDirective>)>,
        else_branch: Option<Vec<IncludeDirective>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum DirectiveToken {
    NonDirective(String),
    IfDirective { kind: IfDirectiveKind, arguments: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfDirectiveKind {
    If,
    IfDef,
    IfNDef,
    Else,
    Elif,
    Endif,
}

fn if_directives_lexing(content: &str) -> Vec<DirectiveToken> {
    let mut tokens = Vec::new();
    let bytes = content.as_bytes();
    let mut i = 0;
    let mut previous_start = i;

    loop {
        // Skip whitespace at line start (including start of file)
        while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
            // Comment/Line - Skip to next line
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            continue;
        }
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
            // Comment/Block - Skip to next */
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i += 2;
            continue;
        }
        if bytes[i] != b'#' {
            // Skip to next line
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            continue;
        }
        i += 1;
        while i < bytes.len() && matches!(bytes[i], b' ' | b'\t') {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }

        // directive detected
        let directive_start = i;
        while i < bytes.len() && bytes[i] != b'\n' {
            i += 1;
        }

        let line = String::from_utf8(bytes[directive_start..i].to_vec()).unwrap().trim().to_string();
        let mut parts = line.split_ascii_whitespace();
        let directive = parts.next().unwrap();
        let kind = match directive {
            "if" => IfDirectiveKind::If,
            "ifdef" => IfDirectiveKind::IfDef,
            "ifndef" => IfDirectiveKind::IfNDef,
            "else" => IfDirectiveKind::Else,
            "elif" => IfDirectiveKind::Elif,
            "endif" => IfDirectiveKind::Endif,
            _ => continue,
        };
        let arguments = parts.collect::<Vec<_>>().join(" ");

        let previous = String::from_utf8(bytes[previous_start..directive_start - 1].to_vec())
            .unwrap()
            .trim()
            .to_string();
        if !previous.is_empty() {
            tokens.push(DirectiveToken::NonDirective(previous));
        }
        tokens.push(DirectiveToken::IfDirective { kind, arguments });

        if i < bytes.len() {
            i += 1;
        }
        previous_start = i;
    }

    let remaining = String::from_utf8(bytes[previous_start..].to_vec()).unwrap().trim().to_string();
    if !remaining.is_empty() {
        tokens.push(DirectiveToken::NonDirective(remaining));
    }

    tokens
}

/// Parse tokens into AST. Returns (ast, remaining_tokens).
fn parse_tokens(tokens: &[DirectiveToken]) -> (Vec<IncludeDirective>, &[DirectiveToken]) {
    let mut ast = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            DirectiveToken::NonDirective(s) => {
                let s = s.trim();
                if !s.is_empty() {
                    ast.push(IncludeDirective::NonDirective(s.to_string()));
                }
                i += 1;
            }
            DirectiveToken::IfDirective { kind, arguments } => match kind {
                IfDirectiveKind::If => {
                    let cond = arguments.clone();
                    let (then_branch, rest) = parse_until_else_elif_endif(&tokens[i + 1..]);
                    let (elif_branches, else_branch, rest) = parse_elif_else_tail(rest);
                    ast.push(IncludeDirective::If {
                        condition: cond,
                        then_branch,
                        elif_branches,
                        else_branch,
                    });
                    i = tokens.len() - rest.len();
                }
                IfDirectiveKind::IfDef => {
                    let symbol = arguments.clone();
                    let (then_branch, else_branch, rest) = parse_conditional_block(&tokens[i + 1..]);
                    ast.push(IncludeDirective::IfDef {
                        symbol,
                        then_branch,
                        else_branch,
                    });
                    i = tokens.len() - rest.len();
                }
                IfDirectiveKind::IfNDef => {
                    let symbol = arguments.clone();
                    let (then_branch, else_branch, rest) = parse_conditional_block(&tokens[i + 1..]);
                    ast.push(IncludeDirective::IfNDef {
                        symbol,
                        then_branch,
                        else_branch,
                    });
                    i = tokens.len() - rest.len();
                }
                IfDirectiveKind::Else | IfDirectiveKind::Elif | IfDirectiveKind::Endif => {
                    i += 1;
                }
            },
        }
    }

    (ast, &tokens[i..])
}

/// Parse a single #if/#ifdef/#ifndef block. Returns (ast_nodes, rest_after_endif).
fn parse_nested_block(tokens: &[DirectiveToken]) -> (Vec<IncludeDirective>, &[DirectiveToken]) {
    match &tokens[0] {
        DirectiveToken::IfDirective { kind, arguments } => match kind {
            IfDirectiveKind::If => {
                let cond = arguments.clone();
                let (then_branch, rest) = parse_until_else_elif_endif(&tokens[1..]);
                let (elif_branches, else_branch, rest) = parse_elif_else_tail(rest);
                let directive = IncludeDirective::If {
                    condition: cond,
                    then_branch,
                    elif_branches,
                    else_branch,
                };
                (vec![directive], rest)
            }
            IfDirectiveKind::IfDef => {
                let symbol = arguments.clone();
                let (then_branch, else_branch, rest) = parse_conditional_block(&tokens[1..]);
                let directive = IncludeDirective::IfDef {
                    symbol,
                    then_branch,
                    else_branch,
                };
                (vec![directive], rest)
            }
            IfDirectiveKind::IfNDef => {
                let symbol = arguments.clone();
                let (then_branch, else_branch, rest) = parse_conditional_block(&tokens[1..]);
                let directive = IncludeDirective::IfNDef {
                    symbol,
                    then_branch,
                    else_branch,
                };
                (vec![directive], rest)
            }
            _ => (vec![], &tokens[1..]),
        },
        _ => (vec![], &tokens[1..]),
    }
}

/// Parse tokens until #else, #elif, or #endif at depth 0. Returns (directives_in_branch, rest).
fn parse_until_else_elif_endif(tokens: &[DirectiveToken]) -> (Vec<IncludeDirective>, &[DirectiveToken]) {
    let mut branch = Vec::new();
    let mut depth = 0;
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            DirectiveToken::NonDirective(s) => {
                if !s.trim().is_empty() {
                    branch.push(IncludeDirective::NonDirective(s.clone()));
                }
                i += 1;
            }
            DirectiveToken::IfDirective { kind, .. } => match kind {
                IfDirectiveKind::If | IfDirectiveKind::IfDef | IfDirectiveKind::IfNDef => {
                    depth += 1;
                    let (sub, rest) = parse_nested_block(&tokens[i..]);
                    branch.extend(sub);
                    let consumed = tokens.len() - i - rest.len();
                    i += consumed;
                    depth -= 1;
                }
                IfDirectiveKind::Elif | IfDirectiveKind::Else if depth == 0 => {
                    return (branch, &tokens[i..]);
                }
                IfDirectiveKind::Endif if depth == 0 => {
                    return (branch, &tokens[i..]);
                }
                IfDirectiveKind::Endif => {
                    depth -= 1;
                    i += 1;
                }
                _ => i += 1,
            },
        }
    }
    (branch, &tokens[i..])
}

/// Parse #elif/#else/#endif tail. Returns (elif_branches, else_branch, rest_after_endif).
#[allow(clippy::type_complexity)]
fn parse_elif_else_tail(tokens: &[DirectiveToken]) -> (Vec<(String, Vec<IncludeDirective>)>, Option<Vec<IncludeDirective>>, &[DirectiveToken]) {
    let mut elif_branches = Vec::new();
    let mut rest = tokens;

    loop {
        if rest.is_empty() {
            return (elif_branches, None, rest);
        }
        match &rest[0] {
            DirectiveToken::IfDirective { kind, arguments } => match kind {
                IfDirectiveKind::Elif => {
                    let cond = arguments.clone();
                    let (branch, next) = parse_until_else_elif_endif(&rest[1..]);
                    elif_branches.push((cond, branch));
                    rest = next;
                }
                IfDirectiveKind::Else => {
                    let (branch, next) = parse_until_endif(&rest[1..]);
                    return (elif_branches, Some(branch), next);
                }
                IfDirectiveKind::Endif => {
                    return (elif_branches, None, &rest[1..]);
                }
                _ => return (elif_branches, None, rest),
            },
            DirectiveToken::NonDirective(_) => return (elif_branches, None, rest),
        }
    }
}

/// Parse until #endif at depth 0. Returns (directives, rest_after_endif).
fn parse_until_endif(tokens: &[DirectiveToken]) -> (Vec<IncludeDirective>, &[DirectiveToken]) {
    let mut branch = Vec::new();
    let mut depth = 0;
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            DirectiveToken::NonDirective(s) => {
                if !s.trim().is_empty() {
                    branch.push(IncludeDirective::NonDirective(s.clone()));
                }
                i += 1;
            }
            DirectiveToken::IfDirective { kind, .. } => match kind {
                IfDirectiveKind::If | IfDirectiveKind::IfDef | IfDirectiveKind::IfNDef => {
                    depth += 1;
                    let (sub, rest) = parse_nested_block(&tokens[i..]);
                    branch.extend(sub);
                    let consumed = tokens.len() - i - rest.len();
                    i += consumed;
                    depth -= 1;
                }
                IfDirectiveKind::Endif if depth == 0 => {
                    return (branch, &tokens[i + 1..]);
                }
                IfDirectiveKind::Endif => {
                    depth -= 1;
                    i += 1;
                }
                _ => i += 1,
            },
        }
    }
    (branch, &tokens[i..])
}

/// Parse #ifdef/#ifndef block (then branch, optional else, until #endif).
fn parse_conditional_block(tokens: &[DirectiveToken]) -> (Vec<IncludeDirective>, Option<Vec<IncludeDirective>>, &[DirectiveToken]) {
    let (then_branch, rest) = parse_until_else_elif_endif(tokens);

    if rest.is_empty() {
        return (then_branch, None, rest);
    }
    match &rest[0] {
        DirectiveToken::IfDirective {
            kind: IfDirectiveKind::Else, ..
        }
        | DirectiveToken::IfDirective {
            kind: IfDirectiveKind::Elif, ..
        } => {
            let (else_branch, after) = parse_until_endif(&rest[1..]);
            (then_branch, Some(else_branch), after)
        }
        DirectiveToken::IfDirective {
            kind: IfDirectiveKind::Endif,
            ..
        } => (then_branch, None, &rest[1..]),
        _ => (then_branch, None, rest),
    }
}

#[cfg(test)]
mod tests;
