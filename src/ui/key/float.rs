use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::{
    data::dirset::LinkDirSet,
    ui::{
        App,
        float::{
            Float, FolderDeleteConfirmCallbackType,
            confirm::{ConfirmChoice, FolderDeleteConfirmState},
        },
        message::{ConfirmMessage, FloatUpdater},
        state::{AppState, EditPart, NormalPart},
    },
};

pub fn handle_folder_delete_confirm_key(
    app: &mut App,
    key: KeyEvent,
    mut state: FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>,
) -> Option<Float> {
    let mut opt_msg = folder_delete_confirm_key(key);
    while let Some(msg) = opt_msg {
        let updater = folder_delete_confirm_message(app, state, msg);
        opt_msg = updater.message;
        match updater.state {
            Some(s) => state = s,
            None => return None,
        }
    }
    Some(Float::FolderDeleteConfirm(state))
}

pub fn folder_delete_confirm_key(key: KeyEvent) -> Option<ConfirmMessage> {
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => Some(ConfirmMessage::Yes),
            KeyCode::Char('n') | KeyCode::Char('N') => Some(ConfirmMessage::No),
            _ => None,
        }
    } else {
        None
    }
}

pub fn folder_delete_confirm_message(
    app: &mut App,
    state: FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>,
    message: ConfirmMessage,
) -> FloatUpdater<ConfirmMessage, FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>> {
    match message {
        ConfirmMessage::Yes => {
            folder_delete_confirm_call(&mut app.state, &mut app.data, state, ConfirmChoice::Yes)
        }
        ConfirmMessage::No | ConfirmMessage::Quit => {
            folder_delete_confirm_call(&mut app.state, &mut app.data, state, ConfirmChoice::No)
        }
    }
}

pub fn folder_delete_confirm_call(
    app_state: &mut AppState,
    data: &mut LinkDirSet,
    mut state: FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>,
    choice: ConfirmChoice,
) -> FloatUpdater<ConfirmMessage, FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>> {
    match app_state {
        AppState::Normal(part) => match &mut **part {
            NormalPart::Folder(folder_state) => {
                state.change_choice(choice);
                state.call(folder_state, data);
                FloatUpdater::new()
            }
            _ => FloatUpdater::new(),
        },
        AppState::Edit(part) => match &mut **part {
            EditPart::Folder(folder_state) => {
                state.change_choice(choice);
                state.call(folder_state.state_mut(), data);
                FloatUpdater::new()
            }
            _ => FloatUpdater::new(),
        },
        _ => FloatUpdater::new(),
    }
}
