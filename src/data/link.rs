use std::{
    env, io,
    path::{self, Path, PathBuf},
};

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
                ErrorKind::InvaildIdentifier,
                "Link name is empty",
            ));
        }
        if !path.is_absolute() {
            return Err(Error::new(ErrorKind::InvaildPath, "path is not absolute"));
        }
        Ok(Self {
            identifier: identifier.to_string(),
            path: path.to_path_buf(),
        })
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn set_identifier(&mut self, identifier: &str) -> Result<(), Error> {
        if identifier.is_empty() {
            return Err(Error::new(
                ErrorKind::InvaildIdentifier,
                "Link name is empty",
            ));
        }
        self.identifier = identifier.to_string();
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn change_identifer(&mut self, identifier: &str) -> Result<(), Error> {
        if identifier.is_empty() {
            return Err(Error::new(
                ErrorKind::InvaildIdentifier,
                "Link name is empty",
            ));
        }
        self.identifier = identifier.to_string();
        Ok(())
    }

    pub fn change_path(&mut self, path: &Path) -> Result<(), Error> {
        if !path.is_absolute() {
            return Err(Error::new(ErrorKind::InvaildPath, "path is not absolute"));
        }
        self.path = path.to_path_buf();
        Ok(())
    }
}

pub fn get_vaild_path(input: &str) -> io::Result<PathBuf> {
    if input.is_empty() {
        Ok(path::absolute(env::current_dir()?)?)
    } else {
        Ok(path::absolute(input)?)
    }
}
