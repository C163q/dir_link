use std::fmt::Display;

use super::link::Link;
use super::dir::LinkDir;

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    message: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvaildIdentifer,
    InvaildPath,
    DuplicatedLinkIdentifer(Link),
    DuplicatedLinkDirIdentifer(LinkDir),
    DuplicatedIdentifer(String),
}

impl Error {
    pub fn new(kind: ErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        self.message
    }
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::InvaildIdentifer => "invaild identifer",
            ErrorKind::InvaildPath => "invaild path",
            ErrorKind::DuplicatedLinkIdentifer(_) => "duplicated link identifer",
            ErrorKind::DuplicatedLinkDirIdentifer(_) => "duplicated link dir identifer",
            ErrorKind::DuplicatedIdentifer(_) => "duplicated identifer",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::error::Error for Error {}
