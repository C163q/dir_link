use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use ratatui::{Terminal, prelude::Backend};

use crate::{
    data::{dirset::LinkDirSet, link::Link},
    ui::App,
};

pub mod data;
pub mod debug;
pub mod ui;

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

#[derive(Debug)]
pub struct Config {
    pub path: Option<PathBuf>,
    pub save: bool,
}

// 临时函数
pub fn output_result(bytes: &[u8], path: &PathBuf) -> io::Result<()> {
    fs::write(path, bytes)
}

pub fn get_data(data_path: &Path) -> io::Result<LinkDirSet> {
    if !data_path.is_file() {
        let mut file = BufWriter::new(File::create(data_path)?);
        let data = LinkDirSet::new();
        file.write_all(serde_json::to_vec(&data)?.as_slice())?;
        Ok(data)
    } else {
        let vec = fs::read(data_path)?;
        let data = serde_json::from_slice(&vec);
        data.map_err(|err| err.into())
    }
}

pub fn run_app<B: Backend>(
    data_path: &Path,
    terminal: Terminal<B>,
    mut config: Config,
) -> io::Result<Option<Link>> {
    // TODO: warn if data file is corrupted
    let (data, success) = match get_data(data_path) {
        Ok(data) => (data, Ok(())),
        Err(err) => (LinkDirSet::new(), Err(err)),
    };

    let path = config.path.take();
    let data_transfer = DataTransfer {
        config: Some(config),
        link: None,
    };

    let data = App::new(data).run(terminal, success, data_transfer)?;

    if data.0.config.as_ref().unwrap().save {
        let mut file = BufWriter::new(File::create(data_path)?);
        file.write_all(serde_json::to_vec(&data.1)?.as_slice())?;
    }

    if let Some(link) = data.0.link() {
        path.map_or_else(
            || Ok(Some(link.clone())),
            |path| {
                output_result(link.path().as_os_str().as_encoded_bytes(), &path)?;
                Ok(None)
            },
        )
    } else {
        Ok(None)
    }
}
