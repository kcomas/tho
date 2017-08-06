
use super::super::super::var::types::DeclareVar;
use super::super::super::var::Varables;
use super::Op;
use std::clone::Clone;

#[derive(Serialize, Deserialize, Debug)]
pub enum VarOp {
    DeclareVar {
        var_name: String,
        var_type: DeclareVar,
    },
    DeleteVar(String),
    SetString {
        var_name: String,
        var_value: Option<String>,
    },
}

fn match_update<T: Clone>(rst: Result<&mut T, String>, var_value: &T) -> Result<(), String> {
    match rst {
        Ok(value) => {
            *value = var_value.clone();
            Ok(())
        }
        Err(msg) => Err(msg),
    }
}

impl Op for VarOp {
    fn run(&self, state: &mut Varables) -> Result<(), String> {
        match self {
            &VarOp::DeclareVar {
                ref var_name,
                ref var_type,
            } => state.declare_var(var_name, var_type),
            &VarOp::DeleteVar(ref var_name) => state.delete_var(var_name),
            &VarOp::SetString {
                ref var_name,
                ref var_value,
            } => {
                let mut rst = state.get_string(var_name);
                match_update(rst, var_value)
            }
        }
    }
}
