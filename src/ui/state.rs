mod common;
mod folder;
mod link;

pub use common::*;
pub use folder::*;
pub use link::*;
use ratatui::widgets::{ListState, TableState};

use crate::data::link::QuitData;

#[derive(Debug)]
pub enum NormalPart {
    Folder(FolderNormalState),
    Link(LinkNormalState),
}

#[derive(Debug)]
pub enum EditPart {
    Folder(FolderEditState),
    Link(LinkEditState),
}

#[derive(Debug)]
pub enum AppState {
    Normal(Box<NormalPart>),
    Edit(Box<EditPart>),
    Quit(Box<QuitData>),
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::Normal(Box::new(NormalPart::Folder(FolderNormalState::new())))
    }

    pub fn folder_list_state(&self) -> Option<&ListState> {
        match self {
            AppState::Normal(part) => match &**part {
                NormalPart::Folder(state) => Some(state.list_state()),
                NormalPart::Link(state) => Some(state.folder_list_state()),
            },
            AppState::Edit(part) => match &**part {
                EditPart::Folder(state) => Some(state.list_state()),
                EditPart::Link(state) => Some(state.folder_list_state()),
            },
            AppState::Quit(_) => None,
        }
    }

    pub fn folder_list_state_mut(&mut self) -> Option<&mut ListState> {
        match self {
            AppState::Normal(part) => match &mut **part {
                NormalPart::Folder(state) => Some(state.list_state_mut()),
                NormalPart::Link(state) => Some(state.folder_list_state_mut()),
            },
            AppState::Edit(part) => match &mut **part {
                EditPart::Folder(state) => Some(state.list_state_mut()),
                EditPart::Link(state) => Some(state.folder_list_state_mut()),
            },
            AppState::Quit(_) => None,
        }
    }

    pub fn link_table_state(&self) -> Option<&TableState> {
        match self {
            AppState::Normal(part) => match &**part {
                NormalPart::Link(state) => Some(state.table_state()),
                _ => None,
            },
            AppState::Edit(part) => match &**part {
                EditPart::Link(state) => Some(state.table_state()),
                _ => None,
            },
            AppState::Quit(_) => None,
        }
    }

    pub fn link_table_state_mut(&mut self) -> Option<&mut TableState> {
        match self {
            AppState::Normal(part) => match &mut **part {
                NormalPart::Link(state) => Some(state.table_state_mut()),
                _ => None,
            },
            AppState::Edit(part) => match &mut **part {
                EditPart::Link(state) => Some(state.table_state_mut()),
                _ => None,
            },
            AppState::Quit(_) => None,
        }
    }

    pub fn is_folder(&self) -> bool {
        match self {
            AppState::Normal(part) => match &**part {
                NormalPart::Folder(_) => true,
                NormalPart::Link(_) => false,
            },
            AppState::Edit(part) => match &**part {
                EditPart::Folder(_) => true,
                EditPart::Link(_) => false,
            },
            AppState::Quit(_) => false,
        }
    }

    pub fn is_link(&self) -> bool {
        match self {
            AppState::Normal(part) => match &**part {
                NormalPart::Folder(_) => false,
                NormalPart::Link(_) => true,
            },
            AppState::Edit(part) => match &**part {
                EditPart::Folder(_) => false,
                EditPart::Link(_) => true,
            },
            AppState::Quit(_) => false,
        }
    }
}
