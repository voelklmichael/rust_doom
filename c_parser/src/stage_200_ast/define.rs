use super::Stage200Ast;
use crate::stage_110_preprocessor::Stage110Preprocessor;

pub(super) fn handle(token: &Stage110Preprocessor) -> Option<Stage200Ast> {
    if let Stage110Preprocessor::Define {
        name,
        params,
        value,
    } = token
    {
        Some(if let Some(params) = params {
            Stage200Ast::MacroFunction {
                name: name.clone(),
                argument_names: params.clone(),
                body: value.clone(),
            }
        } else {
            Stage200Ast::GlobalConstant {
                name: name.clone(),
                value: value.clone(),
            }
        })
    } else {
        None
    }
}
