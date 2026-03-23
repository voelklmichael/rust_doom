use super::Stage200Ast;
use crate::stage_110_preprocessor::Stage110Preprocessor;

pub(super) fn handle(token: &Stage110Preprocessor) -> Option<Stage200Ast> {
    if let Stage110Preprocessor::Comment(x) = token {
        Some(Stage200Ast::Comment(x.clone()))
    } else {
        None
    }
}
