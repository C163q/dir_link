use ratatui::crossterm::event::{KeyEvent};

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

#[derive(Debug, PartialEq)]
pub enum EditMessage {
    Edit,
    HandleInput(KeyEvent),
    Confirm,
    Switch,
    SwitchLeft,
    SwitchRight,
    Quit(Option<usize>),
    Back,
}

#[derive(Debug, PartialEq)]
pub enum PopUpMessage {
    Yes,
    No,
    Quit,
}


