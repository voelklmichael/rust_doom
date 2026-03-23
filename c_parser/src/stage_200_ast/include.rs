use super::Stage200Ast;
use crate::stage_110_preprocessor::Stage110Preprocessor;

pub(super) fn handle(token: &Stage110Preprocessor) -> Option<Stage200Ast> {
    if let Stage110Preprocessor::Include { path, is_system } = token {
        Some(Stage200Ast::Include {
            path: path.clone(),
            is_system: *is_system,
        })
    } else {
        None
    }
}
