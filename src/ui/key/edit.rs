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
        float::{warning::WarningState, Float}, message::{EditMessage, MessageUpdater}, state::{
            AppState, FolderEditState, FolderNormalState, InputPart, LinkEditState,
            LinkNormalState, NormalPart,
        }
    },
};

pub fn folder_handle_input_normal(
    state: &mut FolderEditState,
    data: &mut LinkDirSet,
    key_event: KeyEvent,
) -> MessageUpdater<EditMessage> {
    let select = state.list_state().selected();
    let quit_select = if data.is_empty() {
        select
    } else {
        select.or(Some(0))
    };
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                MessageUpdater::new().with_message(EditMessage::Quit(quit_select))
            }
            (_, code) => match code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    MessageUpdater::new().with_message(EditMessage::Quit(quit_select))
                }
                KeyCode::Enter => MessageUpdater::new().with_message(EditMessage::Confirm),
                KeyCode::Char('a') | KeyCode::Char('e') => {
                    MessageUpdater::new().with_message(EditMessage::Edit)
                }
                _ => MessageUpdater::new(),
            },
        }
    } else {
        MessageUpdater::new()
    }
}

pub fn folder_edit_normal(
    state: &mut FolderEditState,
    _data: &mut LinkDirSet,
) -> MessageUpdater<EditMessage> {
    state.switch_mode();
    MessageUpdater::new()
}

pub fn folder_confirm_normal(
    state: &mut FolderEditState,
    data: &mut LinkDirSet,
) -> MessageUpdater<EditMessage> {
    let name = state.input().value();
    let select = match state.list_state().selected() {
        None => {
            // append
            let build_dir = LinkDir::builder(name);
            let dir = match build_dir {
                Ok(dir) => dir,
                // TODO: handle Err later (identifier empty)
                Err(err) => {
                    let msg = err.message().to_owned();
                    return MessageUpdater::new().with_float(Float::Warning(WarningState::new(msg)));
                }
            };
            // TODO: handle Err later (identifier already exists)
            match data.push(dir) {
                Ok(_) => Some(data.len().saturating_sub(1)),
                Err(err) => {
                    let msg = err.message().to_owned();
                    return MessageUpdater::new()
                        .with_float(Float::Warning(WarningState::new(msg)));
                }
            }
        }
        Some(idx) => {
            // rename
            if name != data[idx].identifier() {
                // TODO: handle Err later (identifier empty or already exists)
                match data.rename(idx, name) {
                    Ok(_) => {}
                    Err(err) => {
                        let msg = err.message().to_owned();
                        return MessageUpdater::new()
                            .with_float(Float::Warning(WarningState::new(msg)));
                    }
                }
            }
            Some(idx)
        }
    };
    MessageUpdater::new().with_message(EditMessage::Quit(select))
}

pub fn folder_quit_normal(
    _state: &mut FolderEditState,
    _data: &mut LinkDirSet,
    select: Option<usize>,
) -> MessageUpdater<EditMessage> {
    MessageUpdater::new().with_state(AppState::Normal(Box::new(NormalPart::Folder(
        FolderNormalState::with_selected(select),
    ))))
}

pub fn folder_handle_input_editing(
    state: &mut FolderEditState,
    _data: &mut LinkDirSet,
    key_event: KeyEvent,
) -> MessageUpdater<EditMessage> {
    let event = Event::Key(key_event);
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                MessageUpdater::new().with_message(EditMessage::Quit(state.list_state().selected()))
            }
            (_, code) => match code {
                KeyCode::Esc => MessageUpdater::new().with_message(EditMessage::Back),
                KeyCode::Enter => MessageUpdater::new().with_message(EditMessage::Confirm),
                _ => {
                    state.input_mut().handle_event(&event);
                    MessageUpdater::new()
                }
            },
        }
    } else {
        state.input_mut().handle_event(&event);
        MessageUpdater::new()
    }
}

pub fn folder_confirm_editing(
    state: &mut FolderEditState,
    _data: &mut LinkDirSet,
) -> MessageUpdater<EditMessage> {
    state.switch_mode();
    MessageUpdater::new().with_message(EditMessage::Confirm)
}

pub fn folder_quit_editing(
    state: &mut FolderEditState,
    data: &mut LinkDirSet,
    select: Option<usize>,
) -> MessageUpdater<EditMessage> {
    let select = if data.is_empty() {
        None
    } else {
        match select {
            Some(idx) => Some(idx),
            _ => Some(0),
        }
    };
    state.switch_mode();
    MessageUpdater::new().with_message(EditMessage::Quit(select))
}

pub fn folder_back_editing(
    state: &mut FolderEditState,
    _data: &mut LinkDirSet,
) -> MessageUpdater<EditMessage> {
    state.switch_mode();
    MessageUpdater::new()
}

pub fn link_handle_input_normal(
    state: &mut LinkEditState,
    data: &mut LinkDir,
    key_event: KeyEvent,
) -> MessageUpdater<EditMessage> {
    let select = state.table_state().selected();
    let quit_select = if data.is_empty() {
        select
    } else {
        select.or(Some(0))
    };
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                MessageUpdater::new().with_message(EditMessage::Quit(quit_select))
            }
            (_, code) => match code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    MessageUpdater::new().with_message(EditMessage::Quit(quit_select))
                }
                KeyCode::Enter => MessageUpdater::new().with_message(EditMessage::Confirm),
                KeyCode::Char('a') | KeyCode::Char('e') => {
                    MessageUpdater::new().with_message(EditMessage::Edit)
                }
                KeyCode::Tab | KeyCode::BackTab => {
                    MessageUpdater::new().with_message(EditMessage::Switch)
                }
                KeyCode::Left => MessageUpdater::new().with_message(EditMessage::SwitchLeft),
                KeyCode::Right => MessageUpdater::new().with_message(EditMessage::SwitchRight),
                _ => MessageUpdater::new(),
            },
        }
    } else {
        MessageUpdater::new()
    }
}

pub fn link_edit_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    state.switch_mode();
    MessageUpdater::new()
}

pub fn link_confirm_normal(
    state: &mut LinkEditState,
    data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    let (key, value) = state.value();
    let value: PathBuf = link::get_vaild_path(value).unwrap_or_default();

    let select = match state.table_state().selected() {
        None => {
            // append
            let build_link = Link::builder(key, &value);
            let link = match build_link {
                Ok(link) => link,
                // TODO: handle Err later (identifier or path empty)
                Err(_) => return MessageUpdater::new().with_message(EditMessage::Quit(Some(0))),
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

    MessageUpdater::new().with_message(EditMessage::Quit(select))
}

pub fn link_switch_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    state.switch_part();
    MessageUpdater::new()
}

pub fn link_switch_left_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    state.set_part(InputPart::Key);
    MessageUpdater::new()
}

pub fn link_switch_right_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    state.set_part(InputPart::Value);
    MessageUpdater::new()
}

pub fn link_switch_or_confirm_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    match state.part() {
        InputPart::Key => {
            state.set_part(InputPart::Value);
            MessageUpdater::new()
        }
        InputPart::Value => MessageUpdater::new().with_message(EditMessage::Confirm),
    }
}

pub fn link_quit_normal(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
    select: Option<usize>,
) -> MessageUpdater<EditMessage> {
    MessageUpdater::new().with_state(AppState::Normal(Box::new(NormalPart::Link(
        LinkNormalState::with_selected(
            state.state().folder_list_state().selected().unwrap(),
            select,
        ),
    ))))
}

pub fn link_handle_input_editing(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
    key_event: KeyEvent,
) -> MessageUpdater<EditMessage> {
    let event = Event::Key(key_event);
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                MessageUpdater::new()
                    .with_message(EditMessage::Quit(state.table_state().selected()))
            }
            (_, code) => match code {
                KeyCode::Esc => MessageUpdater::new().with_message(EditMessage::Back),
                KeyCode::Tab => MessageUpdater::new().with_message(EditMessage::Switch),
                KeyCode::Enter => MessageUpdater::new().with_message(EditMessage::SwitchOrConfirm),
                _ => {
                    match state.part() {
                        InputPart::Key => state.key_input_mut().handle_event(&event),
                        InputPart::Value => state.value_input_mut().handle_event(&event),
                    };
                    MessageUpdater::new()
                }
            },
        }
    } else {
        match state.part() {
            InputPart::Key => state.key_input_mut().handle_event(&event),
            InputPart::Value => state.value_input_mut().handle_event(&event),
        };
        MessageUpdater::new()
    }
}

pub fn link_confirm_editing(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    state.switch_mode();
    MessageUpdater::new().with_message(EditMessage::Confirm)
}

#[inline]
pub fn link_switch_editing(
    state: &mut LinkEditState,
    data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    link_switch_normal(state, data)
}

#[inline]
pub fn link_switch_left_editing(
    state: &mut LinkEditState,
    data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    link_switch_left_normal(state, data)
}

pub fn link_switch_right_editing(
    state: &mut LinkEditState,
    data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    link_switch_right_normal(state, data)
}

pub fn link_switch_or_confirm_editing(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    match state.part() {
        InputPart::Key => {
            state.set_part(InputPart::Value);
            MessageUpdater::new()
        }
        InputPart::Value => MessageUpdater::new().with_message(EditMessage::Confirm),
    }
}

pub fn link_quit_editing(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
    select: Option<usize>,
) -> MessageUpdater<EditMessage> {
    state.switch_mode();
    MessageUpdater::new().with_message(EditMessage::Quit(select))
}

pub fn link_back_editing(
    state: &mut LinkEditState,
    _data: &mut LinkDir,
) -> MessageUpdater<EditMessage> {
    state.switch_mode();
    MessageUpdater::new()
}
