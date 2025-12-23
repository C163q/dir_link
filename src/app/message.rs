use ratatui::crossterm::event::KeyEvent;

use crate::app::{
    float::{Float, FloatState},
    state::AppState,
};

pub trait AppMessage {}

#[derive(Debug, PartialEq)]
pub enum NormalFolderMessage {
    Select,
    MoveUp,
    MoveDown,
    SwitchUp,
    SwitchDown,
    Append,
    Rename,
    Remove,
    Quit,
    Item(usize),
    ToDir(usize),
    Help,
}

impl AppMessage for NormalFolderMessage {}

#[derive(Debug, PartialEq)]
pub enum NormalLinkMessage {
    Back,
    Select,
    MoveUp,
    MoveDown,
    SwitchUp,
    SwitchDown,
    Append,
    Rename,
    Remove,
    Quit,
    Item(usize),
    ToLink(usize),
    Help,
}

impl AppMessage for NormalLinkMessage {}

#[derive(Debug, PartialEq)]
pub enum EditMessage {
    Edit,
    HandleInput(KeyEvent),
    Confirm,
    Switch,
    SwitchLeft,
    SwitchRight,
    SwitchOrConfirm,
    Quit(Option<usize>, bool), // (choice, ask_save)
    Back,
    Help,
}

impl AppMessage for EditMessage {}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfirmMessage {
    Yes,
    No,
    Quit,
    Switch,
    SwitchLeft,
    SwitchRight,
    Choose,
}

impl AppMessage for ConfirmMessage {}

#[derive(Debug, PartialEq, Eq)]
pub enum ChooseMessage<T> {
    Switch,
    SwitchBack,
    SwitchLeft,
    SwitchRight,
    SwitchUp,
    SwitchDown,
    Choose,
    Quit(T),
}

impl<T> AppMessage for ChooseMessage<T> {}

#[derive(Debug, PartialEq, Eq)]
pub enum WarningMessage {
    Quit,
}

impl AppMessage for WarningMessage {}

#[derive(Debug)]
pub struct MessageUpdater<M: AppMessage> {
    pub message: Option<M>,
    pub state: Option<AppState>,
    pub float: Option<Float>,
}

impl Default for MessageUpdater<NormalFolderMessage> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: AppMessage> MessageUpdater<M> {
    pub fn new() -> Self {
        Self {
            message: None,
            state: None,
            float: None,
        }
    }

    pub fn with_message(mut self, message: M) -> Self {
        self.message = Some(message);
        self
    }

    pub fn with_state(mut self, state: AppState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn with_float(mut self, float: Float) -> Self {
        self.float = Some(float);
        self
    }
}

pub struct FloatUpdater<S: FloatState> {
    pub message: Option<S::Message>,
    pub state: Option<S>,
    pub float: Option<Float>,
}

impl<S: FloatState> Default for FloatUpdater<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: FloatState> FloatUpdater<S> {
    pub fn new() -> Self {
        Self {
            message: None,
            state: None,
            float: None,
        }
    }

    pub fn with_message(mut self, message: S::Message) -> Self {
        self.message = Some(message);
        self
    }

    pub fn with_state(mut self, state: S) -> Self {
        self.state = Some(state);
        self
    }

    pub fn with_float(mut self, float: Float) -> Self {
        self.float = Some(float);
        self
    }

    pub fn with_optional_float(mut self, float: Option<Float>) -> Self {
        self.float = float;
        self
    }
}
