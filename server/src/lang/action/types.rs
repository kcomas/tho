
use super::ops::Op;
use super::ops::varops::VarOp;
use super::ops::arrayops::ArrayOp;
use super::super::var::Varables;
use super::super::output::Output;

type ActionResult = Result<Option<String>, String>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    Var {
        op: VarOp,
        success: AfterAction,
        failure: AfterAction,
    },
    Array {
        op: ArrayOp,
        success: AfterAction,
        failure: AfterAction,
    },
}

impl Action {
    fn run_after_op(
        result: Result<String, String>,
        success: &AfterAction,
        failure: &AfterAction,
    ) -> ActionResult {
        match result {
            Ok(message) => success.run(&message),
            Err(message) => failure.run(&message),
        }
    }

    fn do_action<T: Op>(
        state: &mut Varables,
        output: &mut Output,
        op: &T,
        success: &AfterAction,
        failure: &AfterAction,
    ) -> ActionResult {
        let rst = op.run(state, output);
        Action::run_after_op(rst, success, failure)
    }

    pub fn run(&self, state: &mut Varables, output: &mut Output) -> ActionResult {
        match self {
            &Action::Var {
                ref op,
                ref success,
                ref failure,
            } => Action::do_action(state, output, op, success, failure),
            &Action::Array {
                ref op,
                ref success,
                ref failure,
            } => Action::do_action(state, output, op, success, failure),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AfterAction {
    Continue,
    Exit(),
    Log,
    LogMessage(String),
    Error,
    ErrorMessage(String),
    // macro name
    Next(String),
}

impl AfterAction {
    pub fn run(&self, action_message: &str) -> ActionResult {
        match self {
            &AfterAction::Continue => Ok(None),
            &AfterAction::Exit() => Err(String::from("Exit")),
            &AfterAction::Log => Ok(Some(format!("{}", action_message))),
            &AfterAction::LogMessage(ref message) => Ok(Some(format!("{}", message))),
            &AfterAction::Error => Err(format!("{}", action_message)),
            &AfterAction::ErrorMessage(ref message) => Err(format!("{}", message)),
            &AfterAction::Next(ref macro_name) => Ok(None),
        }
    }
}
