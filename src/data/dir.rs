use std::ops::DerefMut;
use std::path::Path;
use std::{collections::HashSet, ops::Deref};

use serde::de::{self, Visitor};
use serde::ser::SerializeStruct;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

use super::err::{Error, ErrorKind};
use super::link::Link;

#[derive(Debug, PartialEq, Eq)]
pub struct LinkDir {
    // 此处不使用HashMap是因为需要保持插入顺序
    map: Vec<Link>,
    set: HashSet<String>,
    identifier: String,
}

impl LinkDir {
    pub fn builder(identifier: &str) -> Result<Self, Error> {
        if identifier.is_empty() {
            return Err(Error::new(
                ErrorKind::InvaildIdentifier,
                "Link name is empty",
            ));
        }
        Ok(Self {
            map: Vec::new(),
            set: HashSet::new(),
            identifier: String::from(identifier),
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

    pub fn set(&self) -> &HashSet<String> {
        &self.set
    }

    pub fn map(&self) -> &Vec<Link> {
        &self.map
    }

    pub fn push(&mut self, link: Link) -> Result<(), Error> {
        if self.set.contains(link.identifier()) {
            return Err(Error::new(
                ErrorKind::DuplicatedLinkIdentifier(link),
                "Same link name already exists",
            ));
        }
        self.set.insert(link.identifier().to_string());
        self.map.push(link);
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Link {
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
                "Same link name already exists",
            ));
        }
        let link = &mut self.map[idx];
        self.set.remove(link.identifier());
        link.set_identifier(identifier)?;
        self.set.insert(identifier.to_string());
        Ok(())
    }

    pub fn relink(&mut self, idx: usize, path: &Path) -> Result<(), Error> {
        self.map[idx].change_path(path)
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.map.swap(a, b);
    }
}

impl Deref for LinkDir {
    type Target = [Link];
    fn deref(&self) -> &Self::Target {
        self.map.as_slice()
    }
}

impl DerefMut for LinkDir {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.map.as_mut_slice()
    }
}

impl Serialize for LinkDir {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("LinkDir", 2)?;
        state.serialize_field("identifier", self.identifier())?;
        state.serialize_field("links", self.map())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for LinkDir {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Identifier,
            Links,
        }

        struct LinkDirVisitor;

        impl<'de> Visitor<'de> for LinkDirVisitor {
            type Value = LinkDir;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct LinkDir")
            }

            fn visit_map<V: serde::de::MapAccess<'de>>(
                self,
                mut map: V,
            ) -> Result<Self::Value, V::Error> {
                let mut identifier: Option<String> = None;
                let mut links: Option<Vec<Link>> = None;

                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::Identifier => {
                            if identifier.is_some() {
                                return Err(de::Error::duplicate_field("identifier"));
                            }
                            identifier = Some(map.next_value()?);
                        }
                        Field::Links => {
                            if links.is_some() {
                                return Err(de::Error::duplicate_field("links"));
                            }
                            links = Some(map.next_value()?);
                        }
                    }
                }

                let identifier =
                    identifier.ok_or_else(|| de::Error::missing_field("identifier"))?;
                let links = links.ok_or_else(|| de::Error::missing_field("links"))?;

                let mut link_dir = LinkDir::builder(&identifier).map_err(de::Error::custom)?;

                for link in links {
                    link_dir.push(link).map_err(de::Error::custom)?;
                }

                Ok(link_dir)
            }
        }

        const FIELDS: &[&str] = &["identifier", "links"];
        deserializer.deserialize_struct("LinkDir", FIELDS, LinkDirVisitor)
    }
}
