
pub mod varops;
pub mod arrayops;

use super::super::var::Varables;
use super::super::output::Output;

pub trait Op {
    fn run(&self, state: &mut Varables, output: &mut Output) -> Result<String, String>;
}
