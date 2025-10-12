use crate::{
    data::dirset::LinkDirSet,
    ui::{
        float::confirm::{ConfirmChoice, FolderDeleteConfirmState},
        state::FolderNormalState,
    },
};

pub mod confirm;

pub type FolderDeleteConfirmCallbackType =
    Box<dyn FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet)>;

#[derive(Debug)]
pub enum Float {
    FolderDeleteConfirm(FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>),
}

pub trait FloatState {}
