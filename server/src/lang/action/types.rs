
use super::ops::Op;
use super::ops::varops::VarOp;
use super::super::var::Varables;

type ActionResult = Result<Option<String>, String>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    Var {
        op: VarOp,
        success: AfterAction,
        failure: AfterAction,
    },
}

impl Action {
    fn run_after_op(
        state: &mut Varables,
        result: Result<(), String>,
        success: &AfterAction,
        failure: &AfterAction,
    ) -> ActionResult {
        match result {
            Ok(_) => success.run("No Error Occured"),
            Err(message) => failure.run(&message),
        }
    }

    fn do_action<T: Op>(
        state: &mut Varables,
        op: &T,
        success: &AfterAction,
        failure: &AfterAction,
    ) -> ActionResult {
        let rst = op.run(state);
        Action::run_after_op(state, rst, success, failure)
    }

    pub fn run(&self, state: &mut Varables) -> ActionResult {
        match self {
            &Action::Var {
                ref op,
                ref success,
                ref failure,
            } => Action::do_action(state, op, success, failure),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AfterAction {
    Continue,
    Exit(),
    Warn,
    WarnMessage(String),
    Panic,
    PanicMessage(String),
    // macro name
    Next(String),
}

impl AfterAction {
    pub fn run(&self, error_message: &str) -> ActionResult {
        match self {
            &AfterAction::Continue => Ok(None),
            &AfterAction::Exit() => Err(String::from("Exit")),
            &AfterAction::Warn => Ok(Some(format!("{}", error_message))),
            &AfterAction::WarnMessage(ref message) => Ok(Some(format!("{}", message))),
            &AfterAction::Panic => Err(format!("{}", error_message)),
            &AfterAction::PanicMessage(ref message) => Err(format!("{}", message)),
            &AfterAction::Next(ref macro_name) => Ok(None),
        }
    }
}
