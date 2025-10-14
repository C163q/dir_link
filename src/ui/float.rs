use std::{array, iter::Flatten};

use crate::{
    data::{dir::LinkDir, dirset::LinkDirSet},
    ui::{
        float::{confirm::{ConfirmChoice, FolderDeleteConfirmState, LinkDeleteConfirmState}, warning::WarningState},
        state::{FolderNormalState, LinkNormalState},
    },
};

pub mod confirm;
pub mod warning;

pub type FolderDeleteConfirmCallbackType =
    Box<dyn FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet)>;
pub type LinkDeleteConfirmCallbackType =
    Box<dyn FnOnce(ConfirmChoice, &mut LinkNormalState, &mut LinkDir)>;

#[derive(Debug)]
pub enum Float {
    FolderDeleteConfirm(FolderDeleteConfirmState<FolderDeleteConfirmCallbackType>),
    LinkDeleteConfirm(LinkDeleteConfirmState<LinkDeleteConfirmCallbackType>),
    Warning(WarningState),
}

pub trait FloatState {}

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
}

impl IntoIterator for FloatActionResult {
    type Item = Float;
    type IntoIter = Flatten<array::IntoIter<Option<Float>, 2>>;

    fn into_iter(self) -> Self::IntoIter {
        [self.primary, self.new].into_iter().flatten()
    }
}
