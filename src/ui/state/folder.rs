use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct FolderNormalState {
    list_state: ListState,
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
