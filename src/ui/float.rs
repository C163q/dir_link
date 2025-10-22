use std::{array, iter::Flatten};

use crate::{
    data::{dir::LinkDir, dirset::LinkDirSet},
    ui::state::{
        FolderNormalState, LinkNormalState,
        confirm::{
            ConfirmChoice, FolderDeleteConfirmState, FolderSaveConfirmState,
            LinkDeleteConfirmState, LinkSaveConfirmState,
        },
        edit::{FolderEditState, LinkEditState},
        warning::{CorruptDataWarningState, WarningState},
    },
};

pub type FolderDeleteConfirmCallbackType =
    Box<dyn FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet)>;
pub type LinkDeleteConfirmCallbackType =
    Box<dyn FnOnce(ConfirmChoice, &mut LinkNormalState, &mut LinkDir)>;
pub type FolderSaveConfirmCallbackType = Box<dyn FnOnce()>;

#[derive(Debug)]
pub enum Float {
    LinkEdit(LinkEditState),
    FolderEdit(FolderEditState),
    FolderDeleteConfirm(FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>),
    LinkDeleteConfirm(LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>),
    Warning(WarningState),
    FolderSaveConfirm(FolderSaveConfirmState),
    LinkSaveConfirm(LinkSaveConfirmState),
    CorruptDataWarning(CorruptDataWarningState),
}

#[derive(Debug)]
pub struct FloatActionResult {
    pub primary: Option<Float>,
    pub new: Option<Float>,
}

impl Default for FloatActionResult {
    fn default() -> Self {
        Self::new()
    }
}

impl FloatActionResult {
    pub fn new() -> Self {
        Self {
            primary: None,
            new: None,
        }
    }

    pub fn with_primary(mut self, float: Float) -> Self {
        self.primary = Some(float);
        self
    }

    pub fn with_new(mut self, float: Float) -> Self {
        self.new = Some(float);
        self
    }

    pub fn with_optional_primary(mut self, float: Option<Float>) -> Self {
        self.primary = float;
        self
    }

    pub fn with_optional_new(mut self, float: Option<Float>) -> Self {
        self.new = float;
        self
    }
}

impl IntoIterator for FloatActionResult {
    type Item = Float;
    type IntoIter = Flatten<array::IntoIter<Option<Float>, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        [self.primary, self.new].into_iter().flatten()
    }
}
