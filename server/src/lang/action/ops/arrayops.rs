
use super::super::super::action::types::Action;
use super::super::super::var::types::Var;
use super::super::super::var::Varables;
use super::super::super::output::Output;
use super::Op;
use std::clone::Clone;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ArrayOp {
    Push { array_name: String, var: Var },
//    Pop {
//        array_name: String,
//        new_var_name: String,
//    },
//    UnShift { array_name: String, var: Var },
//    Shift {
//        array_name: String,
//        new_var_name: String,
//    },
//    Length {
//        array_name: String,
//        new_var_name: String,
//    },
}

impl Op for ArrayOp {
    fn run(&self, state: &mut Varables, output: &mut Output) -> Result<String, String> {
        match self {
            &ArrayOp::Push {
                ref array_name,
                ref var,
            } => {
                let rst = state.get_array(array_name);
                match rst {
                    Ok(option_array) => {
                        if let None = *option_array {
                            *option_array = Some(Box::new(Vec::new()));
                        }
                        if let Some(ref mut arr) = *option_array {
                            arr.push(var.clone());
                            return Ok(format!("Pushed To Array: {}", array_name));
                        }
                        Err(format!(
                            "Unable To Dereference Array {} For Push",
                            array_name
                        ))
                    }
                    Err(msg) => Err(msg),
                }
            }
        }
    }
}
