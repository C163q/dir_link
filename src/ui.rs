use std::io;

use ratatui::crossterm::event::{self, Event};
use ratatui::prelude::*;
use ratatui::widgets::{Clear, ListState, TableState, Widget};

use state::AppState;

use crate::DataTransfer;
use crate::data::dirset::LinkDirSet;
use crate::ui::float::Float;
use state::{FolderNormalState, NormalState};

pub mod float;
pub mod key;
pub mod message;
pub mod state;
pub mod view;

pub struct App {
    state: AppState,
    data: LinkDirSet,
    float: Vec<Float>,
}

pub struct AppData {
    pub cursor: Option<(u16, u16)>,
    // TODO: handle failure
    pub success: bool,
}

impl StatefulWidget for &mut App {
    type State = AppData;
    fn render(self, area: Rect, buf: &mut Buffer, data: &mut Self::State) {
        data.cursor = None;
        view::render_border(area, buf);

        let chunks = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Length(1),
                Constraint::Min(1),
            ])
            .split(area);

        view::render_divider(chunks[1], buf);

        let default_state = &mut ListState::default();
        view::render_left_list(self, chunks[0], buf, default_state);

        let default_state = &mut TableState::default();
        view::render_right_list(self, chunks[2], buf, default_state);

        for float in &mut self.float {
            match float {
                Float::FolderEdit(state) => {
                    let area = view::common::centered_rect(50, 25, area);
                    Clear.render(area, buf);
                    let cursor = view::render_folder_edit(state, area, buf);
                    data.cursor = cursor;
                }
                Float::LinkEdit(state) => {
                    let area = view::common::centered_rect(60, 30, area);
                    Clear.render(area, buf);
                    let cursor = view::render_link_edit(state, area, buf);
                    data.cursor = cursor;
                }
                Float::FolderDeleteConfirm(state) => {
                    let area = view::common::centered_rect(50, 30, area);
                    Clear.render(area, buf);
                    view::render_folder_delete_confirm_float(state, area, buf)
                }
                Float::LinkDeleteConfirm(state) => {
                    let area = view::common::centered_rect(50, 30, area);
                    Clear.render(area, buf);
                    view::render_link_delete_confirm_float(state, area, buf)
                }
                Float::Warning(state) => {
                    let area = view::common::centered_rect(40, 25, area);
                    Clear.render(area, buf);
                    view::render_warning_float(state, area, buf);
                }
                Float::FolderSaveConfirm(state) => {
                    let edit_area = view::common::centered_rect(50, 25, area);
                    let area = view::common::centered_rect(50, 30, area);
                    Clear.render(area, buf);
                    view::render_folder_save_confirm_float(state, edit_area, area, buf);
                }
                Float::LinkSaveConfirm(state) => {
                    let edit_area = view::common::centered_rect(60, 30, area);
                    let area = view::common::centered_rect(50, 30, area);
                    Clear.render(area, buf);
                    view::render_link_save_confirm_float(state, edit_area, area, buf);
                }
            }
        }
    }
}

impl App {
    pub fn new(data: LinkDirSet) -> Self {
        Self {
            state: AppState::Normal(Box::new(NormalState::Folder(FolderNormalState::new()))),
            data,
            float: Vec::new(),
        }
    }

    pub fn run<B: Backend>(
        mut self,
        mut terminal: Terminal<B>,
        success: bool,
        mut data_transfer: DataTransfer,
    ) -> io::Result<(DataTransfer, LinkDirSet)> {
        loop {
            terminal.draw(|f| {
                let mut data = AppData {
                    cursor: None,
                    success,
                };
                f.render_stateful_widget(&mut self, f.area(), &mut data);
                if let Some(pos) = data.cursor {
                    f.set_cursor_position(pos);
                }
            })?;
            self.handle_event()?;
            if let AppState::Quit(data) = &mut self.state {
                // TODO: 性能损耗，之后尝试改进
                data_transfer.link = data.link.take();
                break;
            }
        }
        Ok((data_transfer, self.data))
    }

    pub fn handle_event(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) => key::handle_key_event(self, key),
            Event::Mouse(_) => {} // TODO: handle mouse event
            _ => {}
        }
        Ok(())
    }
}
