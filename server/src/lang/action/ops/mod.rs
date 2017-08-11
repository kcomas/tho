
pub mod varops;

use super::super::var::Varables;

pub trait Op {
    fn run(&self, state: &mut Varables) -> Result<(), String>;
}
