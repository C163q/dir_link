use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub mod edit;
pub mod float;
pub mod normal;

use crate::{
    data::{dir::LinkDir, dirset::LinkDirSet},
    ui::{
        App,
        float::Float,
        message::{EditMessage, MessageUpdater, NormalFolderMessage, NormalLinkMessage},
        state::{AppState, FolderNormalState, LinkNormalState, NormalState},
    },
};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match app.float.pop() {
        None => handle_key_event_basic(app, key),
        Some(float) => handle_key_event_float(app, key, float),
    }
}

pub fn handle_key_event_basic(app: &mut App, key: KeyEvent) {
    let mut opt_mod = None;
    let data = &mut app.data;
    let state = &mut app.state;
    let mut float = None;

    match state {
        AppState::Normal(part) => match &mut **part {
            NormalState::Folder(state) => {
                let mut opt_msg = handle_normal_folder_key_event(key);
                while let Some(msg) = opt_msg {
                    let updater = handle_normal_folder_message(state, data, msg);
                    (opt_msg, opt_mod, float) = (updater.message, updater.state, updater.float);
                }
            }
            NormalState::Link(state) => {
                let mut opt_msg = handle_normal_link_key_event(key);
                while let Some(msg) = opt_msg {
                    let idx = state.folder_list_state().selected().unwrap();
                    let updater = handle_normal_link_message(state, &mut data[idx], msg);
                    (opt_msg, opt_mod, float) = (updater.message, updater.state, updater.float);
                }
            }
        },
        AppState::Quit(_) => {}
    }

    // 在此处跳转状态
    if let Some(mod_change) = opt_mod {
        app.state = mod_change;
    }
    if let Some(f) = float {
        app.float.push(f);
    }
}

pub fn handle_key_event_float(app: &mut App, key: KeyEvent, float: Float) {
    let float_action = match float {
        Float::FolderEdit(state) => edit::handle_edit_folder_key(app, key, state),
        Float::LinkEdit(state) => edit::handle_edit_link_key(app, key, state),
        Float::FolderDeleteConfirm(state) => {
            float::handle_folder_delete_confirm_key(app, key, state)
        }
        Float::LinkDeleteConfirm(state) => float::handle_link_delete_confirm_key(app, key, state),
        Float::Warning(state) => float::handle_warning_key(app, key, state),
        Float::FolderSaveConfirm(state) => float::handle_folder_save_confirm_key(app, key, state),
        Float::LinkSaveConfirm(state) => float::handle_link_save_confirm_key(app, key, state),
    };
    app.float.extend(float_action);
}

pub fn handle_normal_folder_key_event(key: KeyEvent) -> Option<NormalFolderMessage> {
    if key.kind == KeyEventKind::Press {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                Some(NormalFolderMessage::Quit)
            }
            (KeyModifiers::CONTROL, KeyCode::Up) => Some(NormalFolderMessage::SwitchUp),
            (KeyModifiers::CONTROL, KeyCode::Down) => Some(NormalFolderMessage::SwitchDown),
            (_, code) => match code {
                KeyCode::Char('q') | KeyCode::Esc => Some(NormalFolderMessage::Quit),
                KeyCode::Enter | KeyCode::Right => Some(NormalFolderMessage::Select),
                KeyCode::Char('k') | KeyCode::Up => Some(NormalFolderMessage::MoveUp),
                KeyCode::Char('j') | KeyCode::Down => Some(NormalFolderMessage::MoveDown),
                KeyCode::Char('K') => Some(NormalFolderMessage::SwitchUp),
                KeyCode::Char('J') => Some(NormalFolderMessage::SwitchDown),
                KeyCode::Char('a') => Some(NormalFolderMessage::Append),
                KeyCode::Char('r') => Some(NormalFolderMessage::Rename),
                KeyCode::Char('x') => Some(NormalFolderMessage::Remove),
                _ => None,
            },
        }
    } else {
        None
    }
}

pub fn handle_normal_link_key_event(key: KeyEvent) -> Option<NormalLinkMessage> {
    if key.kind == KeyEventKind::Press {
        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                Some(NormalLinkMessage::Quit)
            }
            (KeyModifiers::CONTROL, KeyCode::Up) => Some(NormalLinkMessage::SwitchUp),
            (KeyModifiers::CONTROL, KeyCode::Down) => Some(NormalLinkMessage::SwitchDown),
            (_, code) => match code {
                KeyCode::Char('q') | KeyCode::Esc => Some(NormalLinkMessage::Quit),
                KeyCode::Enter => Some(NormalLinkMessage::Select),
                KeyCode::Left => Some(NormalLinkMessage::Back),
                KeyCode::Char('k') | KeyCode::Up => Some(NormalLinkMessage::MoveUp),
                KeyCode::Char('j') | KeyCode::Down => Some(NormalLinkMessage::MoveDown),
                KeyCode::Char('K') => Some(NormalLinkMessage::SwitchUp),
                KeyCode::Char('J') => Some(NormalLinkMessage::SwitchDown),
                KeyCode::Char('a') => Some(NormalLinkMessage::Append),
                KeyCode::Char('r') => Some(NormalLinkMessage::Rename),
                KeyCode::Char('x') => Some(NormalLinkMessage::Remove),
                _ => None,
            },
        }
    } else {
        None
    }
}

pub fn handle_edit_folder_key_event(key: KeyEvent) -> Option<EditMessage> {
    Some(EditMessage::HandleInput(key))
}

pub fn handle_edit_link_key_event(key: KeyEvent) -> Option<EditMessage> {
    Some(EditMessage::HandleInput(key))
}

pub fn handle_normal_folder_message(
    state: &mut FolderNormalState,
    data: &mut LinkDirSet,
    message: NormalFolderMessage,
) -> MessageUpdater<NormalFolderMessage> {
    match message {
        NormalFolderMessage::Select => normal::folder_select(state, data),
        NormalFolderMessage::MoveUp => normal::folder_move_up(state, data),
        NormalFolderMessage::MoveDown => normal::folder_move_down(state, data),
        NormalFolderMessage::SwitchUp => normal::folder_switch_up(state, data),
        NormalFolderMessage::SwitchDown => normal::folder_switch_down(state, data),
        NormalFolderMessage::Append => normal::folder_append(state, data),
        NormalFolderMessage::Rename => normal::folder_rename(state, data),
        NormalFolderMessage::Remove => normal::folder_remove(state, data),
        NormalFolderMessage::Quit => normal::folder_quit(),
        NormalFolderMessage::Item(idx) => normal::folder_item(state, idx),
        NormalFolderMessage::ToDir(idx) => normal::folder_to_dir(state, data, idx),
    }
}

pub fn handle_normal_link_message(
    state: &mut LinkNormalState,
    data: &mut LinkDir,
    message: NormalLinkMessage,
) -> MessageUpdater<NormalLinkMessage> {
    match message {
        NormalLinkMessage::Back => normal::link_back(state, data),
        NormalLinkMessage::Select => normal::link_select(state, data),
        NormalLinkMessage::MoveUp => normal::link_move_up(state, data),
        NormalLinkMessage::MoveDown => normal::link_move_down(state, data),
        NormalLinkMessage::SwitchUp => normal::link_switch_up(state, data),
        NormalLinkMessage::SwitchDown => normal::link_switch_down(state, data),
        NormalLinkMessage::Append => normal::link_append(state, data),
        NormalLinkMessage::Rename => normal::link_rename(state, data),
        NormalLinkMessage::Remove => normal::link_remove(state, data),
        NormalLinkMessage::Quit => normal::link_quit(),
        NormalLinkMessage::Item(idx) => normal::link_item(state, idx),
        NormalLinkMessage::ToLink(idx) => normal::link_to_link(state, data, idx),
    }
}
