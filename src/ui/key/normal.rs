use crate::{
    data::{dir::LinkDir, dirset::LinkDirSet}, ui::{
        float::{
            confirm::{ConfirmChoice, FolderDeleteConfirmState, LinkDeleteConfirmState}, Float
        },
        message::{MessageUpdater, NormalFolderMessage, NormalLinkMessage},
        state::{
            AppState, EditPart, FolderEditState, FolderNormalState, LinkEditState, LinkNormalState,
            NormalPart,
        },
    }, DataTransfer
};

pub fn folder_select(
    state: &mut FolderNormalState,
    data: &mut LinkDirSet,
) -> MessageUpdater<NormalFolderMessage> {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        None => {
            state.select(Some(0));
            MessageUpdater::new()
        }
        Some(idx) if idx < data.len() => {
            MessageUpdater::new().with_message(NormalFolderMessage::ToDir(idx))
        }
        Some(_) => {
            state.select(Some(data.len() - 1));
            MessageUpdater::new()
        }
    }
}

pub fn folder_move_up(
    state: &mut FolderNormalState,
    data: &LinkDirSet,
) -> MessageUpdater<NormalFolderMessage> {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        None => MessageUpdater::new().with_message(NormalFolderMessage::Item(0)),
        Some(0) => MessageUpdater::new(),
        Some(idx) => MessageUpdater::new().with_message(NormalFolderMessage::Item(idx - 1)),
    }
}

pub fn folder_move_down(
    state: &mut FolderNormalState,
    data: &LinkDirSet,
) -> MessageUpdater<NormalFolderMessage> {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        None => MessageUpdater::new().with_message(NormalFolderMessage::Item(0)),
        Some(idx) => MessageUpdater::new().with_message(NormalFolderMessage::Item(idx + 1)),
    }
}

pub fn folder_switch_up(
    state: &mut FolderNormalState,
    data: &mut LinkDirSet,
) -> MessageUpdater<NormalFolderMessage> {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        None => MessageUpdater::new().with_message(NormalFolderMessage::Item(0)),
        Some(0) => MessageUpdater::new(),
        Some(idx) if idx < data.len() => {
            data.swap(idx, idx - 1);
            MessageUpdater::new().with_message(NormalFolderMessage::MoveUp)
        }
        _ => MessageUpdater::new(),
    }
}

pub fn folder_switch_down(
    state: &mut FolderNormalState,
    data: &mut LinkDirSet,
) -> MessageUpdater<NormalFolderMessage> {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        None => MessageUpdater::new().with_message(NormalFolderMessage::Item(0)),
        Some(idx) if idx + 1 < data.len() => {
            data.swap(idx, idx + 1);
            MessageUpdater::new().with_message(NormalFolderMessage::MoveDown)
        }
        _ => MessageUpdater::new(),
    }
}

pub fn folder_append(
    _state: &mut FolderNormalState,
    _data: &LinkDirSet,
) -> MessageUpdater<NormalFolderMessage> {
    MessageUpdater::new().with_state(AppState::Edit(Box::new(EditPart::Folder(
        FolderEditState::new(None),
    ))))
}

pub fn folder_rename(
    state: &mut FolderNormalState,
    data: &LinkDirSet,
) -> MessageUpdater<NormalFolderMessage> {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        Some(idx) if idx < data.len() => {
            MessageUpdater::new().with_state(AppState::Edit(Box::new(EditPart::Folder(
                FolderEditState::new(Some(idx)).with_value(data[idx].identifier()),
            ))))
        }
        _ => MessageUpdater::new(),
    }
}

pub fn folder_remove(
    state: &mut FolderNormalState,
    data: &mut LinkDirSet,
) -> MessageUpdater<NormalFolderMessage> {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        Some(idx) if idx < data.len() => {
            let remove = move |choice, state: &mut FolderNormalState, data: &mut LinkDirSet| {
                if choice == ConfirmChoice::No {
                    return;
                }
                data.remove(idx);
                state.select(Some(idx.min(data.len().saturating_sub(1))));
            };
            MessageUpdater::new().with_float(Float::FolderDeleteConfirm(
                FolderDeleteConfirmState::new(Box::new(remove)),
            ))
        }
        _ => MessageUpdater::new(),
    }
}

pub fn folder_quit() -> MessageUpdater<NormalFolderMessage> {
    MessageUpdater::new().with_state(AppState::Quit(Box::default()))
}

pub fn folder_item(
    state: &mut FolderNormalState,
    idx: usize,
) -> MessageUpdater<NormalFolderMessage> {
    state.select(Some(idx));
    MessageUpdater::new()
}

pub fn folder_to_dir(
    state: &mut FolderNormalState,
    data: &LinkDirSet,
    idx: usize,
) -> MessageUpdater<NormalFolderMessage> {
    if idx < data.len() {
        MessageUpdater::new().with_state(AppState::Normal(Box::new(NormalPart::Link(
            LinkNormalState::new(state.list_state().selected().unwrap()),
        ))))
    } else {
        MessageUpdater::new()
    }
}

pub fn link_back(
    state: &mut LinkNormalState,
    _data: &LinkDir,
) -> MessageUpdater<NormalLinkMessage> {
    let dir_idx = state.folder_list_state().selected().unwrap_or(0);
    MessageUpdater::new().with_state(AppState::Normal(Box::new(NormalPart::Folder(
        FolderNormalState::with_selected(Some(dir_idx)),
    ))))
}

pub fn link_select(
    state: &mut LinkNormalState,
    data: &LinkDir,
) -> MessageUpdater<NormalLinkMessage> {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        None => {
            state.select(Some(0));
            MessageUpdater::new()
        }
        Some(idx) if idx < data.len() => {
            MessageUpdater::new().with_message(NormalLinkMessage::ToLink(idx))
        }
        Some(_) => {
            state.select(Some(data.len() - 1));
            MessageUpdater::new()
        }
    }
}

pub fn link_move_up(
    state: &mut LinkNormalState,
    data: &LinkDir,
) -> MessageUpdater<NormalLinkMessage> {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        None => MessageUpdater::new().with_message(NormalLinkMessage::Item(0)),
        Some(0) => MessageUpdater::new(),
        Some(idx) => MessageUpdater::new().with_message(NormalLinkMessage::Item(idx - 1)),
    }
}

pub fn link_move_down(
    state: &mut LinkNormalState,
    data: &LinkDir,
) -> MessageUpdater<NormalLinkMessage> {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        None => MessageUpdater::new().with_message(NormalLinkMessage::Item(0)),
        Some(idx) => MessageUpdater::new().with_message(NormalLinkMessage::Item(idx + 1)),
    }
}

pub fn link_switch_up(
    state: &mut LinkNormalState,
    data: &mut LinkDir,
) -> MessageUpdater<NormalLinkMessage> {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        None => MessageUpdater::new().with_message(NormalLinkMessage::Item(0)),
        Some(0) => MessageUpdater::new(),
        Some(idx) if idx < data.len() => {
            data.swap(idx, idx - 1);
            MessageUpdater::new().with_message(NormalLinkMessage::MoveUp)
        }
        _ => MessageUpdater::new(),
    }
}

pub fn link_switch_down(
    state: &mut LinkNormalState,
    data: &mut LinkDir,
) -> MessageUpdater<NormalLinkMessage> {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        None => MessageUpdater::new().with_message(NormalLinkMessage::Item(0)),
        Some(idx) if idx + 1 < data.len() => {
            data.swap(idx, idx + 1);
            MessageUpdater::new().with_message(NormalLinkMessage::MoveDown)
        }
        _ => MessageUpdater::new(),
    }
}

pub fn link_append(
    state: &mut LinkNormalState,
    _data: &LinkDir,
) -> MessageUpdater<NormalLinkMessage> {
    MessageUpdater::new().with_state(AppState::Edit(Box::new(EditPart::Link(
        LinkEditState::new(state.folder_list_state().selected().unwrap(), None),
    ))))
}

pub fn link_rename(
    state: &mut LinkNormalState,
    data: &LinkDir,
) -> MessageUpdater<NormalLinkMessage> {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        Some(idx) if idx < data.len() => {
            MessageUpdater::new().with_state(AppState::Edit(Box::new(EditPart::Link(
                LinkEditState::new(state.folder_list_state().selected().unwrap(), Some(idx))
                    .with_value(data[idx].identifier(), data[idx].path().as_os_str()),
            ))))
        }
        _ => MessageUpdater::new(),
    }
}

pub fn link_remove(
    state: &mut LinkNormalState,
    data: &mut LinkDir,
) -> MessageUpdater<NormalLinkMessage> {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return MessageUpdater::new();
    }
    match opt_idx {
        Some(idx) if idx < data.len() => {
            let remove = move |choice, state: &mut LinkNormalState, data: &mut LinkDir| {
                if choice == ConfirmChoice::No {
                    return;
                }
                data.remove(idx);
                state.select(Some(idx.min(data.len().saturating_sub(1))));
            };
            MessageUpdater::new().with_float(Float::LinkDeleteConfirm(
                LinkDeleteConfirmState::new(Box::new(remove), state.folder_index()),
            ))
        }
        _ => MessageUpdater::new(),
    }
}

pub fn link_quit() -> MessageUpdater<NormalLinkMessage> {
    MessageUpdater::new().with_state(AppState::Quit(Box::default()))
}

pub fn link_item(state: &mut LinkNormalState, idx: usize) -> MessageUpdater<NormalLinkMessage> {
    state.select(Some(idx));
    MessageUpdater::new()
}

pub fn link_to_link(
    _state: &mut LinkNormalState,
    data: &LinkDir,
    idx: usize,
) -> MessageUpdater<NormalLinkMessage> {
    if idx < data.len() {
        MessageUpdater::new().with_state(AppState::Quit(Box::new(DataTransfer::with_link(
            data[idx].clone(),
        ))))
    } else {
        MessageUpdater::new()
    }
}
