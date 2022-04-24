pub mod ui;

pub struct App {
    pub input: String,
    pub messages: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::from("Input text here: "),
            messages: Vec::new(),
        }
    }
}