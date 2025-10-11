use std::path::PathBuf;

use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use crate::{
    data::{
        dir::LinkDir,
        dirset::LinkDirSet,
        link::{self, Link},
    },
    ui::{
        message::EditMessage,
        state::{
            AppState, FolderEditState, FolderNormalState, InputPart, LinkEditState,
            LinkNormalState, NormalPart,
        },
    },
};

pub fn folder_handle_input_normal(
    state: &mut FolderEditState,
    _data: &mut LinkDirSet,
    key_event: KeyEvent,
) -> (Option<EditMessage>, Option<AppState>) {
    let select = state.list_state().selected();
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                (Some(EditMessage::Quit(select)), None)
            }
            (_, code) => match code {
                KeyCode::Esc | KeyCode::Char('q') => (Some(EditMessage::Quit(select)), None),
                KeyCode::Enter => (Some(EditMessage::Confirm), None),
                KeyCode::Char('a') | KeyCode::Char('e') => (Some(EditMessage::Edit), None),
                _ => (None, None),
            },
        }
    } else {
        (None, None)
    }
}

pub fn folder_edit_normal(
    state: &mut FolderEditState,
    _data: &mut LinkDirSet,
) -> (Option<EditMessage>, Option<AppState>) {
    state.switch_mode();
    (None, None)
}

pub fn folder_confirm_normal(
    state: &mut FolderEditState,
    data: &mut LinkDirSet,
) -> (Option<EditMessage>, Option<AppState>) {
    let name = state.input().value();
    let select = match state.list_state().selected() {
        None => {
            // append
            let build_dir = LinkDir::builder(name);
            let dir = match build_dir {
                Ok(dir) => dir,
                // TODO: handle Err later (identifier empty)
                Err(_) => return (Some(EditMessage::Quit(Some(0))), None),
            };
            // TODO: handle Err later (identifier already exists)
            data.push(dir);
            Some(data.len().saturating_sub(1))
        }
        Some(idx) => {
            // rename
            if name != data[idx].identifier() {
                // TODO: handle Err later (identifier empty or already exists)
                data.rename(idx, name);
            }
            Some(idx)
        }
    };
    (Some(EditMessage::Quit(select)), None)
}

pub fn folder_quit_normal(
    _state: &mut FolderEditState,
    _data: &mut LinkDirSet,
    select: Option<usize>,
) -> (Option<EditMessage>, Option<AppState>) {
    (
        None,
        Some(AppState::Normal(Box::new(NormalPart::Folder(
            FolderNormalState::with_selected(select),
        )))),
    )
}

pub fn folder_handle_input_editing(
    state: &mut FolderEditState,
    _data: &mut LinkDirSet,
    key_event: KeyEvent,
) -> (Option<EditMessage>, Option<AppState>) {
    let event = Event::Key(key_event);
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                (Some(EditMessage::Quit(state.list_state().selected())), None)
            }
            (_, code) => match code {
                KeyCode::Esc => (Some(EditMessage::Back), None),
                KeyCode::Enter => (Some(EditMessage::Confirm), None),
                _ => {
                    state.input_mut().handle_event(&event);
                    (None, None)
                }
            },
        }
    } else {
        state.input_mut().handle_event(&event);
        (None, None)
    }
}

pub fn folder_confirm_editing(
    state: &mut FolderEditState,
    _data: &mut LinkDirSet,
) -> (Option<EditMessage>, Option<AppState>) {
    state.switch_mode();
    (Some(EditMessage::Confirm), None)
}

pub fn folder_quit_editing(
    state: &mut FolderEditState,
    data: &mut LinkDirSet,
    select: Option<usize>,
) -> (Option<EditMessage>, Option<AppState>) {
    let select = if data.is_empty() {
        None
    } else {
        match select {
            Some(idx) => Some(idx),
            _ => Some(0),
        }
    };
    state.switch_mode();
    (Some(EditMessage::Quit(select)), None)
}

pub fn folder_back_editing(
    state: &mut FolderEditState,
    _data: &mut LinkDirSet,
) -> (Option<EditMessage>, Option<AppState>) {
    state.switch_mode();
    (None, None)
}

pub fn link_handle_input_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
    key_event: KeyEvent,
) -> (Option<EditMessage>, Option<AppState>) {
    let select = state.table_state().selected();
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                (Some(EditMessage::Quit(select)), None)
            }
            (_, code) => match code {
                KeyCode::Esc | KeyCode::Char('q') => (Some(EditMessage::Quit(select)), None),
                KeyCode::Enter => (Some(EditMessage::Confirm), None),
                KeyCode::Char('a') | KeyCode::Char('e') => (Some(EditMessage::Edit), None),
                KeyCode::Tab | KeyCode::BackTab => (Some(EditMessage::Switch), None),
                KeyCode::Left => (Some(EditMessage::SwitchLeft), None),
                KeyCode::Right => (Some(EditMessage::SwitchRight), None),
                _ => (None, None),
            },
        }
    } else {
        (None, None)
    }
}

pub fn link_edit_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    state.switch_mode();
    (None, None)
}

pub fn link_confirm_normal(
    state: &mut LinkEditState,
    data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    let (key, value) = state.value();
    let value: PathBuf = link::get_vaild_path(value).unwrap_or(PathBuf::new());

    let select = match state.table_state().selected() {
        None => {
            // append
            let build_link = Link::builder(key, &value);
            let link = match build_link {
                Ok(link) => link,
                // TODO: handle Err later (identifier or path empty)
                Err(_) => return (Some(EditMessage::Quit(Some(0))), None),
            };
            // TODO: handle Err later (identifier already exists)
            data.push(link);
            Some(data.len().saturating_sub(1))
        }
        Some(idx) => {
            // rename
            // TODO: handle Err later (identifier or path empty, identifier already exists)
            if key != data[idx].identifier() {
                data.rename(idx, key);
            }
            if value != data[idx].path() {
                data.relink(idx, &value);
            }
            Some(idx)
        }
    };

    (Some(EditMessage::Quit(select)), None)
}

pub fn link_switch_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    state.switch_part();
    (None, None)
}

pub fn link_switch_left_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    state.set_part(InputPart::Key);
    (None, None)
}

pub fn link_switch_right_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    state.set_part(InputPart::Value);
    (None, None)
}

pub fn link_switch_or_confirm_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    match state.part() {
        InputPart::Key => {
            state.set_part(InputPart::Value);
            (None, None)
        }
        InputPart::Value => (Some(EditMessage::Confirm), None),
    }
}

pub fn link_quit_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
    select: Option<usize>,
) -> (Option<EditMessage>, Option<AppState>) {
    (
        None,
        Some(AppState::Normal(Box::new(NormalPart::Link(
            LinkNormalState::with_selected(
                state.state().folder_list_state().selected().unwrap(),
                select,
            ),
        )))),
    )
}

pub fn link_handle_input_editing(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
    key_event: KeyEvent,
) -> (Option<EditMessage>, Option<AppState>) {
    let event = Event::Key(key_event);
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => (
                Some(EditMessage::Quit(state.table_state().selected())),
                None,
            ),
            (_, code) => match code {
                KeyCode::Esc => (Some(EditMessage::Back), None),
                KeyCode::Tab => (Some(EditMessage::Switch), None),
                KeyCode::Enter => (Some(EditMessage::SwitchOrConfirm), None),
                _ => {
                    match state.part() {
                        InputPart::Key => state.key_input_mut().handle_event(&event),
                        InputPart::Value => state.value_input_mut().handle_event(&event),
                    };
                    (None, None)
                }
            },
        }
    } else {
        match state.part() {
            InputPart::Key => state.key_input_mut().handle_event(&event),
            InputPart::Value => state.value_input_mut().handle_event(&event),
        };
        (None, None)
    }
}

pub fn link_confirm_editing(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    state.switch_mode();
    (Some(EditMessage::Confirm), None)
}

#[inline]
pub fn link_switch_editing(
    state: &mut LinkEditState,
    data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    link_switch_normal(state, data)
}

#[inline]
pub fn link_switch_left_editing(
    state: &mut LinkEditState,
    data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    link_switch_left_normal(state, data)
}

pub fn link_switch_right_editing(
    state: &mut LinkEditState,
    data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    link_switch_right_normal(state, data)
}

pub fn link_switch_or_confirm_editing(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    match state.part() {
        InputPart::Key => {
            state.set_part(InputPart::Value);
            (None, None)
        }
        InputPart::Value => (Some(EditMessage::Confirm), None),
    }
}

pub fn link_quit_editing(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
    select: Option<usize>,
) -> (Option<EditMessage>, Option<AppState>) {
    state.switch_mode();
    (Some(EditMessage::Quit(select)), None)
}

pub fn link_back_editing(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> (Option<EditMessage>, Option<AppState>) {
    state.switch_mode();
    (None, None)
}
