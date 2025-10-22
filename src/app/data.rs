use std::{io, path::PathBuf};

use crate::data::link::Link;

pub struct AppData {
    pub cursor: Option<(u16, u16)>,
    // TODO: handle failure
    pub success: io::Result<()>,
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
        }
    }

    pub fn with_link(link: Link) -> Self {
        Self {
            link: Some(link),
            config: None,
        }
    }

    pub fn with_config(path: PathBuf) -> Self {
        Self {
            link: None,
            config: Some(Config {
                path: Some(path),
                save: true,
            }),
        }
    }

    pub fn link(&self) -> Option<&Link> {
        self.link.as_ref()
    }

    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }
}
