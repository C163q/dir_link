use std::ops::DerefMut;
use std::{collections::HashSet, ops::Deref};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::dir::LinkDir;
use super::err::{Error, ErrorKind};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LinkDirSet {
    map: Vec<LinkDir>,
    set: HashSet<String>,
}

impl LinkDirSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn map(&self) -> &Vec<LinkDir> {
        &self.map
    }

    pub fn set(&self) -> &HashSet<String> {
        &self.set
    }

    pub fn push(&mut self, dir: LinkDir) -> Result<(), Error> {
        if self.set.contains(dir.identifier()) {
            return Err(Error::new(
                ErrorKind::DuplicatedLinkDirIdentifier(dir),
                "Directory name already exists",
            ));
        }
        self.set.insert(dir.identifier().to_string());
        self.map.push(dir);
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> LinkDir {
        if index >= self.map.len() {
            panic!("Index out of bounds");
        }

        let item = self.map.remove(index);
        self.set.remove(item.identifier());
        item
    }

    pub fn rename(&mut self, idx: usize, identifier: &str) -> Result<(), Error> {
        if self.set.contains(identifier) {
            return Err(Error::new(
                ErrorKind::DuplicatedIdentifier,
                "Directory name already exists",
            ));
        }
        let dir = &mut self.map[idx];
        self.set.remove(dir.identifier());
        dir.set_identifier(identifier)?;
        self.set.insert(identifier.to_string());
        Ok(())
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.map.swap(a, b);
    }
}

impl Deref for LinkDirSet {
    type Target = [LinkDir];
    fn deref(&self) -> &Self::Target {
        self.map.as_slice()
    }
}

impl DerefMut for LinkDirSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.map.as_mut_slice()
    }
}

impl Serialize for LinkDirSet {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.map.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LinkDirSet {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let map = <Vec<LinkDir> as Deserialize<'de>>::deserialize(deserializer)?;
        let mut ret = Self::new();
        for link_dir in map {
            ret.push(link_dir).map_err(serde::de::Error::custom)?;
        }
        Ok(ret)
    }
}
