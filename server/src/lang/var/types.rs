
use super::super::action::types::Action;

pub trait ShareVar {
    fn type_str<'a>(&self) -> &'a str;

    fn type_num(&self) -> i32;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Var {
    Macro(Option<Vec<Action>>),
    String(Option<String>),
    Int(Option<i64>),
    Float(Option<f64>),
    Size(Option<usize>),
    Bool(Option<bool>),
    Array(Option<Box<Vec<Var>>>),
}

impl ShareVar for Var {
    fn type_str<'a>(&self) -> &'a str {
        match self {
            &Var::Macro(_) => "Macro",
            &Var::String(_) => "String",
            &Var::Int(_) => "Int",
            &Var::Float(_) => "Float",
            &Var::Size(_) => "Size",
            &Var::Bool(_) => "Bool",
            &Var::Array(_) => "Array",
        }
    }

    fn type_num(&self) -> i32 {
        match self {
            &Var::Macro(_) => 0,
            &Var::String(_) => 1,
            &Var::Int(_) => 2,
            &Var::Float(_) => 3,
            &Var::Size(_) => 4,
            &Var::Bool(_) => 5,
            &Var::Array(_) => 6,
        }
    }
}

impl Var {
    pub fn match_declare(&self, declare_match: &DeclareVar) -> bool {
        if self.type_num() == declare_match.type_num() {
            return true;
        }
        false
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeclareVar {
    Macro,
    String,
    Int,
    Float,
    Size,
    Bool,
    Array,
}

impl ShareVar for DeclareVar {
    fn type_str<'a>(&self) -> &'a str {
        match self {
            &DeclareVar::Macro => "Macro",
            &DeclareVar::String => "String",
            &DeclareVar::Int => "Int",
            &DeclareVar::Float => "Float",
            &DeclareVar::Size => "Size",
            &DeclareVar::Bool => "Bool",
            &DeclareVar::Array => "Array",
        }
    }

    fn type_num(&self) -> i32 {
        match self {
            &DeclareVar::Macro => 0,
            &DeclareVar::String => 1,
            &DeclareVar::Int => 2,
            &DeclareVar::Float => 3,
            &DeclareVar::Size => 4,
            &DeclareVar::Bool => 5,
            &DeclareVar::Array => 6,
        }
    }
}
