use core::fmt;
use std::fmt::{Debug, Formatter};

use crate::{
    data::{dir::LinkDir, dirset::LinkDirSet},
    ui::{
        float::Float, state::{
            edit::{FolderEditState, LinkEditState}, AppState, FloatState, FolderNormalState, LinkNormalState, NormalState
        }, App
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfirmChoice {
    Yes,
    #[default]
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfirmCancelChoice {
    Yes,
    No,
    #[default]
    Cancel,
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

#[derive(Debug)]
pub struct FolderSaveConfirmState {
    choice: ConfirmChoice,
    last_state: FolderEditState,
    select: Option<usize>,
}

impl FloatState for FolderSaveConfirmState {}

impl FolderSaveConfirmState {
    pub fn new(state: FolderEditState, select: Option<usize>) -> Self {
        Self {
            choice: ConfirmChoice::No,
            last_state: state,
            select,
        }
    }

    pub fn with_choice(mut self, choice: ConfirmChoice) -> Self {
        self.choice = choice;
        self
    }

    pub fn choice(&self) -> ConfirmChoice {
        self.choice
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

    pub fn call(self, app: &mut App) -> Option<Float> {
        if self.choice == ConfirmChoice::No {
            return Some(Float::FolderEdit(self.last_state));
        }
        app.state = AppState::Normal(Box::new(NormalState::Folder(
            FolderNormalState::with_selected(self.select),
        )));
        None
    }

    pub fn last_state(&self) -> &FolderEditState {
        &self.last_state
    }

    pub fn last_state_mut(&mut self) -> &mut FolderEditState {
        &mut self.last_state
    }

    pub fn select(&self) -> Option<usize> {
        self.select
    }
}

#[derive(Debug)]
pub struct LinkSaveConfirmState {
    choice: ConfirmChoice,
    last_state: LinkEditState,
    select: Option<usize>,
}

impl FloatState for LinkSaveConfirmState {}

impl LinkSaveConfirmState {
    pub fn new(state: LinkEditState, select: Option<usize>) -> Self {
        Self {
            choice: ConfirmChoice::No,
            last_state: state,
            select,
        }
    }

    pub fn with_choice(mut self, choice: ConfirmChoice) -> Self {
        self.choice = choice;
        self
    }

    pub fn choice(&self) -> ConfirmChoice {
        self.choice
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

    pub fn last_state(&self) -> &LinkEditState {
        &self.last_state
    }

    pub fn last_state_mut(&mut self) -> &mut LinkEditState {
        &mut self.last_state
    }

    pub fn select(&self) -> Option<usize> {
        self.select
    }

    pub fn call(self, app: &mut App) -> Option<Float> {
        if self.choice == ConfirmChoice::No {
            return Some(Float::LinkEdit(self.last_state));
        }
        app.state = AppState::Normal(Box::new(NormalState::Link(
            LinkNormalState::with_selected(self.last_state.from(), self.select),
        )));
        None
    }
}
