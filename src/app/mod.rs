pub mod input;
pub mod ui;

pub struct App {
    pub input: String,
    pub input_mode: input::InputMode,
    pub messages: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::from("Input text here: "),
            input_mode: input::InputMode::Editing,
            messages: Vec::new(),
        }
    }
}