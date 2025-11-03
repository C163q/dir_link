use std::path::PathBuf;

use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::{
    App,
    app::{
        data::CursorCache,
        float::{
            Float, FloatActionResult,
            confirm::{FolderSaveConfirmState, LinkSaveConfirmState},
            edit::{FolderEditState, LinkEditState},
            help::{HelpEntry, HelpState},
            warning::WarningState,
        },
        key::common,
        message::{EditMessage, FloatUpdater},
        normal::{FolderNormalState, InputMode, InputPart, LinkNormalState},
        state::{AppState, NormalState},
    },
    data::{
        dir::LinkDir,
        link::{self, Link},
    },
};

#[inline]
pub fn input_handle_key(input: &mut Input, event: &Event, cursor_cache: &mut CursorCache) {
    input.handle_event(event);
    cursor_cache.outdate();
}

#[inline]
pub fn handle_edit_folder_key(
    app: &mut App,
    key: KeyEvent,
    state: FolderEditState,
) -> FloatActionResult {
    common::handle_common_key(
        app,
        key,
        state,
        edit_folder_key,
        edit_folder_message,
        Float::FolderEdit,
    )
}

pub fn edit_folder_key(key: KeyEvent) -> Option<EditMessage> {
    Some(EditMessage::HandleInput(key))
}

pub fn edit_folder_message(
    app: &mut App,
    mut state: FolderEditState,
    msg: EditMessage,
) -> FloatUpdater<FolderEditState> {
    match state.mode() {
        InputMode::Normal => match msg {
            EditMessage::HandleInput(key_event) => {
                let updater = folder_handle_input_normal(app, &state, key_event);
                updater.with_state(state)
            }
            EditMessage::Edit => {
                let updater = folder_edit_normal(app, &mut state);
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
            EditMessage::Quit(select, ask_save) => folder_quit_normal(app, state, select, ask_save),
            EditMessage::Back => FloatUpdater::new().with_state(state),
            EditMessage::Help => {
                let updater = folder_help_normal(app, &mut state);
                updater.with_state(state)
            }
        },
        InputMode::Editing => match msg {
            EditMessage::HandleInput(key_event) => {
                let updater = folder_handle_input_editing(app, &mut state, key_event);
                updater.with_state(state)
            }
            EditMessage::Edit => FloatUpdater::new().with_state(state),
            EditMessage::Confirm => {
                let updater = folder_confirm_editing(app, &mut state);
                updater.with_state(state)
            }
            EditMessage::Switch => FloatUpdater::new().with_state(state),
            EditMessage::SwitchLeft => FloatUpdater::new().with_state(state),
            EditMessage::SwitchRight => FloatUpdater::new().with_state(state),
            EditMessage::SwitchOrConfirm => FloatUpdater::new()
                .with_message(EditMessage::Confirm)
                .with_state(state),
            EditMessage::Quit(select, ask_save) => {
                let updater = folder_quit_editing(app, &mut state, select, ask_save);
                updater.with_state(state)
            }
            EditMessage::Back => {
                let updater = folder_back_editing(app, &mut state);
                updater.with_state(state)
            }
            EditMessage::Help => FloatUpdater::new().with_state(state), // Can't reach
        },
    }
}

pub fn folder_handle_input_normal(
    app: &mut App,
    state: &FolderEditState,
    key_event: KeyEvent,
) -> FloatUpdater<FolderEditState> {
    let select = state.selected();
    let quit_select = if app.data.is_empty() {
        select
    } else {
        select.or(Some(0))
    };
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                FloatUpdater::new().with_message(EditMessage::Quit(quit_select, true))
            }
            (_, code) => match code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    FloatUpdater::new().with_message(EditMessage::Quit(quit_select, true))
                }
                KeyCode::Enter => FloatUpdater::new().with_message(EditMessage::Confirm),
                KeyCode::Char('a') | KeyCode::Char('e') => {
                    FloatUpdater::new().with_message(EditMessage::Edit)
                }
                KeyCode::Char('?') => FloatUpdater::new().with_message(EditMessage::Help),
                _ => FloatUpdater::new(),
            },
        }
    } else {
        FloatUpdater::new()
    }
}

pub fn folder_edit_normal(
    app: &mut App,
    state: &mut FolderEditState,
) -> FloatUpdater<FolderEditState> {
    app.cache.cursor.outdate();
    state.switch_mode();
    FloatUpdater::new()
}

pub fn folder_confirm_normal(
    app: &mut App,
    state: &mut FolderEditState,
) -> FloatUpdater<FolderEditState> {
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
    FloatUpdater::new().with_message(EditMessage::Quit(select, false))
}

pub fn folder_quit_normal(
    app: &mut App,
    state: FolderEditState,
    select: Option<usize>,
    ask_save: bool,
) -> FloatUpdater<FolderEditState> {
    let confirm = if ask_save {
        match state.selected() {
            None => !state.input().value().is_empty(),
            Some(idx) => state.input().value() != app.data[idx].identifier(),
        }
    } else {
        false
    };
    if confirm {
        return FloatUpdater::new().with_float(Float::FolderSaveConfirm(
            FolderSaveConfirmState::new(state, select),
        ));
    }
    app.set_state(AppState::Normal(Box::new(NormalState::Folder(
        FolderNormalState::with_selected(select),
    ))));
    FloatUpdater::new()
}

pub fn folder_help_normal(
    app: &mut App,
    _state: &mut FolderEditState,
) -> FloatUpdater<FolderEditState> {
    app.cache.cursor.outdate();
    let mut help = HelpState::new();
    help.extend(vec![
        HelpEntry::new("<Esc>/<q>", "Close the edit window in normal mode"),
        HelpEntry::new("<Enter>", "Confirm the edit result"),
        HelpEntry::new("<a>/<e>", "Enter editing mode in normal mode"),
        HelpEntry::new("<Esc>", "Back to normal mode from editing mode"),
        HelpEntry::new("<?>", "Open this window in normal mode"),
    ]);
    FloatUpdater::new().with_float(Float::Help(help))
}

pub fn folder_handle_input_editing(
    app: &mut App,
    state: &mut FolderEditState,
    key_event: KeyEvent,
) -> FloatUpdater<FolderEditState> {
    let event = Event::Key(key_event);
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                FloatUpdater::new().with_message(EditMessage::Quit(state.selected(), true))
            }
            (_, code) => match code {
                KeyCode::Esc => FloatUpdater::new().with_message(EditMessage::Back),
                KeyCode::Enter => FloatUpdater::new().with_message(EditMessage::Confirm),
                _ => {
                    input_handle_key(state.input_mut(), &event, &mut app.cache.cursor);
                    FloatUpdater::new()
                }
            },
        }
    } else {
        input_handle_key(state.input_mut(), &event, &mut app.cache.cursor);
        FloatUpdater::new()
    }
}

pub fn folder_confirm_editing(
    app: &mut App,
    state: &mut FolderEditState,
) -> FloatUpdater<FolderEditState> {
    state.switch_mode();
    app.cache.cursor.outdate();
    FloatUpdater::new().with_message(EditMessage::Confirm)
}

pub fn folder_quit_editing(
    app: &mut App,
    state: &mut FolderEditState,
    select: Option<usize>,
    ask_save: bool,
) -> FloatUpdater<FolderEditState> {
    let select = if app.data.is_empty() {
        None
    } else {
        select.or(Some(0))
    };
    state.switch_mode();
    app.cache.cursor.outdate();
    FloatUpdater::new().with_message(EditMessage::Quit(select, ask_save))
}

pub fn folder_back_editing(
    app: &mut App,
    state: &mut FolderEditState,
) -> FloatUpdater<FolderEditState> {
    state.switch_mode();
    app.cache.cursor.outdate();
    FloatUpdater::new()
}

#[inline]
pub fn handle_edit_link_key(
    app: &mut App,
    key: KeyEvent,
    state: LinkEditState,
) -> FloatActionResult {
    common::handle_common_key(
        app,
        key,
        state,
        edit_link_key,
        edit_link_message,
        Float::LinkEdit,
    )
}

pub fn edit_link_key(key: KeyEvent) -> Option<EditMessage> {
    Some(EditMessage::HandleInput(key))
}

pub fn edit_link_message(
    app: &mut App,
    mut state: LinkEditState,
    msg: EditMessage,
) -> FloatUpdater<LinkEditState> {
    match state.mode() {
        InputMode::Normal => match msg {
            EditMessage::HandleInput(key_event) => {
                let updater = link_handle_input_normal(app, &state, key_event);
                updater.with_state(state)
            }
            EditMessage::Edit => {
                let updater = link_edit_normal(app, &mut state);
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
            EditMessage::Quit(select, ask_save) => link_quit_normal(app, state, select, ask_save),
            EditMessage::Back => FloatUpdater::new().with_state(state),
            EditMessage::Help => {
                let updater = link_help_normal(app, &mut state);
                updater.with_state(state)
            }
        },
        InputMode::Editing => match msg {
            EditMessage::HandleInput(key_event) => {
                let updater = link_handle_input_editing(app, &mut state, key_event);
                updater.with_state(state)
            }
            EditMessage::Edit => FloatUpdater::new().with_state(state),
            EditMessage::Confirm => {
                let updater = link_confirm_editing(app, &mut state);
                updater.with_state(state)
            }
            EditMessage::Switch => {
                let updater = link_switch_editing(app, &mut state);
                updater.with_state(state)
            }
            EditMessage::SwitchLeft => {
                let updater = link_switch_left_editing(app, &mut state);
                updater.with_state(state)
            }
            EditMessage::SwitchRight => {
                let updater = link_switch_right_editing(app, &mut state);
                updater.with_state(state)
            }
            EditMessage::SwitchOrConfirm => {
                let updater = link_switch_or_confirm_editing(app, &mut state);
                updater.with_state(state)
            }
            EditMessage::Quit(select, ask_save) => {
                let updater = link_quit_editing(&mut state, select, ask_save);
                updater.with_state(state)
            }
            EditMessage::Back => {
                let updater = link_back_editing(app, &mut state);
                updater.with_state(state)
            }
            EditMessage::Help => FloatUpdater::new().with_state(state),
        },
    }
}

pub fn link_handle_input_normal(
    app: &mut App,
    state: &LinkEditState,
    key_event: KeyEvent,
) -> FloatUpdater<LinkEditState> {
    let select = state.selected();
    let quit_select = if app.data.is_empty() {
        select
    } else {
        select.or(Some(0))
    };
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                FloatUpdater::new().with_message(EditMessage::Quit(quit_select, true))
            }
            (_, code) => match code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    FloatUpdater::new().with_message(EditMessage::Quit(quit_select, true))
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
                KeyCode::Char('?') => FloatUpdater::new().with_message(EditMessage::Help),
                _ => FloatUpdater::new(),
            },
        }
    } else {
        FloatUpdater::new()
    }
}

pub fn link_edit_normal(app: &mut App, state: &mut LinkEditState) -> FloatUpdater<LinkEditState> {
    state.switch_mode();
    app.cache.cursor.outdate();
    FloatUpdater::new()
}

pub fn link_confirm_normal(
    app: &mut App,
    state: &mut LinkEditState,
) -> FloatUpdater<LinkEditState> {
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
                Err(err) => {
                    let msg = err.message().to_owned();
                    return FloatUpdater::new().with_float(Float::Warning(WarningState::new(msg)));
                }
            };
            // TODO: handle Err later (identifier already exists)
            match data.push(link) {
                Ok(_) => Some(data.len().saturating_sub(1)),
                Err(err) => {
                    let msg = err.message().to_owned();
                    return FloatUpdater::new().with_float(Float::Warning(WarningState::new(msg)));
                }
            }
        }
        Some(idx) => {
            // rename
            // TODO: handle Err later (identifier or path empty, identifier already exists)
            if key != data[idx].identifier()
                && let Err(err) = data.rename(idx, key)
            {
                let msg = err.message().to_owned();
                return FloatUpdater::new().with_float(Float::Warning(WarningState::new(msg)));
            }
            if value != data[idx].path()
                && let Err(err) = data.relink(idx, &value)
            {
                let msg = err.message().to_owned();
                return FloatUpdater::new().with_float(Float::Warning(WarningState::new(msg)));
            }
            Some(idx)
        }
    };

    FloatUpdater::new().with_message(EditMessage::Quit(select, false))
}

pub fn link_switch_normal(state: &mut LinkEditState) -> FloatUpdater<LinkEditState> {
    state.switch_part();
    FloatUpdater::new()
}

pub fn link_switch_left_normal(state: &mut LinkEditState) -> FloatUpdater<LinkEditState> {
    state.set_part(InputPart::Key);
    FloatUpdater::new()
}

pub fn link_switch_right_normal(state: &mut LinkEditState) -> FloatUpdater<LinkEditState> {
    state.set_part(InputPart::Value);
    FloatUpdater::new()
}

pub fn link_switch_or_confirm_normal(state: &mut LinkEditState) -> FloatUpdater<LinkEditState> {
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
    state: LinkEditState,
    select: Option<usize>,
    ask_save: bool,
) -> FloatUpdater<LinkEditState> {
    let confirm = if ask_save {
        match state.selected() {
            None => !state.value().0.is_empty() || !state.value().1.is_empty(),
            Some(idx) => {
                state.value().0 != app.data[state.from()][idx].identifier()
                    || state.value().1 != app.data[state.from()][idx].path().as_os_str()
            }
        }
    } else {
        false
    };
    if confirm {
        return FloatUpdater::new().with_float(Float::LinkSaveConfirm(LinkSaveConfirmState::new(
            state, select,
        )));
    }
    app.set_state(AppState::Normal(Box::new(NormalState::Link(
        LinkNormalState::with_selected(state.from(), select),
    ))));
    FloatUpdater::new()
}

pub fn link_help_normal(app: &mut App, _state: &mut LinkEditState) -> FloatUpdater<LinkEditState> {
    app.cache.cursor.outdate();
    let mut help = HelpState::new();
    help.extend(vec![
        HelpEntry::new("<Esc>/<q>", "Close the edit window in normal mode"),
        HelpEntry::new("<Enter>", "Confirm the edit result in normal mode"),
        HelpEntry::new("<a>/<e>", "Enter editing mode in normal mode"),
        HelpEntry::new("<Tab>/<BackTab>", "Switch between key and value input"),
        HelpEntry::new("<Left>/<Right>", "Switch to key/value input directly"),
        HelpEntry::new("<Esc>", "Back to normal mode from editing mode"),
        HelpEntry::new(
            "<Enter>",
            "Switch to value input or confirm edit result in editing mode",
        ),
        HelpEntry::new("<?>", "Open this window in normal mode"),
    ]);
    FloatUpdater::new().with_float(Float::Help(help))
}

pub fn link_handle_input_editing(
    app: &mut App,
    state: &mut LinkEditState,
    key_event: KeyEvent,
) -> FloatUpdater<LinkEditState> {
    let event = Event::Key(key_event);
    if key_event.kind == KeyEventKind::Press {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                FloatUpdater::new().with_message(EditMessage::Quit(state.selected(), true))
            }
            (_, code) => match code {
                KeyCode::Esc => FloatUpdater::new().with_message(EditMessage::Back),
                KeyCode::Tab => FloatUpdater::new().with_message(EditMessage::Switch),
                KeyCode::Enter => FloatUpdater::new().with_message(EditMessage::SwitchOrConfirm),
                _ => {
                    match state.part() {
                        InputPart::Key => {
                            input_handle_key(state.key_input_mut(), &event, &mut app.cache.cursor)
                        }
                        InputPart::Value => {
                            input_handle_key(state.value_input_mut(), &event, &mut app.cache.cursor)
                        }
                    };
                    FloatUpdater::new()
                }
            },
        }
    } else {
        match state.part() {
            InputPart::Key => {
                input_handle_key(state.key_input_mut(), &event, &mut app.cache.cursor)
            }
            InputPart::Value => {
                input_handle_key(state.value_input_mut(), &event, &mut app.cache.cursor)
            }
        };
        FloatUpdater::new()
    }
}

pub fn link_confirm_editing(
    app: &mut App,
    state: &mut LinkEditState,
) -> FloatUpdater<LinkEditState> {
    state.switch_mode();
    app.cache.cursor.outdate();
    FloatUpdater::new().with_message(EditMessage::Confirm)
}

#[inline]
pub fn link_switch_editing(
    app: &mut App,
    state: &mut LinkEditState,
) -> FloatUpdater<LinkEditState> {
    app.cache.cursor.outdate();
    link_switch_normal(state)
}

#[inline]
pub fn link_switch_left_editing(
    app: &mut App,
    state: &mut LinkEditState,
) -> FloatUpdater<LinkEditState> {
    app.cache.cursor.outdate();
    link_switch_left_normal(state)
}

pub fn link_switch_right_editing(
    app: &mut App,
    state: &mut LinkEditState,
) -> FloatUpdater<LinkEditState> {
    app.cache.cursor.outdate();
    link_switch_right_normal(state)
}

pub fn link_switch_or_confirm_editing(
    app: &mut App,
    state: &mut LinkEditState,
) -> FloatUpdater<LinkEditState> {
    app.cache.cursor.outdate();
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
    ask_save: bool,
) -> FloatUpdater<LinkEditState> {
    state.switch_mode();
    FloatUpdater::new().with_message(EditMessage::Quit(select, ask_save))
}

pub fn link_back_editing(app: &mut App, state: &mut LinkEditState) -> FloatUpdater<LinkEditState> {
    app.cache.cursor.outdate();
    state.switch_mode();
    FloatUpdater::new()
}
