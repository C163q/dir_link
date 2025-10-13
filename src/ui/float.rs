use crate::{
    data::{dir::LinkDir, dirset::LinkDirSet},
    ui::{
        float::confirm::{ConfirmChoice, FolderDeleteConfirmState, LinkDeleteConfirmState},
        state::{FolderNormalState, LinkNormalState},
    },
};

pub mod confirm;

pub type FolderDeleteConfirmCallbackType =
    Box<dyn FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet)>;
pub type LinkDeleteConfirmCallbackType =
    Box<dyn FnOnce(ConfirmChoice, &mut LinkNormalState, &mut LinkDir)>;

#[derive(Debug)]
pub enum Float {
    FolderDeleteConfirm(FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>),
    LinkDeleteConfirm(LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>),
}

pub trait FloatState {}
