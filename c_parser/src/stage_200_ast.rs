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
