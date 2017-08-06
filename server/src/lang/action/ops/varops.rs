
use super::super::super::var::types::DeclareVar;
use super::super::super::var::Varables;

#[derive(Serialize, Deserialize, Debug)]
pub enum VarOp {
    DeclareVar {
        var_name: String,
        var_type: DeclareVar,
    },
    DeleteVar(String),
    SetString { var_name: String, var_value: String },
}

impl VarOp {
    pub fn run(&self, state: &mut Varables) -> Result<(), String> {
        match self {
            &VarOp::DeclareVar {
                ref var_name,
                ref var_type,
            } => state.declare_var(var_name, var_type),
            &VarOp::DeleteVar(ref var_name) => state.delete_var(var_name),
            &VarOp::SetString {
                ref var_name,
                ref var_value,
            } => Ok(()),
        }
    }
}
