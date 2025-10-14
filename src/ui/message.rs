use ratatui::crossterm::event::KeyEvent;

use crate::ui::{float::{Float, FloatState}, state::AppState};

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
    Quit(Option<usize>),
    Back,
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
pub enum WarningMessage {
    Quit,
}

impl AppMessage for WarningMessage {}

#[derive(Debug)]
pub struct MessageUpdater<M: AppMessage> {
    pub message: Option<M>,
    pub state: Option<AppState>,
    pub float: Option<Float>
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

pub struct FloatUpdater<M: AppMessage, S: FloatState> {
    pub message: Option<M>,
    pub state: Option<S>,
    pub float: Option<Float>,
}

impl<M: AppMessage, S: FloatState> Default for FloatUpdater<M, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: AppMessage, S: FloatState> FloatUpdater<M, S> {
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

    pub fn with_state(mut self, state: S) -> Self {
        self.state = Some(state);
        self
    }

    pub fn with_float(mut self, float: Float) -> Self {
        self.float = Some(float);
        self
    }
}
