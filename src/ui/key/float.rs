use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::{
    data::{dir::LinkDir, dirset::LinkDirSet},
    ui::{
        float::{
            confirm::{ConfirmChoice, FolderDeleteConfirmState, LinkDeleteConfirmState}, Float, FolderDeleteConfirmCallbackType, LinkDeleteConfirmCallbackType
        }, message::{ConfirmMessage, FloatUpdater}, state::{AppState, EditPart, NormalPart}, App
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
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => Some(ConfirmMessage::Quit),
            KeyCode::Left => Some(ConfirmMessage::SwitchLeft),
            KeyCode::Right => Some(ConfirmMessage::SwitchRight),
            KeyCode::Tab | KeyCode::BackTab => Some(ConfirmMessage::Switch),
            KeyCode::Enter | KeyCode::Char(' ') => Some(ConfirmMessage::Choose),
            _ => None,
        }
    } else {
        None
    }
}

pub fn folder_delete_confirm_message(
    app: &mut App,
    mut state: FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>,
    message: ConfirmMessage,
) -> FloatUpdater<ConfirmMessage, FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>> {
    match message {
        ConfirmMessage::Yes => {
            folder_delete_confirm_call(&mut app.state, &mut app.data, state, ConfirmChoice::Yes)
        }
        ConfirmMessage::No | ConfirmMessage::Quit => {
            folder_delete_confirm_call(&mut app.state, &mut app.data, state, ConfirmChoice::No)
        }
        ConfirmMessage::SwitchLeft => {
            state.change_choice(ConfirmChoice::Yes);
            FloatUpdater::new().with_state(state)
        }
        ConfirmMessage::SwitchRight => {
            state.change_choice(ConfirmChoice::No);
            FloatUpdater::new().with_state(state)
        }
        ConfirmMessage::Switch => {
            state.switch_chioce();
            FloatUpdater::new().with_state(state)
        }
        ConfirmMessage::Choose => {
            let choice = state.choice();
            folder_delete_confirm_call(&mut app.state, &mut app.data, state, choice)
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

pub fn handle_link_delete_confirm_key(
    app: &mut App,
    key: KeyEvent,
    mut state: LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>,
) -> Option<Float> {
    // 它们的按键逻辑是一样的
    let mut opt_msg = folder_delete_confirm_key(key);
    while let Some(msg) = opt_msg {
        let updater = link_delete_confirm_message(app, state, msg);
        opt_msg = updater.message;
        match updater.state {
            Some(s) => state = s,
            None => return None,
        }
    }
    Some(Float::LinkDeleteConfirm(state))
}

pub fn link_delete_confirm_message(
    app: &mut App,
    mut state: LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>,
    message: ConfirmMessage,
) -> FloatUpdater<ConfirmMessage, LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>> {
    match message {
        ConfirmMessage::Yes => {
            link_delete_confirm_call(&mut app.state, &mut app.data[state.dir_idx()], state, ConfirmChoice::Yes)
        }
        ConfirmMessage::No | ConfirmMessage::Quit => {
            link_delete_confirm_call(&mut app.state, &mut app.data[state.dir_idx()], state, ConfirmChoice::No)
        }
        ConfirmMessage::SwitchLeft => {
            state.change_choice(ConfirmChoice::Yes);
            FloatUpdater::new().with_state(state)
        }
        ConfirmMessage::SwitchRight => {
            state.change_choice(ConfirmChoice::No);
            FloatUpdater::new().with_state(state)
        }
        ConfirmMessage::Switch => {
            state.switch_chioce();
            FloatUpdater::new().with_state(state)
        }
        ConfirmMessage::Choose => {
            let choice = state.choice();
            link_delete_confirm_call(&mut app.state, &mut app.data[state.dir_idx()], state, choice)
        }
    }
}

pub fn link_delete_confirm_call(
    app_state: &mut AppState,
    data: &mut LinkDir,
    mut state: LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>,
    choice: ConfirmChoice,
) -> FloatUpdater<ConfirmMessage, LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>> {
    match app_state {
        AppState::Normal(part) => match &mut **part {
            NormalPart::Link(link_state) => {
                state.change_choice(choice);
                state.call(link_state, data);
                FloatUpdater::new()
            }
            _ => FloatUpdater::new(),
        },
        AppState::Edit(part) => match &mut **part {
            EditPart::Link(link_state) => {
                state.change_choice(choice);
                state.call(link_state.state_mut(), data);
                FloatUpdater::new()
            }
            _ => FloatUpdater::new(),
        },
        _ => FloatUpdater::new(),
    }
}
