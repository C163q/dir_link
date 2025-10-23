use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::{
    App,
    app::{
        float::{
            Float, FloatActionResult, FolderDeleteConfirmCallbackType,
            LinkDeleteConfirmCallbackType,
            confirm::{
                ConfirmChoice, FolderDeleteConfirmState, FolderSaveConfirmState,
                LinkDeleteConfirmState, LinkSaveConfirmState,
            },
            warning::{CorruptDataWarningChoice, CorruptDataWarningState, WarningState},
        },
        key::common,
        message::{ChooseMessage, ConfirmMessage, FloatUpdater, WarningMessage},
        state::{AppState, NormalState},
    },
    data::{dir::LinkDir, dirset::LinkDirSet},
};

#[inline]
pub fn handle_folder_delete_confirm_key(
    app: &mut App,
    key: KeyEvent,
    state: FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>,
) -> FloatActionResult {
    common::handle_common_key(
        app,
        key,
        state,
        folder_delete_confirm_key,
        folder_delete_confirm_message,
        Float::FolderDeleteConfirm,
    )
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
) -> FloatUpdater<FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>> {
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
) -> FloatUpdater<FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>> {
    match app_state {
        AppState::Normal(part) => match &mut **part {
            NormalState::Folder(folder_state) => {
                state.change_choice(choice);
                state.call(folder_state, data);
                FloatUpdater::new()
            }
            _ => FloatUpdater::new(),
        },
        _ => FloatUpdater::new(),
    }
}

#[inline]
pub fn handle_link_delete_confirm_key(
    app: &mut App,
    key: KeyEvent,
    state: LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>,
) -> FloatActionResult {
    common::handle_common_key(
        app,
        key,
        state,
        folder_delete_confirm_key,
        link_delete_confirm_message,
        Float::LinkDeleteConfirm,
    )
}

pub fn link_delete_confirm_message(
    app: &mut App,
    mut state: LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>,
    message: ConfirmMessage,
) -> FloatUpdater<LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>> {
    match message {
        ConfirmMessage::Yes => link_delete_confirm_call(
            &mut app.state,
            &mut app.data[state.dir_idx()],
            state,
            ConfirmChoice::Yes,
        ),
        ConfirmMessage::No | ConfirmMessage::Quit => link_delete_confirm_call(
            &mut app.state,
            &mut app.data[state.dir_idx()],
            state,
            ConfirmChoice::No,
        ),
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
            link_delete_confirm_call(
                &mut app.state,
                &mut app.data[state.dir_idx()],
                state,
                choice,
            )
        }
    }
}

pub fn link_delete_confirm_call(
    app_state: &mut AppState,
    data: &mut LinkDir,
    mut state: LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>,
    choice: ConfirmChoice,
) -> FloatUpdater<LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>> {
    match app_state {
        AppState::Normal(part) => match &mut **part {
            NormalState::Link(link_state) => {
                state.change_choice(choice);
                state.call(link_state, data);
                FloatUpdater::new()
            }
            _ => FloatUpdater::new(),
        },
        _ => FloatUpdater::new(),
    }
}

#[inline]
pub fn handle_warning_key(app: &mut App, key: KeyEvent, state: WarningState) -> FloatActionResult {
    common::handle_common_key(
        app,
        key,
        state,
        warning_key,
        warning_message,
        Float::Warning,
    )
}

pub fn warning_key(key: KeyEvent) -> Option<WarningMessage> {
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => Some(WarningMessage::Quit),
            _ => None,
        }
    } else {
        None
    }
}

pub fn warning_message(
    _app: &mut App,
    _state: WarningState,
    message: WarningMessage,
) -> FloatUpdater<WarningState> {
    match message {
        WarningMessage::Quit => FloatUpdater::new(),
    }
}

#[inline]
pub fn handle_folder_save_confirm_key(
    app: &mut App,
    key: KeyEvent,
    state: FolderSaveConfirmState,
) -> FloatActionResult {
    common::handle_common_key(
        app,
        key,
        state,
        folder_save_confirm_key,
        folder_save_confirm_message,
        Float::FolderSaveConfirm,
    )
}

pub fn folder_save_confirm_key(key: KeyEvent) -> Option<ConfirmMessage> {
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

pub fn folder_save_confirm_message(
    app: &mut App,
    mut state: FolderSaveConfirmState,
    message: ConfirmMessage,
) -> FloatUpdater<FolderSaveConfirmState> {
    match message {
        ConfirmMessage::Yes => folder_save_confirm_call(app, ConfirmChoice::Yes, state),
        ConfirmMessage::No | ConfirmMessage::Quit => {
            folder_save_confirm_call(app, ConfirmChoice::No, state)
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
            folder_save_confirm_call(app, choice, state)
        }
    }
}

pub fn folder_save_confirm_call(
    app: &mut App,
    choice: ConfirmChoice,
    mut state: FolderSaveConfirmState,
) -> FloatUpdater<FolderSaveConfirmState> {
    state.change_choice(choice);
    FloatUpdater::new().with_optional_float(state.call(app))
}

#[inline]
pub fn handle_link_save_confirm_key(
    app: &mut App,
    key: KeyEvent,
    state: LinkSaveConfirmState,
) -> FloatActionResult {
    common::handle_common_key(
        app,
        key,
        state,
        link_save_confirm_key,
        link_save_confirm_message,
        Float::LinkSaveConfirm,
    )
}

pub fn link_save_confirm_key(key: KeyEvent) -> Option<ConfirmMessage> {
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

pub fn link_save_confirm_message(
    app: &mut App,
    mut state: LinkSaveConfirmState,
    message: ConfirmMessage,
) -> FloatUpdater<LinkSaveConfirmState> {
    match message {
        ConfirmMessage::Yes => link_save_confirm_call(app, ConfirmChoice::Yes, state),
        ConfirmMessage::No | ConfirmMessage::Quit => {
            link_save_confirm_call(app, ConfirmChoice::No, state)
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
            link_save_confirm_call(app, choice, state)
        }
    }
}

pub fn link_save_confirm_call(
    app: &mut App,
    choice: ConfirmChoice,
    mut state: LinkSaveConfirmState,
) -> FloatUpdater<LinkSaveConfirmState> {
    state.change_choice(choice);
    FloatUpdater::new().with_optional_float(state.call(app))
}

#[inline]
pub fn handle_corrupt_data_warning_key(
    app: &mut App,
    key: KeyEvent,
    state: CorruptDataWarningState,
) -> FloatActionResult {
    common::handle_common_key(
        app,
        key,
        state,
        corrupt_data_warning_key,
        corrupt_data_warning_message,
        Float::CorruptDataWarning,
    )
}

pub fn corrupt_data_warning_key(key: KeyEvent) -> Option<ChooseMessage<bool>> {
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                Some(ChooseMessage::Quit(false))
            }
            KeyCode::Enter | KeyCode::Char(' ') => Some(ChooseMessage::Choose),
            KeyCode::Left => Some(ChooseMessage::SwitchLeft),
            KeyCode::Right => Some(ChooseMessage::SwitchRight),
            KeyCode::Up => Some(ChooseMessage::SwitchUp),
            KeyCode::Down => Some(ChooseMessage::SwitchDown),
            KeyCode::Tab => Some(ChooseMessage::Switch),
            KeyCode::BackTab => Some(ChooseMessage::SwitchBack),
            _ => None,
        }
    } else {
        None
    }
}

pub fn corrupt_data_warning_message(
    app: &mut App,
    mut state: CorruptDataWarningState,
    message: ChooseMessage<bool>,
) -> FloatUpdater<CorruptDataWarningState> {
    match message {
        ChooseMessage::Quit(choice) => {
            if !choice {
                app.option.save = false;
                app.set_state(AppState::Quit(Box::default()));
            }
            FloatUpdater::new()
        }
        ChooseMessage::Choose => match state.choice() {
            CorruptDataWarningChoice::Exit => FloatUpdater::new()
                .with_message(ChooseMessage::Quit(false))
                .with_state(state),
            CorruptDataWarningChoice::NewData => {
                app.option.save = true;
                FloatUpdater::new()
                    .with_message(ChooseMessage::Quit(true))
                    .with_state(state)
            }
        },
        ChooseMessage::SwitchLeft => {
            state.switch_left();
            FloatUpdater::new().with_state(state)
        }
        ChooseMessage::SwitchRight => {
            state.switch_right();
            FloatUpdater::new().with_state(state)
        }
        ChooseMessage::SwitchUp => {
            state.switch_up();
            FloatUpdater::new().with_state(state)
        }
        ChooseMessage::SwitchDown => {
            state.switch_down();
            FloatUpdater::new().with_state(state)
        }
        ChooseMessage::Switch => {
            state.switch();
            FloatUpdater::new().with_state(state)
        }
        ChooseMessage::SwitchBack => {
            state.switch_back();
            FloatUpdater::new().with_state(state)
        }
    }
}
