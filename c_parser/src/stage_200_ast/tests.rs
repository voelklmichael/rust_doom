use super::*;
use crate::stage_100_comments::Stage100Comments;

fn body_chunks_contain(body: &[BodyChunk], s: &str) -> bool {
    body.iter().any(|c| match c {
        BodyChunk::Code(code) => code.contains(s),
        BodyChunk::Comment(_) => false,
    })
}

fn parse_code_to_ast(code: &str) -> Vec<Stage200Ast> {
    let stage100 = Stage100Comments::parse(code);
    let stage110 = Stage110Preprocessor::parse(stage100);
    Stage200Ast::parse(stage110)
}

#[test]
fn test_typedef_struct_forward() {
    let code = r#"typedef struct lumpinfo_s lumpinfo_t;"#;
    let ast = parse_code_to_ast(code);
    let tds: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefStruct { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(tds.len(), 1);
    assert_eq!(tds[0].0, "lumpinfo_t");
    assert!(tds[0].1.is_empty());
}

#[test]
fn test_typedef_struct_with_tag() {
    let code = r#"typedef struct memblock_s { int x; } memblock_t;"#;
    let ast = parse_code_to_ast(code);
    let tds: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefStruct { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(tds.len(), 1);
    assert_eq!(tds[0].0, "memblock_t");
    assert!(body_chunks_contain(&tds[0].1, "int x"));
}

#[test]
fn test_typedef_struct_anonymous() {
    let code = r#"typedef struct { int x; } name_t;"#;
    let ast = parse_code_to_ast(code);
    let tds: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefStruct { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(tds.len(), 1);
    assert_eq!(tds[0].0, "name_t");
    assert!(body_chunks_contain(&tds[0].1, "int x"));
}

#[test]
fn test_struct_def_forward() {
    let code = r#"struct foo;"#;
    let ast = parse_code_to_ast(code);
    let structs: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::StructDef { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(structs.len(), 1);
    assert_eq!(structs[0].0, "foo");
    assert!(structs[0].1.is_empty());
}

#[test]
fn test_struct_def_simple() {
    let code = r#"struct foo { int x; };"#;
    let ast = parse_code_to_ast(code);
    let structs: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::StructDef { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(structs.len(), 1, "expected 1 StructDef, got ast: {:?}", ast);
    assert_eq!(structs[0].0, "foo");
    assert!(body_chunks_contain(&structs[0].1, "int x"));
}

#[test]
fn test_typedef_union_forward() {
    let code = r#"typedef union uinfo_s uinfo_t;"#;
    let ast = parse_code_to_ast(code);
    let unions: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefUnion { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(unions.len(), 1);
    assert_eq!(unions[0].0, "uinfo_t");
    assert!(unions[0].1.is_empty());
}

#[test]
fn test_typedef_union_with_tag() {
    let code = r#"typedef union memblock_s { int x; } memblock_t;"#;
    let ast = parse_code_to_ast(code);
    let unions: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefUnion { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(unions.len(), 1);
    assert_eq!(unions[0].0, "memblock_t");
    assert!(body_chunks_contain(&unions[0].1, "int x"));
}

#[test]
fn test_typedef_union() {
    let code = r#"typedef union { int i; float f; } num_t;"#;
    let ast = parse_code_to_ast(code);
    let unions: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefUnion { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(unions.len(), 1);
    assert_eq!(unions[0].0, "num_t");
    assert!(body_chunks_contain(&unions[0].1, "int i"));
}

#[test]
fn test_typedef_simple_multiword() {
    let code = r#"typedef unsigned long size_t;"#;
    let ast = parse_code_to_ast(code);
    let simples: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefSimple { base_type, name } => Some((base_type, name)),
            _ => None,
        })
        .collect();
    assert_eq!(simples.len(), 1);
    assert_eq!(simples[0].0, "unsigned long");
    assert_eq!(simples[0].1, "size_t");
}

#[test]
fn test_typedef_simple() {
    let code = r#"typedef int fixed_t;"#;
    let ast = parse_code_to_ast(code);
    let simples: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefSimple { base_type, name } => Some((base_type, name)),
            _ => None,
        })
        .collect();
    assert_eq!(simples.len(), 1);
    assert_eq!(simples[0].0, "int");
    assert_eq!(simples[0].1, "fixed_t");
}

#[test]
fn test_enum_def() {
    let code = r#"enum color { RED, GREEN, BLUE };"#;
    let ast = parse_code_to_ast(code);
    let enums: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::EnumDef { name, variants } => Some((name, variants)),
            _ => None,
        })
        .collect();
    assert_eq!(enums.len(), 1);
    assert_eq!(enums[0].0, "color");
    assert_eq!(enums[0].1.len(), 3);
}

#[test]
fn test_struct_body_chunks() {
    let code = r#"struct foo { int x; int y; };"#;
    let ast = parse_code_to_ast(code);
    let structs: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::StructDef { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(structs.len(), 1);
    assert_eq!(structs[0].0, "foo");
    assert!(body_chunks_contain(&structs[0].1, "int x"));
    assert!(body_chunks_contain(&structs[0].1, "int y"));
}

// --- typedef_enum ---
#[test]
fn test_typedef_enum() {
    let code = r#"typedef enum { OFF, ON } state_t;"#;
    let ast = parse_code_to_ast(code);
    let enums: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefEnum { name, variants } => Some((name, variants)),
            _ => None,
        })
        .collect();
    assert_eq!(enums.len(), 1);
    assert_eq!(enums[0].0, "state_t");
    assert_eq!(enums[0].1.len(), 2);
    assert_eq!(enums[0].1[0].name, "OFF");
    assert_eq!(enums[0].1[1].name, "ON");
}

#[test]
fn test_typedef_enum_with_values() {
    let code = r#"typedef enum { RED = 0, GREEN = 1, BLUE = 2 } color_t;"#;
    let ast = parse_code_to_ast(code);
    let enums: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefEnum { name, variants } => Some((name, variants)),
            _ => None,
        })
        .collect();
    assert_eq!(enums.len(), 1);
    assert_eq!(enums[0].0, "color_t");
    assert_eq!(enums[0].1[0].name, "RED");
    assert_eq!(enums[0].1[0].value, Some("0".to_string()));
    assert_eq!(enums[0].1[1].value, Some("1".to_string()));
    assert_eq!(enums[0].1[2].value, Some("2".to_string()));
}

// --- union_def ---
#[test]
fn test_union_def() {
    let code = r#"union bar { int i; float f; };"#;
    let ast = parse_code_to_ast(code);
    let unions: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::UnionDef { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(unions.len(), 1);
    assert_eq!(unions[0].0, "bar");
    assert!(body_chunks_contain(&unions[0].1, "int i"));
}

#[test]
fn test_union_def_forward() {
    let code = r#"union bar;"#;
    let ast = parse_code_to_ast(code);
    let unions: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::UnionDef { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(unions.len(), 1);
    assert_eq!(unions[0].0, "bar");
    assert!(unions[0].1.is_empty());
}

// --- enum_def ---
#[test]
fn test_enum_def_with_values() {
    let code = r#"enum flags { F_A = 1, F_B = 2, F_C = 4 };"#;
    let ast = parse_code_to_ast(code);
    let enums: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::EnumDef { name, variants } => Some((name, variants)),
            _ => None,
        })
        .collect();
    assert_eq!(enums.len(), 1);
    assert_eq!(enums[0].0, "flags");
    assert_eq!(enums[0].1[0].value, Some("1".to_string()));
    assert_eq!(enums[0].1[1].value, Some("2".to_string()));
}

#[test]
fn test_enum_def_forward() {
    let code = r#"enum opaque;"#;
    let ast = parse_code_to_ast(code);
    let enums: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::EnumDef { name, variants } => Some((name, variants)),
            _ => None,
        })
        .collect();
    assert_eq!(enums.len(), 1);
    assert_eq!(enums[0].0, "opaque");
    assert!(enums[0].1.is_empty());
}

// --- function (FunctionDecl / FunctionDef) ---
#[test]
fn test_function_decl_void_params() {
    let code = r#"void foo(void);"#;
    let ast = parse_code_to_ast(code);
    let funcs: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::FunctionDecl {
                return_type,
                name,
                params,
            } => Some((return_type, name, params)),
            _ => None,
        })
        .collect();
    assert_eq!(funcs.len(), 1);
    assert_eq!(funcs[0].0, "void");
    assert_eq!(funcs[0].1, "foo");
    assert!(funcs[0].2.contains("void"));
}

#[test]
fn test_static_function_decl() {
    let code = r#"static void helper(void);"#;
    let ast = parse_code_to_ast(code);
    let funcs: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::FunctionDecl {
                return_type,
                name,
                ..
            } => Some((return_type, name)),
            _ => None,
        })
        .collect();
    assert_eq!(funcs.len(), 1);
    assert_eq!(funcs[0].0, "static void");
    assert_eq!(funcs[0].1, "helper");
}

#[test]
fn test_function_decl() {
    let code = r#"void foo(int x);"#;
    let ast = parse_code_to_ast(code);
    let funcs: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::FunctionDecl {
                return_type,
                name,
                params,
            } => Some((return_type, name, params)),
            _ => None,
        })
        .collect();
    assert_eq!(funcs.len(), 1);
    assert_eq!(funcs[0].0, "void");
    assert_eq!(funcs[0].1, "foo");
    assert!(funcs[0].2.contains("int x"));
}

#[test]
fn test_function_def() {
    let code = r#"void foo(int x) { return; }"#;
    let ast = parse_code_to_ast(code);
    let funcs: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::FunctionDef {
                return_type,
                name,
                params,
                body,
            } => Some((return_type, name, params, body)),
            _ => None,
        })
        .collect();
    assert_eq!(funcs.len(), 1);
    assert_eq!(funcs[0].0, "void");
    assert_eq!(funcs[0].1, "foo");
    assert!(body_chunks_contain(&funcs[0].3, "return"));
}

#[test]
fn test_static_function_def() {
    let code = r#"static void helper(void) { }"#;
    let ast = parse_code_to_ast(code);
    let funcs: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::FunctionDef {
                return_type,
                name,
                ..
            } => Some((return_type, name)),
            _ => None,
        })
        .collect();
    assert_eq!(funcs.len(), 1);
    assert_eq!(funcs[0].0, "static void");
    assert_eq!(funcs[0].1, "helper");
}

// --- OtherDecl (extern, variables) ---
#[test]
fn test_extern_decl() {
    let code = r#"extern void bar(void);"#;
    let ast = parse_code_to_ast(code);
    let others: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::OtherDecl(s) => Some(s),
            _ => None,
        })
        .collect();
    assert_eq!(others.len(), 1);
    assert!(others[0].contains("bar"));
    assert!(others[0].contains("void"));
}

#[test]
fn test_other_decl_variable() {
    let code = r#"int global_var;"#;
    let ast = parse_code_to_ast(code);
    let others: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::OtherDecl(s) => Some(s),
            _ => None,
        })
        .collect();
    assert_eq!(others.len(), 1);
    assert!(others[0].contains("global_var"));
}

// --- Unparsed ---
#[test]
fn test_unparsed() {
    let code = r#";"#;
    let ast = parse_code_to_ast(code);
    let unparsed: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::Unparsed(s) => Some(s),
            _ => None,
        })
        .collect();
    assert_eq!(unparsed.len(), 1);
}

// --- typedef_union additional variants ---
#[test]
fn test_typedef_union_anonymous() {
    let code = r#"typedef union { short s; long l; } u;"#;
    let ast = parse_code_to_ast(code);
    let unions: Vec<_> = ast
        .iter()
        .filter_map(|n| match n {
            Stage200Ast::TypedefUnion { name, body } => Some((name, body)),
            _ => None,
        })
        .collect();
    assert_eq!(unions.len(), 1);
    assert_eq!(unions[0].0, "u");
    assert!(body_chunks_contain(&unions[0].1, "short s"));
}
