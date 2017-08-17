
extern crate server;

use server::lang::Program;
use server::lang::action::ops::varops::VarOp;
use server::lang::action::types::{AfterAction, Action};
use server::lang::var::types::Var;

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

    let mut state = (*p.get_state()).clone();

    let rst = state.get_int("test");

    assert!(rst.is_ok());

    assert_eq!(rst.unwrap().unwrap(), 3);
}
