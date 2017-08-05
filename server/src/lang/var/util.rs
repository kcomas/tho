
use super::types::{Var, DeclareVar, ShareVar};

pub fn declare_error<T>(var_name: &str) -> Result<T, String> {
    Err(format!("Variable: {} has allready been delcared", var_name))
}

pub fn not_found_error<T>(var_name: &str) -> Result<T, String> {
    Err(format!("Variable: {} does not exist", var_name))
}

pub fn wrong_type_error<T>(
    var_name: &str,
    expected: &DeclareVar,
    found: &Var,
) -> Result<T, String> {
    Err(format!(
        "Variable: {} expected variable type: {} found type: {}",
        var_name,
        expected.type_str(),
        found.type_str(),
    ))
}
