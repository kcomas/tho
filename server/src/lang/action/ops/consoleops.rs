
use super::super::super::var::types::Var;

#[derive(Serialize, Deserialize, Debug)]
pub enum ConsoleOp {
    PrintLine { template: String, vars: Vec<Var> },
}
