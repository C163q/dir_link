use core::fmt;
use std::fmt::{Debug, Formatter};

use crate::{
    data::{dir::LinkDir, dirset::LinkDirSet},
    ui::state::{FloatState, FolderNormalState, LinkNormalState},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfirmChoice {
    Yes,
    #[default]
    No,
}

// trait FolderDeleteConfirmCallback = FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet);

pub struct FolderDeleteConfirmState<F>
where
    F: FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet),
{
    choice: ConfirmChoice,
    callback: F,
}

impl<F> Debug for FolderDeleteConfirmState<F>
where
    F: FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet),
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("FolderDeleteConfirmState")
            .field("choice", &self.choice)
            .field("callback", &"FnOnce(ConfirmChoice)")
            .finish()
    }
}

impl<F> FloatState for FolderDeleteConfirmState<F> where
    F: FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet)
{
}

impl<F> FolderDeleteConfirmState<F>
where
    F: FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet),
{
    pub fn new(callback: F) -> Self {
        Self {
            choice: ConfirmChoice::No,
            callback,
        }
    }

    pub fn with_choice(mut self, choice: ConfirmChoice) -> Self {
        self.choice = choice;
        self
    }

    pub fn choice(&self) -> ConfirmChoice {
        self.choice
    }

    pub fn call(self, state: &mut FolderNormalState, data: &mut LinkDirSet) {
        let function = self.callback;
        function(self.choice, state, data);
    }

    pub fn switch_chioce(&mut self) {
        self.choice = match self.choice {
            ConfirmChoice::Yes => ConfirmChoice::No,
            ConfirmChoice::No => ConfirmChoice::Yes,
        }
    }

    pub fn change_choice(&mut self, choice: ConfirmChoice) {
        self.choice = choice;
    }
}

pub struct LinkDeleteConfirmState<F>
where
    F: FnOnce(ConfirmChoice, &mut LinkNormalState, &mut LinkDir),
{
    choice: ConfirmChoice,
    callback: F,
    dir_idx: usize,
}

impl<F> Debug for LinkDeleteConfirmState<F>
where
    F: FnOnce(ConfirmChoice, &mut LinkNormalState, &mut LinkDir),
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("LinkDeleteConfirmState")
            .field("choice", &self.choice)
            .field("callback", &"FnOnce(ConfirmChoice)")
            .finish()
    }
}

impl<F> FloatState for LinkDeleteConfirmState<F> where
    F: FnOnce(ConfirmChoice, &mut LinkNormalState, &mut LinkDir)
{
}

impl<F> LinkDeleteConfirmState<F>
where
    F: FnOnce(ConfirmChoice, &mut LinkNormalState, &mut LinkDir),
{
    pub fn new(callback: F, dir_idx: usize) -> Self {
        Self {
            choice: ConfirmChoice::No,
            callback,
            dir_idx,
        }
    }

    pub fn with_choice(mut self, choice: ConfirmChoice) -> Self {
        self.choice = choice;
        self
    }

    pub fn choice(&self) -> ConfirmChoice {
        self.choice
    }

    pub fn call(self, state: &mut LinkNormalState, data: &mut LinkDir) {
        let function = self.callback;
        function(self.choice, state, data);
    }

    pub fn switch_chioce(&mut self) {
        self.choice = match self.choice {
            ConfirmChoice::Yes => ConfirmChoice::No,
            ConfirmChoice::No => ConfirmChoice::Yes,
        }
    }

    pub fn change_choice(&mut self, choice: ConfirmChoice) {
        self.choice = choice;
    }

    pub fn dir_idx(&self) -> usize {
        self.dir_idx
    }
}
