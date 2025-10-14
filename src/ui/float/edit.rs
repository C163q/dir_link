use std::ffi::OsStr;

use tui_input::Input;

use crate::ui::{
    float::FloatState,
    state::{InputMode, InputPart},
};

#[derive(Debug)]
pub struct FolderEditState {
    selected: Option<usize>,
    mode: InputMode,
    input: Input,
}

impl FloatState for FolderEditState {}

impl FolderEditState {
    pub fn new(selected: Option<usize>) -> Self {
        Self {
            selected,
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

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn with_value(mut self, value: &str) -> Self {
        let input = Input::new(value.to_string());
        self.input = input;
        self
    }
}

#[derive(Debug)]
pub struct LinkEditState {
    selected: Option<usize>,
    from: usize,
    mode: InputMode,
    part: InputPart,
    input: (Input, Input),
}

impl FloatState for LinkEditState {}

impl LinkEditState {
    pub fn new(from: usize, selected: Option<usize>) -> Self {
        Self {
            selected,
            from,
            mode: InputMode::Editing,
            part: InputPart::Key,
            input: (Input::default(), Input::default()),
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

    pub fn value(&self) -> (&str, &str) {
        (self.input.0.value(), self.input.1.value())
    }

    pub fn key_input(&self) -> &Input {
        &self.input.0
    }

    pub fn key_input_mut(&mut self) -> &mut Input {
        &mut self.input.0
    }

    pub fn value_input(&self) -> &Input {
        &self.input.1
    }

    pub fn value_input_mut(&mut self) -> &mut Input {
        &mut self.input.1
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn from(&self) -> usize {
        self.from
    }

    pub fn part(&self) -> &InputPart {
        &self.part
    }

    pub fn switch_part(&mut self) {
        self.part = match self.part {
            InputPart::Key => InputPart::Value,
            InputPart::Value => InputPart::Key,
        }
    }

    pub fn set_part(&mut self, part: InputPart) {
        self.part = part;
    }

    pub fn with_value(mut self, key: &str, value: &OsStr) -> Self {
        let key_input = Input::new(key.to_string());
        let value_input = Input::new(value.to_string_lossy().to_string());
        self.input = (key_input, value_input);
        self
    }
}
