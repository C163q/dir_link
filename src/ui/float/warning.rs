use crate::ui::float::FloatState;

#[derive(Debug, Clone)]
pub struct WarningState {
    message: String,
}

impl FloatState for WarningState {}

impl WarningState {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn get_message(self) -> String {
        self.message
    }
}
