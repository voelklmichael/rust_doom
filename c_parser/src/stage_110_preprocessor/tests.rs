use crate::stage_110_preprocessor::Stage110Preprocessor;

#[test]
fn test_parse_multiline_define() {
    let content = r#"
        #define NEWGAME	\
        "you can't start a new game\n"\
        "while in a network game.\n\n"PRESSKEY   "#;
    let stage100 = crate::stage_100_comments::Stage100Comments::parse(content);
    let result = Stage110Preprocessor::parse(stage100);
    dbg!(&result);
    assert_eq!(result.len(), 1);
    let Stage110Preprocessor::Define {
        name,
        params,
        value,
        ..
    } = &result[0]
    else {
        panic!("expected Define, got {:?}", result[0]);
    };
    assert_eq!(name, "NEWGAME");
    assert_eq!(params, &None);
    assert!(value.contains("you can't start a new game"));
    assert!(value.contains("while in a network game"));
    assert!(value.contains("PRESSKEY"));
}

fn parse_conditionals(content: &str) -> Vec<Stage110Preprocessor> {
    let stage100 = crate::stage_100_comments::Stage100Comments::parse(content);
    Stage110Preprocessor::parse(stage100)
}

fn emitted_code(result: &[Stage110Preprocessor]) -> String {
    result
        .iter()
        .filter_map(|r| match r {
            Stage110Preprocessor::Code(c) => Some(c.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn test_ifdef_blacklisted_takes_else() {
    let content = r#"#ifdef FEATURE_SOUND
sound_then
#else
sound_else
#endif"#;
    let result = parse_conditionals(content);
    let code = emitted_code(&result);
    assert!(!code.contains("sound_then"), "blacklisted should take else");
    assert!(code.contains("sound_else"), "blacklisted should take else");
}

#[test]
fn test_ifdef_whitelisted_takes_then() {
    let content = r#"#ifdef DOOM_GENERIC
doom_then
#else
doom_else
#endif"#;
    let result = parse_conditionals(content);
    let code = emitted_code(&result);
    assert!(code.contains("doom_then"), "whitelisted should take then");
    assert!(!code.contains("doom_else"), "whitelisted should take then");
}

#[test]
fn test_ifndef_blacklisted_takes_then() {
    let content = r#"#ifndef FEATURE_SOUND
no_sound_then
#else
no_sound_else
#endif"#;
    let result = parse_conditionals(content);
    let code = emitted_code(&result);
    assert!(
        code.contains("no_sound_then"),
        "blacklisted = not defined, #ifndef true -> then"
    );
    assert!(!code.contains("no_sound_else"), "blacklisted = not defined");
}

#[test]
fn test_ifndef_whitelisted_takes_else() {
    let content = r#"#ifndef DOOM_GENERIC
doom_not_then
#else
doom_not_else
#endif"#;
    let result = parse_conditionals(content);
    let code = emitted_code(&result);
    assert!(
        !code.contains("doom_not_then"),
        "whitelisted = defined, #ifndef false"
    );
    assert!(
        code.contains("doom_not_else"),
        "whitelisted should take else"
    );
}

#[test]
fn test_if_unevaluated_takes_else() {
    let content = r#"#if FEATURE_SOUND
if_then
#else
if_else
#endif"#;
    let result = parse_conditionals(content);
    let code = emitted_code(&result);
    assert!(
        !code.contains("if_then"),
        "#if unevaluated -> false -> else"
    );
    assert!(
        code.contains("if_else"),
        "#if unevaluated takes else branch"
    );
}

#[test]
fn test_elif_evaluates_known_symbols() {
    // #if FEATURE_SOUND: blacklisted (false) -> skip
    // #elif DOOM_GENERIC: whitelisted (true) -> take this branch
    let content = r#"#if FEATURE_SOUND
aaa_if_then
#elif DOOM_GENERIC
bbb_elif_then
#else
ccc_else_only
#endif"#;
    let result = parse_conditionals(content);
    let code = emitted_code(&result);
    assert!(!code.contains("aaa_if_then"), "FEATURE_SOUND blacklisted");
    assert!(
        code.contains("bbb_elif_then"),
        "DOOM_GENERIC is defined -> take #elif branch"
    );
    assert!(
        !code.contains("ccc_else_only"),
        "skip #else when #elif matches"
    );
}

#[test]
fn test_if_elif_elif_else_only_final() {
    // Multiple #elif -> take ONLY the final #else
    let content = r#"#if A
branch_a
#elif B
branch_b
#elif C
branch_c
#else
branch_final
#endif"#;
    let result = parse_conditionals(content);
    let code = emitted_code(&result);
    assert!(!code.contains("branch_a"));
    assert!(!code.contains("branch_b"));
    assert!(!code.contains("branch_c"));
    assert!(code.contains("branch_final"), "only final #else");
}

#[test]
fn test_nested_ifdef_outer_blacklisted() {
    let content = r#"#ifdef FEATURE_SOUND
outer_then
#else
#ifdef DOOM_GENERIC
inner_then
#else
inner_else
#endif
#endif"#;
    let result = parse_conditionals(content);
    let code = emitted_code(&result);
    assert!(!code.contains("outer_then"), "outer blacklisted -> else");
    assert!(
        code.contains("inner_then"),
        "nested: inner whitelisted -> then"
    );
    assert!(!code.contains("inner_else"), "nested: inner whitelisted");
}

#[test]
fn test_nested_ifdef_both_blacklisted() {
    let content = r#"#ifdef FEATURE_SOUND
outer_then
#else
#ifdef FEATURE_DEHACKED
inner_then
#else
inner_else
#endif
#endif"#;
    let result = parse_conditionals(content);
    let code = emitted_code(&result);
    assert!(!code.contains("outer_then"));
    assert!(!code.contains("inner_then"));
    assert!(
        code.contains("inner_else"),
        "both blacklisted -> else of outer, else of inner"
    );
}

#[test]
fn test_nested_ifdef_outer_whitelisted_inner_blacklisted() {
    let content = r#"#ifdef DOOM_GENERIC
#ifdef FEATURE_SOUND
inner_then
#else
inner_else
#endif
#else
outer_else
#endif"#;
    let result = parse_conditionals(content);
    let code = emitted_code(&result);
    assert!(!code.contains("outer_else"), "outer whitelisted");
    assert!(!code.contains("inner_then"), "inner blacklisted");
    assert!(code.contains("inner_else"), "inner blacklisted -> else");
}
