use crate::ui::state::FloatState;

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CorruptDataWarningChoice {
    #[default]
    Exit,
    NewData,
}

#[derive(Debug)]
pub struct CorruptDataWarningState {
    choice: CorruptDataWarningChoice,
    message: String,
}

impl FloatState for CorruptDataWarningState {}

impl CorruptDataWarningState {
    pub fn new(message: String) -> Self {
        Self {
            choice: CorruptDataWarningChoice::Exit,
            message,
        }
    }

    pub fn choice(&self) -> CorruptDataWarningChoice {
        self.choice
    }

    pub fn set_choice(&mut self, choice: CorruptDataWarningChoice) {
        self.choice = choice;
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn switch_left(&mut self) {
        self.choice = CorruptDataWarningChoice::Exit;
    }

    pub fn switch_right(&mut self) {
        self.choice = CorruptDataWarningChoice::NewData;
    }

    pub fn switch(&mut self) {
        self.choice = match self.choice {
            CorruptDataWarningChoice::Exit => CorruptDataWarningChoice::NewData,
            CorruptDataWarningChoice::NewData => CorruptDataWarningChoice::Exit,
        }
    }

    pub fn switch_back(&mut self) {
        self.switch();
    }

    pub fn switch_up(&mut self) {}

    pub fn switch_down(&mut self) {}
}
