
use super::super::super::var::types::DeclareVar;
use super::super::super::var::Varables;

#[derive(Serialize, Deserialize, Debug)]
pub enum VarOp {
    DeclareVar {
        var_name: String,
        var_type: DeclareVar,
    },
    DeleteVar(String),
    GetVar {
        var_name: String,
        var_type: DeclareVar,
    },
}

impl VarOp {
    pub fn run(&self, state: &mut Varables) {
        match self {
            &VarOp::DeclareVar {
                ref var_name,
                ref var_type,
            } => {
                state.declare_var(var_name, var_type);
            }
            &VarOp::DeleteVar(ref var_name) => {}
            &VarOp::GetVar {
                ref var_name,
                ref var_type,
            } => {}
        }
    }
}
