
use super::ops::varops::VarOp;
use super::super::var::Varables;

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Var(VarOp),
    RunMacro(String),
}

impl Action {
    pub fn run(&self, state: &mut Varables) {
        match self {
            &Action::Var(ref op) => op.run(state),
            &Action::RunMacro(ref macro_name) => {}
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AfterAction {
    Continue,
    Exit(u64),
    Warn(String),
    Panic { message: String, exit_code: u64 },
    Default,
    Next(Action),
}
