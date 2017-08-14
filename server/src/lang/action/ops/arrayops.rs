
use super::super::super::action::types::Action;
use super::super::super::var::types::DeclareVar;
use super::super::super::var::types::Var;
use super::super::super::output::Output;
use super::Op;
use std::clone::Clone;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ArrayOP {
    Push { var: Var },
}
