use std::fmt::Display;

use super::dir::LinkDir;
use super::link::Link;

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    message: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvaildIdentifier,
    InvaildPath,
    DuplicatedLinkIdentifier(Link),
    DuplicatedLinkDirIdentifier(LinkDir),
    DuplicatedIdentifier,
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
            ErrorKind::InvaildIdentifier => "invaild identifier",
            ErrorKind::InvaildPath => "invaild path",
            ErrorKind::DuplicatedLinkIdentifier(_) => "duplicated link identifier",
            ErrorKind::DuplicatedLinkDirIdentifier(_) => "duplicated link dir identifier",
            ErrorKind::DuplicatedIdentifier => "duplicated identifier",
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
