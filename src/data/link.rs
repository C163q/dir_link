use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::err::{Error, ErrorKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Link {
    identifier: String,
    path: PathBuf,
}

impl Link {
    pub fn builder(identifier: &str, path: &Path) -> Result<Self, Error> {
        if identifier.is_empty() {
            return Err(Error::new(
                ErrorKind::InvaildIdentifer,
                "identifer is empty",
            ));
        }
        if path.as_os_str().is_empty() {
            return Err(Error::new(ErrorKind::InvaildPath, "path is empty"));
        }
        Ok(Self {
            identifier: identifier.to_string(),
            path: path.to_path_buf(),
        })
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn change_identifer(&mut self, identifier: &str) -> Result<(), Error> {
        if identifier.is_empty() {
            return Err(Error::new(
                ErrorKind::InvaildIdentifer,
                "identifer is empty",
            ));
        }
        self.identifier = identifier.to_string();
        Ok(())
    }

    pub fn change_path(&mut self, path: &Path) -> Result<(), Error> {
        if path.as_os_str().is_empty() {
            return Err(Error::new(ErrorKind::InvaildPath, "path is empty"));
        }
        self.path = path.to_path_buf();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuitData {
    link: Option<Link>,
}

impl Default for QuitData {
    fn default() -> Self {
        Self::new()
    }
}

impl QuitData {
    pub fn new() -> Self {
        Self { link: None }
    }

    pub fn with_link(link: Link) -> Self {
        Self { link: Some(link) }
    }

    pub fn link(&self) -> Option<&Link> {
        self.link.as_ref()
    }
}
