
#[derive(Serialize, Deserialize, Debug, Clone)]
enum OutputType {
    Log(String),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    messages: Vec<OutputType>,
    exit_status: Option<u64>,
}

impl Output {
    pub fn new() -> Output {
        Output {
            messages: Vec::new(),
            exit_status: None,
        }
    }

    pub fn set_exit_status(&mut self, status: u64) {
        self.exit_status = Some(status);
    }

    pub fn log(&mut self, msg: String) {
        self.messages.push(OutputType::Log(msg));
    }

    pub fn error(&mut self, msg: String) {
        self.messages.push(OutputType::Error(msg));
    }
}
