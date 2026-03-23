fn main() {
    let dir = std::path::Path::new("doomgeneric");
    let mut files = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    files.sort();

    let mut contents = Vec::with_capacity(files.len());
    for file in &files {
        let name = file.file_name().unwrap().to_str().unwrap();
        let content = std::fs::read_to_string(file).unwrap();

        dbg!(&name);
        let ast = lex_parse(&content);

        let unparsed: Vec<_> = ast
            .iter()
            .filter_map(|n| {
                if let stage_200_ast::Stage200Ast::Unparsed(s) = n {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .collect();
        if !unparsed.is_empty() {
            eprintln!("{}: {} Unparsed items (need implementation)", name, unparsed.len());
            for u in unparsed.iter().take(3) {
                eprintln!("  ... {}", u.trim().lines().next().unwrap_or(""));
            }
        }
        contents.push((name, ast));
    }

    let mut by_variant: std::collections::HashMap<&'static str, usize> =
        std::collections::HashMap::new();
    for (_, ast) in &contents {
        for node in ast {
            let key = match node {
                stage_200_ast::Stage200Ast::Comment(_) => "Comment",
                stage_200_ast::Stage200Ast::Include { .. } => "Include",
                stage_200_ast::Stage200Ast::GlobalConstant { .. } => "GlobalConstant",
                stage_200_ast::Stage200Ast::MacroFunction { .. } => "MacroFunction",
                stage_200_ast::Stage200Ast::TypedefEnum { .. } => "TypedefEnum",
                stage_200_ast::Stage200Ast::TypedefStruct { .. } => "TypedefStruct",
                stage_200_ast::Stage200Ast::TypedefUnion { .. } => "TypedefUnion",
                stage_200_ast::Stage200Ast::TypedefSimple { .. } => "TypedefSimple",
                stage_200_ast::Stage200Ast::StructDef { .. } => "StructDef",
                stage_200_ast::Stage200Ast::UnionDef { .. } => "UnionDef",
                stage_200_ast::Stage200Ast::EnumDef { .. } => "EnumDef",
                stage_200_ast::Stage200Ast::FunctionDecl { .. } => "FunctionDecl",
                stage_200_ast::Stage200Ast::FunctionDef { .. } => "FunctionDef",
                stage_200_ast::Stage200Ast::OtherDecl(_) => "OtherDecl",
                stage_200_ast::Stage200Ast::Unparsed(_) => "Unparsed",
            };
            *by_variant.entry(key).or_insert(0) += 1;
        }
    }
    eprintln!("\nAST node counts (feedback for implementation):");
    let mut v: Vec<_> = by_variant.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    for (k, count) in v {
        eprintln!("  {}: {}", k, count);
    }
    dbg!(contents.len());
}

mod stage_100_comments;
mod stage_110_preprocessor;
mod stage_200_ast;

fn lex_parse(content: &str) -> Vec<stage_200_ast::Stage200Ast> {
    let content = stage_100_comments::Stage100Comments::parse(content);
    let content = stage_110_preprocessor::Stage110Preprocessor::parse(content);
    stage_200_ast::Stage200Ast::parse(content)
}
