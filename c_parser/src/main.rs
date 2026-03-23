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

        contents.push((name, ast));
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
