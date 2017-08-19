
extern crate server;

use server::lang::Program;
use server::lang::action::ops::varops::VarOp;
use server::lang::action::types::{AfterAction, Action};
use server::lang::var::types::Var;

#[test]
fn declare_string() {
    let mut p = Program::new();

    p.load_actions(vec![
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test_string"),
                var: Var::String(Some(String::from("asdf"))),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
    ]);

    p.run();

    assert_eq!(p.get_output().get_exit_status().unwrap(), 0);

    let state = p.get_state();

    let rst = state.get_string("test_string");

    assert!(rst.is_ok());

    assert_eq!(rst.unwrap().clone().unwrap(), "asdf");
}

#[test]
fn declare_int() {
    let mut p = Program::new();

    p.load_actions(vec![
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test"),
                var: Var::Int(Some(3)),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
    ]);

    p.run();

    assert_eq!(p.get_output().get_exit_status().unwrap(), 0);

    let state = p.get_state();

    let rst = state.get_int("test");

    assert!(rst.is_ok());

    assert_eq!(rst.unwrap().unwrap(), 3);
}

#[test]
fn decalre_float() {
    let mut p = Program::new();

    p.load_actions(vec![
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test_float"),
                var: Var::Float(Some(4.56)),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
    ]);

    p.run();

    assert_eq!(p.get_output().get_exit_status().unwrap(), 0);

    let state = p.get_state();

    let rst = state.get_float("test_float");

    assert!(rst.is_ok());

    assert_eq!(rst.unwrap().unwrap(), 4.56);
}

#[test]
fn decalre_size() {
    let mut p = Program::new();

    p.load_actions(vec![
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test_size"),
                var: Var::Size(Some(20)),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
    ]);

    p.run();

    assert_eq!(p.get_output().get_exit_status().unwrap(), 0);

    let state = p.get_state();

    let rst = state.get_size("test_size");

    assert!(rst.is_ok());

    assert_eq!(rst.unwrap().unwrap(), 20);
}

#[test]
fn decalre_bool() {
    let mut p = Program::new();

    p.load_actions(vec![
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test_bool"),
                var: Var::Bool(Some(true)),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Error,
        },
    ]);

    p.run();

    assert_eq!(p.get_output().get_exit_status().unwrap(), 0);

    let state = p.get_state();

    let rst = state.get_bool("test_bool");

    assert!(rst.is_ok());

    assert_eq!(rst.unwrap().unwrap(), true);
}
