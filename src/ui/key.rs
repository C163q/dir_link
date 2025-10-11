use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub mod edit;
pub mod normal;

use crate::{
    data::{dir::LinkDir, dirset::LinkDirSet},
    ui::{
        App,
        message::{EditMessage, NormalFolderMessage, NormalLinkMessage},
        state::{
            AppState, EditPart, FolderEditState, FolderNormalState, InputMode, LinkEditState,
            LinkNormalState, NormalPart,
        },
    },
};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    let mut opt_mod = None;
    let data = &mut app.data;
    let state = &mut app.state;

    match state {
        AppState::Normal(part) => match &mut **part {
            NormalPart::Folder(state) => {
                let mut opt_msg = handle_normal_folder_key_event(key);
                while let Some(msg) = opt_msg {
                    (opt_msg, opt_mod) = handle_normal_folder_message(state, data, msg)
                }
            }
            NormalPart::Link(state) => {
                let mut opt_msg = handle_normal_link_key_event(key);
                while let Some(msg) = opt_msg {
                    let idx = state.folder_list_state().selected().unwrap();
                    (opt_msg, opt_mod) = handle_normal_link_message(state, &mut data[idx], msg);
                }
            }
        },
        AppState::Edit(part) => match &mut **part {
            EditPart::Folder(state) => {
                let mut opt_msg = handle_edit_folder_key_event(key);
                while let Some(msg) = opt_msg {
                    (opt_msg, opt_mod) = handle_edit_folder_message(state, data, msg)
                }
            }
            EditPart::Link(state) => {
                let mut opt_msg = handle_edit_link_key_event(key);
                while let Some(msg) = opt_msg {
                    let idx = state.folder_list_state().selected().unwrap();
                    (opt_msg, opt_mod) = handle_edit_link_message(state, &mut data[idx], msg)
                }
            }
        },
        AppState::Quit(_) => {}
    }

    // 在此处跳转状态
    if let Some(mod_change) = opt_mod {
        app.state = mod_change;
    }
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
) -> (Option<NormalFolderMessage>, Option<AppState>) {
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
) -> (Option<NormalLinkMessage>, Option<AppState>) {
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

pub fn handle_edit_folder_message(
    state: &mut FolderEditState,
    data: &mut LinkDirSet,
    message: EditMessage,
) -> (Option<EditMessage>, Option<AppState>) {
    match state.mode() {
        InputMode::Normal => match message {
            EditMessage::HandleInput(key_event) => {
                edit::folder_handle_input_normal(state, data, key_event)
            }
            EditMessage::Edit => edit::folder_edit_normal(state, data),
            EditMessage::Confirm => edit::folder_confirm_normal(state, data),
            EditMessage::Switch => (None, None),
            EditMessage::SwitchLeft => (None, None),
            EditMessage::SwitchRight => (None, None),
            EditMessage::SwitchOrConfirm => (Some(EditMessage::Confirm), None),
            EditMessage::Quit(select) => edit::folder_quit_normal(state, data, select),
            EditMessage::Back => (None, None),
        },
        InputMode::Editing => match message {
            EditMessage::HandleInput(key_event) => {
                edit::folder_handle_input_editing(state, data, key_event)
            }
            EditMessage::Edit => (None, None),
            EditMessage::Confirm => edit::folder_confirm_editing(state, data),
            EditMessage::Switch => (None, None),
            EditMessage::SwitchLeft => (None, None),
            EditMessage::SwitchRight => (None, None),
            EditMessage::SwitchOrConfirm => (Some(EditMessage::Confirm), None),
            EditMessage::Quit(select) => edit::folder_quit_editing(state, data, select),
            EditMessage::Back => edit::folder_back_editing(state, data),
        },
    }
}

pub fn handle_edit_link_message(
    state: &mut LinkEditState,
    data: &mut LinkDir,
    message: EditMessage,
) -> (Option<EditMessage>, Option<AppState>) {
    match state.mode() {
        InputMode::Normal => match message {
            EditMessage::HandleInput(key_event) => {
                edit::link_handle_input_normal(state, data, key_event)
            }
            EditMessage::Edit => edit::link_edit_normal(state, data),
            EditMessage::Confirm => edit::link_confirm_normal(state, data),
            EditMessage::Switch => edit::link_switch_normal(state, data),
            EditMessage::SwitchLeft => edit::link_switch_left_normal(state, data),
            EditMessage::SwitchRight => edit::link_switch_right_normal(state, data),
            EditMessage::SwitchOrConfirm => edit::link_switch_or_confirm_normal(state, data),
            EditMessage::Quit(select) => edit::link_quit_normal(state, data, select),
            EditMessage::Back => (None, None),
        },
        InputMode::Editing => match message {
            EditMessage::HandleInput(key_event) => {
                edit::link_handle_input_editing(state, data, key_event)
            }
            EditMessage::Edit => (None, None),
            EditMessage::Confirm => edit::link_confirm_editing(state, data),
            EditMessage::Switch => edit::link_switch_editing(state, data),
            EditMessage::SwitchLeft => edit::link_switch_left_editing(state, data),
            EditMessage::SwitchRight => edit::link_switch_right_editing(state, data),
            EditMessage::SwitchOrConfirm => edit::link_switch_or_confirm_editing(state, data),
            EditMessage::Quit(select) => edit::link_quit_editing(state, data, select),
            EditMessage::Back => edit::link_back_editing(state, data),
        },
    }
}
