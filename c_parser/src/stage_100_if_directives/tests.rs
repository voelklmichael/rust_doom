use super::{IncludeDirective, if_directives};

#[test]
fn test_if_directives_no_directive() {
    let ast = if_directives(r#"int x = 1;"#);
    assert_eq!(ast, vec![IncludeDirective::NonDirective("int x = 1;".to_string())]);
}

#[test]
fn test_if_directives_if_directive() {
    let content = r#"
        /* bla */ #if     1   
        int x = 1;
        #endif
    "#;
    let ast = if_directives(content);
    assert_eq!(
        ast,
        vec![
            IncludeDirective::NonDirective("/* bla */".to_string()),
            IncludeDirective::If {
                condition: "1".to_string(),
                then_branch: vec![IncludeDirective::NonDirective("int x = 1;".to_string())],
                elif_branches: vec![],
                else_branch: None,
            },
        ]
    );
}

#[test]
fn test_if_directives_nested_if() {
    let content = r#"
#if 1
outer
#if 0
inner_false
#else
inner_else
#endif
#endif
    "#;
    let ast = if_directives(content);
    let expected = vec![IncludeDirective::If {
        condition: "1".to_string(),
        then_branch: vec![
            IncludeDirective::NonDirective("outer".to_string()),
            IncludeDirective::If {
                condition: "0".to_string(),
                then_branch: vec![IncludeDirective::NonDirective("inner_false".to_string())],
                elif_branches: vec![],
                else_branch: Some(vec![IncludeDirective::NonDirective("inner_else".to_string())]),
            },
        ],
        elif_branches: vec![],
        else_branch: None,
    }];
    assert_eq!(ast, expected);
}

#[test]
fn test_if_directives_nested_ifdef() {
    let content = r#"
#ifdef FOO
#ifdef BAR
both
#else
only_foo
#endif
#endif
    "#;
    let ast = if_directives(content);
    let expected = vec![IncludeDirective::IfDef {
        symbol: "FOO".to_string(),
        then_branch: vec![IncludeDirective::IfDef {
            symbol: "BAR".to_string(),
            then_branch: vec![IncludeDirective::NonDirective("both".to_string())],
            else_branch: Some(vec![IncludeDirective::NonDirective("only_foo".to_string())]),
        }],
        else_branch: None,
    }];
    assert_eq!(ast, expected);
}

#[test]
fn test_if_directives_if_with_elif_and_nested() {
    let content = r#"
#if A
first
#elif B
#if 1
second_nested
#endif
#else
third
#endif
    "#;
    let ast = if_directives(content);
    let expected = vec![IncludeDirective::If {
        condition: "A".to_string(),
        then_branch: vec![IncludeDirective::NonDirective("first".to_string())],
        elif_branches: vec![(
            "B".to_string(),
            vec![IncludeDirective::If {
                condition: "1".to_string(),
                then_branch: vec![IncludeDirective::NonDirective("second_nested".to_string())],
                elif_branches: vec![],
                else_branch: None,
            }],
        )],
        else_branch: Some(vec![IncludeDirective::NonDirective("third".to_string())]),
    }];
    assert_eq!(ast, expected);
}
