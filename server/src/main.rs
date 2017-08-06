
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod lang;

use lang::Program;
use lang::action::ops::varops::VarOp;
use lang::action::ops::consoleops::ConsoleOp;
use lang::action::types::{AfterAction, Action};
use lang::var::types::{Var, DeclareVar};

fn main() {
    let mut p = Program::new();

    p.load_actions(vec![
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test"),
                var_type: DeclareVar::String,
            },
            success: AfterAction::Continue,
            failure: AfterAction::Panic,
        },
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("delete_me"),
                var_type: DeclareVar::Int,
            },
            success: AfterAction::Continue,
            failure: AfterAction::Panic,
        },
        Action::Var {
            op: VarOp::DeleteVar(String::from("delete_me")),
            success: AfterAction::Continue,
            failure: AfterAction::Panic,
        },
        Action::Var {
            op: VarOp::SetString {
                var_name: String::from("test"),
                var_value: Some(String::from("woot")),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Panic,
        },
    ]);

    p.run();

    println!("Actions {}", p.get_actions_str().unwrap());
    println!("State {}", p.get_state_str().unwrap());
}
