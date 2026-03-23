mod stage_100_if_directives;
mod stage_110_if_directives_whitelist;

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
        let ast = include_lex_parse(&content);
        contents.push((name, ast));
    }

    dbg!(contents.len());
}

fn include_lex_parse(content: &str) -> String {
    let if_directives_ast = stage_100_if_directives::if_directives(content);
    let whitelisted_ast =
        stage_110_if_directives_whitelist::if_directives_whitelist(if_directives_ast);
    whitelisted_ast
}
