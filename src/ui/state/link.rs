use crate::ui::state::FolderNormalState;

use ratatui::widgets::{ListState, TableState};

#[derive(Debug)]
pub struct LinkNormalState {
    folder_state: FolderNormalState,
    table_state: TableState,
}

impl LinkNormalState {
    pub fn new(from: usize) -> Self {
        Self::with_selected(from, Some(0))
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

    pub fn folder_index(&self) -> usize {
        self.folder_list_state().selected().unwrap()
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
