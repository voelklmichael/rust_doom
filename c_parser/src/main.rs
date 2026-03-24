mod stage_100_if_directives;
mod stage_110_if_directives_whitelist;
mod stage_200_lexing;
mod stage_300_parsing;
mod stage_320_parsing;
mod stage_340_parsing;

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

fn include_lex_parse(content: &str) {
    let if_directives_ast = stage_100_if_directives::if_directives(content);
    let whitelisted = stage_110_if_directives_whitelist::if_directives_whitelist(if_directives_ast);
    let lexed = stage_200_lexing::lexing(whitelisted);
    let tu = stage_300_parsing::parsing_stage_300(lexed);
    let tu320 = stage_320_parsing::parsing_stage_320(tu);
    let parsed = stage_340_parsing::parsing_stage_340(tu320);
    //dbg!(&parsed);
    parsed.0.iter().for_each(|x| match x {
        stage_340_parsing::ExternalDecl340::UnparsedDeclaration(lexed_tokens) => {
            dbg!(&lexed_tokens);
        }
        _ => {}
    });
}
