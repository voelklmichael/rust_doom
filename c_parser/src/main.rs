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

fn lex_parse(content: &str) {
    let content = stage_100_comments::Stage100Comments::parse(content);
    let content = stage_110_preprocessor::Stage110Preprocessor::parse(content);
    let content = stage_200_ast::Stage200Ast::parse(content);
}

mod stage_110_preprocessor;
mod stage_200_ast {
    use crate::stage_110_preprocessor::Stage110Preprocessor;
    pub(crate) enum Stage200Ast {
        Comment(String),
        GlobalConstant {
            name: String,
            value: String,
        },
        MarcoFunction {
            name: String,
            argument_names: Vec<String>,
            body: String,
        },
        Include {
            path: String,
            is_system: bool,
        },
    }
    impl Stage200Ast {
        pub fn parse(tokens: Vec<Stage110Preprocessor>) -> Vec<Self> {
            let mut results = Vec::new();
            for token in tokens {
                results.extend(Self::parse_single(token));
            }
            results
        }

        fn parse_single(token: Stage110Preprocessor) -> Vec<Self> {
            match token {
                Stage110Preprocessor::Comment(x) => vec![Self::Comment(x)],
                Stage110Preprocessor::Code(x) => Self::parse_code(x),
                Stage110Preprocessor::Include { path, is_system } => {
                    vec![Self::Include { path, is_system }]
                }
                Stage110Preprocessor::Define {
                    name,
                    params,
                    value,
                } => vec![if let Some(params) = params {
                    Self::MarcoFunction {
                        name,
                        argument_names: params,
                        body: value,
                    }
                } else {
                    Self::GlobalConstant { name, value }
                }],
                Stage110Preprocessor::Undef { name: _ } => Default::default(),
            }
        }

        fn parse_code(x: String) -> Vec<Stage200Ast> {
            todo!()
        }
    }
}
