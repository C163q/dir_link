use std::io;
use std::ops::Deref;

use ratatui::crossterm::event::{self, Event};
use ratatui::prelude::*;
use ratatui::widgets::{Clear, ListState, TableState, Widget};

use state::AppState;

use crate::data::dirset::LinkDirSet;
use crate::data::link::QuitData;
use crate::ui::state::EditPart;
use state::{FolderNormalState, NormalPart};

pub mod key;
pub mod message;
pub mod state;
pub mod view;

pub struct App {
    state: AppState,
    data: LinkDirSet,
}

pub struct AppData {
    cursor: Option<(u16, u16)>,
    // TODO: handle failure
    success: bool,
}

impl StatefulWidget for &mut App {
    type State = AppData;
    fn render(self, area: Rect, buf: &mut Buffer, data: &mut Self::State) {
        data.cursor = None;
        view::render_border(self, area, buf);

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

        match &mut self.state {
            AppState::Edit(part) => match &mut **part {
                EditPart::Folder(state) => {
                    let area = view::common::centered_rect(50, 25, area);
                    Clear.render(area, buf);
                    let cursor = view::render_folder_edit(state, area, buf);
                    data.cursor = cursor;
                }
                EditPart::Link(_) => {
                    // TODO
                }
            },
            AppState::Quit(_) => {
                // TODO
            }
            AppState::Normal(_) => {}
        }
    }
}

impl App {
    pub fn new(data: LinkDirSet) -> Self {
        Self {
            state: AppState::Normal(Box::new(NormalPart::Folder(FolderNormalState::new()))),
            data,
        }
    }

    pub fn run<B: Backend>(mut self, mut terminal: Terminal<B>, success: bool) -> io::Result<(QuitData, LinkDirSet)> {
        let quit_data = loop {
            terminal.draw(|f| {
                let mut data = AppData { cursor: None, success };
                f.render_stateful_widget(&mut self, f.area(), &mut data);
                if let Some(pos) = data.cursor {
                    f.set_cursor_position(pos);
                }
            })?;
            self.handle_event()?;
            if let AppState::Quit(data) = &self.state {
                // TODO: 性能损耗，之后尝试改进
                break data.deref().clone();
            }
        };
        Ok((quit_data, self.data))
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
