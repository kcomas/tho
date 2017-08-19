
use super::super::super::action::types::Action;
use super::super::super::var::types::Var;
use super::super::super::var::Varables;
use super::super::super::output::Output;
use super::Op;
use std::clone::Clone;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VarOp {
    DeclareVar { var_name: String, var: Var },
    DeleteVar(String),
    SetMacro {
        var_name: String,
        var_value: Option<Vec<Action>>,
    },
    RunMacro(String), // macro name
    SetString {
        var_name: String,
        var_value: Option<String>,
    },
    SetInt {
        var_name: String,
        var_value: Option<i64>,
    },
    SetFloat {
        var_name: String,
        var_value: Option<f64>,
    },
    SetSize {
        var_name: String,
        var_value: Option<usize>,
    },
    SetBool {
        var_name: String,
        var_value: Option<bool>,
    },
}

fn match_update<T: Clone>(
    rst: Result<&mut T, String>,
    var_name: &str,
    var_value: &T,
) -> Result<String, String> {
    match rst {
        Ok(value) => {
            *value = var_value.clone();
            Ok(format!("Set Value For: {}", var_name))
        }
        Err(msg) => Err(msg),
    }
}

pub fn run_macro(
    rst: Result<Option<Vec<Action>>, String>,
    var_name: &String,
    state: &mut Varables,
    output: &mut Output,
) -> Result<String, String> {
    match rst {
        Ok(option) => {
            match option {
                Some(actions) => {
                    for action in actions.iter() {
                        let action_rst = action.run(state, output);
                        match action_rst {
                            Ok(option) => {
                                if let Some(string) = option {
                                    output.log(format!("Macro Log {}: {}", var_name, string));
                                }
                            }
                            Err(string) => {
                                return Err(format!("Macro Error {}: {}", var_name, string));
                            }
                        }
                    }
                    Ok(format!("Ran Macro: {}", var_name))
                }
                None => Err(String::from("Macro Declared But Not Set")),
            }
        }
        Err(msg) => Err(msg),
    }
}

impl Op for VarOp {
    fn run(&self, state: &mut Varables, output: &mut Output) -> Result<String, String> {
        match self {
            &VarOp::DeclareVar {
                ref var_name,
                ref var,
            } => state.declare_var(var_name, var),
            &VarOp::DeleteVar(ref var_name) => state.delete_var(var_name),
            &VarOp::SetMacro {
                ref var_name,
                ref var_value,
            } => {
                let rst = state.get_macro_mut(var_name);
                match_update(rst, var_name, var_value)
            }
            &VarOp::RunMacro(ref var_name) => {
                let rst = state.get_macro(var_name);
                run_macro(rst, var_name, state, output)
            }
            &VarOp::SetString {
                ref var_name,
                ref var_value,
            } => {
                let rst = state.get_string_mut(var_name);
                match_update(rst, var_name, var_value)
            }
            &VarOp::SetInt {
                ref var_name,
                ref var_value,
            } => {
                let rst = state.get_int_mut(var_name);
                match_update(rst, var_name, var_value)
            }
            &VarOp::SetFloat {
                ref var_name,
                ref var_value,
            } => {
                let rst = state.get_float_mut(var_name);
                match_update(rst, var_name, var_value)
            }
            &VarOp::SetSize {
                ref var_name,
                ref var_value,
            } => {
                let rst = state.get_size_mut(var_name);
                match_update(rst, var_name, var_value)
            }
            &VarOp::SetBool {
                ref var_name,
                ref var_value,
            } => {
                let rst = state.get_bool_mut(var_name);
                match_update(rst, var_name, var_value)
            }
        }
    }
}
