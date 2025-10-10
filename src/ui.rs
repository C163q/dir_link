use std::io;
use std::ops::Deref;

use ratatui::crossterm::event::{self, Event};
use ratatui::prelude::*;
use ratatui::widgets::{ListState, TableState, Widget};

use state::AppState;

use crate::data::dirset::LinkDirSet;
use crate::data::link::QuitData;
use state::{FolderNormalState, NormalPart};

pub mod key;
pub mod message;
pub mod state;
pub mod view;

pub struct App {
    state: AppState,
    data: LinkDirSet,
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
    }
}

impl App {
    pub fn new(data: LinkDirSet) -> Self {
        Self {
            state: AppState::Normal(Box::new(NormalPart::Folder(FolderNormalState::new()))),
            data,
        }
    }

    pub fn run<B: Backend>(&mut self, mut terminal: Terminal<B>, success: bool) -> io::Result<QuitData> {
        loop {
            terminal.draw(|f| {
                f.render_widget(&mut *self, f.area());
            })?;
            self.handle_event()?;
            if let AppState::Quit(data) = &self.state {
                // TODO: 性能损耗，之后尝试改进
                break Ok(data.deref().clone());
            }
        }
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
