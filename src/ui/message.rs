use ratatui::crossterm::event::KeyEvent;

use crate::ui::state::AppState;

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
pub enum PopUpMessage {
    Yes,
    No,
    Quit,
}

impl AppMessage for PopUpMessage {}

#[derive(Debug)]
pub struct MessageUpdater<M: AppMessage> {
    pub message: Option<M>,
    pub state: Option<AppState>,
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
}
