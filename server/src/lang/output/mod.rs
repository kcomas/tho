
#[derive(Serialize, Deserialize, Debug, Clone)]
enum OutputType {
    Warning(String),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    messages: Vec<OutputType>,
}

impl Output {
    pub fn new() -> Output {
        Output { messages: Vec::new() }
    }

    pub fn warn(&mut self, msg: String) {
        self.messages.push(OutputType::Warning(msg));
    }

    pub fn error(&mut self, msg: String) {
        self.messages.push(OutputType::Error(msg));
    }
}
