use std::{io, path::PathBuf};

use crate::data::{dirset::LinkDirSet, link::Link};

pub struct RuntimeError {
    // Some if fails to read
    pub read_data: Option<io::Error>,
    // Some if fails to save
    pub save: Option<io::Error>,
    // Some if fails to write link data
    pub write_link: Option<io::Error>,
}

impl Default for RuntimeError {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeError {
    pub fn new() -> Self {
        Self {
            read_data: None,
            save: None,
            write_link: None,
        }
    }

    pub fn with_read_data(mut self, err: io::Error) -> Self {
        self.read_data = Some(err);
        self
    }

    pub fn with_save(mut self, err: io::Error) -> Self {
        self.save = Some(err);
        self
    }

    pub fn with_write_link(mut self, err: io::Error) -> Self {
        self.write_link = Some(err);
        self
    }
}

pub struct AppData {
    pub cursor: Option<(u16, u16)>,
    // TODO: handle failure
    pub runtime_error: RuntimeError,
}

pub struct AppOption {
    pub save: bool,
}

#[derive(Debug)]
pub struct Config {
    pub path: Option<PathBuf>,
    pub save: bool,
}

#[derive(Debug)]
pub struct DataTransfer {
    pub link: Option<Link>,
    pub config: Option<Config>,
    pub data: Option<LinkDirSet>,
}

impl Default for DataTransfer {
    fn default() -> Self {
        Self::new()
    }
}

impl DataTransfer {
    pub fn new() -> Self {
        Self {
            link: None,
            config: None,
            data: None,
        }
    }

    pub fn with_link(link: Link) -> Self {
        Self {
            link: Some(link),
            config: None,
            data: None,
        }
    }

    pub fn with_path(path: PathBuf) -> Self {
        Self {
            link: None,
            config: Some(Config {
                path: Some(path),
                save: true,
            }),
            data: None,
        }
    }

    pub fn link(&self) -> Option<&Link> {
        self.link.as_ref()
    }

    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }
}
