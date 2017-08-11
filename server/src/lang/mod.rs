
pub mod var;
pub mod action;

use self::action::types::Action;
use self::var::Varables;

use serde_json;
use std::process;

pub struct Program {
    state: Varables,
    actions: Vec<Action>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            state: Varables::new(),
            actions: Vec::new(),
        }
    }

    pub fn load_actions(&mut self, actions: Vec<Action>) {
        self.actions = actions;
    }

    pub fn load_actions_str(&mut self, actions: &str) {
        let rst = serde_json::from_str(actions);
        if let Ok(act) = rst {
            self.load_actions(act);
        } else {
            panic!("Unable To Load Actions");
        }
    }

    pub fn get_actions_str(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.actions)
    }

    pub fn load_state(&mut self, state: Varables) {
        self.state = state;
    }

    pub fn load_state_str(&mut self, state: &str) {
        let rst = serde_json::from_str(state);
        if let Ok(st) = rst {
            self.load_state(st);
        } else {
            panic!("Unable To Load State");
        }
    }

    pub fn get_state_str(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.state)
    }

    pub fn run(&mut self) {
        for action in self.actions.iter() {
            let rst = action.run(&mut self.state);
            match rst {
                Ok(option) => {
                    if let Some(string) = option {
                        println!("{}", string);
                    }
                }
                Err(string) => {
                    println!("{}", string);
                    process::exit(1);
                }
            }
        }
    }
}
