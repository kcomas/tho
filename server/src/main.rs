
extern crate server;

use server::lang::Program;
use server::lang::action::ops::varops::VarOp;
use server::lang::action::ops::arrayops::ArrayOp;
use server::lang::action::types::{AfterAction, Action};
use server::lang::var::types::Var;

fn main() {
    let mut p = Program::new();

    p.load_actions(vec![
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test_string"),
                var: Var::String(None),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test_float"),
                var: Var::Float(Some(1.2)),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("delete_me"),
                var: Var::Int(None),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
        Action::Var {
            op: VarOp::DeleteVar(String::from("delete_me")),
            success: AfterAction::Log,
            failure: AfterAction::Error,
        },
        Action::Var {
            op: VarOp::SetString {
                var_name: String::from("test_string"),
                var_value: Some(String::from("woot")),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("arr"),
                var: Var::Array(None),
            },
            success: AfterAction::Log,
            failure: AfterAction::Error,
        },
        Action::Array {
            op: ArrayOp::Push {
                array_name: String::from("arr"),
                var: Var::Int(Some(2)),
            },
            success: AfterAction::Log,
            failure: AfterAction::Error,
        },
        Action::Array {
            op: ArrayOp::Push {
                array_name: String::from("arr"),
                var: Var::String(Some(String::from("Idx 2"))),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
    ]);

    p.run();

    println!("Actions {}", p.get_actions_str_pretty().unwrap());
    println!("State {}", p.get_state_str_pretty().unwrap());
    println!("Output {}", p.get_output_str_pretty().unwrap());
}
