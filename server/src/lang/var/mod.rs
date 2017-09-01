
use std::collections::HashMap;

pub mod types;
mod util;

use super::action::types::Action;
use self::types::{Var, DeclareVar, ShareVar};
use self::util::{declare_error, not_found_error, wrong_type_error};

pub type VarArrayType = Option<Box<Vec<Var>>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Varables {
    data: HashMap<String, Var>,
}

impl Varables {
    pub fn new() -> Varables {
        Varables { data: HashMap::new() }
    }

    pub fn declare_var(&mut self, var_name: &str, var: &Var) -> Result<String, String> {
        if let Some(_) = self.data.get(var_name) {
            return declare_error(var_name);
        }
        self.data.insert(String::from(var_name), var.clone());
        Ok(format!("Created {}: {}", var.type_str(), var_name))
    }

    pub fn re_declare_var(&mut self, var_name: &str, var: &Var) -> Result<String, String> {
        if let Some(_) = self.data.get(var_name) {
            if let Err(msg) = self.delete_var(var_name) {
                return Err(msg);
            }
        }
        self.declare_var(var_name, var)
    }

    pub fn delete_var(&mut self, var_name: &str) -> Result<String, String> {
        if let Some(_) = self.data.remove(var_name) {
            return Ok(format!("Deleted Var: {}", var_name));
        }
        not_found_error(var_name)
    }

    pub fn get_macro_mut(&mut self, var_name: &str) -> Result<&mut Option<Vec<Action>>, String> {
        let rst = self.get_var_mut(var_name);
        match rst {
            Ok(var) => {
                if let Var::Macro(ref mut actions) = *var {
                    return Ok(actions);
                }
                wrong_type_error(var_name, &DeclareVar::Macro, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_macro(&self, var_name: &str) -> Result<Option<Vec<Action>>, String> {
        let rst = self.get_var(var_name);
        match rst {
            Ok(var) => {
                if let Var::Macro(ref actions) = *var {
                    return Ok(actions.clone());
                }
                wrong_type_error(var_name, &DeclareVar::Macro, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_string_mut(&mut self, var_name: &str) -> Result<&mut Option<String>, String> {
        let rst = self.get_var_mut(var_name);
        match rst {
            Ok(var) => {
                if let Var::String(ref mut string) = *var {
                    return Ok(string);
                }
                wrong_type_error(var_name, &DeclareVar::String, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_string(&self, var_name: &str) -> Result<&Option<String>, String> {
        let rst = self.get_var(var_name);
        match rst {
            Ok(var) => {
                if let Var::String(ref string) = *var {
                    return Ok(string);
                }
                wrong_type_error(var_name, &DeclareVar::String, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_int_mut(&mut self, var_name: &str) -> Result<&mut Option<i64>, String> {
        let rst = self.get_var_mut(var_name);
        match rst {
            Ok(var) => {
                if let Var::Int(ref mut int) = *var {
                    return Ok(int);
                }
                wrong_type_error(var_name, &DeclareVar::Int, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_int(&self, var_name: &str) -> Result<&Option<i64>, String> {
        let rst = self.get_var(var_name);
        match rst {
            Ok(var) => {
                if let Var::Int(ref int) = *var {
                    return Ok(int);
                }
                wrong_type_error(var_name, &DeclareVar::Int, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_float_mut(&mut self, var_name: &str) -> Result<&mut Option<f64>, String> {
        let rst = self.get_var_mut(var_name);
        match rst {
            Ok(var) => {
                if let Var::Float(ref mut float) = *var {
                    return Ok(float);
                }
                wrong_type_error(var_name, &DeclareVar::Float, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_float(&self, var_name: &str) -> Result<&Option<f64>, String> {
        let rst = self.get_var(var_name);
        match rst {
            Ok(var) => {
                if let Var::Float(ref float) = *var {
                    return Ok(float);
                }
                wrong_type_error(var_name, &DeclareVar::Float, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_size_mut(&mut self, var_name: &str) -> Result<&mut Option<usize>, String> {
        let rst = self.get_var_mut(var_name);
        match rst {
            Ok(var) => {
                if let Var::Size(ref mut size) = *var {
                    return Ok(size);
                }
                wrong_type_error(var_name, &DeclareVar::Size, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_size(&self, var_name: &str) -> Result<&Option<usize>, String> {
        let rst = self.get_var(var_name);
        match rst {
            Ok(var) => {
                if let Var::Size(ref size) = *var {
                    return Ok(size);
                }
                wrong_type_error(var_name, &DeclareVar::Size, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_bool_mut(&mut self, var_name: &str) -> Result<&mut Option<bool>, String> {
        let rst = self.get_var_mut(var_name);
        match rst {
            Ok(var) => {
                if let Var::Bool(ref mut boolean) = *var {
                    return Ok(boolean);
                }
                wrong_type_error(var_name, &DeclareVar::Bool, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_bool(&self, var_name: &str) -> Result<&Option<bool>, String> {
        let rst = self.get_var(var_name);
        match rst {
            Ok(var) => {
                if let Var::Bool(ref boolean) = *var {
                    return Ok(boolean);
                }
                wrong_type_error(var_name, &DeclareVar::Bool, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_array_mut(&mut self, var_name: &str) -> Result<&mut VarArrayType, String> {
        let rst = self.get_var_mut(var_name);
        match rst {
            Ok(var) => {
                if let Var::Array(ref mut array) = *var {
                    return Ok(array);
                }
                wrong_type_error(var_name, &DeclareVar::Array, var)
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn get_array(&self, var_name: &str) -> Result<&VarArrayType, String> {
        let rst = self.get_var(var_name);
        match rst {
            Ok(var) => {
                if let Var::Array(ref array) = *var {
                    return Ok(array);
                }
                wrong_type_error(var_name, &DeclareVar::Array, var)
            }
            Err(msg) => Err(msg),
        }
    }

    fn get_var_mut(&mut self, var_name: &str) -> Result<&mut Var, String> {
        if let Some(var) = self.data.get_mut(var_name) {
            return Ok(var);
        }
        not_found_error(var_name)
    }

    pub fn get_var(&self, var_name: &str) -> Result<&Var, String> {
        if let Some(var) = self.data.get(var_name) {
            return Ok(var);
        }
        not_found_error(var_name)
    }
}
