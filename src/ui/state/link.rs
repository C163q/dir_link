use std::ffi::{OsStr, OsString};

use super::{InputMode, InputPart};
use crate::{ui::state::FolderNormalState};

use ratatui::widgets::{ListState, TableState};

#[derive(Debug)]
pub struct LinkNormalState {
    folder_state: FolderNormalState,
    table_state: TableState,
}

#[derive(Debug)]
pub struct LinkEditState {
    state: LinkNormalState,
    mode: InputMode,
    part: InputPart,
    value: (String, OsString),
}

impl LinkNormalState {
    pub fn new(from: usize) -> Self {
        Self::with_selected(from, None)
    }

    pub fn table_state(&self) -> &TableState {
        &self.table_state
    }

    pub fn folder_state(&self) -> &FolderNormalState {
        &self.folder_state
    }

    pub fn table_state_mut(&mut self) -> &mut TableState {
        &mut self.table_state
    }

    pub fn folder_list_state(&self) -> &ListState {
        self.folder_state.list_state()
    }

    pub fn folder_list_state_mut(&mut self) -> &mut ListState {
        self.folder_state.list_state_mut()
    }

    pub fn with_selected(from: usize, selected: Option<usize>) -> Self {
        let table_state = TableState::default().with_selected(selected);
        Self {
            folder_state: FolderNormalState::with_selected(Some(from)),
            table_state,
        }
    }

    pub fn select(&mut self, selected: Option<usize>) {
        self.table_state.select(selected);
    }
}

impl LinkEditState {
    pub fn new(from: usize, selected: Option<usize>) -> Self {
        Self {
            state: LinkNormalState::with_selected(from, selected),
            mode: InputMode::Normal,
            part: InputPart::Key,
            value: (String::new(), OsString::new()),
        }
    }

    pub fn state(&self) -> &LinkNormalState {
        &self.state
    }

    pub fn table_state(&self) -> &TableState {
        self.state.table_state()
    }

    pub fn table_state_mut(&mut self) -> &mut TableState {
        self.state.table_state_mut()
    }

    pub fn folder_list_state(&self) -> &ListState {
        self.state.folder_list_state()
    }

    pub fn folder_list_state_mut(&mut self) -> &mut ListState {
        self.state.folder_list_state_mut()
    }

    pub fn with_value(mut self, key: &str, value: &OsStr) -> Self {
        self.value = (key.to_string(), value.to_os_string());
        self
    }
}

