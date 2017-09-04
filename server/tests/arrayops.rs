
extern crate server;

use server::lang::Program;
use server::lang::action::ops::varops::VarOp;
use server::lang::action::ops::arrayops::ArrayOp;
use server::lang::action::types::{AfterAction, Action};
use server::lang::var::types::Var;

#[test]
fn push_push_pop_length() {
    let mut p = Program::new();

    p.load_actions(vec![
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test_array"),
                var: Var::Array(None),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
        Action::Array {
            op: ArrayOp::Push {
                array_name: String::from("test_array"),
                var: Var::String(Some(String::from("hello"))),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
        Action::Array {
            op: ArrayOp::Push {
                array_name: String::from("test_array"),
                var: Var::Int(Some(2)),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
        Action::Array {
            op: ArrayOp::Pop {
                array_name: String::from("test_array"),
                var_name: Some(String::from("test_i")),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
        Action::Array {
            op: ArrayOp::Length {
                array_name: String::from("test_array"),
                var_name: String::from("test_length"),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
    ]);

    p.run();

    assert_eq!(p.get_output().get_exit_status().unwrap(), 0);

    let state = p.get_state();

    let popped = state.get_int("test_i");

    assert!(popped.is_ok());

    assert_eq!(popped.unwrap().clone().unwrap(), 2);

    let length = state.get_size("test_length");

    assert!(length.is_ok());

    assert_eq!(length.unwrap().clone().unwrap(), 1);
}
