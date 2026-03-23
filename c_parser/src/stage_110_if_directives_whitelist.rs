use super::stage_100_if_directives::IncludeDirective;

pub fn if_directives_whitelist(ast: Vec<IncludeDirective>) -> String {
    let mut results = Vec::new();
    for node in ast {
        match node {
            IncludeDirective::NonDirective(code) => results.push(code),
            IncludeDirective::IfDef {
                symbol,
                then_branch,
                else_branch,
            } => todo!("{symbol}"),
            IncludeDirective::IfNDef {
                symbol,
                then_branch,
                else_branch,
            } => {
                let inlude_guard_whitelist = [];
            }
            IncludeDirective::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => todo!("{condition}"),
        }
    }
    results.join("\n")
}
