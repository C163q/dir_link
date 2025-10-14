use std::path::PathBuf;

use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use crate::{
    data::{
        dir::LinkDir,
        link::{self, Link},
    },
    ui::{
        App,
        float::{
            Float, FloatActionResult,
            edit::{FolderEditState, LinkEditState},
            warning::WarningState,
        },
        message::{EditMessage, FloatUpdater},
        state::{AppState, FolderNormalState, InputMode, InputPart, LinkNormalState, NormalState},
    },
};

pub fn handle_edit_folder_key(
    app: &mut App,
    key: KeyEvent,
    mut state: FolderEditState,
) -> FloatActionResult {
    let mut new_float = None;
    let mut opt_msg = edit_folder_key(key);
    while let Some(msg) = opt_msg {
        let updater = edit_folder_message(app, state, msg);
        opt_msg = updater.message;
        match updater.state {
            Some(s) => state = s,
            None => return FloatActionResult::new().with_optional_new(updater.float),
        }
        new_float = updater.float;
    }
    FloatActionResult::new()
        .with_primary(Float::FolderEdit(state))
        .with_optional_new(new_float)
}

pub fn edit_folder_key(key: KeyEvent) -> Option<EditMessage> {
    Some(EditMessage::HandleInput(key))
}

pub fn edit_folder_message(
    app: &mut App,
    mut state: FolderEditState,
    msg: EditMessage,
) -> FloatUpdater<EditMessage, FolderEditState> {
    match state.mode() {
        InputMode::Normal => match msg {
            EditMessage::HandleInput(key_event) => {
                let updater = folder_handle_input_normal(app, &state, key_event);
                updater.with_state(state)
            }
            EditMessage::Edit => {
                let updater = folder_edit_normal(&mut state);
                updater.with_state(state)
            }
            EditMessage::Confirm => {
                let updater = folder_confirm_normal(app, &mut state);
                updater.with_state(state)
            }
            EditMessage::Switch => FloatUpdater::new().with_state(state),
            EditMessage::SwitchLeft => FloatUpdater::new().with_state(state),
            EditMessage::SwitchRight => FloatUpdater::new().with_state(state),
            EditMessage::SwitchOrConfirm => FloatUpdater::new()
                .with_message(EditMessage::Confirm)
                .with_state(state),
            EditMessage::Quit(select) => folder_quit_normal(app, select),
            EditMessage::Back => FloatUpdater::new().with_state(state),
        },
        InputMode::Editing => match msg {
            EditMessage::HandleInput(key_event) => {
                let updater = folder_handle_input_editing(app, &mut state, key_event);
                updater.with_state(state)
            }
            EditMessage::Edit => FloatUpdater::new().with_state(state),
            EditMessage::Confirm => {
                let updater = folder_confirm_editing(&mut state);
                updater.with_state(state)
            }
            EditMessage::Switch => FloatUpdater::new().with_state(state),
            EditMessage::SwitchLeft => FloatUpdater::new().with_state(state),
            EditMessage::SwitchRight => FloatUpdater::new().with_state(state),
            EditMessage::SwitchOrConfirm => FloatUpdater::new()
                .with_message(EditMessage::Confirm)
                .with_state(state),
            EditMessage::Quit(select) => {
                let updater = folder_quit_editing(app, &mut state, select);
                updater.with_state(state)
            }
            EditMessage::Back => {
                let updater = folder_back_editing(&mut state);
                updater.with_state(state)
            }
        },
    }
}

pub fn folder_handle_input_normal(
    app: &mut App,
    state: &FolderEditState,
    key_event: KeyEvent,
) -> FloatUpdater<EditMessage, FolderEditState> {
    let select = state.selected();
    let quit_select = if app.data.is_empty() {
        select
    } else {
        select.or(Some(0))
    };
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                FloatUpdater::new().with_message(EditMessage::Quit(quit_select))
            }
            (_, code) => match code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    FloatUpdater::new().with_message(EditMessage::Quit(quit_select))
                }
                KeyCode::Enter => FloatUpdater::new().with_message(EditMessage::Confirm),
                KeyCode::Char('a') | KeyCode::Char('e') => {
                    FloatUpdater::new().with_message(EditMessage::Edit)
                }
                _ => FloatUpdater::new(),
            },
        }
    } else {
        FloatUpdater::new()
    }
}

pub fn folder_edit_normal(
    state: &mut FolderEditState,
) -> FloatUpdater<EditMessage, FolderEditState> {
    state.switch_mode();
    FloatUpdater::new()
}

pub fn folder_confirm_normal(
    app: &mut App,
    state: &mut FolderEditState,
) -> FloatUpdater<EditMessage, FolderEditState> {
    let data = &mut app.data;
    let name = state.input().value();
    let select = match state.selected() {
        None => {
            // append
            let build_dir = LinkDir::builder(name);
            let dir = match build_dir {
                Ok(dir) => dir,
                // TODO: handle Err later (identifier empty)
                Err(err) => {
                    let msg = err.message().to_owned();
                    return FloatUpdater::new().with_float(Float::Warning(WarningState::new(msg)));
                }
            };
            // TODO: handle Err later (identifier already exists)
            match data.push(dir) {
                Ok(_) => Some(data.len().saturating_sub(1)),
                Err(err) => {
                    let msg = err.message().to_owned();
                    return FloatUpdater::new().with_float(Float::Warning(WarningState::new(msg)));
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
                        return FloatUpdater::new()
                            .with_float(Float::Warning(WarningState::new(msg)));
                    }
                }
            }
            Some(idx)
        }
    };
    FloatUpdater::new().with_message(EditMessage::Quit(select))
}

pub fn folder_quit_normal(
    app: &mut App,
    select: Option<usize>,
) -> FloatUpdater<EditMessage, FolderEditState> {
    app.state = AppState::Normal(Box::new(NormalState::Folder(
        FolderNormalState::with_selected(select),
    )));
    FloatUpdater::new()
}

pub fn folder_handle_input_editing(
    _app: &mut App,
    state: &mut FolderEditState,
    key_event: KeyEvent,
) -> FloatUpdater<EditMessage, FolderEditState> {
    let event = Event::Key(key_event);
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                FloatUpdater::new().with_message(EditMessage::Quit(state.selected()))
            }
            (_, code) => match code {
                KeyCode::Esc => FloatUpdater::new().with_message(EditMessage::Back),
                KeyCode::Enter => FloatUpdater::new().with_message(EditMessage::Confirm),
                _ => {
                    state.input_mut().handle_event(&event);
                    FloatUpdater::new()
                }
            },
        }
    } else {
        state.input_mut().handle_event(&event);
        FloatUpdater::new()
    }
}

pub fn folder_confirm_editing(
    state: &mut FolderEditState,
) -> FloatUpdater<EditMessage, FolderEditState> {
    state.switch_mode();
    FloatUpdater::new().with_message(EditMessage::Confirm)
}

pub fn folder_quit_editing(
    app: &mut App,
    state: &mut FolderEditState,
    select: Option<usize>,
) -> FloatUpdater<EditMessage, FolderEditState> {
    let select = if app.data.is_empty() {
        None
    } else {
        select.or(Some(0))
    };
    state.switch_mode();
    FloatUpdater::new().with_message(EditMessage::Quit(select))
}

pub fn folder_back_editing(
    state: &mut FolderEditState,
) -> FloatUpdater<EditMessage, FolderEditState> {
    state.switch_mode();
    FloatUpdater::new()
}

pub fn handle_edit_link_key(
    app: &mut App,
    key: KeyEvent,
    mut state: LinkEditState,
) -> FloatActionResult {
    let mut new_float = None;
    let mut opt_msg = edit_link_key(key);
    while let Some(msg) = opt_msg {
        let updater = edit_link_message(app, state, msg);
        opt_msg = updater.message;
        match updater.state {
            Some(s) => state = s,
            None => return FloatActionResult::new().with_optional_new(updater.float),
        }
        new_float = updater.float;
    }
    FloatActionResult::new()
        .with_primary(Float::LinkEdit(state))
        .with_optional_new(new_float)
}

pub fn edit_link_key(key: KeyEvent) -> Option<EditMessage> {
    Some(EditMessage::HandleInput(key))
}

pub fn edit_link_message(
    app: &mut App,
    mut state: LinkEditState,
    msg: EditMessage,
) -> FloatUpdater<EditMessage, LinkEditState> {
    match state.mode() {
        InputMode::Normal => match msg {
            EditMessage::HandleInput(key_event) => {
                let updater = link_handle_input_normal(app, &state, key_event);
                updater.with_state(state)
            }
            EditMessage::Edit => {
                let updater = link_edit_normal(&mut state);
                updater.with_state(state)
            }
            EditMessage::Confirm => {
                let updater = link_confirm_normal(app, &mut state);
                updater.with_state(state)
            }
            EditMessage::Switch => {
                let updater = link_switch_normal(&mut state);
                updater.with_state(state)
            }
            EditMessage::SwitchLeft => {
                let updater = link_switch_left_normal(&mut state);
                updater.with_state(state)
            }
            EditMessage::SwitchRight => {
                let updater = link_switch_right_normal(&mut state);
                updater.with_state(state)
            }
            EditMessage::SwitchOrConfirm => {
                let updater = link_switch_or_confirm_normal(&mut state);
                updater.with_state(state)
            }
            EditMessage::Quit(select) => link_quit_normal(app, &state, select),
            EditMessage::Back => FloatUpdater::new().with_state(state),
        },
        InputMode::Editing => match msg {
            EditMessage::HandleInput(key_event) => {
                let updater = link_handle_input_editing(app, &mut state, key_event);
                updater.with_state(state)
            }
            EditMessage::Edit => FloatUpdater::new().with_state(state),
            EditMessage::Confirm => {
                let updater = link_confirm_editing(&mut state);
                updater.with_state(state)
            }
            EditMessage::Switch => {
                let updater = link_switch_editing(&mut state);
                updater.with_state(state)
            }
            EditMessage::SwitchLeft => {
                let updater = link_switch_left_editing(&mut state);
                updater.with_state(state)
            }
            EditMessage::SwitchRight => {
                let updater = link_switch_right_editing(&mut state);
                updater.with_state(state)
            }
            EditMessage::SwitchOrConfirm => {
                let updater = link_switch_or_confirm_editing(&mut state);
                updater.with_state(state)
            }
            EditMessage::Quit(select) => {
                let updater = link_quit_editing(&mut state, select);
                updater.with_state(state)
            }
            EditMessage::Back => {
                let updater = link_back_editing(&mut state);
                updater.with_state(state)
            }
        },
    }
}

pub fn link_handle_input_normal(
    app: &mut App,
    state: &LinkEditState,
    key_event: KeyEvent,
) -> FloatUpdater<EditMessage, LinkEditState> {
    let select = state.selected();
    let quit_select = if app.data.is_empty() {
        select
    } else {
        select.or(Some(0))
    };
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                FloatUpdater::new().with_message(EditMessage::Quit(quit_select))
            }
            (_, code) => match code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    FloatUpdater::new().with_message(EditMessage::Quit(quit_select))
                }
                KeyCode::Enter => FloatUpdater::new().with_message(EditMessage::Confirm),
                KeyCode::Char('a') | KeyCode::Char('e') => {
                    FloatUpdater::new().with_message(EditMessage::Edit)
                }
                KeyCode::Tab | KeyCode::BackTab => {
                    FloatUpdater::new().with_message(EditMessage::Switch)
                }
                KeyCode::Left => FloatUpdater::new().with_message(EditMessage::SwitchLeft),
                KeyCode::Right => FloatUpdater::new().with_message(EditMessage::SwitchRight),
                _ => FloatUpdater::new(),
            },
        }
    } else {
        FloatUpdater::new()
    }
}

pub fn link_edit_normal(state: &mut LinkEditState) -> FloatUpdater<EditMessage, LinkEditState> {
    state.switch_mode();
    FloatUpdater::new()
}

pub fn link_confirm_normal(
    app: &mut App,
    state: &mut LinkEditState,
) -> FloatUpdater<EditMessage, LinkEditState> {
    let (key, value) = state.value();
    let value: PathBuf = link::get_vaild_path(value).unwrap_or_default();
    let data = &mut app.data[state.from()];

    let select = match state.selected() {
        None => {
            // append
            let build_link = Link::builder(key, &value);
            let link = match build_link {
                Ok(link) => link,
                // TODO: handle Err later (identifier or path empty)
                Err(_) => return FloatUpdater::new().with_message(EditMessage::Quit(Some(0))),
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

    FloatUpdater::new().with_message(EditMessage::Quit(select))
}

pub fn link_switch_normal(state: &mut LinkEditState) -> FloatUpdater<EditMessage, LinkEditState> {
    state.switch_part();
    FloatUpdater::new()
}

pub fn link_switch_left_normal(
    state: &mut LinkEditState,
) -> FloatUpdater<EditMessage, LinkEditState> {
    state.set_part(InputPart::Key);
    FloatUpdater::new()
}

pub fn link_switch_right_normal(
    state: &mut LinkEditState,
) -> FloatUpdater<EditMessage, LinkEditState> {
    state.set_part(InputPart::Value);
    FloatUpdater::new()
}

pub fn link_switch_or_confirm_normal(
    state: &mut LinkEditState,
) -> FloatUpdater<EditMessage, LinkEditState> {
    match state.part() {
        InputPart::Key => {
            state.set_part(InputPart::Value);
            FloatUpdater::new()
        }
        InputPart::Value => FloatUpdater::new().with_message(EditMessage::Confirm),
    }
}

pub fn link_quit_normal(
    app: &mut App,
    state: &LinkEditState,
    select: Option<usize>,
) -> FloatUpdater<EditMessage, LinkEditState> {
    app.state = AppState::Normal(Box::new(NormalState::Link(LinkNormalState::with_selected(
        state.from(),
        select,
    ))));
    FloatUpdater::new()
}

// NEW ^^^ / vvv OLD

pub fn link_handle_input_editing(
    _app: &mut App,
    state: &mut LinkEditState,
    key_event: KeyEvent,
) -> FloatUpdater<EditMessage, LinkEditState> {
    let event = Event::Key(key_event);
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                FloatUpdater::new().with_message(EditMessage::Quit(state.selected()))
            }
            (_, code) => match code {
                KeyCode::Esc => FloatUpdater::new().with_message(EditMessage::Back),
                KeyCode::Tab => FloatUpdater::new().with_message(EditMessage::Switch),
                KeyCode::Enter => FloatUpdater::new().with_message(EditMessage::SwitchOrConfirm),
                _ => {
                    match state.part() {
                        InputPart::Key => state.key_input_mut().handle_event(&event),
                        InputPart::Value => state.value_input_mut().handle_event(&event),
                    };
                    FloatUpdater::new()
                }
            },
        }
    } else {
        match state.part() {
            InputPart::Key => state.key_input_mut().handle_event(&event),
            InputPart::Value => state.value_input_mut().handle_event(&event),
        };
        FloatUpdater::new()
    }
}

pub fn link_confirm_editing(state: &mut LinkEditState) -> FloatUpdater<EditMessage, LinkEditState> {
    state.switch_mode();
    FloatUpdater::new().with_message(EditMessage::Confirm)
}

#[inline]
pub fn link_switch_editing(state: &mut LinkEditState) -> FloatUpdater<EditMessage, LinkEditState> {
    link_switch_normal(state)
}

#[inline]
pub fn link_switch_left_editing(
    state: &mut LinkEditState,
) -> FloatUpdater<EditMessage, LinkEditState> {
    link_switch_left_normal(state)
}

pub fn link_switch_right_editing(
    state: &mut LinkEditState,
) -> FloatUpdater<EditMessage, LinkEditState> {
    link_switch_right_normal(state)
}

pub fn link_switch_or_confirm_editing(
    state: &mut LinkEditState,
) -> FloatUpdater<EditMessage, LinkEditState> {
    match state.part() {
        InputPart::Key => {
            state.set_part(InputPart::Value);
            FloatUpdater::new()
        }
        InputPart::Value => FloatUpdater::new().with_message(EditMessage::Confirm),
    }
}

pub fn link_quit_editing(
    state: &mut LinkEditState,
    select: Option<usize>,
) -> FloatUpdater<EditMessage, LinkEditState> {
    state.switch_mode();
    FloatUpdater::new().with_message(EditMessage::Quit(select))
}

pub fn link_back_editing(state: &mut LinkEditState) -> FloatUpdater<EditMessage, LinkEditState> {
    state.switch_mode();
    FloatUpdater::new()
}
