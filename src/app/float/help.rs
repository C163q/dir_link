use std::ops::Deref;

use crate::app::{float::FloatState, message::WarningMessage};

#[derive(Debug)]
pub struct HelpEntry {
    key: String,
    operation: String,
}

impl HelpEntry {
    pub fn new<K, O>(key: K, operation: O) -> Self
    where
        K: Into<String>,
        O: Into<String>,
    {
        HelpEntry {
            key: key.into(),
            operation: operation.into(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.operation
    }

    pub fn get_pair(&self) -> (&str, &str) {
        (&self.key, &self.operation)
    }
}

#[derive(Debug)]
pub struct HelpState {
    entries: Vec<HelpEntry>,
}

impl Default for HelpState {
    fn default() -> Self {
        Self::new()
    }
}

impl HelpState {
    pub fn new() -> Self {
        HelpState {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: HelpEntry) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[HelpEntry] {
        &self.entries
    }
}

impl Deref for HelpState {
    type Target = [HelpEntry];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl Extend<HelpEntry> for HelpState {
    fn extend<I: IntoIterator<Item = HelpEntry>>(&mut self, iter: I) {
        self.entries.extend(iter);
    }
}

impl FloatState for HelpState {
    type Message = WarningMessage;
}
