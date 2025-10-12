
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfirmChoice {
    Yes,
    #[default]
    No,
}

pub struct DeleteConfirmState<F>
where
    F: FnOnce(ConfirmChoice),
{
    choice: ConfirmChoice,
    callback: F,
}

impl<F> DeleteConfirmState<F>
where
    F: FnOnce(ConfirmChoice),
{
    pub fn new(callback: F) -> Self {
        Self {
            choice: ConfirmChoice::No,
            callback,
        }
    }

    pub fn with_choice(mut self, choice: ConfirmChoice) -> Self {
        self.choice = choice;
        self
    }

    pub fn choice(&self) -> ConfirmChoice {
        self.choice
    }

    pub fn call(self) {
        let function = self.callback;
        function(self.choice);
    }

    pub fn switch_chioce(&mut self) {
        self.choice = match self.choice {
            ConfirmChoice::Yes => ConfirmChoice::No,
            ConfirmChoice::No => ConfirmChoice::Yes,
        }
    }

    pub fn change_choice(&mut self, choice: ConfirmChoice) {
        self.choice = choice;
    }
}
