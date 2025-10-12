use super::InputMode;

use ratatui::widgets::ListState;
use tui_input::Input;

#[derive(Debug)]
pub struct FolderNormalState {
    list_state: ListState,
}

#[derive(Debug)]
pub struct FolderEditState {
    state: FolderNormalState,
    mode: InputMode,
    input: Input,
}

impl Default for FolderNormalState {
    fn default() -> Self {
        Self::new()
    }
}

impl FolderNormalState {
    pub fn new() -> Self {
        Self::with_selected(Some(0))
    }

    pub fn list_state(&self) -> &ListState {
        &self.list_state
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    pub fn select(&mut self, selected: Option<usize>) {
        self.list_state.select(selected);
    }

    pub fn with_selected(selected: Option<usize>) -> Self {
        let list_state = ListState::default().with_selected(selected);
        Self { list_state }
    }
}

impl FolderEditState {
    pub fn new(selected: Option<usize>) -> Self {
        Self {
            state: FolderNormalState::with_selected(selected),
            mode: InputMode::Editing,
            input: Input::default(),
        }
    }

    pub fn mode(&self) -> &InputMode {
        &self.mode
    }

    pub fn switch_mode(&mut self) {
        self.mode = match self.mode {
            InputMode::Normal => InputMode::Editing,
            InputMode::Editing => InputMode::Normal,
        }
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn state(&self) -> &FolderNormalState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut FolderNormalState {
        &mut self.state
    }

    pub fn list_state(&self) -> &ListState {
        self.state.list_state()
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        self.state.list_state_mut()
    }

    pub fn with_value(mut self, value: &str) -> Self {
        let input = Input::new(value.to_string());
        self.input = input;
        self
    }
}
