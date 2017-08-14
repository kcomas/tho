
pub mod var;
pub mod action;
pub mod output;

use self::action::types::Action;
use self::var::Varables;
use self::output::Output;

use serde_json;
use std::process;

type JSONResult = Result<String, serde_json::Error>;

pub struct Program {
    output: Output,
    state: Varables,
    actions: Vec<Action>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            output: Output::new(),
            state: Varables::new(),
            actions: Vec::new(),
        }
    }

    pub fn load_actions(&mut self, actions: Vec<Action>) {
        self.actions = actions;
    }

    pub fn load_actions_str(&mut self, actions: &str) -> bool {
        let rst = serde_json::from_str(actions);
        if let Ok(act) = rst {
            self.load_actions(act);
            return true;
        }
        false
    }

    pub fn get_actions_str(&self) -> JSONResult {
        serde_json::to_string(&self.actions)
    }

    pub fn get_actions_str_pretty(&self) -> JSONResult {
        serde_json::to_string_pretty(&self.actions)
    }


    pub fn load_state(&mut self, state: Varables) {
        self.state = state;
    }

    pub fn load_state_str(&mut self, state: &str) -> bool {
        let rst = serde_json::from_str(state);
        if let Ok(st) = rst {
            self.load_state(st);
            return true;
        }
        false
    }

    pub fn get_state_str(&self) -> JSONResult {
        serde_json::to_string(&self.state)
    }

    pub fn get_state_str_pretty(&self) -> JSONResult {
        serde_json::to_string_pretty(&self.state)
    }

    pub fn get_output_str(&self) -> JSONResult {
        serde_json::to_string(&self.output)
    }

    pub fn get_output_str_pretty(&self) -> JSONResult {
        serde_json::to_string_pretty(&self.output)
    }

    pub fn run(&mut self) {
        for action in self.actions.iter() {
            let rst = action.run(&mut self.state, &mut self.output);
            match rst {
                Ok(option) => {
                    if let Some(string) = option {
                        self.output.warn(string);
                    }
                    self.output.set_exit_status(0);
                }
                Err(string) => {
                    self.output.error(string);
                    self.output.set_exit_status(1);
                    break;
                }
            }
        }
    }
}
