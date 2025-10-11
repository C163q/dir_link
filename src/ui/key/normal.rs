use crate::{
    data::{dir::LinkDir, dirset::LinkDirSet},
    ui::{
        message::{NormalFolderMessage, NormalLinkMessage},
        state::{
            AppState, EditPart, FolderEditState, FolderNormalState, LinkEditState, LinkNormalState,
            NormalPart,
        },
    }, DataTransfer,
};

pub fn folder_select(
    state: &mut FolderNormalState,
    data: &mut LinkDirSet,
) -> (Option<NormalFolderMessage>, Option<AppState>) {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        None => {
            state.select(Some(0));
            (None, None)
        }
        Some(idx) if idx < data.len() => (Some(NormalFolderMessage::ToDir(idx)), None),
        Some(_) => {
            state.select(Some(data.len() - 1));
            (None, None)
        }
    }
}

pub fn folder_move_up(
    state: &mut FolderNormalState,
    data: &LinkDirSet,
) -> (Option<NormalFolderMessage>, Option<AppState>) {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        None => (Some(NormalFolderMessage::Item(0)), None),
        Some(0) => (None, None),
        Some(idx) => (Some(NormalFolderMessage::Item(idx - 1)), None),
    }
}

pub fn folder_move_down(
    state: &mut FolderNormalState,
    data: &LinkDirSet,
) -> (Option<NormalFolderMessage>, Option<AppState>) {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        None => (Some(NormalFolderMessage::Item(0)), None),
        Some(idx) => (Some(NormalFolderMessage::Item(idx + 1)), None),
    }
}

pub fn folder_switch_up(
    state: &mut FolderNormalState,
    data: &mut LinkDirSet,
) -> (Option<NormalFolderMessage>, Option<AppState>) {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        None => (Some(NormalFolderMessage::Item(0)), None),
        Some(0) => (None, None),
        Some(idx) if idx < data.len() => {
            data.swap(idx, idx - 1);
            (None, None)
        }
        _ => (None, None),
    }
}

pub fn folder_switch_down(
    state: &mut FolderNormalState,
    data: &mut LinkDirSet,
) -> (Option<NormalFolderMessage>, Option<AppState>) {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        None => (Some(NormalFolderMessage::Item(0)), None),
        Some(idx) if idx + 1 < data.len() => {
            data.swap(idx, idx + 1);
            (None, None)
        }
        _ => (None, None),
    }
}

pub fn folder_append(
    _state: &mut FolderNormalState,
    _data: &LinkDirSet,
) -> (Option<NormalFolderMessage>, Option<AppState>) {
    (
        None,
        Some(AppState::Edit(Box::new(EditPart::Folder(
            FolderEditState::new(None),
        )))),
    )
}

pub fn folder_rename(
    state: &mut FolderNormalState,
    data: &LinkDirSet,
) -> (Option<NormalFolderMessage>, Option<AppState>) {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        Some(idx) if idx < data.len() => (
            None,
            Some(AppState::Edit(Box::new(EditPart::Folder(
                FolderEditState::new(Some(idx)).with_value(data[idx].identifier()),
            )))),
        ),
        _ => (None, None),
    }
}

pub fn folder_remove(
    state: &mut FolderNormalState,
    data: &mut LinkDirSet,
) -> (Option<NormalFolderMessage>, Option<AppState>) {
    let opt_idx = state.list_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        Some(idx) if idx < data.len() => {
            data.remove(idx);
            state.select(Some(idx.min(data.len().saturating_sub(1))));
            (None, None)
        }
        _ => (None, None),
    }
}

pub fn folder_quit() -> (Option<NormalFolderMessage>, Option<AppState>) {
    (None, Some(AppState::Quit(Box::default())))
}

pub fn folder_item(
    state: &mut FolderNormalState,
    idx: usize,
) -> (Option<NormalFolderMessage>, Option<AppState>) {
    state.select(Some(idx));
    (None, None)
}

pub fn folder_to_dir(
    state: &mut FolderNormalState,
    data: &LinkDirSet,
    idx: usize,
) -> (Option<NormalFolderMessage>, Option<AppState>) {
    if idx < data.len() {
        (
            None,
            Some(AppState::Normal(Box::new(NormalPart::Link(
                LinkNormalState::new(state.list_state().selected().unwrap()),
            )))),
        )
    } else {
        (None, None)
    }
}

pub fn link_back(
    state: &mut LinkNormalState,
    _data: &LinkDir,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    let dir_idx = state.folder_list_state().selected().unwrap_or(0);
    (
        None,
        Some(AppState::Normal(Box::new(NormalPart::Folder(
            FolderNormalState::with_selected(Some(dir_idx)),
        )))),
    )
}

pub fn link_select(
    state: &mut LinkNormalState,
    data: &LinkDir,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        None => {
            state.select(Some(0));
            (None, None)
        }
        Some(idx) if idx < data.len() => (Some(NormalLinkMessage::ToLink(idx)), None),
        Some(_) => {
            state.select(Some(data.len() - 1));
            (None, None)
        }
    }
}

pub fn link_move_up(
    state: &mut LinkNormalState,
    data: &LinkDir,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        None => (Some(NormalLinkMessage::Item(0)), None),
        Some(0) => (None, None),
        Some(idx) => (Some(NormalLinkMessage::Item(idx - 1)), None),
    }
}

pub fn link_move_down(
    state: &mut LinkNormalState,
    data: &LinkDir,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        None => (Some(NormalLinkMessage::Item(0)), None),
        Some(idx) => (Some(NormalLinkMessage::Item(idx + 1)), None),
    }
}

pub fn link_switch_up(
    state: &mut LinkNormalState,
    data: &mut LinkDir,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        None => (Some(NormalLinkMessage::Item(0)), None),
        Some(0) => (None, None),
        Some(idx) if idx < data.len() => {
            data.swap(idx, idx - 1);
            (None, None)
        }
        _ => (None, None),
    }
}

pub fn link_switch_down(
    state: &mut LinkNormalState,
    data: &mut LinkDir,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        None => (Some(NormalLinkMessage::Item(0)), None),
        Some(idx) if idx + 1 < data.len() => {
            data.swap(idx, idx + 1);
            (None, None)
        }
        _ => (None, None),
    }
}

pub fn link_append(
    state: &mut LinkNormalState,
    _data: &LinkDir,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    (
        None,
        Some(AppState::Edit(Box::new(EditPart::Link(
            LinkEditState::new(state.folder_list_state().selected().unwrap(), None),
        )))),
    )
}

pub fn link_rename(
    state: &mut LinkNormalState,
    data: &LinkDir,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        Some(idx) if idx < data.len() => (
            None,
            Some(AppState::Edit(Box::new(EditPart::Link(
                LinkEditState::new(state.folder_list_state().selected().unwrap(), Some(idx))
                    .with_value(data[idx].identifier(), data[idx].path().as_os_str()),
            )))),
        ),
        _ => (None, None),
    }
}

pub fn link_remove(
    state: &mut LinkNormalState,
    data: &mut LinkDir,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    let opt_idx = state.table_state().selected();
    if data.is_empty() {
        return (None, None);
    }
    match opt_idx {
        Some(idx) if idx < data.len() => {
            data.remove(idx);
            state.select(Some(idx.min(data.len().saturating_sub(1))));
            (None, None)
        }
        _ => (None, None),
    }
}

pub fn link_quit() -> (Option<NormalLinkMessage>, Option<AppState>) {
    (None, Some(AppState::Quit(Box::default())))
}

pub fn link_item(
    state: &mut LinkNormalState,
    idx: usize,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    state.select(Some(idx));
    (None, None)
}

pub fn link_to_link(
    _state: &mut LinkNormalState,
    data: &LinkDir,
    idx: usize,
) -> (Option<NormalLinkMessage>, Option<AppState>) {
    if idx < data.len() {
        (
            None,
            Some(AppState::Quit(Box::new(DataTransfer::with_link(
                data[idx].clone(),
            )))),
        )
    } else {
        (None, None)
    }
}
