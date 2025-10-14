mod common;
mod float;
mod folder;
mod link;

pub use common::*;
pub use float::*;
pub use folder::*;
pub use link::*;
use ratatui::widgets::{ListState, TableState};

use crate::DataTransfer;

#[derive(Debug)]
pub enum NormalState {
    Folder(FolderNormalState),
    Link(LinkNormalState),
}

#[derive(Debug)]
pub enum AppState {
    Normal(Box<NormalState>),
    Quit(Box<DataTransfer>),
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::Normal(Box::new(NormalState::Folder(FolderNormalState::new())))
    }

    pub fn folder_list_state(&self) -> Option<&ListState> {
        match self {
            AppState::Normal(part) => match &**part {
                NormalState::Folder(state) => Some(state.list_state()),
                NormalState::Link(state) => Some(state.folder_list_state()),
            },
            AppState::Quit(_) => None,
        }
    }

    pub fn folder_list_state_mut(&mut self) -> Option<&mut ListState> {
        match self {
            AppState::Normal(part) => match &mut **part {
                NormalState::Folder(state) => Some(state.list_state_mut()),
                NormalState::Link(state) => Some(state.folder_list_state_mut()),
            },
            AppState::Quit(_) => None,
        }
    }

    pub fn link_table_state(&self) -> Option<&TableState> {
        match self {
            AppState::Normal(part) => match &**part {
                NormalState::Link(state) => Some(state.table_state()),
                _ => None,
            },
            AppState::Quit(_) => None,
        }
    }

    pub fn link_table_state_mut(&mut self) -> Option<&mut TableState> {
        match self {
            AppState::Normal(part) => match &mut **part {
                NormalState::Link(state) => Some(state.table_state_mut()),
                _ => None,
            },
            AppState::Quit(_) => None,
        }
    }

    pub fn is_folder(&self) -> bool {
        match self {
            AppState::Normal(part) => match &**part {
                NormalState::Folder(_) => true,
                NormalState::Link(_) => false,
            },
            AppState::Quit(_) => false,
        }
    }

    pub fn is_link(&self) -> bool {
        match self {
            AppState::Normal(part) => match &**part {
                NormalState::Folder(_) => false,
                NormalState::Link(_) => true,
            },
            AppState::Quit(_) => false,
        }
    }
}
