use std::io;

use ratatui::prelude::*;
use ratatui::widgets::{Clear, ListState, TableState};
use ratatui::{
    Terminal,
    crossterm::event::{self, Event},
};

use crate::app::data::{AppData, AppOption, DataTransfer};
use crate::app::float::Float;
use crate::app::float::warning::CorruptDataWarningState;
use crate::app::normal::FolderNormalState;
use crate::app::state::{AppState, NormalState};
use crate::data::dirset::LinkDirSet;
use crate::ui;

pub mod data;
pub mod float;
pub mod key;
pub mod message;
pub mod normal;
pub mod state;

pub struct App {
    pub state: AppState,
    pub data: LinkDirSet,
    pub option: AppOption,
    float: Vec<Float>,
}

impl StatefulWidget for &mut App {
    type State = AppData;
    fn render(self, area: Rect, buf: &mut Buffer, data: &mut Self::State) {
        data.cursor = None;
        ui::render_main_border(area, buf);

        let chunks = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Length(1),
                Constraint::Min(1),
            ])
            .split(area);

        ui::render_main_divider(chunks[1], buf);

        let default_state = &mut ListState::default();
        ui::render_left_list(self, chunks[0], buf, default_state);

        let default_state = &mut TableState::default();
        ui::render_right_list(self, chunks[2], buf, default_state);

        for float in &mut self.float {
            match float {
                Float::FolderEdit(state) => {
                    let area = ui::common::centered_rect(50, 25, area);
                    Clear.render(area, buf);
                    let cursor = ui::render_folder_edit(state, area, buf);
                    data.cursor = cursor;
                }
                Float::LinkEdit(state) => {
                    let area = ui::common::centered_rect(60, 30, area);
                    Clear.render(area, buf);
                    let cursor = ui::render_link_edit(state, area, buf);
                    data.cursor = cursor;
                }
                Float::FolderDeleteConfirm(state) => {
                    let area = ui::common::centered_rect(50, 30, area);
                    Clear.render(area, buf);
                    ui::render_folder_delete_confirm_float(state, area, buf)
                }
                Float::LinkDeleteConfirm(state) => {
                    let area = ui::common::centered_rect(50, 30, area);
                    Clear.render(area, buf);
                    ui::render_link_delete_confirm_float(state, area, buf)
                }
                Float::Warning(state) => {
                    let area = ui::common::centered_rect(40, 25, area);
                    Clear.render(area, buf);
                    ui::render_warning_float(state, area, buf);
                }
                Float::FolderSaveConfirm(state) => {
                    let edit_area = ui::common::centered_rect(50, 25, area);
                    let area = ui::common::centered_rect(50, 30, area);
                    Clear.render(area, buf);
                    ui::render_folder_save_confirm_float(state, edit_area, area, buf);
                }
                Float::LinkSaveConfirm(state) => {
                    let edit_area = ui::common::centered_rect(60, 30, area);
                    let area = ui::common::centered_rect(50, 30, area);
                    Clear.render(area, buf);
                    ui::render_link_save_confirm_float(state, edit_area, area, buf);
                }
                Float::CorruptDataWarning(state) => {
                    let area = ui::common::centered_rect(50, 40, area);
                    Clear.render(area, buf);
                    ui::render_corrupt_data_warning_float(state, area, buf);
                }
            }
        }
    }
}

impl App {
    #[inline]
    pub fn set_state(&mut self, state: AppState) {
        self.state = state;
    }

    #[inline]
    pub fn get_float(&mut self) -> Option<Float> {
        self.float.pop()
    }

    #[inline]
    pub fn add_float(&mut self, float: Float) {
        self.float.push(float);
    }

    #[inline]
    pub fn extend_float<I: IntoIterator<Item = Float>>(&mut self, floats: I) {
        self.float.extend(floats);
    }

    #[inline]
    pub fn inspect_floats(&self) -> &Vec<Float> {
        &self.float
    }

    pub fn new(data: LinkDirSet) -> Self {
        Self {
            state: AppState::Normal(Box::new(NormalState::Folder(FolderNormalState::new()))),
            data,
            float: Vec::new(),
            option: AppOption { save: true },
        }
    }

    fn check_corrept_data(&mut self, data: &AppData) {
        if let Err(err) = &data.success {
            self.option.save = false;
            self.float
                .push(Float::CorruptDataWarning(CorruptDataWarningState::new(
                    err.to_string(),
                )));
        }
    }

    pub fn run<B: Backend>(
        mut self,
        mut terminal: Terminal<B>,
        success: io::Result<()>,
        mut data_transfer: DataTransfer,
    ) -> io::Result<(DataTransfer, LinkDirSet)> {
        let mut data = AppData {
            cursor: None,
            success,
        };
        self.check_corrept_data(&data);

        loop {
            terminal.draw(|f| {
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
        if let Some(config) = &mut data_transfer.config {
            config.save = self.option.save;
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
